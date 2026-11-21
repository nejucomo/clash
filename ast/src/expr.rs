use clash_chumka::{Parsable, Parser, maps_parser};
use clash_interning::Interns;
use derive_more::{Display, From};

use crate::literal::{Literal, Symbol, Text};

/// A literal expression
#[derive(Debug, Display, From)]
pub enum Expr {
    /// A literal
    Lit(Literal),
}

impl<S> Parsable<S> for Expr
where
    S: Interns<Symbol> + Interns<Text>,
{
    fn parser<'i>() -> impl Parser<'i, Self, S> {
        maps_parser(Expr::Lit)
    }
}
