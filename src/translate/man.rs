mod parser;
mod token;

use crate::error::Error;
use parser::parse;
use token::Token;

/// Translate a `man(1)` / `--help` style usage string into a regex that
/// matches a valid invocation.
///
/// # Errors
///
/// Returns [`Error::AmbiguousWord`] when a word is neither all-lowercase nor
/// all-uppercase.
pub fn translate(usage: &str) -> Result<String, Error> {
    render(&parse(usage))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_regex_wraps_optional_token_in_a_group() {
        let token = Token::Optional(vec![Token::Flags("p".to_string())]);

        assert_eq!(to_regex(&token), Ok("(-[p]+ *)? *".to_string()));
    }

    #[test]
    fn render_concatenates_token_fragments() {
        let tokens = vec![
            Token::Text("ftp".to_string()),
            Token::Flags("p".to_string()),
        ];

        assert_eq!(render(&tokens), Ok("ftp *-[p]+ *".to_string()));
    }

    #[test]
    fn word_to_regex_rejects_mixed_case_word() {
        assert_eq!(word_to_regex("HOST"), Ok("[a-zA-Z0-9_.]+".to_string()));
    }
}
