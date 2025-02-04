use super::Identifier;

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [AttributeAccessor](crate::ast::AttributeAccessor) ::= "." [Identifier](crate::ast::Identifier)
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AttributeAccessor(Identifier);

impl AttributeAccessor {
    pub fn identifier(&self) -> &Identifier {
        &self.0
    }
}

impl From<Identifier> for AttributeAccessor {
    fn from(value: Identifier) -> Self {
        Self(value)
    }
}

#[cfg(feature = "walker")]
impl Walkable for AttributeAccessor {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_attribute_accessor(self);
        self.0.walk(visitor);
    }
}

impl std::fmt::Display for AttributeAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".{}", self.0)
    }
}
