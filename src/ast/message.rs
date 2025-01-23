use super::prelude::{Attribute, Identifier, Pattern};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
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

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
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

    // Note: a Message and Term Identifier may be the same, e,g, `product = ...` versus `-product = ...`.
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    // Note: Differentiates a Message and Term Identifier name using the '-' prefix
    pub fn identifier_name(&self) -> String {
        self.identifier.to_string()
    }

    pub fn attributes(&self) -> &MessageAttributes {
        &self.attributes
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} = {}", self.identifier, self.attributes)
    }
}
