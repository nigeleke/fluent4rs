use super::prelude::{Attribute, Identifier, Pattern};

#[derive(Debug, PartialEq)]
pub enum MessageAttributes {
    Patterned(Pattern, Vec<Attribute>),
    Plain(Vec<Attribute>),
}

impl std::fmt::Display for MessageAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Patterned(pattern, attributes) => {
                let attributes = attributes
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join("");
                format!("{}{}", pattern, attributes)
            }
            Self::Plain(attributes) => attributes
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(""),
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, PartialEq)]
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

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} = {}", self.identifier, self.attributes)
    }
}