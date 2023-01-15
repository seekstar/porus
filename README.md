# porus

[![Test](https://github.com/bhuztez/porus/actions/workflows/test.yml/badge.svg)](https://github.com/bhuztez/porus/actions/workflows/test.yml)
[![Coverage Status](https://coveralls.io/repos/github/bhuztez/porus/badge.svg?branch=master)](https://coveralls.io/github/bhuztez/porus?branch=master)

porus is Rust library designed for competitive programming, especially
for being used by solutions submitted to online judges. So that you
don't have to copy and paste library code into your solution.


## Requirements

* Rust nightly (components: rustc-dev, rust-src, llvm-tools)
* xargo
* Python 3.7+

## Quick start

```console
$ git clone git://github.com/bhuztez/porus.git
$ cd porus
$ pip3 install --user -r requirements.txt
$ ./c.py submit solutions/judge.u-aizu.ac.jp/ITP1/ITP1_1_A.rs
Memory: 2068, Time: 0, Length: 4344
$
```

## Examples

* [AOJ](AOJ.md) ([AIZU ONLINE JUDGE](http://judge.u-aizu.ac.jp/onlinejudge/))
* [LC](LC.md) ([LeetCode](https://leetcode.com/))
