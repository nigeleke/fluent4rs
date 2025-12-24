use super::{CommentLine, Message, Term};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Entry](crate::ast::Entry) ::= ([Message](crate::ast::Message) line_end)
///  | ([Term](crate::ast::Term) line_end)
///  | [CommentLine](crate::ast::CommentLine)
///
/// Entries are the main building blocks of Fluent.
///
/// They define translations and contextual and semantic information about the
/// translations. During the AST construction, adjacent comment lines of the same
/// comment type (defined by the number of #) are joined together. Single-# comments
/// directly preceding [Message](crate::ast::Message)s and [Term](crate::ast::Term)s
/// are attached to the [Message](crate::ast::Message) or [Term](crate::ast::Term)
/// and are not standalone Entries.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Entry {
    #[doc(hidden)]
    Message(Message),

    #[doc(hidden)]
    Term(Term),

    #[doc(hidden)]
    CommentLine(CommentLine),
}

#[cfg(feature = "walker")]
impl Walkable for Entry {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_entry(self);
        match self {
            Self::Message(message) => Walker::walk(message, visitor),
            Self::Term(term) => Walker::walk(term, visitor),
            Self::CommentLine(comment) => Walker::walk(comment, visitor),
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Message(message) => message.to_string(),
            Self::Term(term) => term.to_string(),
            Self::CommentLine(line) => line.to_string(),
        };
        write!(f, "{value}")
    }
}
