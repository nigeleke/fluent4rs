#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Junk(Vec<String>);

impl From<&[String]> for Junk {
    fn from(value: &[String]) -> Self {
        Self(Vec::from(value))
    }
}

#[cfg(feature = "walker")]
impl Walkable for Junk {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_junk(self);
    }
}

impl std::fmt::Display for Junk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self
            .0
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{value}")
    }
}
