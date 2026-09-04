mod parser;
mod token;

use crate::cli::Format;
use crate::error::Error;
use parser::parse;
use token::Token;

/// Translate a usage string into a regex that matches a valid invocation.
///
/// # Errors
///
/// Returns [`Error::AmbiguousWord`] when a word is neither all-lowercase nor
/// all-uppercase, and [`Error::UnsupportedFormat`] for a format that has no
/// translator yet.
pub fn translate(usage: &str, format: Format) -> Result<String, Error> {
    match format {
        Format::Man => render(&parse(usage)),
        Format::Psql => Err(Error::UnsupportedFormat(Format::Psql)),
    }
}

/// Regex fragment for one token, always trailed by ` *`.
fn to_regex(token: &Token) -> Result<String, Error> {
    let fragment = match token {
        Token::Optional(inner) => format!("({})? *", render(inner)?),
        Token::Flags(chars) => format!("-[{chars}]+ *"),
        Token::Text(word) => format!("{} *", word_to_regex(word)?),
    };
    Ok(fragment)
}

/// Concatenate the regex fragments of every token.
fn render(tokens: &[Token]) -> Result<String, Error> {
    tokens.iter().map(to_regex).collect()
}

/// Translate a single bare word into a regex fragment.
///
/// * all-lowercase words are literal text and returned unchanged
/// * all-uppercase words are replaceable placeholders
/// * a mixed-case word is [`Error::AmbiguousWord`]
fn word_to_regex(word: &str) -> Result<String, Error> {
    if word.to_lowercase() == word {
        Ok(word.to_string())
    } else if word.to_uppercase() == word {
        Ok(String::from("[a-zA-Z0-9_.]+"))
    } else {
        Err(Error::AmbiguousWord(word.to_string()))
    }
}
