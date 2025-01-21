use super::prelude::{BlockPlaceable, BlockText, InlinePlaceable, InlineText};

#[derive(Clone, Debug)]
pub enum PatternElement {
    InlineText(InlineText),
    BlockText(BlockText),
    InlinePlaceable(InlinePlaceable),
    BlockPlaceable(BlockPlaceable),
}
