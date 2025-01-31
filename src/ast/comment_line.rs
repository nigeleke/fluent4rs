#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CommentLine {
    lead: String,
    comment: Option<String>,
}

impl CommentLine {
    pub fn new(lead: String, comment: Option<String>) -> Self {
        Self { lead, comment }
    }
}

#[cfg(feature = "walker")]
impl Walkable for CommentLine {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_comment_line(self);
    }
}

impl std::fmt::Display for CommentLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let comment = self.comment.as_ref().map_or("".into(), |c| format!(" {c}"));
        writeln!(f, "{}{}", self.lead, comment)
    }
}
