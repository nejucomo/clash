use std::rc::Rc;

use chumsky::Parser as _;
use clash_chumka::{Parsable, Parser};
use clash_interning::Interns;
use derive_more::{Display, From};

use crate::{Number, StringLiteral, Symbol};

/// An expression
#[derive(Debug, Display, From)]
pub enum Expr {
    /// A symbol
    Sym(Rc<Symbol>),
    /// A number
    Num(Number),
    /// A string literal
    Str(Rc<StringLiteral>),
}

impl<S> Parsable<S> for Expr
where
    S: Interns<Symbol> + Interns<StringLiteral>,
{
    fn parser<'i>() -> impl Parser<'i, Self, S> {
        Rc::<Symbol>::parser()
            .map(Expr::Sym)
            .or(Number::parser().map(Expr::Num))
            .or(Rc::<StringLiteral>::parser().map(Expr::Str))
    }
}
