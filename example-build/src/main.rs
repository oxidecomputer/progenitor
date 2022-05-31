// Copyright 2021 Oxide Computer Company

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
    let client = Client::new("https://foo/bar");
    //let _ = client.enrol(&types::EnrolBody {
    let x = client
        .enrol()
        .body(types::EnrolBody {
            host: "".to_string(),
            key: "".to_string(),
        })
        .send();
}
