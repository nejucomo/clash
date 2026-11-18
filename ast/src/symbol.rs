use aliri_braid::{Validator, braid};
use chumsky::Parser as _;
use chumsky::primitive::{any, one_of};
use clash_chumka::{Errors, Parsable, ParsableStateless, Parser};

/// These characters may appear in symbols in any position
pub const SYMBOL_SYMBOLS: &str = "-+_";

/// A symbol
#[braid(validator)]
pub struct Symbol;

impl Validator for Symbol {
    type Error = Errors<'static>;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        Self::validate_stateless_owned(raw)
    }
}

impl<S> Parsable<S> for Symbol {
    fn parser<'i>() -> impl Parser<'i, Self, S> {
        fn initial<'i, S>() -> impl Parser<'i, char, S> {
            one_of(SYMBOL_SYMBOLS).or(any().filter(|&c: &char| c.is_alphabetic()))
        }

        #[allow(clippy::is_digit_ascii_radix)]
        fn subsequent<'i, S>() -> impl Parser<'i, char, S> {
            initial().or(any().filter(|&c: &char| c.is_digit(10)))
        }

        #[allow(unsafe_code)]
        initial()
            .then(subsequent().repeated())
            .to_slice()
            .map(|s| unsafe { Symbol::new_unchecked(s.to_string()) })
    }
}
