use super::prelude::Identifier;

#[derive(Clone, Debug)]
pub struct AttributeAccessor(Identifier);

impl From<Identifier> for AttributeAccessor {
    fn from(value: Identifier) -> Self {
        Self(value)
    }
}
