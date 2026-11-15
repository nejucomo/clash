use std::rc::Rc;

use chumsky::Parser as _;
use clash_ast::Expr;

use crate::clash_parser::ClashParser;
use crate::hasparser::HasParser;
use crate::srctext::ws;

pub(crate) fn expr<'i>() -> impl ClashParser<'i, Expr> {
    Expr::parser()
}

impl<'i> HasParser<'i> for Expr {
    fn parser() -> impl ClashParser<'i, Self> {
        i64::parser_into()
            .or(Rc::<String>::parser_into())
            .then_ignore(ws())
    }
}
