use std::rc::Rc;

/// Any type which can intern `T` values
pub trait Interns<T> {
    /// Get an owned shared reference to the interned equivalent of `value`
    fn get_interned(&mut self, value: T) -> Rc<T> {
        self.ref_interned(value).clone()
    }

    /// Get a reference to the interned equivalent of `value`
    fn ref_interned(&mut self, value: T) -> &Rc<T>;
}
