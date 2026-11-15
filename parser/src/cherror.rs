pub(crate) type ChumskyError<'e> = chumsky::error::Rich<'e, char>;
pub(crate) type ChumskyFull<'e> = chumsky::extra::Full<ChumskyError<'e>, (), ()>;

// pub(crate) type Span = chumsky::span::SimpleSpan;
