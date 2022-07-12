// Copyright 2021 Oxide Computer Company

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
    // Should you need to set custom headers or other reqwest::ClientBuilder
    // methods, you can use Client::new_with_client as shown below.
    //
    // let mut val = reqwest::header::HeaderValue::from_static("super-secret");
    // val.set_sensitive(true);
    // let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert(reqwest::header::AUTHORIZATION, val);
    // // Insert more headers if necessary
    //
    // let client_builder = reqwest::ClientBuilder::new()
    //     // Set custom timeout
    //     .connect_timeout(Duration::new(60, 0))
    //     // Set custom headers
    //     .default_headers(headers)
    //     .build()
    //     .unwrap();
    // let client Client::new_with_client("https://foo/bar", client_builder);

    let client = Client::new("https://foo/bar");
    let _ = client.enrol(&types::EnrolBody {
        host: "".to_string(),
        key: "".to_string(),
    });
}
