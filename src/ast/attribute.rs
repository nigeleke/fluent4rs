use super::prelude::{Identifier, Pattern};

#[derive(Debug, PartialEq)]
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
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
