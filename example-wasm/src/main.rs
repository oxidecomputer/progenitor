// Copyright 2021 Oxide Computer Company

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
    let client = Client::new("https://foo/bar");
    let _ = client.enrol(
        "auth-token",
        &types::EnrolBody {
            host: String::new(),
            key: String::new(),
        },
    );
}
