// Copyright 2026 Oxide Computer Company

use futures::TryStreamExt;
use progenitor_client::{ClientHooks, Error, OperationInfo};
use reqwest::{StatusCode, header};

mod positional {
    progenitor::generate_api!(
        spec = "tests/data/conditional.yaml",
        conditional_responses = true
    );
}
mod builder {
    progenitor::generate_api!(
        spec = "tests/data/conditional.yaml",
        conditional_responses = true,
        interface = Builder
    );
}
mod legacy {
    progenitor::generate_api!("tests/data/conditional.yaml");
}

const ETAG: &str = "\"v2\"";
const BASE: &str = "http://127.0.0.1:0";

// Exercise generated request construction and status dispatch without a server.
fn respond(request: reqwest::Request) -> reqwest::Response {
    let (status, body) = match request.url().path() {
        "/item" => match request
            .headers()
            .get(header::IF_NONE_MATCH)
            .map(|v| v.to_str().unwrap())
        {
            Some(ETAG) => (StatusCode::NOT_MODIFIED, ""),
            Some("\"invalid-json\"") => (StatusCode::OK, ""),
            Some("\"missing\"") => (StatusCode::NOT_FOUND, ""),
            _ => (StatusCode::OK, r#""current""#),
        },
        "/plain" => (StatusCode::NOT_MODIFIED, ""),
        "/pages"
            if request
                .url()
                .query_pairs()
                .any(|(key, _)| key == "page_token") =>
        {
            (StatusCode::NOT_MODIFIED, "")
        }
        "/pages" => (
            StatusCode::OK,
            r#"{"items":["first"],"next_page":"second"}"#,
        ),
        path => panic!("unexpected path {path}"),
    };
    http::Response::builder()
        .status(status)
        .header(header::ETAG, ETAG)
        .body(body)
        .unwrap()
        .into()
}

macro_rules! mock_client {
    ($client:ty) => {
        impl ClientHooks for $client {
            async fn exec(
                &self,
                request: reqwest::Request,
                _: &OperationInfo,
            ) -> reqwest::Result<reqwest::Response> {
                Ok(respond(request))
            }
        }
    };
}
mock_client!(positional::Client);
mock_client!(builder::Client);
mock_client!(legacy::Client);

#[tokio::test]
async fn revalidate() {
    use positional::ConditionalResponse::{Modified, NotModified};
    let client = positional::Client::new(BASE).unwrap();
    let Modified(fresh) = client.get_item(None).await.unwrap() else {
        panic!("expected item")
    };
    assert_eq!(fresh.status(), StatusCode::OK);
    assert_eq!(fresh.as_str(), "current");
    let tag = fresh.headers()[header::ETAG].to_str().unwrap();
    let NotModified(cached) = client.get_item(Some(tag)).await.unwrap() else {
        panic!("expected 304")
    };
    assert_eq!(cached.status(), StatusCode::NOT_MODIFIED);
    assert_eq!(cached.headers()[header::ETAG], ETAG);
    assert!(
        matches!(client.get_item(Some("\"v1\"")).await.unwrap(), Modified(value) if value.as_str() == "current")
    );
    assert!(
        matches!(client.get_item(Some("\"missing\"")).await, Err(Error::ErrorResponse(r)) if r.status() == StatusCode::NOT_FOUND)
    );
    assert!(matches!(
        client.get_item(Some("\"invalid-json\"")).await,
        Err(Error::InvalidResponsePayload(_, _))
    ));
}

#[tokio::test]
async fn requires_opt_in_and_a_declared_304() {
    let client = legacy::Client::new(BASE).unwrap();
    let _: String = client.get_item(None).await.unwrap().into_inner();
    assert!(
        matches!(client.get_item(Some(ETAG)).await, Err(Error::UnexpectedResponse(r)) if r.status() == StatusCode::NOT_MODIFIED)
    );
    let client = positional::Client::new(BASE).unwrap();
    let result: Result<positional::ResponseValue<String>, _> = client.get_plain().await;
    assert!(
        matches!(result, Err(Error::UnexpectedResponse(r)) if r.status() == StatusCode::NOT_MODIFIED)
    );
}

#[tokio::test]
async fn conditional_builder_and_page_streams() {
    let client = builder::Client::new(BASE).unwrap();
    assert!(
        matches!(client.get_item().if_none_match(ETAG).send().await.unwrap(), builder::ConditionalResponse::NotModified(r) if r.headers()[header::ETAG] == ETAG)
    );
    let mut stream = client.get_pages().stream();
    assert_eq!(stream.try_next().await.unwrap().as_deref(), Some("first"));
    assert!(
        matches!(stream.try_next().await, Err(Error::UnexpectedResponse(r)) if r.status() == StatusCode::NOT_MODIFIED)
    );

    let client = positional::Client::new(BASE).unwrap();
    let mut stream = client.get_pages_stream(None);
    assert_eq!(stream.try_next().await.unwrap().as_deref(), Some("first"));
    assert!(
        matches!(stream.try_next().await, Err(Error::UnexpectedResponse(r)) if r.status() == StatusCode::NOT_MODIFIED)
    );
}
