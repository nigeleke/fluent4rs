use super::prelude::{CallArguments, Identifier};

#[derive(Clone, Debug, PartialEq)]
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
