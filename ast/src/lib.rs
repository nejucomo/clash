//! The `clash` Abstract Syntax Tree types

use derive_more::Display;

#[cfg(test)]
mod tests;

pub mod errors;

/// An expression
#[derive(Debug, Display)]
pub enum Expr {
    /// An int expression
    Int(i64),
}
