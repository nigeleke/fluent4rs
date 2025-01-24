use super::prelude::{Attribute, Identifier, Pattern};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
pub enum MessageArguments {
    Patterned(Pattern, Vec<Attribute>),
    Plain(Vec<Attribute>),
}

impl MessageArguments {
    pub fn pattern(&self) -> Option<&Pattern> {
        match self {
            Self::Patterned(pattern, _) => Some(pattern),
            Self::Plain(_) => None,
        }
    }

    pub fn attributes(&self) -> &[Attribute] {
        match self {
            Self::Patterned(_, attributes) => attributes.as_slice(),
            Self::Plain(attributes) => attributes.as_slice(),
        }
    }
}

impl std::fmt::Display for MessageArguments {
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
    arguments: MessageArguments,
}

impl Message {
    pub fn new(identifier: Identifier, arguments: MessageArguments) -> Self {
        Self {
            identifier,
            arguments,
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

    pub fn pattern(&self) -> Option<&Pattern> {
        self.arguments.pattern()
    }

    pub fn attributes(&self) -> &[Attribute] {
        self.arguments.attributes()
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} = {}", self.identifier, self.arguments)
    }
}
