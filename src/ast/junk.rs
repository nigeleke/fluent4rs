#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Junk](crate::ast::Junk) ::= junk_line (junk_line - "#" - "-" - [a-zA-Z])*
///
/// Junk represents unparsed content.
///
/// Junk is parsed line-by-line until a line is found which looks like it might
/// be a beginning of a new message, term, or a comment. Any whitespace
/// following a broken Entry is also considered part of Junk.
///
/// [Parser::parse](crate::parser::Parser::parse) treats Junk as a
/// [Fluent4rsError](crate::error::Fluent4rsError).
///
/// [Parser::parse_with_junk](crate::parser::Parser::parse_with_junk) will return [Junk](crate::ast::Junk)
/// in the [Resource](crate::ast::Resource).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Junk(Vec<String>);

impl From<&[String]> for Junk {
    fn from(value: &[String]) -> Self {
        Self(Vec::from(value))
    }
}

#[cfg(feature = "walker")]
impl Walkable for Junk {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_junk(self);
    }
}

impl std::fmt::Display for Junk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self
            .0
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{value}")
    }
}
