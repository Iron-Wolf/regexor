//! Binary-level tests: run the built `regexor` executable and check its
//! stdout / stderr / exit status, the way a shell user would.

use assert_cmd::Command;
use predicates::str::contains;

fn regexor() -> Command {
    Command::cargo_bin("regexor").expect("binary `regexor` is built")
}

#[test]
fn prints_the_translated_regex_and_exits_zero() {
    regexor()
        .args(["--input", "ftp [-pinegvd] [HOST]"])
        .assert()
        .success()
        .stdout("ftp *(-[pinegvd]+ *)? *([a-zA-Z0-9_.]+ *)? *\n");
}

#[test]
fn rejects_a_missing_input_argument() {
    regexor().assert().failure().stderr(contains("--input"));
}

