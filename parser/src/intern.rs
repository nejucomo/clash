use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Debug, Default)]
pub(crate) struct Interner<T>
where
    T: Ord,
{
    set: BTreeSet<Rc<T>>,
}

impl<T> Interner<T>
where
    T: Ord,
{
    pub(crate) fn intern(&mut self, entry: T) -> Rc<T> {
        self.set.get_or_insert(Rc::new(entry)).clone()
    }
}
