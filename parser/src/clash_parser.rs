use chumsky::Parser;
use chumsky::extra::{Full, SimpleState};

use crate::cherror::ChumskyError;
use crate::intern::Interner;

pub(crate) type State = SimpleState<Interner<String>>;

pub(crate) trait ClashParser<'i, O>:
    Sized + Parser<'i, &'i str, O, Full<ChumskyError<'i>, State, ()>>
{
}

impl<'i, T, O> ClashParser<'i, O> for T where
    T: Parser<'i, &'i str, O, Full<ChumskyError<'i>, State, ()>>
{
}
