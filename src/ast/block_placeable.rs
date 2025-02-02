use super::InlinePlaceable;

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BlockPlaceable {
    blank_block: String,
    inline_placeable: InlinePlaceable,
}

impl BlockPlaceable {
    pub fn new(blank_block: String, inline_placeable: InlinePlaceable) -> Self {
        Self {
            blank_block,
            inline_placeable,
        }
    }
}

#[cfg(feature = "walker")]
impl Walkable for BlockPlaceable {
    fn walk(&self, visitor: &mut dyn Visitor) {
        self.inline_placeable.walk(visitor);
    }
}

impl std::fmt::Display for BlockPlaceable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.blank_block, self.inline_placeable)
    }
}
