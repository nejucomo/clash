use std::rc::Rc;

use chumsky::extra::{Full, SimpleState};
use clash_interning::Interns;

use crate::Error;

/// An extension trait of [chumksy::Parser] streamlined for `clash` support
pub trait Parser<'i, O, S>:
    Sized + chumsky::Parser<'i, &'i str, O, Full<Error<'i>, SimpleState<S>, ()>>
{
    /// Intern the parsed value
    fn interned(self) -> impl Parser<'i, Rc<O>, S>
    where
        S: Interns<O>,
    {
        self.map_with(|o, xtra| xtra.state().get_interned(o))
    }
}

impl<'i, O, S, T> Parser<'i, O, S> for T where
    T: Sized + chumsky::Parser<'i, &'i str, O, Full<Error<'i>, SimpleState<S>, ()>>
{
}
