use chumsky::extra::{Full, SimpleState};

use crate::{Error, Errors};

/// An extension trait of [chumksy::Parser] streamlined for `clash` support
pub trait Parser<'i, O, S>:
    Sized + chumsky::Parser<'i, &'i str, O, Full<Error<'i>, SimpleState<S>, ()>>
{
    /// Parse an `input` string with default parser state
    fn parse_chumka(self, input: &'i str) -> Result<O, Errors<'i>>
    where
        S: Default,
    {
        self.parse(input).into_result().map_err(Errors::from)
    }

    /// Check an `input` string with default parser state
    fn check_chumka(self, input: &'i str) -> Result<(), Errors<'i>>
    where
        S: Default,
    {
        self.check(input).into_result().map_err(Errors::from)
    }

    /// Map the output with `&mut S`
    fn map_with_chumka_state<U, F>(self, f: F) -> impl Parser<'i, U, S>
    where
        F: Fn(O, &mut S) -> U,
    {
        self.map_with(move |o, xtra| f(o, xtra.state()))
    }
}

impl<'i, O, S, T> Parser<'i, O, S> for T where
    T: Sized + chumsky::Parser<'i, &'i str, O, Full<Error<'i>, SimpleState<S>, ()>>
{
}
