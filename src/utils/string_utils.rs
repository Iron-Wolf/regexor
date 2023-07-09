/// The input `String` is splitted in smaller parts and returned in a `Vector`.
/// The result contain each part of the command, to be easily processed 
/// for regex translation.
pub fn split_string(cmd_string: String) -> Vec<String> {
    let mut result = vec![];

    let words: Vec<&str> = cmd_string.split(' ').collect();
    let mut group = String::from("");
    
    for w in words {
        if w.ends_with(']') || w.ends_with("...") {
            group.push_str(w);
            result.push(group.clone());
            group.clear();
        }
        else if w.starts_with('[') {
            group.push_str(w);
            group.push(' ');
        }
        else {
            result.push(w.to_string());
        }
    }
    return result;
}

/// Remove the first and last characters of the input `String`.
/// Returns an empty string for **zero** or **one-sized** strings 
/// and handle multi-byte unicode characters correctly.
pub fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}