use super::{Attribute, Identifier, Pattern};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

#[cfg(feature = "walker")]
impl Walkable for MessageArguments {
    fn walk(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::Patterned(pattern, attributes) => {
                Walker::walk(pattern, visitor);
                attributes.iter().for_each(|a| Walker::walk(a, visitor));
            }
            Self::Plain(attributes) => attributes.iter().for_each(|a| Walker::walk(a, visitor)),
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

/// [Message](crate::ast::Message) ::= [Identifier](crate::ast::Identifier) blank_inline? "=" blank_inline? (([Pattern](crate::ast::Pattern) [Attribute](crate::ast::Attribute)*) | (Attribute+))
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

    /// Returns the message identifier.
    ///
    ///  Note: a [Message](crate::ast::Message) and [Term](crate::ast::Term) [Identifier](crate::ast::Identifier)
    ///  may be the same, e,g, `product = ...` versus `-product = ...`.
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    /// Returns the message identifier _name_.
    ///
    /// Note: Differentiates the [Message](crate::ast::Message) and [Term](crate::ast::Term)
    /// [Identifier](crate::ast::Identifier) name by using the '-' prefix for the [Term](crate::ast::Term).
    pub fn identifier_name(&self) -> String {
        self.identifier.to_string()
    }

    /// Returns the message [Pattern](crate::ast::Pattern), if provided.
    pub fn pattern(&self) -> Option<&Pattern> {
        self.arguments.pattern()
    }

    /// Returns the message [Attribute](crate::ast::Attribute)s.
    pub fn attributes(&self) -> &[Attribute] {
        self.arguments.attributes()
    }
}

#[cfg(feature = "walker")]
impl Walkable for Message {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_message(self);
        Walker::walk(&self.identifier, visitor);
        Walker::walk(&self.arguments, visitor);
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} = {}", self.identifier, self.arguments)
    }
}
