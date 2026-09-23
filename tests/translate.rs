//! public function tests: access function that are
//! available outside the crate

use regexor::{translate, Error, Format};

#[test]
fn translates_man_usage_strings() {
    let cases = [
        (
            "ftp [-pinegvd] [HOST]",
            "ftp *(-[pinegvd]+ *)? *([a-zA-Z0-9_.]+ *)? *",
        ),
        (
            "ftp [-46AadefginpRtVv] [-N NETRC] [-o OUTPUT]",
            "ftp *(-[46AadefginpRtVv]+ *)? *(-[N]+ *[a-zA-Z0-9_.]+ *)? *(-[o]+ *[a-zA-Z0-9_.]+ *)? *",
        ),
    ];

    for (input, expected) in cases {
        let result = translate(input, Format::Man).expect("should always succeed");
        assert_eq!(result, expected);
    }
}

#[test]
fn rejects_unimplemented_formats() {
    assert_eq!(
        translate("cmd", Format::Psql),
        Err(Error::UnsupportedFormat(Format::Psql)),
    );
}
