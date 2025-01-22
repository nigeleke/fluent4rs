use super::prelude::{Attribute, Identifier, Pattern};

#[derive(Debug, PartialEq)]
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

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attributes = self
            .attributes
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(
            f,
            "-{} = {}{}{}\n",
            self.identifier,
            self.pattern,
            attributes.is_empty().then_some("").unwrap_or(" "),
            attributes
        )
    }
}
