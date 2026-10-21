// Copyright 2025 Oxide Computer Company

//! Coverage for array query parameters in httpmock output.

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

use httpmock::MockServer;
use httpmock_query_array_builder::Client;
use httpmock_query_array_httpmock::MockServerExt;

/// Sends `query` to the server and returns the number of mock hits.
async fn hits(expected: &[&str], query: &str) -> usize {
    let server = MockServer::start_async().await;
    let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let mock = server.list_widgets(|when, then| {
        when.tags(&expected);
        then.no_content();
    });
    let _ = reqwest::get(format!("{}/widgets{query}", server.base_url())).await;
    mock.calls_async().await
}

/// The mock matches what the generated client actually sends.
#[tokio::test]
async fn test_httpmock_query_array_matches_client() {
    let server = MockServer::start_async().await;
    let expected = vec!["a".to_string(), "b c".to_string()];
    let mock = server.list_widgets(|when, then| {
        when.tags(&expected);
        then.no_content();
    });

    Client::new(&server.base_url())
        .list_widgets()
        .tags(expected.clone())
        .send()
        .await
        .expect("request should match mock");
    mock.assert_async().await;
}

/// Values may arrive in any order, but extra, missing, or duplicated values
/// must not match.
#[tokio::test]
async fn test_httpmock_query_array_exact() {
    assert_eq!(hits(&["a", "b"], "?tags=a&tags=b").await, 1);
    assert_eq!(hits(&["a", "b"], "?tags=b&tags=a").await, 1);
    assert_eq!(hits(&["a", "b"], "?tags=a&tags=b&tags=c").await, 0);
    assert_eq!(hits(&["a", "b"], "?tags=a").await, 0);
    assert_eq!(hits(&["a", "a"], "?tags=a&tags=b").await, 0);
    assert_eq!(hits(&["a", "a"], "?tags=a&tags=a").await, 1);
}

/// Values are matched literally rather than as regex patterns.
#[tokio::test]
async fn test_httpmock_query_array_literal() {
    assert_eq!(hits(&["a.c"], "?tags=a.c").await, 1);
    assert_eq!(hits(&["a.c"], "?tags=abc").await, 0);
    assert_eq!(hits(&["a"], "?tags=ab").await, 0);
}

/// An empty array means the client sends no parameter at all.
#[tokio::test]
async fn test_httpmock_query_array_empty() {
    assert_eq!(hits(&[], "").await, 1);
    assert_eq!(hits(&[], "?tags=a").await, 0);
}
