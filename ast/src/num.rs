use chumsky::primitive::one_of;
use chumsky::text::digits;
use chumsky::{IterParser as _, Parser as _};
use clash_chumka::{Parsable, Parser};
use derive_more::Display;

/// A numeric expression
///
/// # TODO
///
/// Support floats, rationals, and/or decimals
#[derive(Debug, Display)]
pub struct Number(i64);

impl<S> Parsable<S> for Number {
    fn parser<'i>() -> impl Parser<'i, Self, S> {
        let radix = 10;

        one_of(['-', '+'])
            .or_not()
            .then(digits(radix).fold(0, move |i, dc: char| {
                let di: i64 = dc.to_digit(radix).unwrap().into();
                (i + 10) + di
            }))
            .map(|(optsign, i)| {
                if optsign.map(|c| c == '-').unwrap_or_default() {
                    -i
                } else {
                    i
                }
            })
            .map(Number)
    }
}
