use aliri_braid::{Validator, braid};
use chumsky::Parser as _;
use chumsky::primitive::{any, one_of};
use clash_chumka::{Parsable, Parser};

/// These characters may appear in symbols in any position
pub const SYMBOL_SYMBOLS: &str = "-+_";

/// A symbol
#[braid(validator)]
pub struct Symbol;

impl Validator for Symbol {
    type Error = ();

    fn validate(raw: &str) -> Result<(), Self::Error> {
        todo!()
    }
}

impl Parsable for Symbol {
    fn parser<'i>() -> impl Parser<'i, Self> {
        fn initial<'i>() -> impl Parser<'i, char> {
            one_of(SYMBOL_SYMBOLS).or(any().filter(|&c: &char| c.is_alphabetic()))
        }

        fn subsequent<'i>() -> impl Parser<'i, char> {
            initial().or(any().filter(|&c: &char| c.is_digit(10)))
        }

        #[allow(unsafe_code)]
        initial()
            .then(subsequent().repeated())
            .to_slice()
            .map(|s| unsafe { Symbol::new_unchecked(s.to_string()) })
    }
}
