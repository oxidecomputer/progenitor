// Copyright 2022 Oxide Computer Company

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use progenitor_impl::{
    GenerationSettings, Generator, InterfaceStyle, TagStyle, TypeImpl,
    TypePatch,
};

use openapiv3::OpenAPI;

fn load_api<P>(p: P) -> OpenAPI
where
    P: AsRef<Path> + std::clone::Clone + std::fmt::Debug,
{
    let mut f = File::open(p.clone()).unwrap();
    match serde_json::from_reader(f) {
        Ok(json_value) => json_value,
        _ => {
            f = File::open(p).unwrap();
            serde_yaml::from_reader(f).unwrap()
        }
    }
}

#[track_caller]
fn verify_apis(openapi_file: &str) {
    let mut in_path = PathBuf::from("../sample_openapi");
    in_path.push(openapi_file);
    let openapi_stem = openapi_file.split('.').next().unwrap();

    let spec = load_api(in_path);

    // Positional generation.
    let mut generator = Generator::default();
    let output = generator.generate_text_normalize_comments(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}-positional.out", openapi_stem),
        &output,
    );

    // Builder generation with derives and patches.
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_tag(TagStyle::Merged)
            .with_derive("JsonSchema")
            .with_patch("Name", TypePatch::default().with_derive("Hash"))
            .with_conversion(
                schemars::schema::SchemaObject {
                    instance_type: Some(
                        schemars::schema::InstanceType::Integer.into(),
                    ),
                    format: Some("int32".to_string()),
                    ..Default::default()
                },
                "usize",
                [TypeImpl::Display].into_iter(),
            ),
    );
    let output = generator.generate_text_normalize_comments(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}-builder.out", openapi_stem),
        &output,
    );

    // Builder generation with tags.
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_tag(TagStyle::Separate),
    );
    let output = generator.generate_text_normalize_comments(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}-builder-tagged.out", openapi_stem),
        &output,
    );

    // CLI generation.
    let output = generator.cli_text(&spec, "sdk").unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}-cli.out", openapi_stem),
        &output,
    );
}

#[test]
fn test_keeper() {
    verify_apis("keeper.json");
}

#[test]
fn test_buildomat() {
    verify_apis("buildomat.json");
}

#[test]
fn test_nexus() {
    verify_apis("nexus.json");
}

#[test]
fn test_propolis_server() {
    verify_apis("propolis-server.json");
}

#[test]
fn test_param_override() {
    verify_apis("param-overrides.json");
}

#[test]
fn test_yaml() {
    verify_apis("param-overrides.yaml");
}

// TODO this file is full of inconsistencies and incorrectly specified types.
// It's an interesting test to consider whether we try to do our best to
// interpret the intent or just fail.
#[ignore]
#[test]
fn test_github() {
    verify_apis("api.github.com.json");
}
