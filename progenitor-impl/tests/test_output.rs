// Copyright 2021 Oxide Computer Company

use std::{fs::File, path::PathBuf};

use progenitor_impl::{
    GenerationSettings, Generator, InterfaceStyle, TagStyle,
};

#[track_caller]
fn verify_apis(openapi_file: &str) {
    let mut in_path = PathBuf::from("../sample_openapi");
    in_path.push(format!("{}.json", openapi_file));

    let file = File::open(in_path).unwrap();
    let spec = serde_json::from_reader(file).unwrap();

    let mut generator = Generator::default();
    let output = generator.generate_text(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}-positional.out", openapi_file),
        &output,
    );

    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_tag(TagStyle::Merged),
    );
    let output = generator.generate_text(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}-builder.out", openapi_file),
        &output,
    );

    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_tag(TagStyle::Separate),
    );
    let output = generator.generate_text(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}-builder-tagged.out", openapi_file),
        &output,
    );
}

#[test]
fn test_keeper() {
    verify_apis("keeper");
}

#[test]
fn test_buildomat() {
    verify_apis("buildomat");
}

#[test]
fn test_nexus() {
    verify_apis("nexus");
}

// TODO this file is full of inconsistencies and incorrectly specified types.
// It's an interesting test to consider whether we try to do our best to
// interpret the intent or just fail.
#[ignore]
#[test]
fn test_github() {
    verify_apis("api.github.com");
}
