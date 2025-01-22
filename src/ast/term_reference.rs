// TermReference       ::= "-" Identifier AttributeAccessor? CallArguments?

use super::prelude::{AttributeAccessor, CallArguments, Identifier};

#[derive(Clone, Debug, PartialEq)]
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

impl std::fmt::Display for TermReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-{}{}{}{}",
            self.identifier,
            self.attribute_accessor
                .as_ref()
                .map_or("".to_string(), |aa| aa.to_string()),
            if self.call_arguments.is_some() {
                " "
            } else {
                ""
            },
            self.call_arguments
                .as_ref()
                .map_or("".to_string(), |ca| ca.to_string())
        )
    }
}
