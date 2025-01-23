use super::prelude::{Identifier, Pattern};

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    identifier: Identifier,
    pattern: Pattern,
}

impl Attribute {
    pub fn new(identifier: Identifier, pattern: Pattern) -> Self {
        Self {
            identifier,
            pattern,
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n    .{} = {}", self.identifier, self.pattern)
    }
}
