#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// block_text ::= blank_block blank_inline indented_char inline_text?
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BlockText(String);

impl From<&str> for BlockText {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for BlockText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
