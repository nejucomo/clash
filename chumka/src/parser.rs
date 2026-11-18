use chumsky::extra::{Full, SimpleState};

use crate::Error;

/// An extension trait of [chumksy::Parser] streamlined for `clash` support
pub trait Parser<'i, O, S = ()>:
    Sized + chumsky::Parser<'i, &'i str, O, Full<Error<'i>, SimpleState<S>, ()>>
{
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
