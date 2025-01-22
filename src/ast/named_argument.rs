use super::prelude::{Identifier, Literal};

#[derive(Clone, Debug, PartialEq)]
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

impl std::fmt::Display for NamedArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.identifier, self.literal)
    }
}
