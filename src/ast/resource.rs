use super::prelude::{Entry, Junk};

#[derive(Debug, PartialEq)]
pub enum ResourceItem {
    Entry(Entry),
    BlankBlock(String),
    Junk(Junk),
}

impl std::fmt::Display for ResourceItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Entry(entry) => entry.to_string(),
            Self::BlankBlock(block) => block.to_string(),
            Self::Junk(junk) => junk.to_string(),
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, PartialEq)]
pub struct Resource(Vec<ResourceItem>);

impl From<Vec<ResourceItem>> for Resource {
    fn from(value: Vec<ResourceItem>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = self
            .0
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{stringified}")
    }
}