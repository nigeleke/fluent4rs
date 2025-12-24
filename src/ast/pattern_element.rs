use super::{BlockPlaceable, BlockText, InlinePlaceable, InlineText};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [PatternElement](crate::ast::PatternElement) ::= [inline_text](crate::ast::InlineText)
///    | [block_text](crate::ast::BlockText)
///    | inline_placeable
///    | block_placeable
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PatternElement {
    #[doc(hidden)]
    InlineText(InlineText),

    #[doc(hidden)]
    BlockText(BlockText),

    #[doc(hidden)]
    InlinePlaceable(InlinePlaceable),

    #[doc(hidden)]
    BlockPlaceable(BlockPlaceable),
}

#[cfg(feature = "walker")]
impl Walkable for PatternElement {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_pattern_element(self);
        match self {
            Self::InlineText(_text) => {}
            Self::BlockText(_block) => {}
            Self::InlinePlaceable(text) => Walker::walk(text, visitor),
            Self::BlockPlaceable(block) => Walker::walk(block, visitor),
        }
    }
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
