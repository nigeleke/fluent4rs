use super::prelude::{
    FunctionReference, InlinePlaceable, MessageReference, NumberLiteral, StringLiteral,
    TermReference, VariableReference,
};

#[derive(Clone, Debug)]
pub enum InlineExpression {
    StringLiteral(StringLiteral),
    NumberLiteral(NumberLiteral),
    FunctionReference(FunctionReference),
    MessageReference(MessageReference),
    TermReference(TermReference),
    VariableReference(VariableReference),
    InlinePlaceable(Box<InlinePlaceable>),
}
