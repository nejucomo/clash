//! AST-related error types

use std::num::ParseIntError;

use thiserror::Error;

/// A failure to parse
#[derive(Debug, Error)]
pub enum ParseError {
    /// An error parsing an int
    #[error(transparent)]
    Int(#[from] ParseIntError),
}
