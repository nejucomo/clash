use derive_more::{Display, From};

/// An expression
#[derive(Debug, Display, From)]
pub enum Expr {
    /// An int literal
    Int(i64),
    /// A string literal
    Str(String),
}
