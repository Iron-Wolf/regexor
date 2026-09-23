mod man;
mod psql;

use crate::cli::Format;
use crate::error::Error;

/// Translate a usage string into a regex that matches a valid invocation.
///
/// # Errors
///
/// Returns [`Error::AmbiguousWord`] when a word is neither all-lowercase nor
/// all-uppercase, and [`Error::UnsupportedFormat`] for a format that has no
/// translator yet.
pub fn translate(usage: &str, format: Format) -> Result<String, Error> {
    match format {
        Format::Man => man::translate(usage),
        Format::Psql => psql::translate(usage),
    }
}
