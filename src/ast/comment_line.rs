#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [CommentLine](crate::ast::CommentLine) ::= ("###" | "##" | "#") ("\u0020" comment_char*)? line_end
///
/// Adjacent comment lines of the same comment type are joined together during
/// the AST construction.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CommentLine {
    lead: String,
    comment: Option<String>,
}

impl CommentLine {
    /// Constructs a new `CommentLine` representing a comment line in a Fluent Translation List (FTL) file.
    ///
    /// # Arguments
    /// * `lead` - The leading part of the line, including the `#` symbols and any immediate following whitespace.
    /// * `comment` - The optional comment text after the initial whitespace following the `#` symbols.
    ///               If `None`, the line consists only of the leading hashes (an "empty" comment).
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
