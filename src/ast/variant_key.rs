use super::{Identifier, NumberLiteral};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [VariantKey](crate::ast::VariantKey) ::= "[" blank? ([NumberLiteral](crate::ast::NumberLiteral) | [Identifier](crate::ast::Identifier)) blank? "]"
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum VariantKey {
    NumberLiteral(NumberLiteral),
    Identifier(Identifier),
}

#[cfg(feature = "walker")]
impl Walkable for VariantKey {
    fn walk(&self, depth: usize, visitor: &mut dyn Visitor) {
        visitor.visit_variant_key(depth, self);
        match self {
            Self::NumberLiteral(literal) => literal.walk(depth + 1, visitor),
            Self::Identifier(identifier) => identifier.walk(depth + 1, visitor),
        }
    }
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
