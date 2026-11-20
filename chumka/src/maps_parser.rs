use chumsky::Parser as _;

use crate::{Parsable, Parser};

/// Parse a parsable via a variant constructor
pub fn maps_parser<'i, S, I, O, F>(f: F) -> impl Parser<'i, O, S>
where
    F: Fn(I) -> O,
    I: Parsable<S>,
{
    I::parser().map(f)
}
