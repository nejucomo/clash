use aliri_braid::{Validator, braid};
use chumsky::Parser as _;
use chumsky::primitive::{just, none_of, one_of};
use clash_chumka::{Errors, Parsable, ParsableStateless, Parser};

pub const QUOTE_SINGLE: char = '\'';
pub const QUOTE_DOUBLE: char = '"';
pub const ESCAPE: char = '\\';
/// Note: ESCAPEES excludes either quote char
pub const ESCAPEES: &str = r#"\ntr0"#;

/// A string literal
#[braid(validator)]
pub struct Text;

impl<S> Parsable<S> for Text {
    fn parser<'i>() -> impl Parser<'i, Self, S> {
        #[allow(unsafe_code)]
        strlit(QUOTE_SINGLE)
            .or(strlit(QUOTE_DOUBLE))
            .to_slice()
            .map(|s| unsafe { Self::new_unchecked(s.to_string()) })
    }
}

fn strlit<'i, S>(quote: char) -> impl Parser<'i, (), S> {
    body(quote).repeated().delimited_by_chars(quote, quote)
}

fn body<'i, S>(quote: char) -> impl Parser<'i, (), S> {
    escape_sequence().or(unescaped(quote))
}

/// Either quote may be escaped regardless of which `quote` delimits the literal
fn escape_sequence<'i, S>() -> impl Parser<'i, (), S> {
    just(ESCAPE)
        .ignore_then(one_of(ESCAPEES).or(one_of([QUOTE_SINGLE, QUOTE_DOUBLE])))
        .ignored()
}

fn unescaped<'i, S>(quote: char) -> impl Parser<'i, (), S> {
    none_of([quote, ESCAPE]).ignored() // FIXME: require it to be printable, with ` ` as the only acceptable whitespace, and not `quote`
}

impl Validator for Text {
    type Error = Errors<'static>;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        Self::validate_stateless_owned(raw)
    }
}
