use super::prelude::InlinePlaceable;

#[derive(Clone, Debug, PartialEq)]
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

impl std::fmt::Display for BlockPlaceable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.blank_block, self.inline_placeable)
    }
}
