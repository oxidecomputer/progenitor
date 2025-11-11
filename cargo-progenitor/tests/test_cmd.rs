// Copyright 2025 Oxide Computer Company

use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_help() {
    cargo_bin_cmd!("cargo-progenitor")
        .arg("progenitor")
        .arg("--help")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic("tests/data/test_help.stdout"));
}
