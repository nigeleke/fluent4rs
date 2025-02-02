use super::{Attribute, Identifier, Pattern};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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

    // Note: a Message and Term Identifier may be the same, e,g, `product = ...` versus `-product = ...`.
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    // Note: Differentiates a Message and Term Identifier name using the '-' prefix
    pub fn identifier_name(&self) -> String {
        format!("-{}", self.identifier)
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

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
