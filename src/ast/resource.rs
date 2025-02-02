use super::prelude::{Entry, Junk};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ResourceItem {
    Entry(Entry),
    BlankBlock(String),
    Junk(Junk),
}

#[cfg(feature = "walker")]
impl Walkable for ResourceItem {
    fn walk(&self, visitor: &mut dyn Visitor) {
        match self {
            Self::Entry(entry) => entry.walk(visitor),
            Self::Junk(junk) => {
                println!("Visiting junk {}", junk);

                junk.walk(visitor)
            }
            _ => {}
        }
    }
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

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Resource(Vec<ResourceItem>);

impl Resource {
    pub fn entries(&self) -> Vec<&Entry> {
        self.0
            .iter()
            .filter_map(|ri| match ri {
                ResourceItem::Entry(entry) => Some(entry),
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn junk(&self) -> Vec<&Junk> {
        self.0
            .iter()
            .filter_map(|ri| match ri {
                ResourceItem::Junk(junk) => Some(junk),
                _ => None,
            })
            .collect::<Vec<_>>()
    }
}

impl From<Vec<ResourceItem>> for Resource {
    fn from(value: Vec<ResourceItem>) -> Self {
        Self(value)
    }
}

#[cfg(feature = "walker")]
impl Walkable for Resource {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_resource(self);
        self.0.iter().for_each(|e| e.walk(visitor));
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
