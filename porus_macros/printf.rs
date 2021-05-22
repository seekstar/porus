use crate::common::{make_tuple, parse_args};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::ToTokens;
use rustc_parse_format::{Argument, Count, ParseMode, Parser, Piece, Position};
use std::collections::HashMap;
use syn::Expr;

use rustc_span::edition::DEFAULT_EDITION;
use rustc_span::{SessionGlobals, SESSION_GLOBALS};

struct Scoped<'a, T, I: Iterator<Item = T>> {
    it: I,
    session_globals: &'a SessionGlobals
}

impl<'a, T, I: Iterator<Item = T>> Iterator for Scoped<'a, T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        SESSION_GLOBALS.set(self.session_globals, || self.it.next())
    }
}

impl<'a, T, I: Iterator<Item = T>> Scoped<'a, T, I> {
    fn new<'b>(it: I, session_globals: &'b SessionGlobals) -> Scoped<'b, T, I> {
        Scoped {
            it,
            session_globals
        }
    }
}

pub fn printf(tokens: TokenStream) -> TokenStream {
    let (s, mut args) = parse_args(tokens).unwrap();

    let mut named_arguments = HashMap::new();

    let mut sizes = quote! { 1usize };
    let mut format = quote! { [] };
    let mut arguments = quote! {};

    let session_globals = SessionGlobals::new(DEFAULT_EDITION);

    for p in Scoped::new(Parser::new(s.value().as_str(), None, None, false, ParseMode::Format), &session_globals) {
        match p {
            Piece::String(s) => {
                let size = Literal::usize_suffixed(s.len());
                let lit = Literal::byte_string(s.as_ref());
                sizes = quote!( #sizes + #size );
                format = quote!( concat(#format, *#lit) );
            }
            Piece::NextArgument(Argument {
                position: pos,
                format: fmt,
            }) => {
                let arg: Box<dyn ToTokens> = match pos {
                    Position::ArgumentNamed(name) => {
                        let lit = Literal::usize_unsuffixed(match named_arguments.get(&name) {
                            None => {
                                let index = args.len();
                                named_arguments.insert(name, index);
                                let ident = Ident::new(
                                    &SESSION_GLOBALS.set(&session_globals, || name.as_str()),
                                    Span::call_site());
                                args.push(Expr::Verbatim(quote! { #ident }));
                                index
                            }
                            Some(&index) => index,
                        });
                        Box::new(quote! { printf_args.#lit })
                    }
                    Position::ArgumentImplicitlyIs(i) | Position::ArgumentIs(i) => {
                        let lit = Literal::usize_unsuffixed(i);
                        Box::new(quote! { printf_args.#lit })
                    }
                };

                match fmt.ty {
                    "" | "s" => {
                        sizes = quote!( #sizes + 4usize );
                        format = quote!( concat(#format, *b"%.*s") );
                        arguments = quote!( #arguments , Bytes::len(#arg), Bytes::as_ptr(#arg) );
                    }
                    "c" => {
                        sizes = quote!( #sizes + 2usize );
                        format = quote!( concat(#format, *b"%c") );
                        arguments = quote!( #arguments , Into::<isize>::into(#arg) );
                    }
                    "u" | "usize" | "u64" | "u32" | "u16" | "u8" | "i" | "isize" | "i64"
                    | "i32" | "i16" | "i8" => {
                        let lit = Literal::byte_string(fmt.ty[..1].as_ref());
                        let mut pre = String::from("PRI");
                        pre.push_str(&fmt.ty[1..]);
                        pre.make_ascii_uppercase();

                        let prefix = Ident::new(pre.as_str(), Span::call_site());
                        let into_type = Ident::new(fmt.ty, Span::call_site());
                        sizes = quote!( #sizes + porus::fmt::#prefix.len() + 2usize );
                        format = quote!( concat(#format, concat(concat(*b"%", porus::fmt::#prefix), *#lit)) );
                        arguments = quote!( #arguments , Into::<#into_type>::into(#arg) );
                    }
                    "f" => {
                        let prec: String = match fmt.precision {
                            Count::CountIs(n) => format!("{}", n),
                            Count::CountIsName(_name) => panic!("named argument not supported"),
                            Count::CountIsParam(_i) => panic!("param not supported"),
                            Count::CountImplied => {
                                panic!("precision is required by floating number format")
                            }
                        };

                        let size = Literal::usize_suffixed(prec.len());
                        let lit = Literal::byte_string(prec.as_str().as_ref());
                        sizes = quote!(#sizes + #size + #size + 3 + FLOAT_ESCAPE_PREFIX.len() + FLOAT_ESCAPE_SUFFIX.len());
                        format = quote!(
                        concat(#format,
                               concat(
                                   concat(concat(FLOAT_ESCAPE_PREFIX, *#lit), FLOAT_ESCAPE_SUFFIX),
                                   concat(concat(*b"%.", *#lit), *b"f")
                               )
                        ));
                        arguments = quote!( #arguments , #arg );
                    }
                    x => {
                        panic!("unknown format: {}", x);
                    }
                };
            }
        }
    }

    let args_tuple = make_tuple(args);
    quote! {
        {
            let printf_args = #args_tuple ;
            #[allow(unused_imports)]
            use porus::fmt::{concat, Bytes, FLOAT_ESCAPE_PREFIX, FLOAT_ESCAPE_SUFFIX};
            use u64 as u;
            use i64 as i;
            const printf_format : [u8; #sizes ] = concat(#format, [0]);
            unsafe { porus::libc::printf(printf_format.as_slice().as_ptr() #arguments); }
        }
    }
}
