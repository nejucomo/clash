use std::rc::Rc;

use clash_interning::Interns;

use crate::{Errors, ErrorsIntoOwned as _, Parser};

/// A type is parsable if there's a default [Parser] which produces a value of the type
pub trait Parsable<S>: Sized {
    /// The default [Parser]
    fn parser<'i>() -> impl Parser<'i, Self, S>;
}

/// A [Parsable] type which works with state `()`
pub trait ParsableStateless: Parsable<()> {
    /// The default [Parser] as the stateless [Parser] type
    fn parser_stateless<'i>() -> impl Parser<'i, Self, ()> {
        Self::parser()
    }

    /// A shorthand particularly streamlined for implementing `aliri_braid::Validator` from a parser
    fn validate_stateless_owned(raw: &str) -> Result<(), Errors<'static>> {
        Self::parser_stateless()
            .check_chumka(raw)
            .errors_into_owned()
    }
}

impl<T> ParsableStateless for T where T: Parsable<()> {}

/// Any [Rc] type is [Parsable] if the state can intern it
impl<S, T> Parsable<S> for Rc<T>
where
    T: Parsable<S>,
    S: Interns<T>,
{
    fn parser<'i>() -> impl Parser<'i, Self, S> {
        T::parser().map_with_chumka_state(|output, state: &mut S| state.get_interned(output))
    }
}
