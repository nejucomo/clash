use test_case_dir::test_case_dir;

use crate::Expr;
use crate::errors::ParseError;

#[test_case_dir("tests/parse-then-display")]
fn parse_then_display(input: &str, expected: &str) -> Result<(), ParseError> {
    let expr: Expr = input.trim().parse()?;
    let actual = expr.to_string();
    assert_eq!(&actual, expected.trim());
    Ok(())
}
