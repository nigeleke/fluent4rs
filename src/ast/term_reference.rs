// TermReference       ::= "-" Identifier AttributeAccessor? CallArguments?

use super::prelude::{AttributeAccessor, CallArguments, Identifier};

#[derive(Clone, Debug)]
pub struct TermReference {
    identifier: Identifier,
    attribute_accessor: Option<AttributeAccessor>,
    call_arguments: Option<CallArguments>,
}

impl TermReference {
    pub fn new(
        identifier: Identifier,
        attribute_accessor: Option<AttributeAccessor>,
        call_arguments: Option<CallArguments>,
    ) -> Self {
        Self {
            identifier,
            attribute_accessor,
            call_arguments,
        }
    }
}
