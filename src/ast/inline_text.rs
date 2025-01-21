#[derive(Clone, Debug)]
pub struct InlineText(String);

impl From<&str> for InlineText {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
