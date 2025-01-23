use super::prelude::{CommentLine, Message, Term};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash, Eq))]
pub enum Entry {
    Message(Message),
    Term(Term),
    CommentLine(CommentLine),
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
