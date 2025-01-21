use super::prelude::{Attribute, Identifier, Pattern};

#[derive(Debug)]
pub struct Term {
    identifier: Identifier,
    pattern: Pattern,
    attributes: Vec<Attribute>,
}

impl Term {
    pub fn new(identifier: Identifier, pattern: Pattern, attributes: Vec<Attribute>) -> Self {
        Self {
            identifier,
            pattern,
            attributes,
        }
    }
}
