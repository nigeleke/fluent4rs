use super::prelude::Identifier;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
pub struct VariableReference(Identifier);

impl From<Identifier> for VariableReference {
    fn from(value: Identifier) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for VariableReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}
