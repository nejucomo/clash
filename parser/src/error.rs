use derive_more::From;
use thiserror::Error;

use crate::cherror::ChumskyError;

#[derive(Debug, Error, From)]
#[error("parse errors: {cases:#?}")]
pub struct Error<'e> {
    cases: Vec<ChumskyError<'e>>,
}

pub type Result<'e, T> = std::result::Result<T, Error<'e>>;
