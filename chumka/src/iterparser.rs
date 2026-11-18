use chumsky::extra::{Full, SimpleState};

use crate::Error;

/// An extension trait of [chumksy::IterParser] streamlined for `clash` support
pub trait IterParser<'i, O, S = ()>:
    Sized + chumsky::IterParser<'i, &'i str, O, Full<Error<'i>, SimpleState<S>, ()>>
{
}

impl<'i, O, S, T> IterParser<'i, O, S> for T where
    T: Sized + chumsky::IterParser<'i, &'i str, O, Full<Error<'i>, SimpleState<S>, ()>>
{
}
