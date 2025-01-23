#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash, Eq))]
pub struct CommentLine {
    lead: String,
    comment: Option<String>,
}

impl CommentLine {
    pub fn new(lead: String, comment: Option<String>) -> Self {
        Self { lead, comment }
    }
}

impl std::fmt::Display for CommentLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let comment = self.comment.as_ref().map_or("".into(), |c| format!(" {c}"));
        writeln!(f, "{}{}", self.lead, comment)
    }
}
