use super::Argument;

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [CallArguments](crate::ast::CallArguments) ::= blank? "(" blank? argument_list blank? ")"
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CallArguments(Vec<Argument>);

impl CallArguments {
    pub fn arguments(&self) -> &[Argument] {
        self.0.as_slice()
    }
}

impl From<&[Argument]> for CallArguments {
    fn from(value: &[Argument]) -> Self {
        Self(Vec::from(value))
    }
}

#[cfg(feature = "walker")]
impl Walkable for CallArguments {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_call_arguments(self);
        self.0.iter().for_each(|a| Walker::walk(a, visitor));
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
