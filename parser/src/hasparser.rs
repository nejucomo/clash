use std::rc::Rc;

use chumsky::Parser as _;
use chumsky::prelude::{choice, just, none_of};
use chumsky::{IterParser as _, text};

use crate::clash_parser::ClashParser;

pub(crate) trait HasParser<'i>: Sized {
    fn parser() -> impl ClashParser<'i, Self>;

    fn parser_into<O>() -> impl ClashParser<'i, O>
    where
        O: From<Self>,
    {
        Self::parser().map(O::from)
    }
}

impl<'i> HasParser<'i> for i64 {
    fn parser() -> impl ClashParser<'i, Self> {
        text::digits(10).collect::<String>().from_str().unwrapped()
    }
}

impl<'i> HasParser<'i> for Rc<String> {
    fn parser() -> impl ClashParser<'i, Self> {
        String::parser().map_with(|sval, st| st.state().intern(sval))
    }
}

impl<'i> HasParser<'i> for String {
    fn parser() -> impl ClashParser<'i, Self> {
        /// A string literal delimited by `quote`
        fn quoted<'i>(quote: char) -> impl ClashParser<'i, String> {
            none_of([quote, '\\'])
                .or(escape())
                .repeated()
                .collect::<String>()
                .delimited_by(just(quote), just(quote))
        }

        /// A backslash escape sequence, yielding the char it denotes
        fn escape<'i>() -> impl ClashParser<'i, char> {
            just('\\').ignore_then(choice((
                just('\\'),
                just('\''),
                just('"'),
                just('n').to('\n'),
                just('t').to('\t'),
                just('r').to('\r'),
                just('0').to('\0'),
            )))
        }

        choice((quoted('\''), quoted('"')))
    }
}
