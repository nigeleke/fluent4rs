use super::Identifier;

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [VariableReference](crate::ast::VariableReference) ::= "$" [Identifier](crate::ast::Identifier)
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VariableReference(Identifier);

impl VariableReference {
    /// Returns a reference to the underlying identifier of this variable reference.
    ///
    /// The identifier is the name without the leading `$` sigil.
    pub fn identifier(&self) -> &Identifier {
        &self.0
    }

    /// Returns the string representation of this variable reference as it appears in Fluent source,
    /// including the leading dollar sign (e.g., `$count`, `$userName`).
    ///
    /// This is useful when serializing, pretty-printing, or displaying the reference.
    pub fn identifier_name(&self) -> String {
        format!("${}", self.0)
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
        Walker::walk(&self.0, visitor);
    }
}

impl std::fmt::Display for VariableReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}
