use super::prelude::{CommentLine, Message, Term};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Entry {
    Message(Message),
    Term(Term),
    CommentLine(CommentLine),
}

#[cfg(feature = "walker")]
impl Walkable for Entry {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_entry(self);
        match self {
            Self::Message(message) => message.walk(visitor),
            Self::Term(term) => term.walk(visitor),
            Self::CommentLine(comment) => comment.walk(visitor),
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
