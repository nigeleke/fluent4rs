use super::prelude::{InlineExpression, NamedArgument};

#[derive(Clone, Debug)]
pub enum Argument {
    NamedArgument(NamedArgument),
    InlineExpression(InlineExpression),
}
