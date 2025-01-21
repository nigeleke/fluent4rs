use super::prelude::{InlineExpression, SelectExpression};

#[derive(Clone, Debug)]
pub enum InlinePlaceable {
    SelectExpression(SelectExpression),
    InlineExpression(InlineExpression),
}
