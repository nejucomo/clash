use chumsky::extra::{Full, SimpleState};

use crate::Error;

/// An extension trait of [chumksy::Parser] streamlined for `clash` support
pub trait Parser<'i, O, S>:
    chumsky::Parser<'i, &'i str, O, Full<Error<'i>, SimpleState<S>, ()>>
{
}

impl<'i, O, S, T> Parser<'i, O, S> for T where
    T: chumsky::Parser<'i, &'i str, O, Full<Error<'i>, SimpleState<S>, ()>>
{
}
