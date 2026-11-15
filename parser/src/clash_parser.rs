use std::fmt::Display;
use std::str::FromStr;

use chumsky::Parser;
use chumsky::extra::Full;

use crate::cherror::ChumskyError;

pub(crate) trait ClashParser<'i, O>:
    Sized + Parser<'i, &'i str, O, Full<ChumskyError<'i>, (), ()>>
{
    fn try_map_from_str<T>(self) -> impl ClashParser<'i, T>
    where
        O: AsRef<str>,
        T: FromStr,
        T::Err: Display,
    {
        self.try_map_errdisp(|s| s.as_ref().parse())
    }

    fn try_map_errdisp<F, T, E>(self, f: F) -> impl ClashParser<'i, T>
    where
        F: Fn(O) -> Result<T, E>,
        E: Display,
    {
        self.try_map(move |x: O, span| {
            f(x).map_err(|e: E| ChumskyError::custom(span, e.to_string()))
        })
    }
}

impl<'i, T, O> ClashParser<'i, O> for T where
    T: Parser<'i, &'i str, O, Full<ChumskyError<'i>, (), ()>>
{
}
