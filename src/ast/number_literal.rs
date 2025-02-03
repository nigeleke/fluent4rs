#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [NumberLiteral](crate::ast::NumberLiteral) ::= "-"? digits ("." digits)?
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct NumberLiteral(String);

impl From<&str> for NumberLiteral {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[cfg(feature = "walker")]
impl Walkable for NumberLiteral {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_number_literal(self);
    }
}

impl std::fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
