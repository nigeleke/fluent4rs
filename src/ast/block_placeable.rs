use super::prelude::InlinePlaceable;

#[derive(Clone, Debug)]
pub struct BlockPlaceable(InlinePlaceable);

impl From<InlinePlaceable> for BlockPlaceable {
    fn from(value: InlinePlaceable) -> Self {
        Self(value)
    }
}
