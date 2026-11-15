use test_case_dir::test_case_dir;

use crate::parse;

#[test_case_dir("tests/parse-then-display")]
fn parse_then_display(input: &str, expected: &str) {
    let expected = expected.trim();
    let pres = parse(input);
    let actual = pres
        .map(|o| format!("ok: {o}"))
        .unwrap_or_else(|e| format!("err: {e}"));
    {
        let input = input.trim();
        eprintln!("input: {input}\nactual: {actual}\nexpected: {expected}");
    }
    assert_eq!(&actual, expected);
}
