use super::prelude::{Attribute, Identifier, Pattern};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash, Eq))]
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

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attributes = self
            .attributes
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("");
        writeln!(f, "-{} = {}{}", self.identifier, self.pattern, attributes)
    }
}
