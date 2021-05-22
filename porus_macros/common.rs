use proc_macro2::{Span, TokenStream};
use syn::parse::{ParseStream, Parser, Result};
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
    Parser::parse2(args, tokens)
}

pub fn make_tuple(args: Punctuated<Expr, Comma>) -> Expr {
    Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: Paren(Span::call_site()),
        elems: args,
    })
}
