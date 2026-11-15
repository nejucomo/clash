use chumsky::Parser as _;
use chumsky::combinator::Repeated;
use chumsky::primitive::one_of;

use crate::cherror::ChumskyFull;
use crate::clash_parser::ClashParser;

pub(crate) fn ws<'i>() -> Repeated<impl ClashParser<'i, ()>, (), &'i str, ChumskyFull<'i>> {
    ws_char().repeated()
}

pub(crate) fn ws_char<'i>() -> impl ClashParser<'i, ()> {
    one_of(" \n").ignored()
}
