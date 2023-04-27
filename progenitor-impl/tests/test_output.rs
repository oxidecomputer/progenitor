// Copyright 2022 Oxide Computer Company

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use progenitor_impl::{
    space_out_items, GenerationSettings, Generator, InterfaceStyle, TagStyle,
    TypeImpl, TypePatch,
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
    space_out_items(
        rustfmt_wrapper::rustfmt_config(rustfmt_config, content).unwrap(),
    )
    .unwrap()
}

#[track_caller]
fn verify_apis(openapi_file: &str) {
    let mut in_path = PathBuf::from("../sample_openapi");
    in_path.push(openapi_file);
    let openapi_stem = openapi_file.split('.').next().unwrap();

    let spec = load_api(in_path);
    verify_spec(openapi_stem, &spec);
}

fn verify_spec(openapi_stem: &str, spec: &OpenAPI) {
    // Positional generation.
    let mut generator = Generator::default();
    let output = generate_formatted(&mut generator, &spec);
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
    let output = generate_formatted(&mut generator, &spec);
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
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/{}-builder-tagged.out", openapi_stem),
        &output,
    );

    // CLI generation.
    let tokens = generator.cli(&spec, "sdk").unwrap();
    let output = reformat_code(tokens);

    expectorate::assert_contents(
        format!("tests/output/{}-cli.out", openapi_stem),
        &output,
    );

    // httpmock generation.
    let code = generator.httpmock(&spec, "sdk").unwrap();

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
        format!("tests/output/{}-httpmock.out", openapi_stem),
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

// TODO this file is not fully supported.
// See https://github.com/oxidecomputer/progenitor/issues/444 for details
#[test]
fn test_github() {
    let in_path = PathBuf::from("../sample_openapi/api.github.com.json");

    let mut spec = load_api(in_path);

    spec.paths.paths.remove("/orgs/{org}");
    spec.paths.paths.remove("/markdown/raw");
    spec.paths.paths.remove("/repos/{owner}/{repo}/releases");
    spec.paths
        .paths
        .remove("/repos/{owner}/{repo}/releases/{release_id}");
    spec.paths
        .paths
        .remove("/projects/columns/{column_id}/cards");

    // Remove "conditions" property from "repository-ruleset" schema
    let schema_name = "repository-ruleset";
    let schemas = &mut spec.components.as_mut().unwrap().schemas;
    let broken_schema = schemas.get(schema_name).unwrap();
    let mut broken_schema_item = broken_schema.as_item().unwrap().to_owned();
    match &broken_schema_item.schema_kind {
        openapiv3::SchemaKind::Type(openapiv3::Type::Object(x)) => {
            let mut new_object = x.clone();
            new_object.properties.remove("conditions");
            broken_schema_item.schema_kind = openapiv3::SchemaKind::Type(
                openapiv3::Type::Object(new_object),
            );
            schemas.insert(
                schema_name.to_string(),
                openapiv3::ReferenceOr::Item(broken_schema_item),
            );
        }
        _ => panic!("Unexpected type"),
    }

    verify_spec("github", &spec);
}
