use super::prelude::{Identifier, NumberLiteral};

#[derive(Clone, Debug, PartialEq)]
pub enum VariantKey {
    NumberLiteral(NumberLiteral),
    Identifier(Identifier),
}
