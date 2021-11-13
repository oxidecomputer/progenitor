// Copyright 2021 Oxide Computer Company

use std::{fs::File, path::PathBuf};

use progenitor_impl::Generator;

#[track_caller]
fn verify_file(openapi_file: &str) {
    let mut in_path = PathBuf::from("../sample_openapi");
    in_path.push(format!("{}.json", openapi_file));

    let file = File::open(in_path).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = Generator::new();
    let output = generator.generate_text(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}.out", openapi_file),
        &output,
    )
}

#[test]
fn test_keeper() {
    verify_file("keeper");
}

#[test]
fn test_buildomat() {
    verify_file("buildomat");
}

#[test]
fn test_nexus() {
    verify_file("nexus");
}

// TODO this file is full of inconsistencies and incorrectly specified types.
// It's an interesting test to consider whether we try to do our best to
// interpret the intent or just fail.
#[ignore]
#[test]
fn test_github() {
    verify_file("api.github.com");
}
