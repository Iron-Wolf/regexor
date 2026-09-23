use super::token::Token;

/// Split a usage string into tokens.\
/// The `[ ... ]` groups are parsed recursively.
pub fn parse(usage: &str) -> Vec<Token> {
    split_usage(usage)
        .into_iter()
        .map(|part| {
            // A closing bracket always comes paired with an opening one,
            // so testing the first character is enough.
            if part.starts_with('[') {
                Token::Optional(parse(remove_first_and_last(&part)))
            } else if let Some(flags) = part.strip_prefix('-') {
                Token::Flags(flags.to_string())
            } else {
                Token::Text(part)
            }
        })
        .collect()
}

/// Split a usage string into its component parts.
///
/// Each `[ ... ]` group (and any trailing `...` repetition marker) is kept
/// together as a single part, so it can be translated as a unit.
fn split_usage(usage: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut group = String::new();

    for word in usage.split(' ') {
        if word.ends_with(']') || word.ends_with("...") {
            group.push_str(word);
            // Move `group` into `parts` and reset it with the default value
            parts.push(std::mem::take(&mut group));
        } else if word.starts_with('[') {
            group.push_str(word);
            group.push(' ');
        } else {
            parts.push(word.to_string());
        }
    }

    parts
}

/// Remove the first and last characters of `value`.
///
/// Returns an empty string for zero- or one-character inputs, and handles
/// multibyte Unicode characters correctly.
fn remove_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_splits_flags_and_optional_groups() {
        let tokens = parse("ftp [-pinegvd] [HOST]");

        assert_eq!(
            tokens,
            vec![
                Token::Text("ftp".to_string()),
                Token::Optional(vec![Token::Flags("pinegvd".to_string())]),
                Token::Optional(vec![Token::Text("HOST".to_string())]),
            ]
        );
    }

    #[test]
    fn split_usage_keeps_bracketed_group_together() {
        let parts = split_usage("cmd [-o OUTPUT]");

        assert_eq!(parts, vec!["cmd", "[-o OUTPUT]"]);
    }

    #[test]
    fn remove_first_and_last_strips_surrounding_brackets() {
        assert_eq!(remove_first_and_last("[HOST]"), "HOST");
    }
}
