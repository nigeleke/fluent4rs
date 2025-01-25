use super::prelude::{CallArguments, Identifier};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FunctionReference {
    identifier: Identifier,
    call_arguments: CallArguments,
}

impl FunctionReference {
    pub fn new(identifier: Identifier, call_arguments: CallArguments) -> Self {
        Self {
            identifier,
            call_arguments,
        }
    }
}

impl std::fmt::Display for FunctionReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.identifier, self.call_arguments)
    }
}
