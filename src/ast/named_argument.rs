use super::prelude::{Identifier, Literal};

#[derive(Clone, Debug)]
pub struct NamedArgument {
    identifier: Identifier,
    literal: Literal,
}

impl NamedArgument {
    pub fn new(identifier: Identifier, literal: Literal) -> Self {
        Self {
            identifier,
            literal,
        }
    }
}
