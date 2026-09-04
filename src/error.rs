use std::fmt;

use crate::cli::Format;

/// Something that prevented a usage string from being translated.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// A word mixed upper- and lower-case, so it is neither a literal word
    /// nor an uppercase placeholder.
    AmbiguousWord(String),
    /// The format is recognised but has no translator yet.
    UnsupportedFormat(Format),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AmbiguousWord(word) => {
                write!(
                    f,
                    "word `{word}` is neither all-lowercase nor all-uppercase"
                )
            }
            Error::UnsupportedFormat(format) => {
                write!(f, "the {format:?} format is not supported yet")
            }
        }
    }
}

impl std::error::Error for Error {}
