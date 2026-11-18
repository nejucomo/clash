use crate::Parser;

/// A type is parsable if there's a default [Parser] which produces a value of the type
pub trait Parsable<S>: Sized {
    /// The default [Parser]
    fn parser<'i>() -> impl Parser<'i, Self, S>;
}
