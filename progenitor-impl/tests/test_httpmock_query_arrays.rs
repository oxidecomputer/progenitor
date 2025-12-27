// Copyright 2025 Oxide Computer Company

//! Compile-only coverage for array query parameters in httpmock output.

/// Generated builder output for the query-array spec.
#[allow(dead_code, unused_imports)]
mod httpmock_query_array_builder {
    include!("output/src/httpmock_query_array_builder.rs");
}

/// Generated httpmock output for the query-array spec.
#[allow(dead_code, unused_imports)]
mod httpmock_query_array_httpmock {
    include!("output/src/httpmock_query_array_httpmock.rs");
}

/// Ensures the generated httpmock code type-checks.
#[test]
fn test_httpmock_query_array_param_compiles() {}
