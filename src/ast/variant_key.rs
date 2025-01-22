use super::prelude::{Identifier, NumberLiteral};

#[derive(Clone, Debug, PartialEq)]
pub enum VariantKey {
    NumberLiteral(NumberLiteral),
    Identifier(Identifier),
}

impl std::fmt::Display for VariantKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::NumberLiteral(literal) => literal.to_string(),
            Self::Identifier(identifier) => identifier.to_string(),
        };
        write!(f, "{value}")
    }
}
