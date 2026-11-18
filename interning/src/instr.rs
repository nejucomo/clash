use std::fmt::Display;
use std::rc::Rc;

use derive_more::From;

/// A [String]-like wrapper around `Rc<String>`
#[derive(Clone, Debug, From, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InternedString(Rc<String>);

impl InternedString {
    /// Take the inner value
    pub fn into_inner(self) -> Rc<String> {
        self.0
    }

    /// Ref the inner value
    pub fn ref_inner(&self) -> &Rc<String> {
        &self.0
    }
}

impl Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Performance Note: copies via `str::to_string`
impl<'a> From<&'a str> for InternedString {
    fn from(s: &'a str) -> Self {
        Self::from(s.to_string())
    }
}

/// Performance Note: copies via `str::to_string`
impl From<Box<str>> for InternedString {
    fn from(s: Box<str>) -> Self {
        Self::from(String::from(s))
    }
}

impl From<String> for InternedString {
    fn from(s: String) -> Self {
        Self::from(Rc::new(s))
    }
}

/// Performance Note: calls `Rc::copy_or_clone`
impl From<InternedString> for String {
    fn from(instr: InternedString) -> Self {
        instr.0.as_str().to_string()
    }
}

impl From<InternedString> for Rc<String> {
    fn from(instr: InternedString) -> Self {
        instr.into_inner()
    }
}

impl AsRef<str> for InternedString {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<Rc<String>> for InternedString {
    fn as_ref(&self) -> &Rc<String> {
        self.ref_inner()
    }
}
