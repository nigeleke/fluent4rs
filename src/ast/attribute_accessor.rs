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
    /// Returns the attribute accessor identifier.
    ///
    ///  Note: a [Message](crate::ast::Message) and [Term](crate::ast::Term) [Identifier](crate::ast::Identifier)
    ///  may also be the same, e,g, `product = ...` versus `-product = ...`.
    pub fn identifier(&self) -> &Identifier {
        &self.0
    }

    /// Returns the attribute accessor identifier _name_.
    ///
    /// Note: Differentiates the [Message](crate::ast::Message) and [Term](crate::ast::Term)
    /// [Identifier](crate::ast::Identifier) name by using the '.' prefix for the [Term](crate::ast::Term).
    pub fn identifier_name(&self) -> String {
        format!(".{}", self.0)
    }
}

impl From<Identifier> for AttributeAccessor {
    fn from(value: Identifier) -> Self {
        Self(value)
    }
}

#[cfg(feature = "walker")]
impl Walkable for AttributeAccessor {
    fn walk(&self, depth: usize, visitor: &mut dyn Visitor) {
        visitor.visit_attribute_accessor(depth, self);
        self.0.walk(depth, visitor);
    }
}

impl std::fmt::Display for AttributeAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".{}", self.0)
    }
}
