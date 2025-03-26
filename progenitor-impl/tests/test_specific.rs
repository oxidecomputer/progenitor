// Copyright 2024 Oxide Computer Company

use dropshot::{
    endpoint, ApiDescription, Body, ConfigDropshot, ConfigLogging, ConfigLoggingLevel,
    EmptyScanParams, HttpError, HttpResponseOk, HttpResponseUpdatedNoContent, HttpServerStarter,
    PaginationParams, Path, Query, RequestContext, ResultsPage, TypedBody,
};
use futures::StreamExt;
use http::Response;
use openapiv3::OpenAPI;
use progenitor_impl::{space_out_items, GenerationSettings, Generator, InterfaceStyle};
use schemars::JsonSchema;
use serde::Deserialize;
use std::{
    net::{Ipv4Addr, SocketAddr},
    str::from_utf8,
    sync::{Arc, Mutex},
};

fn generate_formatted(generator: &mut Generator, spec: &OpenAPI) -> String {
    let content = generator.generate_tokens(spec).unwrap();
    let rustfmt_config = rustfmt_wrapper::config::Config {
        normalize_doc_attributes: Some(true),
        wrap_comments: Some(true),
        ..Default::default()
    };
    space_out_items(rustfmt_wrapper::rustfmt_config(rustfmt_config, content).unwrap()).unwrap()
}

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
    _rqctx: RequestContext<()>,
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

    api.openapi("pagination-demo", semver::Version::new(9000, 0, 0))
        .write(&mut out)
        .unwrap();

    let out = from_utf8(&out).unwrap();

    let spec = serde_json::from_str::<OpenAPI>(out).unwrap();

    let mut generator = Generator::default();
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/src/{}.rs", "test_renamed_parameters"),
        &output,
    )
}

#[endpoint {
    method = GET,
    path = "/",
}]
async fn freeform_response(_rqctx: RequestContext<()>) -> Result<Response<Body>, HttpError> {
    unreachable!();
}

/// Test freeform responses.
#[test]
fn test_freeform_response() {
    let mut api = ApiDescription::new();
    api.register(freeform_response).unwrap();

    let mut out = Vec::new();

    api.openapi("pagination-demo", semver::Version::new(9000, 0, 0))
        .write(&mut out)
        .unwrap();

    let out = from_utf8(&out).unwrap();
    let spec = serde_json::from_str::<OpenAPI>(out).unwrap();

    let mut generator = Generator::default();
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/src/{}.rs", "test_freeform_response"),
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
    #[serde(default = "yes_yes")]
    something: Option<bool>,
}

fn forty_two() -> u32 {
    42
}

fn yes_yes() -> Option<bool> {
    Some(true)
}

#[endpoint {
    method = POST,
    path = "/",
}]
async fn default_params(
    _rqctx: RequestContext<()>,
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

    api.openapi("pagination-demo", semver::Version::new(9000, 0, 0))
        .write(&mut out)
        .unwrap();

    let out = from_utf8(&out).unwrap();
    let spec = serde_json::from_str::<OpenAPI>(out).unwrap();

    let mut generator = Generator::default();
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/src/{}.rs", "test_default_params_positional"),
        &output,
    );

    let mut generator =
        Generator::new(GenerationSettings::default().with_interface(InterfaceStyle::Builder));
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/src/{}.rs", "test_default_params_builder"),
        &output,
    );
}

#[derive(Debug)]
struct PaginatedU32sContext {
    all_values: std::ops::Range<u32>,
    // Record of `(offset, limit)` pairs we received
    page_pairs: Mutex<Vec<(usize, usize)>>,
}

#[endpoint {
    method = GET,
    path = "/",
}]
async fn paginated_u32s(
    rqctx: RequestContext<Arc<PaginatedU32sContext>>,
    query_params: Query<PaginationParams<EmptyScanParams, u32>>,
) -> Result<HttpResponseOk<ResultsPage<u32>>, HttpError> {
    let ctx = rqctx.context();
    let page_params = query_params.into_inner();
    let limit = usize::try_from(
        rqctx
            .page_limit(&page_params)
            .expect("invalid page limit")
            .get(),
    )
    .expect("non-usize limit");

    let offset = match page_params.page {
        dropshot::WhichPage::First(EmptyScanParams {}) => 0,
        dropshot::WhichPage::Next(offset) => usize::try_from(offset + 1).expect("non-usize offset"),
    };

    ctx.page_pairs.lock().unwrap().push((offset, limit));
    let values = ctx.all_values.clone().skip(offset).take(limit).collect();
    let result = ResultsPage::new(values, &(), |&x, &()| x).expect("bad results page");

    Ok(HttpResponseOk(result))
}

#[tokio::test]
async fn test_stream_pagination() {
    const TEST_NAME: &str = "test_stream_pagination";

    let mut api = ApiDescription::new();
    api.register(paginated_u32s).unwrap();

    let mut out = Vec::new();

    api.openapi(TEST_NAME, semver::Version::new(1, 0, 0))
        .write(&mut out)
        .unwrap();

    let out = from_utf8(&out).unwrap();
    let spec = serde_json::from_str::<OpenAPI>(out).unwrap();

    // Test both interface styles.
    let mut generator =
        Generator::new(GenerationSettings::new().with_interface(InterfaceStyle::Positional));
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(
        format!("tests/output/src/{TEST_NAME}_positional.rs"),
        &output,
    );
    let mut generator =
        Generator::new(GenerationSettings::new().with_interface(InterfaceStyle::Builder));
    let output = generate_formatted(&mut generator, &spec);
    expectorate::assert_contents(format!("tests/output/src/{TEST_NAME}_builder.rs"), &output);

    // Run the Dropshot server.
    let config_dropshot = ConfigDropshot {
        bind_address: SocketAddr::from((Ipv4Addr::LOCALHOST, 0)),
        ..Default::default()
    };
    let config_logging = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Debug,
    };
    let log = config_logging
        .to_logger(TEST_NAME)
        .expect("failed to create logger");
    let server_ctx = Arc::new(PaginatedU32sContext {
        all_values: 0..35,
        page_pairs: Mutex::default(),
    });
    let server = HttpServerStarter::new(&config_dropshot, api, Arc::clone(&server_ctx), &log)
        .expect("failed to create server")
        .start();

    let server_addr = format!("http://{}", server.local_addr());

    // Test the positional client.
    #[allow(dead_code)]
    mod gen_client_positional {
        // This is weird: we're now `include!`ing the file we just used to
        // confirm the generated code is what we expect. If changes are made to
        // progenitor that affect this generated code, keep in mind that when
        // this test executes, the above check is against what we _currently_
        // produce, while this `include!` is what was on disk before the test
        // ran. This can be surprising if you're running the test with
        // `EXPECTORATE=overwrite`, because the above check will overwrite the
        // file on disk, but then the test proceeds and gets to this point,
        // where it uses what was on disk _before_ expectorate overwrote it.
        include!("output/src/test_stream_pagination_positional.rs");
    }

    let client = gen_client_positional::Client::new(&server_addr);

    let page_limit = 10.try_into().unwrap();
    let mut stream = client.paginated_u32s_stream(Some(page_limit));

    let mut all_values = Vec::new();
    while let Some(result) = stream.next().await {
        match result {
            Ok(value) => {
                all_values.push(value);
            }
            Err(err) => {
                panic!("unexpected error: {err}");
            }
        }
    }

    // Ensure we got all the results we expected.
    let expected_values = (0..35).collect::<Vec<_>>();
    assert_eq!(expected_values, all_values);

    // Ensure the server saw the page requests we expect: we should always see a
    // limit of 10, and we should see offsets increasing by 10 until we get to
    // (30, 10); that will return 5 items, so we should see one final (35, 10)
    // for the client to confirm there are no more results.
    let expected_pages = vec![(0, 10), (10, 10), (20, 10), (30, 10), (35, 10)];
    assert_eq!(expected_pages, *server_ctx.page_pairs.lock().unwrap());

    // Repeat the test with the builder client.
    server_ctx.page_pairs.lock().unwrap().clear();

    #[allow(dead_code, unused_imports)]
    mod gen_client_builder {
        // This is weird: we're now `include!`ing the file we just used to
        // confirm the generated code is what we expect. If changes are made to
        // progenitor that affect this generated code, keep in mind that when
        // this test executes, the above check is against what we _currently_
        // produce, while this `include!` is what was on disk before the test
        // ran. This can be surprising if you're running the test with
        // `EXPECTORATE=overwrite`, because the above check will overwrite the
        // file on disk, but then the test proceeds and gets to this point,
        // where it uses what was on disk _before_ expectorate overwrote it.
        include!("output/src/test_stream_pagination_builder.rs");
    }

    let client = gen_client_builder::Client::new(&server_addr);

    let mut stream = client.paginated_u32s().limit(page_limit).stream();

    let mut all_values = Vec::new();
    while let Some(result) = stream.next().await {
        match result {
            Ok(value) => {
                all_values.push(value);
            }
            Err(err) => {
                panic!("unexpected error: {err}");
            }
        }
    }

    // Ensure we got all the results we expected.
    let expected_values = (0..35).collect::<Vec<_>>();
    assert_eq!(expected_values, all_values);

    // Ensure the server saw the page requests we expect: we should always see a
    // limit of 10, and we should see offsets increasing by 10 until we get to
    // (30, 10); that will return 5 items, so we should see one final (35, 10)
    // for the client to confirm there are no more results.
    let expected_pages = vec![(0, 10), (10, 10), (20, 10), (30, 10), (35, 10)];
    assert_eq!(expected_pages, *server_ctx.page_pairs.lock().unwrap());

    server.close().await.expect("failed to close server");
}
