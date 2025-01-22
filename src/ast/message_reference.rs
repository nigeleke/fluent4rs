use super::prelude::{AttributeAccessor, Identifier};

#[derive(Clone, Debug, PartialEq)]
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
