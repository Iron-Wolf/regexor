#[cfg(test)]
mod tests {
    use super::super::*; // Import content of parent file (main.rs)

    #[test]
    fn test_split_and_translate() {
        // prepare
        let test_cases = vec![
            ("ftp [-pinegvd] [HOST]",
             "ftp *(-[pinegvd]+ *)? *([a-zA-Z0-9_.]+ *)? *"),
            ("ftp [-46AadefginpRtVv] [-N NETRC] [-o OUTPUT]",
             "ftp *(-[46AadefginpRtVv]+ *)? *(-[N]+ *[a-zA-Z0-9_.]+ *)? *(-[o]+ *[a-zA-Z0-9_.]+ *)? *"),
        ];

        for (input, expected) in test_cases {
            // run
            let output = translate_man(input.to_string());
            // verify
            assert_eq!(output, expected);
        }
    }
}