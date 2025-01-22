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

impl std::fmt::Display for MessageReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.identifier,
            if self.attribute_accessor.is_some() {
                " "
            } else {
                ""
            },
            self.attribute_accessor
                .as_ref()
                .map_or("".to_string(), |aa| aa.to_string())
        )
    }
}
