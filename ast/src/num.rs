use chumsky::Parser as _;
use chumsky::primitive::one_of;
use chumsky::text::digits;
use clash_chumka::{Error, Parsable, Parser};
use derive_more::Display;

/// A numeric expression
///
/// # TODO
///
/// Support floats, rationals, and/or decimals
#[derive(Debug, Display)]
pub struct Number(i64);

impl<S> Parsable<S> for Number {
    fn parser<'i>() -> impl Parser<'i, Self, S> {
        lexeme().to_slice().try_map(|s: &str, span| {
            s.parse()
                .map(Number)
                .map_err(|e| Error::custom(span, format!("invalid number {s:?}: {e}")))
        })
    }
}

/// Parse the textual form of a [Number] without interpreting its value
pub(crate) fn lexeme<'i, S>() -> impl Parser<'i, (), S> {
    one_of(['-', '+'])
        .or_not()
        .then(digits(10))
        .ignored()
}
