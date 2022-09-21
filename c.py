#!/usr/bin/env python3

if __name__ == '__main__':
    import sys
    import os

    try:
        import wronganswer
    except ImportError:
        ROOT=os.path.dirname(os.path.abspath(__file__))
        sys.path.append(os.path.dirname(ROOT)+"/wronganswer")

    from wronganswer.project import main
    main("Porus Project")
    quit()

import os
from subprocess import DEVNULL, run
import json
from functools import wraps

SOLUTION_PATTERN = r'^(?:[^/]+)/(?P<oj>[\w\-.]+)(?:/.*)?/(?P<pid>[A-Za-z0-9_\-]+)\.rs$'

NATIVE = run(["rustc", "-vV"], stdin=DEVNULL, capture_output=True, check=True).stdout.decode().split("host: ", 1)[1].split("\n", 1)[0]
SYSROOT = os.path.expanduser("~/.xargo")

LLVMBC = os.environ.get("LLVMBC", "0").lower() in ("true", "1", "yes")

def features(mode, target):
    if target is None:
        yield "local-judge"
    if mode == 'release':
        yield "online-judge"

def target_dir(mode, target):
    yield 'target'
    if target is not None:
        yield target
    if mode == 'coverage':
        yield from ('cov', 'build', 'debug')
    else:
        yield mode

def libname(path):
    basename = os.path.splitext(os.path.basename(path))[0].split('-', 1)[0]
    if basename.startswith("lib"):
        return basename[3:]
    return basename

def cargo_argv(mode, target):
    if mode == 'release':
        yield 'xargo'
    else:
        yield 'cargo'
    yield from ('rustc', '--lib')
    yield '-v' if VERBOSE else '-q'
    if mode == 'release':
        yield '--release'
        if target is not None:
            yield from ('--target', target)
        else:
            yield from ('--target', NATIVE)
    if mode == 'coverage':
        yield from ('--target-dir', os.path.dirname(os.path.join(ROOTDIR, *target_dir(mode, target))))
    yield from ('--features', ",".join(features(mode, target)))
    yield from ("--message-format", "json")

def release_flags():
    yield from ("-C", "linker-plugin-lto")

def coverage_flags():
    yield from ("-Z", "profile")
    yield from ("-C", "codegen-units=1")
    yield from ("-C", "opt-level=0")
    yield from ("-C", "link-dead-code")
    yield from ("-C", "overflow-checks=off")
    yield from ("-C", "inline-threshold=0")

def rustc_argv_prefix(mode, target, filename, *libs):
    yield from ('rustc', '--edition', '2021')

    if VERBOSE:
        yield '-v'
    if mode == 'debug':
        yield from ('-C', 'debuginfo=2')
    if mode == 'release':
        yield from ("--crate-type", "cdylib")
        if LLVMBC:
            yield from ("--emit", "llvm-bc")
        else:
            yield from ("--emit", "asm")
        yield from ("-C", "llvm-args=-disable-debug-info-print")
        yield from ("-C", "lto=fat")
        yield from ("-C", "opt-level=2")
        yield from ("-C", "panic=abort")
        yield from ("-C", "codegen-units=1")
        if target is not None and target != NATIVE:
            yield from ("--sysroot", SYSROOT)
        else:
            yield from ("--sysroot", os.path.join(SYSROOT, "HOST"))
        if target is not None:
            yield from ('--target', target)
        else:
            yield from ('--target', NATIVE)
    if mode == 'coverage':
        yield from coverage_flags()

    for feature in features(mode, target):
        yield from ('--cfg', f'feature="{feature}"')

    if mode == 'debug':
        yield from ('-C', 'incremental='+os.path.join(ROOTDIR, *target_dir(mode, target), "incremental"))
    yield from ('-L', 'dependency='+os.path.join(ROOTDIR, *target_dir(mode, None), "deps"))

    for lib in libs:
        yield from ('--extern', '{}={}'.format(libname(lib), lib))

    yield from ("-Z", "macro-backtrace")
    yield from ("-Z", "proc-macro-backtrace=yes")

def rustc_argv(mode, target, filename, *libs):
    yield from rustc_argv_prefix(mode, target, filename, *libs)
    yield from ("-o", dest_filename(filename, mode, target))
    yield "-"

def lru1(func):
    last = None

    @wraps(func)
    def wrapper(*args, **kwargs):
        nonlocal last
        key = (args, tuple(kwargs.items()))
        if last is None or last[0] != key:
            last = key, func(*args, **kwargs)
        return last[1]

    return wrapper


@task("Compile porus")
@lru1
def compile_libs(mode='debug', target=None):
    from wronganswer.subprocess import run
    env = os.environ.copy()
    if mode == 'coverage':
        env["CARGO_INCREMENTAL"] = "0"
        env["RUSTFLAGS"] = " ".join(coverage_flags())
    if mode == 'release':
        env["RUSTFLAGS"] = " ".join(release_flags())

    output = run(list(cargo_argv(mode, target)), stdin=DEVNULL, cwd=ROOTDIR, env=env, capture_output=True, check=True).stdout
    packages = [json.loads(line) for line in output.splitlines()]

    return [ package["filenames"][0]
             for package in packages
             if package["reason"] == "compiler-artifact"
             if "porus" in package["target"]["name"]]

# mode=coverage, target=None, run coverage locally
# mode=debug, target=None, to run locally
# mode=release, target=None, to run locally, generate assembly
# mode=release, target=??? to_submit, generate assembly

def get_compile_argv(filename, *libs, mode='debug', target=None):
    env = os.environ.copy()
    dest = dest_filename(filename, mode, target)
    return dest, list(rustc_argv(mode, target, filename, *libs)), env


PRELUDE = b'''#![feature(proc_macro_hygiene)]
#![feature(array_methods)]
#![feature(exclusive_range_pattern)]
#![feature(custom_inner_attributes)]
#![feature(generic_const_exprs)]
#![feature(iter_intersperse)]
#![deny(stable_features)]
#![allow(incomplete_features)]
#![cfg_attr(not(debug_assertions), no_std)]
extern crate porus;
#[porus_macros::transform_forloop]
mod solution {
#[allow(unused_imports)]
use porus::prelude::*;
'''

EPILUDE = b'''}

#[cfg(feature = "online-judge")]
#[export_name = "main"]
pub extern "C" fn porus_start() -> i32 {
  main();
  0
}
'''

def read_source(filename):
    name = os.path.relpath(filename, ROOTDIR)
    if os.sep != '/':
        name = name.replace(os.sep, '/')
    info = get_solution_info(name)
    with open(filename, 'rb') as f:
        source = f.read()
    if info and info[0] == 'leetcode.com':
        return PRELUDE + source + b'''}'''
    else:
        return PRELUDE + source + EPILUDE

def get_submit_env(name, envs):
    return None

def rustc_expand_argv(mode, target, filename, *libs):
    yield from rustc_argv_prefix(mode, target, filename, *libs)
    yield from ("-Z", "unstable-options")
    yield from ("-Z", "unpretty=expanded")
    yield "-"

@command.add_command
def expand(
        filename: Argument(help="path to solution"),
        recompile: Argument("-r", "--recompile", action="store_true", help="force recompile") = False,
        mode: Argument("--mode") = 'debug',
        target = None):
    kwargs = {}
    if mode != 'debug' or target is not None:
        kwargs['mode'] = mode
        kwargs['target'] = target

    libs = compile_libs(**kwargs)
    argv = list(rustc_expand_argv(mode, target, filename, *libs))
    source = read_source(filename)
    run(argv, input=source, check=True)

if LLVMBC:
    _dest_filename = dest_filename

    def dest_filename(filename, mode, target):
        name = _dest_filename(filename, mode, target)
        if mode == 'release':
            name = name[:-2] + ".bc"
        return name
