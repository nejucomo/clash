use std::rc::Rc;

use clash_interning::Interns;

use crate::Parser;

/// A type is parsable if there's a default [Parser] which produces a value of the type
pub trait Parsable<S = ()>: Sized {
    /// The default [Parser]
    fn parser<'i>() -> impl Parser<'i, Self, S>;
}

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
