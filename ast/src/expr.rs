use std::rc::Rc;

use chumsky::Parser as _;
use clash_chumka::{Parsable, Parser, maps_parser};
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
        use Expr::*;

        maps_parser(Sym).or(maps_parser(Num)).or(maps_parser(Str))
    }
}
