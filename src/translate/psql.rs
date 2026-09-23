use crate::cli::Format;
use crate::error::Error;

/// Translate a `psql` meta-command usage string into a regex.
///
/// # Errors
///
/// Always returns [`Error::UnsupportedFormat`]; this translator does not
/// exist yet.
pub fn translate(_usage: &str) -> Result<String, Error> {
    Err(Error::UnsupportedFormat(Format::Psql))
}
