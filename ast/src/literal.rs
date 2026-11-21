//! Literal expressions

use std::rc::Rc;

mod num;
mod symbol;
mod text;

pub use self::num::Number;
pub use self::symbol::Symbol;
pub use self::text::Text;

use chumsky::Parser as _;
use clash_chumka::{Parsable, Parser, maps_parser};
use clash_interning::Interns;
use derive_more::{Display, From};

/// An expression
#[derive(Debug, Display, From)]
pub enum Literal {
    /// A symbol
    Sym(Rc<Symbol>),
    /// A number
    Num(Number),
    /// A string literal
    Text(Rc<Text>),
}

impl<S> Parsable<S> for Literal
where
    S: Interns<Symbol> + Interns<Text>,
{
    fn parser<'i>() -> impl Parser<'i, Self, S> {
        use Literal::*;

        // A number must not run directly into symbol characters, such as `42foo`.
        let not_symbol_suffix = || crate::literal::symbol::subsequent().not();
        let num = maps_parser(Num).then_ignore(not_symbol_suffix());

        // The `Sym` and `Num` grammars are disjoint, so this guard does not change which inputs parse. Rather, it ensures that number-like input reports the `Num` error (e.g. overflow) instead of `Sym` failing on its first digit.
        let num_lexeme = crate::literal::num::lexeme().then_ignore(not_symbol_suffix());
        let sym = num_lexeme.not().ignore_then(maps_parser(Sym));

        num.or(sym).or(maps_parser(Text))
    }
}
