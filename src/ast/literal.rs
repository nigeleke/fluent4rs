use super::prelude::{NumberLiteral, StringLiteral};

#[derive(Clone, Debug)]
pub enum Literal {
    Number(NumberLiteral),
    String(StringLiteral),
}

impl From<NumberLiteral> for Literal {
    fn from(value: NumberLiteral) -> Self {
        Self::Number(value)
    }
}

impl From<StringLiteral> for Literal {
    fn from(value: StringLiteral) -> Self {
        Self::String(value)
    }
}
