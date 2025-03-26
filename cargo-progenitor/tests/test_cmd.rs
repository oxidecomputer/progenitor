// Copyright 2025 Oxide Computer Company

use assert_cmd::Command;

#[test]
fn test_help() {
    Command::cargo_bin("cargo-progenitor")
        .unwrap()
        .arg("progenitor")
        .arg("--help")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic("tests/data/test_help.stdout"));
}
