use super::prelude::{InlineExpression, NamedArgument};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "hash", derive(Hash, Eq))]
pub enum Argument {
    NamedArgument(NamedArgument),
    InlineExpression(InlineExpression),
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::NamedArgument(argument) => argument.to_string(),
            Self::InlineExpression(argument) => argument.to_string(),
        };
        write!(f, "{value}")
    }
}
