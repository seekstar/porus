#![feature(rustc_private)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::style)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::correctness)]
#![deny(clippy::restriction)]
#![deny(stable_features)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::indexing_slicing)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate rustc_parse_format;

use proc_macro::TokenStream;

mod common;
mod printf;

#[proc_macro]
pub fn printf(stream: TokenStream) -> TokenStream {
    printf::printf(stream.into()).into()
}
