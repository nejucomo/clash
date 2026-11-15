/// A string literal parser with python-like syntax: `'` vs `"` interchangeability to reduce quote escaping for literals containing only one of the two types
pub(crate) fn string_literal<'i>() -> impl ClashParser<'i, String> {
    todo!("implement this")
}
