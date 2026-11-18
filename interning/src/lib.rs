//! Interning facilities for `clash`

#![feature(btree_set_entry)]

mod interner;
mod interns;

pub use self::interner::Interner;
pub use self::interns::Interns;
