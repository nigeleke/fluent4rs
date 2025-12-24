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
    /// Constructs a new `FunctionReference` representing a built-in or custom function call in Fluent.
    /// Functions are used for advanced formatting, such as `NUMBER(value, style: "percent")`
    /// or `DATETIME(date, month: "long")`.
    ///
    /// # Arguments
    /// * `identifier` - The name of the function (e.g., `NUMBER`, `PLATFORM`, or a custom function).
    /// * `call_arguments` - The arguments passed to the function, which may include positional
    ///                      and/or named arguments.
    pub fn new(identifier: Identifier, call_arguments: CallArguments) -> Self {
        Self {
            identifier,
            call_arguments,
        }
    }

    /// Returns a reference to the function identifier.
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    /// Returns the string name of the function identifier.
    ///
    /// This is equivalent to `self.identifier.to_string()` and is useful when serializing
    /// or displaying the function call.
    pub fn identifier_name(&self) -> String {
        self.identifier.to_string()
    }

    /// Returns a reference to the call arguments of this function reference.
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
