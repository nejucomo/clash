use test_case_dir::test_case_dir;

use crate::{ParseError, parse};

#[test_case_dir("tests/parse-then-display")]
fn parse_then_display(input: &str, expected: &str) -> Result<(), ParseError> {
    let expr = parse(input)?;
    let actual = expr.to_string();
    assert_eq!(&actual, expected.trim());
    Ok(())
}
