use super::prelude::{Identifier, NumberLiteral};

#[derive(Clone, Debug)]
pub enum VariantKey {
    NumberLiteral(NumberLiteral),
    Identifier(Identifier),
}
