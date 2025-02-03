use super::{Attribute, Identifier, Pattern};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Term](crate::ast::Term) ::= "-" [Identifier](crate::ast::Identifier) blank_inline? "=" blank_inline? [Pattern](crate::ast::Pattern) [Attribute](crate::ast::Attribute)*
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Term {
    identifier: Identifier,
    pattern: Pattern,
    attributes: Vec<Attribute>,
}

impl Term {
    pub fn new(identifier: Identifier, pattern: Pattern, attributes: Vec<Attribute>) -> Self {
        Self {
            identifier,
            pattern,
            attributes,
        }
    }

    /// Returns the term identifier.
    ///
    ///  Note: a [Message](crate::ast::Message) and [Term](crate::ast::Term) [Identifier](crate::ast::Identifier)
    ///  may be the same, e,g, `product = ...` versus `-product = ...`.
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    /// Returns the term identifier _name_.
    ///
    /// Note: Differentiates the [Message](crate::ast::Message) and [Term](crate::ast::Term)
    /// [Identifier](crate::ast::Identifier) name by using the '-' prefix for the [Term](crate::ast::Term).
    pub fn identifier_name(&self) -> String {
        format!("-{}", self.identifier)
    }

    /// Returns the term [Pattern](crate::ast::Pattern).
    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Returns the message [Attribute](crate::ast::Attribute)s.
    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

#[cfg(feature = "walker")]
impl Walkable for Term {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_term(self);
        self.pattern().walk(visitor);
        self.attributes().iter().for_each(|a| a.walk(visitor));
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attributes = self
            .attributes
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("");
        writeln!(f, "-{} = {}{}", self.identifier, self.pattern, attributes)
    }
}
