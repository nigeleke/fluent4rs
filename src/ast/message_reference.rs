use super::{AttributeAccessor, Identifier};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [MessageReference](crate::ast::MessageReference) ::= [Identifier](crate::ast::Identifier) [AttributeAccessor](crate::ast::AttributeAccessor)?
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MessageReference {
    identifier: Identifier,
    attribute_accessor: Option<AttributeAccessor>,
}

impl MessageReference {
    pub fn new(identifier: Identifier, attribute_accessor: Option<AttributeAccessor>) -> Self {
        Self {
            identifier,
            attribute_accessor,
        }
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn attribute_accessor(&self) -> Option<&AttributeAccessor> {
        self.attribute_accessor.as_ref()
    }
}

#[cfg(feature = "walker")]
impl Walkable for MessageReference {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_message_reference(self);
        self.identifier.walk(visitor);
        self.attribute_accessor
            .iter()
            .for_each(|aa| aa.walk(visitor));
    }
}

impl std::fmt::Display for MessageReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.identifier,
            self.attribute_accessor
                .as_ref()
                .map_or("".to_string(), |aa| aa.to_string())
        )
    }
}
