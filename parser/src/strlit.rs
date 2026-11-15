use chumsky::IterParser as _;
use chumsky::Parser as _;
use chumsky::prelude::{choice, just, none_of};

use crate::clash_parser::ClashParser;

/// A string literal parser with python-like syntax: `'` vs `"` interchangeability to reduce quote escaping for literals containing only one of the two types
///
/// Within either quote style, the backslash escapes are: `\\`, `\'`, `\"`, `\n`, `\t`, `\r`, and `\0`. Any other escape is a parse error. Raw newlines are permitted within a literal.
pub(crate) fn string_literal<'i>() -> impl ClashParser<'i, String> {
    choice((quoted('\''), quoted('"')))
}

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
