use std::collections::BTreeSet;
use std::rc::Rc;

use crate::Interns;

/// A structure for interning `T: Ord` values
///
/// # Implementation
///
/// This is a thin newtype wrapper of `[BTreeSet]<[Rc]<T>>`.
#[derive(Debug)]
pub struct Interner<T>(BTreeSet<Rc<T>>)
where
    T: Ord;

impl<T> Default for Interner<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self(BTreeSet::default())
    }
}

impl<T> Interns<T> for Interner<T>
where
    T: Ord,
{
    fn ref_interned(&mut self, value: T) -> &Rc<T> {
        self.0.get_or_insert(Rc::new(value))
    }
}
