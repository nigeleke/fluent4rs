use super::ast::prelude::Resource;

use thiserror::*;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("failed to parse text; reason: {0}")]
    FailedToParse(String),
}

type Result<T> = std::result::Result<T, ParserError>;

pub struct Parser;

impl Parser {
    pub fn parse(text: &str) -> Result<Resource> {
        super::grammar::resource()
            .parse(text.as_bytes())
            .map(|p| {
                println!("parsed {:#?}", p);
                p
            })
            .map_err(|e| ParserError::FailedToParse(e.to_string()))
            .and_then(junk_as_error)
    }
}

fn junk_as_error(resource: Resource) -> Result<Resource> {
    let junk = resource.junk();
    if junk.is_empty() {
        Ok(resource)
    } else {
        let with_errors = format!(
            "Invalid entries: {}",
            junk.iter()
                .map(|j| j.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );
        Err(ParserError::FailedToParse(with_errors))
    }
}
