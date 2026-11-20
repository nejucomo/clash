//! A [Debug]-faithful view of [chumsky::error::RichReason]
use std::fmt::{Debug, Display, Formatter, Result};

use chumsky::error::RichReason as ChumskyRichReason;

use super::pattern::RichPattern;

/// A borrowing wrapper of [chumsky::error::RichReason]
///
/// [chumsky::error::RichReason] derives [Debug], but its contained [chumsky::error::RichPattern]s render with a [Display]-like [Debug]. This wrapper renders those patterns via [RichPattern] so the whole structure is unambiguous and close to Rust syntax, while [Display] delegates to the user-readable [chumsky] rendering.
#[derive(Copy, Clone)]
pub struct RichReason<'a, 'i>(pub &'a ChumskyRichReason<'i, char>);

impl<'a, 'i> Debug for RichReason<'a, 'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0 {
            ChumskyRichReason::ExpectedFound { expected, found } => f
                .debug_struct("ExpectedFound")
                .field(
                    "expected",
                    &expected.iter().map(RichPattern).collect::<Vec<_>>(),
                )
                .field("found", &found.as_deref())
                .finish(),
            ChumskyRichReason::Custom(msg) => f.debug_tuple("Custom").field(msg).finish(),
        }
    }
}

impl<'a, 'i> Display for RichReason<'a, 'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self.0, f)
    }
}
