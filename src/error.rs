use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to parse text; reason: {0}")]
    FailedToParse(String),
}
