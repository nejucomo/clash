use chumsky::Parser;
use chumsky::extra::Full;

use crate::cherror::ChumskyError;

pub(crate) trait ClashParser<'i, O>:
    Sized + Parser<'i, &'i str, O, Full<ChumskyError<'i>, (), ()>>
{
}

impl<'i, T, O> ClashParser<'i, O> for T where
    T: Parser<'i, &'i str, O, Full<ChumskyError<'i>, (), ()>>
{
}
