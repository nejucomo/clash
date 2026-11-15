//! The parser for `clash`

#![feature(btree_set_entry)]
#![allow(missing_docs)] // expedient

mod cherror;
mod clash_parser;
mod error;
mod expr;
mod hasparser;
mod intern;
mod srctext;

#[cfg(test)]
mod tests;

use chumsky::Parser as _;
use chumsky::extra::SimpleState;
use clash_ast::Expr;

pub use self::error::{Error, Result};

pub fn parse(source: &str) -> Result<'_, Expr> {
    let x = expr::expr()
        .parse_with_state(source, &mut SimpleState::default())
        .into_result()?;
    Ok(x)
}
