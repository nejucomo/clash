use std::fmt::Display;

use derive_more::From;
use indent::indent_by;

use crate::Error;

/// A set of errors produced when parsing fails multiple possible variations
#[derive(Debug, From)]
pub struct Errors<'i>(Vec<Error<'i>>);

impl<'i> std::error::Error for Errors<'i> {}

impl<'i> Display for Errors<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Parse Errors:")?;
        for (i, e) in self.0.iter().enumerate() {
            let k = i + 1;
            let indent = k.ilog10() as usize + 3; // 3 for log10, `)`, and ` `
            writeln!(f, "{k}) {}", indent_by(indent, e.to_string()))?;
        }
        Ok(())
    }
}
