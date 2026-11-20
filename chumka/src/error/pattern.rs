//! A [Debug]-faithful view of [chumsky::error::RichPattern]
use std::fmt::{Debug, Display, Formatter, Result};

use chumsky::error::RichPattern as ChumskyRichPattern;

/// A borrowing wrapper of [chumsky::error::RichPattern]
///
/// [chumsky::error::RichPattern]'s [Debug] impl is nearly identical to its [Display] impl, which is ambiguous (ex: a `Label` and an `Identifier` may render the same). This wrapper provides a structural [Debug] impl close to Rust syntax, while [Display] delegates to the user-readable [chumsky] rendering.
#[derive(Copy, Clone)]
pub struct RichPattern<'a, 'i>(pub &'a ChumskyRichPattern<'i, char>);

impl<'a, 'i> Debug for RichPattern<'a, 'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ChumskyRichPattern::*;

        match self.0 {
            Token(tok) => f.debug_tuple("Token").field(&**tok).finish(),
            Label(label) => f.debug_tuple("Label").field(label).finish(),
            Identifier(ident) => f.debug_tuple("Identifier").field(ident).finish(),
            Any => f.write_str("Any"),
            SomethingElse => f.write_str("SomethingElse"),
            EndOfInput => f.write_str("EndOfInput"),
            // `RichPattern` is `#[non_exhaustive]`, so fall back on its own (ambiguous) `Debug` impl:
            other => write!(f, "{other:?}"),
        }
    }
}

impl<'a, 'i> Display for RichPattern<'a, 'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self.0, f)
    }
}
