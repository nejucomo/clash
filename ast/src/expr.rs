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

        // A signed number such as `-42` is also a valid symbol, so numbers take precedence. A number immediately followed by symbol characters, such as `-4foo`, is instead a symbol.
        let num = maps_parser(Num).then_ignore(crate::symbol::subsequent().not());

        num.or(maps_parser(Sym)).or(maps_parser(Str))
    }
}
