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
        let not_symbol_suffix = || crate::symbol::subsequent().not();
        let num = maps_parser(Num).then_ignore(not_symbol_suffix());

        // Input that looks like a number is never a symbol, even when the number is invalid (e.g. it overflows).
        let num_lexeme = crate::num::lexeme().then_ignore(not_symbol_suffix());
        let sym = num_lexeme.not().ignore_then(maps_parser(Sym));

        num.or(sym).or(maps_parser(Str))
    }
}
