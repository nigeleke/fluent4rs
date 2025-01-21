use super::prelude::{Identifier, Pattern};

#[derive(Debug)]
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
