use super::prelude::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeAccessor(Identifier);

impl From<Identifier> for AttributeAccessor {
    fn from(value: Identifier) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for AttributeAccessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".{}", self.0)
    }
}
