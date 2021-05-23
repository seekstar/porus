use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::ToTokens;
use rustc_parse_format::{Argument, Count, ParseMode, Parser, Piece, Position};
use rustc_span::edition::DEFAULT_EDITION;
use rustc_span::{SessionGlobals, SESSION_GLOBALS};
use std::collections::HashMap;
use syn::parse::{ParseStream, Parser as SynParser, Result};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren};
use syn::{Expr, ExprTuple, LitStr};

fn args(input: ParseStream) -> Result<(LitStr, Punctuated<Expr, Comma>)> {
    let s: LitStr = input.parse()?;

    if !input.is_empty() {
        let _: Comma = input.parse()?;
    }

    let args = Punctuated::parse_terminated(input)?;

    Ok((s, args))
}

pub fn parse_args(tokens: TokenStream) -> Result<(LitStr, Punctuated<Expr, Comma>)> {
    SynParser::parse2(args, tokens)
}

fn sargs(input: ParseStream) -> Result<(Expr, LitStr, Punctuated<Expr, Comma>)> {
    let s: Expr = input.parse()?;
    let _: Comma = input.parse()?;
    let (fmt, a) = args(input)?;
    Ok((s, fmt, a))
}

pub fn parse_sargs(tokens: TokenStream) -> Result<(Expr, LitStr, Punctuated<Expr, Comma>)> {
    SynParser::parse2(sargs, tokens)
}

fn make_tuple(args: Punctuated<Expr, Comma>) -> Expr {
    Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: Paren(Span::call_site()),
        elems: args,
    })
}

#[allow(clippy::needless_pass_by_value)]
pub fn parse_scanf(
    s: LitStr,
    args: Punctuated<Expr, Comma>,
) -> (TokenStream, TokenStream, TokenStream) {
    let mut sizes = quote! { 1usize };
    let mut format = quote! { [] };
    let mut arguments = quote! {};
    let mut count: i32 = 0;

    for p in Parser::new(s.value().as_str(), None, None, false, ParseMode::Format) {
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
                    Position::ArgumentNamed(_name) => panic!("named argument not supported"),
                    Position::ArgumentImplicitlyIs(i) | Position::ArgumentIs(i) => {
                        let lit = Literal::usize_unsuffixed(i);
                        Box::new(quote! { scanf_args.#lit })
                    }
                };

                match fmt.ty {
                    "n" => {
                        sizes = quote!( #sizes + 2usize );
                        format = quote!( concat(#format, *b"%n") );
                        arguments = quote!( #arguments , Into::<&mut i32>::into(#arg) );
                        count += 1;
                    }
                    "" | "s" => {
                        sizes = quote!( #sizes + 2usize );
                        format = quote!( concat(#format, *b"%s") );
                        arguments = quote!( #arguments , BytesMut::as_mut_ptr(#arg) );
                        count += 1;
                    }
                    "c" => {
                        sizes = quote!( #sizes + 2usize );
                        format = quote!( concat(#format, *b"%c") );
                        arguments = quote!( #arguments , Into::<&mut u8>::into(#arg) );
                        count += 1;
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
                        arguments = quote!( #arguments , Into::<&mut #into_type>::into(#arg) );
                        count += 1;
                    }
                    "f" | "lf" => {
                        sizes = quote!( #sizes + 3usize );
                        format = quote!( concat(#format, *b"%lf") );
                        arguments = quote!( #arguments , Into::<&mut f64>::into(#arg) );
                        count += 1;
                    }
                    x => {
                        panic!("unknown format: {}", x);
                    }
                };
            }
        }
    }

    let count_lit = Literal::i32_suffixed(count);
    let args_tuple = make_tuple(args);

    (
        quote! {
           let scanf_args = #args_tuple;
           use porus::fmt::concat;
           const scanf_format : [u8; #sizes ] = concat(#format, [0]);
        },
        quote! {
            scanf_format.as_slice().as_ptr() #arguments
        },
        quote! {
            #count_lit
        },
    )
}

struct Scoped<'a, T, I: Iterator<Item = T>> {
    it: I,
    session_globals: &'a SessionGlobals,
}

impl<'a, T, I: Iterator<Item = T>> Iterator for Scoped<'a, T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        SESSION_GLOBALS.set(self.session_globals, || self.it.next())
    }
}

impl<'a, T, I: Iterator<Item = T>> Scoped<'a, T, I> {
    fn new(it: I, session_globals: &'a SessionGlobals) -> Scoped<'a, T, I> {
        Scoped {
            it,
            session_globals,
        }
    }
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::needless_pass_by_value)]
pub fn parse_printf(s: LitStr, mut args: Punctuated<Expr, Comma>) -> (TokenStream, TokenStream) {
    let mut named_arguments = HashMap::new();

    let mut sizes = quote! { 1usize };
    let mut format = quote! { [] };
    let mut arguments = quote! {};

    let session_globals = SessionGlobals::new(DEFAULT_EDITION);

    for p in Scoped::new(
        Parser::new(s.value().as_str(), None, None, false, ParseMode::Format),
        &session_globals,
    ) {
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
                                    Span::call_site(),
                                );
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
    (
        quote! {
           let printf_args = #args_tuple;
           use porus::fmt::concat;
           const printf_format : [u8; #sizes ] = concat(#format, [0]);
        },
        quote! {
            printf_format.as_slice().as_ptr() #arguments
        },
    )
}
