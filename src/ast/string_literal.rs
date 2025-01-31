#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StringLiteral(String);

impl From<&str> for StringLiteral {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[cfg(feature = "walker")]
impl Walkable for StringLiteral {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_string_literal(self);
    }
}

impl std::fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}
