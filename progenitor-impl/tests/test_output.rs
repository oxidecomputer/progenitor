// Copyright 2025 Oxide Computer Company

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use progenitor_impl::{
    FileLayout, GenerationSettings, Generator, InterfaceStyle, TagStyle, TypeImpl, TypePatch,
    space_out_items,
};

use openapiv3::OpenAPI;
use proc_macro2::TokenStream;

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

fn generate_formatted(generator: &mut Generator, spec: &OpenAPI) -> String {
    let content = generator.generate_tokens(&spec).unwrap();
    reformat_code(content)
}

fn reformat_code(content: TokenStream) -> String {
    let rustfmt_config = rustfmt_wrapper::config::Config {
        format_strings: Some(true),
        normalize_doc_attributes: Some(true),
        wrap_comments: Some(true),
        ..Default::default()
    };
    space_out_items(rustfmt_wrapper::rustfmt_config(rustfmt_config, content).unwrap()).unwrap()
}

#[track_caller]
fn verify_apis(openapi_file: &str) {
    let mut in_path = PathBuf::from("../sample_openapi");
    in_path.push(openapi_file);
    let openapi_stem = openapi_file.split('.').next().unwrap().replace('-', "_");

    let spec = load_api(in_path);

    // Positional generation.
    let mut generator = Generator::default();
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/src/{}_positional.rs", openapi_stem),
        &output,
    );

    // Builder generation with derives and patches.
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_tag(TagStyle::Merged)
            .with_derive("schemars::JsonSchema")
            .with_patch("Name", TypePatch::default().with_derive("Hash"))
            .with_conversion(
                schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Integer.into()),
                    format: Some("int32".to_string()),
                    ..Default::default()
                },
                "usize",
                [TypeImpl::Display].into_iter(),
            ),
    );
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/src/{}_builder.rs", openapi_stem),
        &output,
    );

    // Builder generation with tags.
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_cli_bounds("std::clone::Clone")
            .with_tag(TagStyle::Separate),
    );
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/src/{}_builder_tagged.rs", openapi_stem),
        &output,
    );

    // CLI generation.
    let tokens = generator
        .cli(&spec, &format!("crate::{openapi_stem}_builder"))
        .unwrap();
    let output = reformat_code(tokens);

    expectorate::assert_contents(format!("tests/output/src/{}_cli.rs", openapi_stem), &output);

    // httpmock generation.
    let code = generator
        .httpmock(&spec, &format!("crate::{openapi_stem}_builder"))
        .unwrap();

    // TODO pending #368
    let output = rustfmt_wrapper::rustfmt_config(
        rustfmt_wrapper::config::Config {
            format_strings: Some(true),
            ..Default::default()
        },
        code,
    )
    .unwrap();

    let output = progenitor_impl::space_out_items(output).unwrap();
    expectorate::assert_contents(
        format!("tests/output/src/{}_httpmock.rs", openapi_stem),
        &output,
    );
}

#[test]
fn test_keeper() {
    verify_apis("keeper.json");
}

#[test]
fn test_split_files_by_section() {
    let spec = load_api("../sample_openapi/keeper.json");
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_tag(TagStyle::Merged),
    );
    let files = generator
        .generate_files(&spec, FileLayout::BySection)
        .unwrap();

    let paths = files
        .iter()
        .map(|file| file.path.to_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();

    assert!(paths.contains("lib.rs"));
    assert!(paths.contains("client.rs"));
    assert!(paths.contains("operations.rs"));
    assert!(paths.contains("builder.rs"));
    assert!(paths.contains("prelude.rs"));
    assert!(paths.contains("types/mod.rs"));
    assert!(paths.contains("types/error.rs"));
    assert!(paths.contains("types/builder.rs"));

    let lib = files
        .iter()
        .find(|file| file.path == PathBuf::from("lib.rs"))
        .unwrap();
    assert!(lib.contents.contains("mod client"));
    assert!(lib.contents.contains("mod operations"));
    assert!(lib.contents.contains("pub mod builder"));
    assert!(!lib.contents.contains("include!"));
}

#[test]
fn test_split_files_by_tag() {
    let spec = load_api("../sample_openapi/nexus.json");
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_tag(TagStyle::Merged),
    );
    let files = generator.generate_files(&spec, FileLayout::ByTag).unwrap();

    let paths = files
        .iter()
        .map(|file| file.path.to_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();

    assert!(paths.contains("lib.rs"));
    assert!(paths.contains("client.rs"));
    assert!(paths.contains("operations/mod.rs"));
    assert!(paths.contains("operations/disks.rs"));
    assert!(paths.contains("operations/instances.rs"));
    assert!(paths.contains("operations/vpcs.rs"));
    assert!(paths.contains("builder/mod.rs"));
    assert!(paths.contains("builder/disks.rs"));
    assert!(paths.contains("builder/instances.rs"));
    assert!(paths.contains("builder/vpcs.rs"));
    assert!(paths.contains("prelude.rs"));
    assert!(paths.contains("types/error.rs"));
    assert!(paths.contains("types/mod.rs"));
    assert!(paths.contains("types/builder.rs"));

    let lib = files
        .iter()
        .find(|file| file.path == PathBuf::from("lib.rs"))
        .unwrap();
    assert!(lib.contents.contains("mod operations"));
    assert!(lib.contents.contains("pub mod builder"));
    assert!(!lib.contents.contains("include!"));
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

#[test]
fn test_param_collision() {
    verify_apis("param-collision.json");
}

#[test]
fn test_cli_gen() {
    verify_apis("cli-gen.json");
}

#[test]
fn test_nexus_with_different_timeout() {
    const OPENAPI_FILE: &'static str = "nexus.json";

    let mut in_path = PathBuf::from("../sample_openapi");
    in_path.push(OPENAPI_FILE);
    let openapi_stem = OPENAPI_FILE.split('.').next().unwrap().replace('-', "_");

    let spec = load_api(in_path);

    let mut generator = Generator::new(GenerationSettings::default().with_timeout(75));
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/src/{}_with_timeout.rs", openapi_stem),
        &output,
    );
}

// TODO this file is full of inconsistencies and incorrectly specified types.
// It's an interesting test to consider whether we try to do our best to
// interpret the intent or just fail.
#[ignore]
#[test]
fn test_github() {
    verify_apis("api.github.com.json");
}
