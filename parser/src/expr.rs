use chumsky::{IterParser as _, Parser as _, text};
use clash_ast::Expr;

use crate::clash_parser::ClashParser;
use crate::srctext::ws;

pub(crate) fn expr<'i>() -> impl ClashParser<'i, Expr> {
    int().map(Expr::Int).then_ignore(ws())
}

fn int<'i>() -> impl ClashParser<'i, i64> {
    text::digits(10).collect::<String>().try_map_from_str()
}
