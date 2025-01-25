use super::prelude::Argument;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CallArguments(Vec<Argument>);

impl From<&[Argument]> for CallArguments {
    fn from(value: &[Argument]) -> Self {
        Self(Vec::from(value))
    }
}

impl std::fmt::Display for CallArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arguments = self
            .0
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "({arguments})")
    }
}
