use super::PatternElement;

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Pattern(Vec<PatternElement>);

impl From<&[PatternElement]> for Pattern {
    fn from(value: &[PatternElement]) -> Self {
        Self(Vec::from(value))
    }
}

#[cfg(feature = "walker")]
impl Walkable for Pattern {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_pattern(self);
        self.0.iter().for_each(|pe| pe.walk(visitor));
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self
            .0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{value}")
    }
}
