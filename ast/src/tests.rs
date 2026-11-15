use aliri_braid::Validator;
use test_case_dir::test_case_dir;

use crate::{StringLiteral, Symbol};

/// Validate `input` with `V` and compare the [Debug] rendering of the outcome to `expected`
///
/// `input` is used verbatim, whereas a single trailing newline is stripped from `expected`.
fn check_validate<V: Validator<Error: std::fmt::Debug>>(input: &str, expected: &str) {
    let expected = expected.strip_suffix('\n').unwrap_or(expected);
    let actual = format!("{:?}", V::validate(input));
    eprintln!("input: {input:?}\nactual: {actual}\nexpected: {expected}");
    assert_eq!(actual, expected);
}

#[test_case_dir("tests/strlit-validate")]
fn strlit_validate(input: &str, expected: &str) {
    check_validate::<StringLiteral>(input, expected);
}

#[test_case_dir("tests/symbol-validate")]
fn symbol_validate(input: &str, expected: &str) {
    check_validate::<Symbol>(input, expected);
}
