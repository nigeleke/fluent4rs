use super::{Identifier, Pattern};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Attribute](crate::ast::Attribute) ::= line_end blank? "." [Identifier](crate::ast::Identifier) blank_inline? "=" blank_inline? [Pattern](crate::ast::Pattern)
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Attribute {
    identifier: Identifier,
    pattern: Pattern,
}

impl Attribute {
    pub fn new(identifier: Identifier, pattern: Pattern) -> Self {
        Self {
            identifier,
            pattern,
        }
    }

    /// Returns the attribute identifier.
    ///
    ///  Note: a [Message](crate::ast::Message) and [Term](crate::ast::Term) [Identifier](crate::ast::Identifier)
    ///  may also be the same, e,g, `product = ...` versus `-product = ...`.
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    /// Returns the attribute identifier _name_.
    ///
    /// Note: Differentiates the [Message](crate::ast::Message) and [Term](crate::ast::Term)
    /// [Identifier](crate::ast::Identifier) name by using the '.' prefix for the [Term](crate::ast::Term).
    pub fn identifier_name(&self) -> String {
        format!(".{}", self.identifier)
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }
}

#[cfg(feature = "walker")]
impl Walkable for Attribute {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_attribute(self);
        self.identifier.walk(visitor);
        self.pattern.walk(visitor);
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n    .{} = {}", self.identifier, self.pattern)
    }
}
