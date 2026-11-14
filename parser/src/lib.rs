//! The parser for `clash`

#![allow(missing_docs)] // expedient

#[cfg(test)]
mod tests;

use chumsky::Parser;
use clash_ast::Expr;
use derive_more::From;
use thiserror::Error;

#[derive(Debug, Error, From)]
#[error("parse errors: {cases:#?}")]
pub struct ParseError {
    cases: Vec<chumsky::error::EmptyErr>,
}

pub fn parse(source: &str) -> Result<Expr, ParseError> {
    let x = expr().parse(source).into_result()?;
    Ok(x)
}

fn expr<'a>() -> impl Parser<'a, &'a str, Expr> {
    use chumsky::number::{format, number};

    number::<{ format::RUST_LITERAL }, _, _, _>().map(Expr::Int)
}
