#[derive(Debug)]
pub struct CommentLine {
    lead: String,
    comment: Option<String>,
}

impl CommentLine {
    pub fn new(lead: String, comment: Option<String>) -> Self {
        Self { lead, comment }
    }
}
