//! The parser for `clash`

#![allow(missing_docs)] // expedient

mod cherror;
mod clash_parser;
mod error;
mod expr;
mod srctext;

#[cfg(test)]
mod tests;

use chumsky::Parser as _;
use clash_ast::Expr;

pub use self::error::{Error, Result};

pub fn parse(source: &str) -> Result<'_, Expr> {
    let x = expr::expr().parse(source).into_result()?;
    Ok(x)
}
