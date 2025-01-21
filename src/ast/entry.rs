use super::prelude::{CommentLine, Message, Term};

#[derive(Debug)]
pub enum Entry {
    Message(Message),
    Term(Term),
    CommentLine(CommentLine),
}
