// Copyright 2025 Oxide Computer Company

#[test]
fn test_help() {
    assert_cmd::cargo::cargo_bin_cmd!()
        .arg("progenitor")
        .arg("--help")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic("tests/data/test_help.stdout"));
}
