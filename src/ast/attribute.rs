use super::{Identifier, Pattern};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
