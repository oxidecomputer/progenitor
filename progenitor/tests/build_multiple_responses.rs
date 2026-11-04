// Copyright 2026 Oxide Computer Company

//! Operations whose success or error responses have more than one type are
//! represented with generated enums. Check that the generated clients compile
//! and decode each response into the right variant.

use std::{
    io::{Read, Write},
    net::TcpListener,
};

mod positional {
    progenitor::generate_api!("../sample_openapi/multiple-responses.json");
}

mod builder {
    progenitor::generate_api!(
        spec = "../sample_openapi/multiple-responses.json",
        interface = Builder,
        tags = Merged,
    );
}

mod builder_tagged {
    progenitor::generate_api!(
        spec = "../sample_openapi/multiple-responses.json",
        interface = Builder,
        tags = Separate,
    );
}

/// A minimal HTTP server that answers each request based on its method and
/// path. Returns the base URL.
fn serve() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut request = Vec::new();
            let mut buf = [0u8; 4096];
            while !request.windows(4).any(|w| w == b"\r\n\r\n") {
                let n = stream.read(&mut buf).unwrap();
                if n == 0 {
                    break;
                }
                request.extend_from_slice(&buf[..n]);
            }
            let request = String::from_utf8_lossy(&request);
            let mut words = request.split_whitespace();
            let method = words.next().unwrap_or_default();
            let path = words.next().unwrap_or_default();

            let json = "application/json";
            let (status, content_type, body) = match (method, path) {
                ("GET", "/things/ok") => ("200 OK", json, r#"{"name":"widget"}"#),
                ("GET", "/things/later") => ("202 Accepted", json, r#"{"queued":true}"#),
                ("GET", "/things/bad") => ("400 Bad Request", json, r#"{"message":"no"}"#),
                ("GET", "/things/gone") => ("404 Not Found", "", ""),
                ("GET", "/things/down") => {
                    ("503 Service Unavailable", json, r#"{"request_id":"r1"}"#)
                }
                ("PUT", "/things/new") => ("201 Created", json, r#"{"name":"new"}"#),
                ("PUT", "/things/old") => ("204 No Content", "", ""),
                ("PUT", "/things/odd") => ("200 OK", json, r#"{"request_id":"r2"}"#),
                ("GET", "/things/a/contents") => {
                    ("200 OK", "application/octet-stream", "raw bytes")
                }
                ("GET", "/things/b/contents") => ("202 Accepted", json, r#"{"percent":42}"#),
                _ => ("418 I'm a teapot", "", ""),
            };
            let mut response = format!("HTTP/1.1 {status}\r\nconnection: close\r\n");
            if !content_type.is_empty() {
                response.push_str(&format!("content-type: {content_type}\r\n"));
            }
            response.push_str(&format!("content-length: {}\r\n\r\n{body}", body.len()));
            stream.write_all(response.as_bytes()).unwrap();
        }
    });
    format!("http://{addr}")
}

#[tokio::test]
async fn test_success_responses() {
    use positional::{Client, types};
    let client = Client::new(&serve());

    let response = client.get_thing("ok").await.unwrap();
    assert_eq!(response.status(), 200);
    match response.into_inner() {
        types::GetThingResponses::Thing(thing) => assert_eq!(thing.name, "widget"),
        other => panic!("unexpected {other:?}"),
    }

    let response = client.get_thing("later").await.unwrap();
    assert_eq!(response.status(), 202);
    match response.into_inner() {
        types::GetThingResponses::Response202(map) => {
            assert_eq!(map.get("queued"), Some(&serde_json::Value::Bool(true)))
        }
        other => panic!("unexpected {other:?}"),
    }

    let body = types::Thing {
        name: "new".to_string(),
    };
    let response = client.put_thing("new", &body).await.unwrap();
    assert_eq!(response.status(), 201);
    assert!(matches!(
        response.into_inner(),
        types::PutThingResponses::Thing(types::Thing { name }) if name == "new"
    ));

    let response = client.put_thing("old", &body).await.unwrap();
    assert_eq!(response.status(), 204);
    assert!(matches!(
        response.into_inner(),
        types::PutThingResponses::Empty
    ));

    // A 2xx status other than those listed falls through to the `default`
    // response, as it does for operations with a single response type.
    let response = client.put_thing("odd", &body).await.unwrap();
    assert_eq!(response.status(), 200);
    assert!(matches!(
        response.into_inner(),
        types::PutThingResponses::InternalError(_)
    ));
}

#[tokio::test]
async fn test_error_responses() {
    use positional::{Client, types};
    let client = Client::new(&serve());

    let progenitor_client::Error::ErrorResponse(error) = client.get_thing("bad").await.unwrap_err()
    else {
        panic!("expected an error response");
    };
    assert_eq!(error.status(), 400);
    match error.into_inner() {
        types::GetThingErrorResponses::BadRequest(bad) => assert_eq!(bad.message, "no"),
        other => panic!("unexpected {other:?}"),
    }

    let progenitor_client::Error::ErrorResponse(error) =
        client.get_thing("gone").await.unwrap_err()
    else {
        panic!("expected an error response");
    };
    assert_eq!(error.status(), 404);
    assert!(matches!(
        error.into_inner(),
        types::GetThingErrorResponses::Empty
    ));

    let progenitor_client::Error::ErrorResponse(error) =
        client.get_thing("down").await.unwrap_err()
    else {
        panic!("expected an error response");
    };
    assert_eq!(error.status(), 503);
    match error.into_inner() {
        types::GetThingErrorResponses::InternalError(e) => assert_eq!(e.request_id, "r1"),
        other => panic!("unexpected {other:?}"),
    }
}

#[tokio::test]
async fn test_stream_and_builder() {
    use builder::{Client, types};
    let client = Client::new(&serve());

    let response = client.get_thing_contents().id("a").send().await.unwrap();
    assert_eq!(response.status(), 200);
    // Debug is implemented by hand because `ByteStream` doesn't implement it.
    assert!(format!("{response:?}").contains("Stream(.."));
    let types::GetThingContentsResponses::Stream(stream) = response.into_inner() else {
        panic!("expected a stream");
    };
    let mut stream = stream.into_inner();
    let mut bytes = Vec::new();
    while let Some(chunk) = futures::StreamExt::next(&mut stream).await {
        bytes.extend_from_slice(&chunk.unwrap());
    }
    assert_eq!(bytes, b"raw bytes");

    let response = client.get_thing_contents().id("b").send().await.unwrap();
    assert_eq!(response.status(), 202);
    assert!(matches!(
        response.into_inner(),
        types::GetThingContentsResponses::Progress(types::Progress { percent: 42 })
    ));

    let response = builder_tagged::Client::new(&serve())
        .get_thing()
        .id("ok")
        .send()
        .await
        .unwrap();
    assert!(matches!(
        response.into_inner(),
        builder_tagged::types::GetThingResponses::Thing(_)
    ));
}
