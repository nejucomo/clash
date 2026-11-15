use aliri_braid::{Validator, braid};
use thiserror::Error;

pub const QUOTE_SINGLE: char = '\'';
pub const QUOTE_DOUBLE: char = '"';
pub const ESCAPE: char = '\\';
pub const ESCAPEES: &str = r#"\'"ntr0"#;

/// A string literal
#[braid(validator)]
pub struct StringLiteral;

#[derive(Debug, Error)]
pub enum MalformedStringLiteral {
    #[error("empty string literal")]
    Empty,

    #[error("invalid quote {0:?}, expected {QUOTE_SINGLE:?} or {QUOTE_DOUBLE:?}")]
    InvalidQuote(char),

    #[error("unexpected suffix: {0:?}")]
    UnexpectedSuffix(String),

    #[error("unexpected escapee: {0:?}, drop the {ESCAPE:?} prefix")]
    UnexpectedEscapee(char),

    #[error("missing terminal {0:?} quote")]
    MissingTerminalQuote(char),
}

aliri_braid::from_infallible!(MalformedStringLiteral);

impl Validator for StringLiteral {
    type Error = MalformedStringLiteral;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        use MalformedStringLiteral::*;
        use State::*;

        enum State {
            Init,
            Base { quote: char },
            Escaped { quote: char },
            Terminated,
        }

        let mut chars = raw.chars();
        let mut st = Init;

        for c in chars.by_ref() {
            match st {
                Init if c == QUOTE_SINGLE || c == QUOTE_DOUBLE => {
                    st = Base { quote: c };
                }

                Init => {
                    return Err(InvalidQuote(c));
                }

                Base { quote } if c == quote => st = Terminated,
                Base { quote } if c == ESCAPE => st = Escaped { quote },
                Base { .. } => {}

                Escaped { .. } if !ESCAPEES.contains(c) => {
                    return Err(UnexpectedEscapee(c));
                }
                Escaped { quote } => {
                    st = Base { quote };
                }

                Terminated => {
                    return Err(UnexpectedSuffix(chars.as_str().to_string()));
                }
            }
        }

        match st {
            Init => Err(Empty),
            Base { quote } | Escaped { quote } => Err(MissingTerminalQuote(quote)),
            Terminated => Ok(()),
        }
    }
}
