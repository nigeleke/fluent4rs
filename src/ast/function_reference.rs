use super::{CallArguments, Identifier};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [FunctionReference](crate::ast::FunctionReference) ::= [Identifier](crate::ast::Identifier) [CallArguments](crate::ast::CallArguments)
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

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn identifier_name(&self) -> String {
        self.identifier.to_string()
    }

    pub fn call_arguments(&self) -> &CallArguments {
        &self.call_arguments
    }
}

#[cfg(feature = "walker")]
impl Walkable for FunctionReference {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_function_reference(self);
        Walker::walk(&self.identifier, visitor);
        Walker::walk(&self.call_arguments, visitor);
    }
}

impl std::fmt::Display for FunctionReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.identifier, self.call_arguments)
    }
}
