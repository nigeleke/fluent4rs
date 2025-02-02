use super::{CallArguments, Identifier};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable};

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

#[cfg(feature = "walker")]
impl Walkable for FunctionReference {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_function_reference(self);
        self.identifier.walk(visitor);
        self.call_arguments.walk(visitor);
    }
}

impl std::fmt::Display for FunctionReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.identifier, self.call_arguments)
    }
}
