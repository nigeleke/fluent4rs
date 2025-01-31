use super::prelude::{AttributeAccessor, CallArguments, Identifier};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TermReference {
    identifier: Identifier,
    attribute_accessor: Option<AttributeAccessor>,
    call_arguments: Option<CallArguments>,
}

impl TermReference {
    pub fn new(
        identifier: Identifier,
        attribute_accessor: Option<AttributeAccessor>,
        call_arguments: Option<CallArguments>,
    ) -> Self {
        Self {
            identifier,
            attribute_accessor,
            call_arguments,
        }
    }
}

#[cfg(feature = "walker")]
impl Walkable for TermReference {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_term_reference(self);
        self.identifier.walk(visitor);
        self.attribute_accessor
            .iter()
            .for_each(|aa| aa.walk(visitor));
        self.call_arguments.iter().for_each(|ca| ca.walk(visitor));
    }
}

impl std::fmt::Display for TermReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-{}{}{}",
            self.identifier,
            self.attribute_accessor
                .as_ref()
                .map_or("".to_string(), |aa| aa.to_string()),
            self.call_arguments
                .as_ref()
                .map_or("".to_string(), |ca| ca.to_string())
        )
    }
}
