use std::str::FromStr;

use crate::Expr;
use crate::errors::ParseError;

impl FromStr for Expr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i: i64 = s.parse()?;
        Ok(Expr::Int(i))
    }
}
