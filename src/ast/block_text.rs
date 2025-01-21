#[derive(Clone, Debug)]
pub struct BlockText(String);

impl From<&str> for BlockText {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
