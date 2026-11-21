use aliri_braid::{Validator, braid};
use chumsky::Parser as _;
use chumsky::primitive::{any, choice, one_of};
use clash_chumka::{Errors, Parsable, ParsableStateless, Parser};

/// These characters may appear in symbols in any position
pub const SYMBOL_SYMBOLS: &str = "-+_";

/// The sign characters, which may begin a [Number](crate::Number) as well as a [Symbol]
pub const SIGNS: &str = "-+";

/// A symbol
///
/// # Grammar
///
/// A symbol is a non-empty sequence of [SYMBOL_SYMBOLS], alphabetic, and decimal digit characters, where the first character after any leading [SIGNS] must not be a digit. This ensures no symbol is also a valid [Number](crate::Number), such as `-42`.
#[braid(validator)]
pub struct Symbol;

impl<S> Parsable<S> for Symbol {
    fn parser<'i>() -> impl Parser<'i, Self, S> {
        let unsigned = || unsigned_initial().then(subsequent().repeated()).ignored();

        #[allow(unsafe_code)]
        choice((
            one_of(SIGNS)
                .repeated()
                .at_least(1)
                .then(unsigned().or_not())
                .ignored(),
            unsigned(),
        ))
        .to_slice()
        .map(|s| unsafe { Self::new_unchecked(s.to_string()) })
    }
}

/// Parse a character which may follow any leading [SIGNS] of a [Symbol]
fn unsigned_initial<'i, S>() -> impl Parser<'i, char, S> {
    one_of(SYMBOL_SYMBOLS)
        .filter(|&c: &char| !SIGNS.contains(c))
        .or(any().filter(|&c: &char| c.is_alphabetic()))
}

/// Parse a non-digit character which may appear in a [Symbol]
fn initial<'i, S>() -> impl Parser<'i, char, S> {
    one_of(SYMBOL_SYMBOLS).or(any().filter(|&c: &char| c.is_alphabetic()))
}

/// Parse a character which may appear in a [Symbol] after the initial character
#[allow(clippy::is_digit_ascii_radix)]
pub(crate) fn subsequent<'i, S>() -> impl Parser<'i, char, S> {
    initial().or(any().filter(|&c: &char| c.is_digit(10)))
}

impl Validator for Symbol {
    type Error = Errors<'static>;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        Self::validate_stateless_owned(raw)
    }
}
