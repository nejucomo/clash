use std::rc::Rc;

use aliri_braid::Validator;
use clash_chumka::{Parsable, ParsableStateless as _, Parser as _};
use clash_interning::{Interner, Interns};
use test_case_dir::test_case_dir;

use crate::literal::{Literal, Number, Symbol, Text};

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
    check_validate::<Text>(input, expected);
}

#[test_case_dir("tests/symbol-validate")]
fn symbol_validate(input: &str, expected: &str) {
    check_validate::<Symbol>(input, expected);
}

/// Parse `input` as an [Literal] and compare the [Debug] rendering of the outcome to `expected`
///
/// `input` is used verbatim, whereas a single trailing newline is stripped from `expected`.
#[test_case_dir("tests/expr-parse")]
fn expr_parse(input: &str, expected: &str) {
    let expected = expected.strip_suffix('\n').unwrap_or(expected);
    let actual = format!(
        "{:?}",
        <Literal as Parsable<LiteralParseState>>::parser().parse_chumka(input)
    );
    eprintln!("input: {input:?}\nactual: {actual}\nexpected: {expected}");
    assert_eq!(actual, expected);
}

/// Assert that `input` is not both a valid [Symbol] and a valid [Number]
///
/// `input` is used verbatim.
#[test_case_dir("tests/symbol-number-non-overlap")]
fn symbol_number_non_overlap(input: &str) {
    let sym = Symbol::parser_stateless().parse_chumka(input);
    let num = Number::parser_stateless().parse_chumka(input);
    eprintln!("input: {input:?}\nsymbol: {sym:?}\nnumber: {num:?}");
    assert!(!(sym.is_ok() && num.is_ok()));
}

/// The parser state for [expr_parse]
#[derive(Debug, Default)]
struct LiteralParseState {
    symbols: Interner<Symbol>,
    strlits: Interner<Text>,
}

impl Interns<Symbol> for LiteralParseState {
    fn ref_interned(&mut self, value: Symbol) -> &Rc<Symbol> {
        self.symbols.ref_interned(value)
    }
}

impl Interns<Text> for LiteralParseState {
    fn ref_interned(&mut self, value: Text) -> &Rc<Text> {
        self.strlits.ref_interned(value)
    }
}
