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
