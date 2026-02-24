#[cfg(feature = "parser-chumsky")]
mod chumsky;

#[cfg(feature = "parser-pom")]
mod pom;

#[cfg(feature = "parser-chumsky")]
pub use chumsky::*;
#[cfg(feature = "parser-pom")]
pub use pom::*;

#[cfg(all(feature = "parser-pom", feature = "parser-chumsky"))]
compile_error!(
    "Features 'parser-pom' and 'parser-chumsky' are mutually exclusive. Enable only one."
);
