#[cfg(feature = "parser-chumsky")]
mod chumsky;

#[cfg(feature = "parser-pom")]
mod pom;

#[cfg(feature = "parser-chumsky")]
pub use chumsky::*;
#[cfg(feature = "parser-pom")]
pub use pom::*;

#[cfg(all(feature = "parser-pom", feature = "parser-chumsky"))]
compile_error!("Use one feature 'parser-pom' or 'parser-chumsky' only.");

#[cfg(not(any(feature = "parser-pom", feature = "parser-chumsky")))]
compile_error!("One feature 'parser-pom' or 'parser-chumsky' must be enabled.");
