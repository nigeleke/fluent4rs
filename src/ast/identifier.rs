#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Identifier](crate::ast::Identifier) ::= [a-zA-Z] [a-zA-Z0-9_-]*
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(any(test, feature = "hash"), derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Identifier(String);

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[cfg(feature = "walker")]
impl Walkable for Identifier {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier(self);
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
