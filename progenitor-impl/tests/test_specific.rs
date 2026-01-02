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

mod multipart_tests {
    use openapiv3::OpenAPI;
    use progenitor_impl::{GenerationSettings, Generator, InterfaceStyle};

    fn make_multipart_spec(operation_id: &str, properties: serde_json::Value) -> OpenAPI {
        serde_json::from_value(serde_json::json!({
            "openapi": "3.0.0",
            "info": { "title": "Test", "version": "1.0.0" },
            "paths": {
                "/upload": {
                    "post": {
                        "operationId": operation_id,
                        "requestBody": {
                            "content": {
                                "multipart/form-data": {
                                    "schema": {
                                        "type": "object",
                                        "properties": properties
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": { "description": "OK" }
                        }
                    }
                }
            }
        }))
        .unwrap()
    }

    fn make_multipart_spec_with_encoding(
        operation_id: &str,
        properties: serde_json::Value,
        encoding: serde_json::Value,
    ) -> OpenAPI {
        serde_json::from_value(serde_json::json!({
            "openapi": "3.0.0",
            "info": { "title": "Test", "version": "1.0.0" },
            "paths": {
                "/upload": {
                    "post": {
                        "operationId": operation_id,
                        "requestBody": {
                            "content": {
                                "multipart/form-data": {
                                    "schema": {
                                        "type": "object",
                                        "properties": properties
                                    },
                                    "encoding": encoding
                                }
                            }
                        },
                        "responses": {
                            "200": { "description": "OK" }
                        }
                    }
                }
            }
        }))
        .unwrap()
    }

    fn make_builder_generator() -> Generator {
        Generator::new(GenerationSettings::default().with_interface(InterfaceStyle::Builder))
    }

    fn make_positional_generator() -> Generator {
        Generator::new(GenerationSettings::default().with_interface(InterfaceStyle::Positional))
    }

    fn assert_generates_ok(spec: &OpenAPI, msg: &str) -> String {
        let mut generator = make_builder_generator();
        let result = generator.generate_tokens(spec);
        assert!(result.is_ok(), "{}: {:?}", msg, result.err());
        result.unwrap().to_string()
    }

    fn assert_generates_err(spec: &OpenAPI, expected_err: &str) {
        let mut generator = make_builder_generator();
        let result = generator.generate_tokens(spec);
        assert!(result.is_err(), "Expected generation to fail");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains(expected_err),
            "Error should contain '{}': {}",
            expected_err,
            err
        );
    }

    fn binary_file_prop() -> serde_json::Value {
        serde_json::json!({ "type": "string", "format": "binary" })
    }

    fn string_prop() -> serde_json::Value {
        serde_json::json!({ "type": "string" })
    }

    fn integer_prop() -> serde_json::Value {
        serde_json::json!({ "type": "integer" })
    }

    fn boolean_prop() -> serde_json::Value {
        serde_json::json!({ "type": "boolean" })
    }

    fn number_prop() -> serde_json::Value {
        serde_json::json!({ "type": "number", "format": "double" })
    }

    fn string_array_prop() -> serde_json::Value {
        serde_json::json!({ "type": "array", "items": { "type": "string" } })
    }

    fn integer_array_prop() -> serde_json::Value {
        serde_json::json!({ "type": "array", "items": { "type": "integer" } })
    }

    fn simple_object_prop() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            }
        })
    }

    #[test]
    fn test_multipart_binary_only_succeeds() {
        let spec = make_multipart_spec(
            "upload_file",
            serde_json::json!({
                "file": binary_file_prop()
            }),
        );
        assert_generates_ok(&spec, "Binary-only multipart should be supported");
    }

    #[test]
    fn test_multipart_mixed_types_supported() {
        let spec = make_multipart_spec(
            "upload_with_name",
            serde_json::json!({
                "file": binary_file_prop(),
                "name": string_prop()
            }),
        );
        let code = assert_generates_ok(&spec, "Mixed types in multipart should be supported");
        assert!(code.contains("UploadWithNameForm"));
    }

    #[test]
    fn test_multipart_nested_objects_supported() {
        let spec = make_multipart_spec(
            "upload_with_metadata",
            serde_json::json!({
                "file": binary_file_prop(),
                "metadata": {
                    "type": "object",
                    "properties": {
                        "author": { "type": "string" }
                    }
                }
            }),
        );
        assert_generates_ok(&spec, "Nested objects in multipart should be supported");
    }

    #[test]
    fn test_multipart_integer_fields_supported() {
        let spec = make_multipart_spec(
            "upload_with_count",
            serde_json::json!({
                "file": binary_file_prop(),
                "count": integer_prop()
            }),
        );
        assert_generates_ok(&spec, "Integer fields in multipart should be supported");
    }

    #[test]
    fn test_multipart_array_fields_supported() {
        let spec = make_multipart_spec(
            "upload_with_tags",
            serde_json::json!({
                "file": binary_file_prop(),
                "tags": string_array_prop()
            }),
        );
        assert_generates_ok(&spec, "Array fields in multipart should be supported");
    }

    #[test]
    fn test_multipart_multiple_binary_files_succeeds() {
        let spec = make_multipart_spec(
            "upload_multiple",
            serde_json::json!({
                "primary": binary_file_prop(),
                "secondary": binary_file_prop(),
                "thumbnail": binary_file_prop()
            }),
        );
        assert_generates_ok(&spec, "Multiple binary files should be supported");
    }

    #[test]
    fn test_multipart_encoding_content_type() {
        let spec = make_multipart_spec_with_encoding(
            "upload_with_encoding",
            serde_json::json!({
                "file": binary_file_prop(),
                "metadata": {
                    "type": "object",
                    "properties": {
                        "author": string_prop(),
                        "tags": string_array_prop()
                    }
                }
            }),
            serde_json::json!({
                "file": { "contentType": "application/octet-stream" },
                "metadata": { "contentType": "application/json" }
            }),
        );
        let code = assert_generates_ok(&spec, "Encoding object should be supported");
        assert!(code.contains("UploadWithEncodingForm"));
    }

    #[test]
    fn test_multipart_array_json_serialization() {
        let spec = make_multipart_spec(
            "upload_with_array",
            serde_json::json!({
                "tags": string_array_prop()
            }),
        );
        let code = assert_generates_ok(&spec, "Array fields should be supported");
        // TokenStream::to_string() uses spaces between tokens
        assert!(code.contains("serde_json :: to_string"));
    }

    #[test]
    fn test_multipart_object_json_serialization() {
        let spec = make_multipart_spec(
            "upload_with_object",
            serde_json::json!({
                "metadata": simple_object_prop()
            }),
        );
        let code = assert_generates_ok(&spec, "Object fields should be supported");
        assert!(code.contains("serde_json :: to_string"));
    }

    #[test]
    fn test_multipart_boolean_fields_supported() {
        let spec = make_multipart_spec(
            "upload_with_flag",
            serde_json::json!({
                "file": binary_file_prop(),
                "is_public": boolean_prop()
            }),
        );
        let code = assert_generates_ok(&spec, "Boolean fields in multipart should be supported");
        assert!(code.contains("UploadWithFlagForm"));
    }

    #[test]
    fn test_multipart_number_fields_supported() {
        let spec = make_multipart_spec(
            "upload_with_score",
            serde_json::json!({
                "file": binary_file_prop(),
                "score": number_prop()
            }),
        );
        assert_generates_ok(&spec, "Number fields in multipart should be supported");
    }

    #[test]
    fn test_multipart_required_fields() {
        let spec: OpenAPI = serde_json::from_value(serde_json::json!({
            "openapi": "3.0.0",
            "info": { "title": "Test", "version": "1.0.0" },
            "paths": {
                "/upload": {
                    "post": {
                        "operationId": "upload_required",
                        "requestBody": {
                            "required": true,
                            "content": {
                                "multipart/form-data": {
                                    "schema": {
                                        "type": "object",
                                        "required": ["file", "name"],
                                        "properties": {
                                            "file": binary_file_prop(),
                                            "name": string_prop(),
                                            "description": string_prop()
                                        }
                                    }
                                }
                            }
                        },
                        "responses": { "200": { "description": "OK" } }
                    }
                }
            }
        }))
        .unwrap();

        let code = assert_generates_ok(&spec, "Required fields in multipart should be supported");
        assert!(code.contains("UploadRequiredForm"));
    }

    #[test]
    fn test_multipart_with_schema_reference() {
        let spec: OpenAPI = serde_json::from_value(serde_json::json!({
            "openapi": "3.0.0",
            "info": { "title": "Test", "version": "1.0.0" },
            "paths": {
                "/upload": {
                    "post": {
                        "operationId": "upload_with_ref",
                        "requestBody": {
                            "content": {
                                "multipart/form-data": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "file": binary_file_prop(),
                                            "metadata": { "$ref": "#/components/schemas/Metadata" }
                                        }
                                    }
                                }
                            }
                        },
                        "responses": { "200": { "description": "OK" } }
                    }
                }
            },
            "components": {
                "schemas": {
                    "Metadata": {
                        "type": "object",
                        "properties": {
                            "author": { "type": "string" },
                            "version": { "type": "integer" }
                        }
                    }
                }
            }
        }))
        .unwrap();

        let code = assert_generates_ok(&spec, "Schema references in multipart should be supported");
        assert!(code.contains("serde_json :: to_string"));
    }

    #[test]
    fn test_multipart_non_object_schema_fails() {
        let spec: OpenAPI = serde_json::from_value(serde_json::json!({
            "openapi": "3.0.0",
            "info": { "title": "Test", "version": "1.0.0" },
            "paths": {
                "/upload": {
                    "post": {
                        "operationId": "upload_bad",
                        "requestBody": {
                            "content": {
                                "multipart/form-data": {
                                    "schema": {
                                        "type": "string",
                                        "format": "binary"
                                    }
                                }
                            }
                        },
                        "responses": { "200": { "description": "OK" } }
                    }
                }
            }
        }))
        .unwrap();

        assert_generates_err(&spec, "object schema");
    }

    #[test]
    #[should_panic(expected = "unreachable")]
    fn test_multipart_empty_properties_panics() {
        let spec = make_multipart_spec("upload_empty", serde_json::json!({}));
        let mut generator = make_builder_generator();
        let _ = generator.generate_tokens(&spec);
    }

    #[test]
    fn test_multipart_array_of_integers() {
        let spec = make_multipart_spec(
            "upload_ids",
            serde_json::json!({
                "ids": integer_array_prop()
            }),
        );
        let code = assert_generates_ok(&spec, "Array of integers in multipart should be supported");
        assert!(code.contains("serde_json :: to_string"));
    }

    #[test]
    fn test_multipart_deeply_nested_object() {
        let spec = make_multipart_spec(
            "upload_nested",
            serde_json::json!({
                "config": {
                    "type": "object",
                    "properties": {
                        "settings": {
                            "type": "object",
                            "properties": {
                                "level1": {
                                    "type": "object",
                                    "properties": {
                                        "value": { "type": "string" }
                                    }
                                }
                            }
                        }
                    }
                }
            }),
        );
        assert_generates_ok(
            &spec,
            "Deeply nested objects in multipart should be supported",
        );
    }

    #[test]
    fn test_multipart_multiple_encoding_entries() {
        let spec = make_multipart_spec_with_encoding(
            "upload_multi_encoding",
            serde_json::json!({
                "image": binary_file_prop(),
                "document": binary_file_prop(),
                "data": simple_object_prop()
            }),
            serde_json::json!({
                "image": { "contentType": "image/png" },
                "document": { "contentType": "application/pdf" },
                "data": { "contentType": "application/xml" }
            }),
        );
        let code = assert_generates_ok(&spec, "Multiple encoding entries should be supported");
        assert!(code.contains("image/png") || code.contains("image / png"));
    }

    #[test]
    fn test_multipart_positional_interface() {
        let spec = make_multipart_spec(
            "upload_file",
            serde_json::json!({
                "file": binary_file_prop(),
                "name": string_prop()
            }),
        );

        let mut generator = make_positional_generator();
        let result = generator.generate_tokens(&spec);
        assert!(
            result.is_ok(),
            "Multipart should work with positional interface: {:?}",
            result.err()
        );
        assert!(result.unwrap().to_string().contains("UploadFileForm"));
    }

    #[test]
    fn test_multipart_enum_string_field() {
        let spec = make_multipart_spec(
            "upload_with_category",
            serde_json::json!({
                "file": binary_file_prop(),
                "category": {
                    "type": "string",
                    "enum": ["image", "document", "video"]
                }
            }),
        );
        assert_generates_ok(&spec, "Enum string fields in multipart should be supported");
    }
}
