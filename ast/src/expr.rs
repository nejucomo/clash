use std::rc::Rc;

use derive_more::{Display, From};

use crate::StringLiteral;

/// An expression
#[derive(Debug, Display, From)]
pub enum Expr {
    /// A symbol
    Sym(Rc<String>),
    /// An int literal
    Int(i64),
    /// A string literal
    Str(Rc<StringLiteral>),
}
