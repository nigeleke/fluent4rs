use super::{Identifier, Literal};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [NamedArgument](crate::ast::NamedArgument) ::= [Identifier](crate::ast::Identifier) blank? ":" blank? ([StringLiteral](crate::ast::StringLiteral) | [NumberLiteral](crate::ast::NumberLiteral))
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct NamedArgument {
    identifier: Identifier,
    literal: Literal,
}

impl NamedArgument {
    pub fn new(identifier: Identifier, literal: Literal) -> Self {
        Self {
            identifier,
            literal,
        }
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

#[cfg(feature = "walker")]
impl Walkable for NamedArgument {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_named_argument(self);
        Walker::walk(&self.identifier, visitor);
        Walker::walk(&self.literal, visitor);
    }
}

impl std::fmt::Display for NamedArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.identifier, self.literal)
    }
}
