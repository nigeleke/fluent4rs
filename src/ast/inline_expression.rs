use super::prelude::{
    FunctionReference, InlinePlaceable, MessageReference, NumberLiteral, StringLiteral,
    TermReference, VariableReference,
};

#[derive(Clone, Debug, PartialEq)]
pub enum InlineExpression {
    StringLiteral(StringLiteral),
    NumberLiteral(NumberLiteral),
    FunctionReference(FunctionReference),
    MessageReference(MessageReference),
    TermReference(TermReference),
    VariableReference(VariableReference),
    InlinePlaceable(Box<InlinePlaceable>),
}

impl std::fmt::Display for InlineExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::StringLiteral(literal) => literal.to_string(),
            Self::NumberLiteral(literal) => literal.to_string(),
            Self::FunctionReference(_reference) => unimplemented!(),
            Self::MessageReference(_reference) => unimplemented!(),
            Self::TermReference(reference) => reference.to_string(),
            Self::VariableReference(reference) => reference.to_string(),
            Self::InlinePlaceable(inline) => inline.to_string(),
        };
        write!(f, "{{ {value} }}")
    }
}
