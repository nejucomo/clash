/// The internal / individual case error type for [chumka::Parser]
pub type Error<'i> = chumsky::error::Rich<'i, char>;
