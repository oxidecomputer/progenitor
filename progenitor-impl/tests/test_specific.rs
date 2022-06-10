// Copyright 2022 Oxide Computer Company

use std::{str::from_utf8, sync::Arc};

use dropshot::{
    endpoint, ApiDescription, HttpError, HttpResponseUpdatedNoContent, Path,
    Query, RequestContext, TypedBody,
};
use http::Response;
use hyper::Body;
use openapiv3::OpenAPI;
use progenitor_impl::Generator;
use schemars::JsonSchema;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, JsonSchema)]
struct CursedPath {
    #[serde(rename = "ref")]
    reef: String,
    #[serde(rename = "type")]
    tripe: String,
    #[serde(rename = "trait")]
    trade: String,
}

#[allow(dead_code)]
#[derive(Deserialize, JsonSchema)]
struct CursedQuery {
    #[serde(rename = "if")]
    iffy: String,
    #[serde(rename = "in")]
    inn: String,
    #[serde(rename = "use")]
    youse: String,
}

#[endpoint {
    method = GET,
    path = "/{ref}/{type}/{trait}",
}]
async fn renamed_parameters(
    _rqctx: Arc<RequestContext<()>>,
    _path: Path<CursedPath>,
    _query: Query<CursedQuery>,
) -> Result<HttpResponseUpdatedNoContent, HttpError> {
    unreachable!();
}

/// Test parameters that conflict with Rust reserved words and therefore must
/// be renamed.
#[test]
fn test_renamed_parameters() {
    let mut api = ApiDescription::new();
    api.register(renamed_parameters).unwrap();

    let mut out = Vec::new();

    api.openapi("pagination-demo", "9000")
        .write(&mut out)
        .unwrap();

    let out = from_utf8(&out).unwrap();

    let spec = serde_json::from_str::<OpenAPI>(out).unwrap();

    let mut generator = Generator::default();
    let output = generator.generate_text(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}.out", "test_renamed_parameters"),
        &output,
    )
}

#[endpoint {
    method = GET,
    path = "/",
}]
async fn freeform_response(
    _rqctx: Arc<RequestContext<()>>,
) -> Result<Response<Body>, HttpError> {
    unreachable!();
}

/// Test freeform responses.
#[test]
fn test_freeform_response() {
    let mut api = ApiDescription::new();
    api.register(freeform_response).unwrap();

    let mut out = Vec::new();

    api.openapi("pagination-demo", "9000")
        .write(&mut out)
        .unwrap();

    let out = from_utf8(&out).unwrap();
    let spec = serde_json::from_str::<OpenAPI>(out).unwrap();

    let mut generator = Generator::default();
    let output = generator.generate_text(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}.out", "test_freeform_response"),
        &output,
    )
}

#[derive(Deserialize, JsonSchema)]
#[allow(dead_code)]
struct BodyWithDefaults {
    s: String,
    #[serde(default)]
    yes: bool,
    #[serde(default = "forty_two", rename = "forty-two")]
    forty_two: u32,
}

fn forty_two() -> u32 {
    42
}

#[endpoint {
    method = POST,
    path = "/",
}]
async fn default_params(
    _rqctx: Arc<RequestContext<()>>,
    _body: TypedBody<BodyWithDefaults>,
) -> Result<Response<Body>, HttpError> {
    unreachable!();
}

/// Test default type values.
#[test]
fn test_default_params() {
    let mut api = ApiDescription::new();
    api.register(default_params).unwrap();

    let mut out = Vec::new();

    api.openapi("pagination-demo", "9000")
        .write(&mut out)
        .unwrap();

    let out = from_utf8(&out).unwrap();
    let spec = serde_json::from_str::<OpenAPI>(out).unwrap();

    let mut generator = Generator::default();
    let output = generator.generate_text(&spec).unwrap();
    expectorate::assert_contents(
        format!("tests/output/{}.out", "test_default_params"),
        &output,
    )
}
