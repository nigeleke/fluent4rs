use super::prelude::Argument;

#[derive(Clone, Debug)]
pub struct CallArguments(Vec<Argument>);

impl From<&[Argument]> for CallArguments {
    fn from(value: &[Argument]) -> Self {
        Self(Vec::from(value))
    }
}
