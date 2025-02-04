use super::Identifier;

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [VariableReference](crate::ast::VariableReference) ::= "$" [Identifier](crate::ast::Identifier)
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VariableReference(Identifier);

impl VariableReference {
    pub fn identifier(&self) -> &Identifier {
        &self.0
    }
}

impl From<Identifier> for VariableReference {
    fn from(value: Identifier) -> Self {
        Self(value)
    }
}

#[cfg(feature = "walker")]
impl Walkable for VariableReference {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_variable_reference(self);
        self.0.walk(visitor);
    }
}

impl std::fmt::Display for VariableReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}
