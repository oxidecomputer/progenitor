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
    let e = progenitor_client::Error::ErrorResponse(progenitor_client::ResponseValue::new(
        mine,
        reqwest::StatusCode::FORBIDDEN,
        reqwest::header::HeaderMap::default(),
    ));

    (Err(e) as Result<(), progenitor_client::Error<MyErr>>).unwrap();
}
