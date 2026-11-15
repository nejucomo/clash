use aliri_braid::{Validator, braid};
use thiserror::Error;

/// These characters may appear in symbols in any position
pub const SYMBOL_SYMBOLS: &str = "-+_";

/// A symbol
#[braid(validator)]
pub struct Symbol;

#[derive(Debug, Error)]
pub enum MalformedSymbol {
    #[error("empty symbol")]
    Empty,

    #[error("invalid initial symbol char {0:?}; expected alphabetic or one of {SYMBOL_SYMBOLS:?}")]
    InvalidInitialChar(char),

    #[error("invalid symbol char {0:?}; expected alphanumeric or one of {SYMBOL_SYMBOLS:?}")]
    InvalidChar(char),
}

aliri_braid::from_infallible!(MalformedSymbol);

impl Validator for Symbol {
    type Error = MalformedSymbol;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        use MalformedSymbol::*;

        if raw.is_empty() {
            Err(Empty)
        } else {
            let mut first = true;
            for c in raw.chars() {
                if first {
                    first = false;

                    if !(SYMBOL_SYMBOLS.contains(c) || c.is_alphabetic()) {
                        return Err(InvalidInitialChar(c));
                    }
                } else if !(SYMBOL_SYMBOLS.contains(c) || c.is_alphanumeric()) {
                    return Err(InvalidChar(c));
                }
            }
            Ok(())
        }
    }
}
