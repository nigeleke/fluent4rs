use super::prelude::InlinePlaceable;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockPlaceable(InlinePlaceable);

impl From<InlinePlaceable> for BlockPlaceable {
    fn from(value: InlinePlaceable) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for BlockPlaceable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
