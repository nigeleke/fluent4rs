use super::{AttributeAccessor, CallArguments, Identifier};

#[cfg(feature = "walker")]
use crate::walker::{Visitor, Walkable, Walker};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [TermReference](crate::ast::TermReference) ::= "-" [Identifier](crate::ast::Identifier) [AttributeAccessor](crate::ast::AttributeAccessor)? [CallArguments](crate::ast::CallArguments)?
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Eq, PartialOrd, Ord, Hash))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

    /// Returns the message identifier.
    ///
    ///  Note: a [MessageReference](crate::ast::MessageReference) and [TermReference](crate::ast::TermReference) [Identifier](crate::ast::Identifier)
    ///  may be the same, e,g, `product = ...` versus `-product = ...`.
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    /// Returns the message identifier _name_.
    ///
    /// Note: Differentiates the [MessageReference](crate::ast::MessageReference) and [TermReference](crate::ast::TermReference)
    /// [Identifier](crate::ast::Identifier) name by using the '-' prefix for the [TermReference](crate::ast::TermReference).
    pub fn identifier_name(&self) -> String {
        format!("-{}", self.identifier)
    }

    pub fn attribute_accessor(&self) -> Option<&AttributeAccessor> {
        self.attribute_accessor.as_ref()
    }

    pub fn call_arguments(&self) -> Option<&CallArguments> {
        self.call_arguments.as_ref()
    }
}

#[cfg(feature = "walker")]
impl Walkable for TermReference {
    fn walk(&self, visitor: &mut dyn Visitor) {
        visitor.visit_term_reference(self);
        Walker::walk(&self.identifier, visitor);
        self.attribute_accessor
            .iter()
            .for_each(|aa| Walker::walk(aa, visitor));
        self.call_arguments
            .iter()
            .for_each(|ca| Walker::walk(ca, visitor));
    }
}

impl std::fmt::Display for TermReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-{}{}{}",
            self.identifier,
            self.attribute_accessor
                .as_ref()
                .map_or("".to_string(), |aa| aa.to_string()),
            self.call_arguments
                .as_ref()
                .map_or("".to_string(), |ca| ca.to_string())
        )
    }
}
