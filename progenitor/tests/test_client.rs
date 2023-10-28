// Copyright 2022 Oxide Computer Company

// Validate that we get useful output from a user-typed error.
#[test]
#[should_panic = "Error Response: \
    status: 403 Forbidden; \
    headers: {}; \
    value: MyErr { msg: \"things went bad\" }"]
fn test_error() {
    #[derive(Debug)]
    struct MyErr {
        #[allow(dead_code)]
        msg: String,
    }

    let mine = MyErr {
        msg: "things went bad".to_string(),
    };
    let e = progenitor_client::Error::ErrorResponse(
        progenitor_client::ResponseValue::new(
            mine,
            reqwest::StatusCode::FORBIDDEN,
            reqwest::header::HeaderMap::default(),
        ),
    );

    (Err(e) as Result<(), progenitor_client::Error<MyErr>>).unwrap();
}

// Validate how form data text fields are passed
#[test]
fn test_to_form_data_field() -> Result<(), progenitor_client::Error<String>> {
    use progenitor_client::to_form_string as to;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Nested {
        x: String,
        y: usize,
    }
    #[derive(Serialize)]
    struct Body {
        a: String,
        b: isize,
        c: Nested,
        d: Option<Nested>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        e: Option<usize>,
    }

    let body = Body {
        a: "foo".to_string(),
        b: -42,
        c: Nested {
            x: "bar".to_string(),
            y: 42,
        },
        d: None,
        e: None,
    };

    let map = serde_json::to_value(body)
        .unwrap()
        .as_object()
        .unwrap()
        .clone();
    assert_eq!(to(map.get("a").unwrap())?, "foo");
    assert_eq!(to(map.get("b").unwrap())?, "-42");
    assert_eq!(to(map.get("c").unwrap())?, r#"{"x":"bar","y":42}"#);
    assert_eq!(to(map.get("d").unwrap())?, "null");
    assert_eq!(map.get("e"), None);
    Ok(())
}
