use std::fmt::Debug;

use chumsky::error::{Error as ChumskyError, Rich, RichPattern};
use chumsky::label::LabelError;
use chumsky::span::SimpleSpan;
use chumsky::util::MaybeRef;
use derive_more::Display;

/// The input type [Error] is parameterized over
///
/// [Rich] implements [ChumskyError] and [LabelError] generically over any input with `char` tokens and [SimpleSpan] spans, so delegating calls must name the input type explicitly.
type Input<'i> = &'i str;

/// The internal / individual case error type for [chumka::Parser]
///
/// This wraps [Rich] only to provide a structural [Debug] impl, since [Rich]'s [Debug] is identical to its [Display].
#[derive(Display)]
pub struct Error<'i>(Rich<'i, char>);

impl<'i> Error<'i> {
    /// Create an error with a custom message and span
    pub fn custom<M: Into<String>>(span: SimpleSpan, msg: M) -> Self {
        Error(Rich::custom(span, msg))
    }

    /// Clone any borrowed tokens so the error no longer borrows from the input
    pub fn into_owned<'b>(self) -> Error<'b> {
        Error(self.0.into_owned())
    }
}

impl<'i> Debug for Error<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("span", self.0.span())
            .field("reason", self.0.reason())
            .field("found", &self.0.found())
            .finish()
    }
}

impl<'i> ChumskyError<'i, Input<'i>> for Error<'i> {
    fn merge(self, other: Self) -> Self {
        Error(<Rich<'i, char> as ChumskyError<'i, Input<'i>>>::merge(
            self.0, other.0,
        ))
    }
}

impl<'i, L> LabelError<'i, Input<'i>, L> for Error<'i>
where
    L: TryInto<RichPattern<'i, char>>,
{
    fn expected_found<E: IntoIterator<Item = L>>(
        expected: E,
        found: Option<MaybeRef<'i, char>>,
        span: SimpleSpan,
    ) -> Self {
        Error(
            <Rich<'i, char> as LabelError<'i, Input<'i>, L>>::expected_found(expected, found, span),
        )
    }

    fn merge_expected_found<E: IntoIterator<Item = L>>(
        self,
        expected: E,
        found: Option<MaybeRef<'i, char>>,
        span: SimpleSpan,
    ) -> Self
    where
        Self: ChumskyError<'i, Input<'i>>,
    {
        Error(
            <Rich<'i, char> as LabelError<'i, Input<'i>, L>>::merge_expected_found(
                self.0, expected, found, span,
            ),
        )
    }

    fn replace_expected_found<E: IntoIterator<Item = L>>(
        self,
        expected: E,
        found: Option<MaybeRef<'i, char>>,
        span: SimpleSpan,
    ) -> Self {
        Error(
            <Rich<'i, char> as LabelError<'i, Input<'i>, L>>::replace_expected_found(
                self.0, expected, found, span,
            ),
        )
    }

    fn label_with(&mut self, label: L) {
        <Rich<'i, char> as LabelError<'i, Input<'i>, L>>::label_with(&mut self.0, label);
    }

    fn in_context(&mut self, label: L, span: SimpleSpan) {
        <Rich<'i, char> as LabelError<'i, Input<'i>, L>>::in_context(&mut self.0, label, span);
    }
}
