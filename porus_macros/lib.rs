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
extern crate rustc_span;

use proc_macro::TokenStream;

mod common;
use common::{parse_args, parse_printf, parse_sargs, parse_scanf};

#[proc_macro]
pub fn printf(stream: TokenStream) -> TokenStream {
    let (fmt, args) = parse_args(stream.into()).unwrap();
    let (format, arguments) = parse_printf(fmt, args);

    (quote! {
        {
            #format
            #[allow(unused_imports)]
            use porus::fmt::{Bytes, FLOAT_ESCAPE_PREFIX, FLOAT_ESCAPE_SUFFIX};
            use u64 as u;
            use i64 as i;
            unsafe { porus::libc::printf(#arguments); }
        }
    })
    .into()
}

#[proc_macro]
pub fn scanf(stream: TokenStream) -> TokenStream {
    let (fmt, args) = parse_args(stream.into()).unwrap();
    let (format, arguments, count) = parse_scanf(fmt, args);

    (quote! {
        {
            #format
            #[allow(unused_imports)]
            use porus::fmt::BytesMut;
            use u64 as u;
            use i64 as i;
            unsafe { porus::libc::scanf(#arguments) == #count }
        }
    })
    .into()
}

#[proc_macro]
pub fn sscanf(stream: TokenStream) -> TokenStream {
    let (s, fmt, args) = parse_sargs(stream.into()).unwrap();
    let (format, arguments, count) = parse_scanf(fmt, args);

    (quote! {
        {
            let scanf_str = #s;
            #format
            #[allow(unused_imports)]
            use porus::fmt::{Bytes, BytesMut};
            use u64 as u;
            use i64 as i;
            unsafe { porus::libc::sscanf(Bytes::as_ptr(scanf_str), #arguments) == #count }
        }
    })
    .into()
}
