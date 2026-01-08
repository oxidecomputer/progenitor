// Copyright 2025 Oxide Computer Company

//! Compile-only coverage for string body parameters in httpmock output.
//!
//! Without the fix, string types (`&str`) would fail to compile because they
//! don't implement `Deserialize` and are unsized, which causes `json_body_obj`
//! to fail.

/// Generated builder output for the string-body spec.
#[allow(dead_code, unused_imports)]
mod httpmock_string_body_builder {
    include!("output/src/httpmock_string_body_builder.rs");
}

/// Generated httpmock output for the string-body spec.
#[allow(dead_code, unused_imports)]
mod httpmock_string_body_httpmock {
    include!("output/src/httpmock_string_body_httpmock.rs");
}

/// Ensures the generated httpmock code type-checks.
#[test]
fn test_httpmock_string_body_param_compiles() {}
