use thiserror::Error;

#[derive(Debug, Error)]
/// Represents errors that can occur during parsing of a Fluent Translation List (FTL) resource.
pub enum Fluent4rsError {
    /// Error raised by pom parser duing parsing of fluent file text.
    #[cfg(feature = "parser-pom")]
    #[error(transparent)]
    PomParser(#[from] pom::Error),

    /// Error raised by chumsky parser duing parsing of fluent file text.
    #[cfg(feature = "parser-chumsky")]
    #[error("{0}")]
    ChumskyParser(String),

    /// Parsing completed successully according to the fluent grammar, but Junk entries
    /// found in resultant AST. By default these are treated as an error in fluent4rs.
    /// Use `parse_with_junk` to include these AST elements in the final tree.
    #[error("Unwanted junk found in Fluent grammar: {0}")]
    UnwantedJunk(String),
}
