use crate::clash_parser::State;

pub(crate) type ChumskyError<'e> = chumsky::error::Rich<'e, char>;
pub(crate) type ChumskyFull<'e> = chumsky::extra::Full<ChumskyError<'e>, State, ()>;

// pub(crate) type Span = chumsky::span::SimpleSpan;
