//! Library-level tests: exercise the public `translate` API directly.

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
        assert_eq!(translate(input, Format::Man).unwrap(), expected);
    }
}

#[test]
fn rejects_unimplemented_formats() {
    assert_eq!(
        translate("cmd", Format::Psql),
        Err(Error::UnsupportedFormat(Format::Psql)),
    );
}
