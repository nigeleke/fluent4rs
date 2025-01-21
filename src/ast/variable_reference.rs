use super::prelude::Identifier;

#[derive(Clone, Debug)]
pub struct VariableReference(Identifier);

impl From<Identifier> for VariableReference {
    fn from(value: Identifier) -> Self {
        Self(value)
    }
}
