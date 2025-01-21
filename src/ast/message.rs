use super::prelude::{Attribute, Identifier, Pattern};

#[derive(Debug)]
pub(crate) enum MessageAttributes {
    Patterned(Pattern, Vec<Attribute>),
    Plain(Vec<Attribute>),
}

#[derive(Debug)]
pub struct Message {
    identifier: Identifier,
    attributes: MessageAttributes,
}

impl Message {
    pub fn new(identifier: Identifier, attributes: MessageAttributes) -> Self {
        Self {
            identifier,
            attributes,
        }
    }
}
