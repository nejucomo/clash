//! Integration tests of the `test_case_dir` macro against the vectors in `tests/vectors/`

use test_case_dir::test_case_dir;

#[test_case_dir("vectors/concat")]
fn concat(a: &[u8], b: &[u8], expected: &[u8]) {
    assert_eq!([a, b].concat(), expected);
}

#[test_case_dir("vectors/reverse")]
fn reverse(input: &[u8], expected: &[u8]) {
    let actual: Vec<u8> = input.iter().rev().copied().collect();
    assert_eq!(actual, expected);
}

/// Test vector parameters may be declared in any order; files are matched by name
#[test_case_dir("vectors/reverse")]
fn reverse_swapped_params(expected: &[u8], input: &[u8]) {
    let actual: Vec<u8> = expected.iter().rev().copied().collect();
    assert_eq!(actual, input);
}

/// `&str` parameters receive the Test Vector File via `include_str!`
#[test_case_dir("vectors/reverse")]
fn reverse_str(input: &str, expected: &str) {
    let actual: String = input.chars().rev().collect();
    assert_eq!(actual, expected);
}

/// `&str` and `&[u8]` parameters may be mixed, with any lifetime
#[test_case_dir("vectors/concat")]
fn concat_mixed(a: &'static str, b: &[u8], expected: &str) {
    assert_eq!([a.as_bytes(), b].concat(), expected.as_bytes());
}
