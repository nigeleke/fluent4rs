use super::prelude::{AttributeAccessor, Identifier};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
