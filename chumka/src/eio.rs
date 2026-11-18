use crate::{Error, Errors};

/// Convert reference-based errors into owned errors
pub trait ErrorsIntoOwned {
    /// The owned type
    ///
    /// In every case we convert `Thing<'i>` into `Thing<'static>` but the type system cannot express that higher-order contraint that both `Self` and `OwnedErrors` are `Thingy<'some_lifetime>`.
    type OwnedErrors;

    /// Clone any errors from references to owned equivalents
    fn errors_into_owned(self) -> Self::OwnedErrors;
}

impl<'i> ErrorsIntoOwned for Errors<'i> {
    type OwnedErrors = Errors<'static>;

    fn errors_into_owned(self) -> Self::OwnedErrors {
        self.into_iter().map(Error::into_owned).collect()
    }
}

impl<'i> ErrorsIntoOwned for Error<'i> {
    type OwnedErrors = Error<'static>;

    fn errors_into_owned(self) -> Self::OwnedErrors {
        self.into_owned()
    }
}

impl<T, E> ErrorsIntoOwned for Result<T, E>
where
    E: ErrorsIntoOwned,
{
    type OwnedErrors = Result<T, E::OwnedErrors>;

    fn errors_into_owned(self) -> Self::OwnedErrors {
        self.map_err(|e| e.errors_into_owned())
    }
}
