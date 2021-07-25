#![feature(core_intrinsics)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(is_sorted)]
#![feature(array_methods)]
#![feature(associated_type_bounds)]
#![feature(const_fn_trait_bound)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(const_trait_impl)]
#![feature(iter_intersperse)]
#![feature(maybe_uninit_extra)]
#![feature(rustc_attrs)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(slice_ptr_len)]
#![feature(slice_ptr_get)]
#![feature(ptr_as_uninit)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![cfg_attr(feature = "online-judge", feature(lang_items))]
#![doc(test(attr(feature(proc_macro_hygiene))))]
#![no_std]
#![allow(incomplete_features)]
#![deny(stable_features)]
#![deny(unused_features)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::style)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::correctness)]
#![deny(clippy::restriction)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::as_conversions)]
#![allow(clippy::expect_used)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::blanket_clippy_restriction_lints)]

//! [`porus`](self) is a library for competitive programming. Since
//! most popular online judges accept only a single file within tens
//! of kilobytes, solutions have to be preproccessed before submitting
//! to online judges. Right now, [`porus`](self) piggybacks on
//! [wronganswer](https://github.com/bhuztez/wronganswer) to do the
//! preprocessing. For example, to submit to
//! [AOJ](http://judge.u-aizu.ac.jp/onlinejudge/) the solution to
//! [ITP1_1_A](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A)
//!

//! ```bash
//! $ ./c.py submit solutions/judge.u-aizu.ac.jp/ITP1/ITP1_1_A.rs
//! Memory: 2068, Time: 0, Length: 4344
//! $
//! ```
//!

//! ## Abstract Data Types
//! * [`Pool`](pool)
//! * [`Collection`](collection)
//! * [`List`](list)
//! * [`Stack`](stack)
//! * [`Deque`](deque)
//!

//! ## Data Structures
//!

extern crate alloc;
extern crate porus_macros;

pub mod fmt;
pub mod libc;
pub mod math;

pub mod allocator;
pub mod sys;

pub mod iter;

pub mod capacity;
pub mod collection;
pub mod deque;
pub mod heap;
pub mod list;
pub mod pool;
pub mod set;
pub mod stack;

pub mod chunk;
pub mod string;

pub mod dheap;
pub mod dlist;
pub mod flist;

#[macro_use]
pub mod prelude;

#[allow(clippy::missing_const_for_fn)]
#[cfg(feature = "online-judge")]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[cfg(feature = "online-judge")]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    ::core::intrinsics::abort()
}

#[global_allocator]
static _A: sys::System = sys::System;

#[cfg(feature = "online-judge")]
#[alloc_error_handler]
fn oom(_info: ::core::alloc::Layout) -> ! {
    ::core::intrinsics::abort()
}
