/// Return the regex translation of the input String
pub fn reg_str(str_part: String) -> String {
    if str_part.to_lowercase() == str_part {
        // literal text
        return str_part.clone();
    } else if str_part.to_uppercase() == str_part {
        // replaceable text
        return String::from("[a-zA-Z0-9_.]+");
    } else {
        return "".to_string();
    }
}