// Copyright 2022 Oxide Computer Company

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use progenitor_impl::{
    space_out_items, GenerationSettings, Generator, InterfaceStyle, TagStyle, TypeImpl, TypePatch,
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
    let output = format!(
        "{}\n{}",
        "#![allow(elided_named_lifetimes)]",
        generate_formatted(&mut generator, &spec),
    );
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

// TODO this file is full of inconsistencies and incorrectly specified types.
// It's an interesting test to consider whether we try to do our best to
// interpret the intent or just fail.
#[ignore]
#[test]
fn test_github() {
    verify_apis("api.github.com.json");
}
