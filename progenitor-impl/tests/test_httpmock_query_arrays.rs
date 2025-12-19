// Copyright 2025 Oxide Computer Company

use openapiv3::OpenAPI;
use progenitor_impl::{space_out_items, Generator};

fn format_httpmock(generator: &mut Generator, spec: &OpenAPI) -> String {
    let code = generator.httpmock(spec, "crate::types").unwrap();
    let output = rustfmt_wrapper::rustfmt_config(
        rustfmt_wrapper::config::Config {
            format_strings: Some(true),
            ..Default::default()
        },
        code,
    )
    .unwrap();
    space_out_items(output).unwrap()
}

#[test]
fn test_httpmock_query_array_param() {
    let spec = serde_yaml::from_str::<OpenAPI>(
        r#"
openapi: 3.0.0
info:
  title: httpmock-query-array
  version: "0.0.0"
paths:
  /widgets:
    get:
      operationId: listWidgets
      parameters:
        - name: tags
          in: query
          required: true
          schema:
            type: array
            items:
              type: string
      responses:
        "204":
          description: no-content
"#,
    )
    .unwrap();

    let mut generator = Generator::default();
    let output = format_httpmock(&mut generator, &spec);

    assert!(
        output.contains("matches_query_param(req, \"tags\", &value)"),
        "expected array query params to use serialized query matching"
    );
    assert!(
        !output.contains("query_param(\"tags\", value.to_string())"),
        "expected array query params to avoid Display-based matching"
    );
}
