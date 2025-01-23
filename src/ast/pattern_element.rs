use super::prelude::{BlockPlaceable, BlockText, InlinePlaceable, InlineText};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash, Eq))]
pub enum PatternElement {
    InlineText(InlineText),
    BlockText(BlockText),
    InlinePlaceable(InlinePlaceable),
    BlockPlaceable(BlockPlaceable),
}

impl std::fmt::Display for PatternElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::InlineText(text) => text.to_string(),
            Self::BlockText(block) => block.to_string(),
            Self::InlinePlaceable(text) => text.to_string(),
            Self::BlockPlaceable(block) => block.to_string(),
        };
        write!(f, "{value}")
    }
}