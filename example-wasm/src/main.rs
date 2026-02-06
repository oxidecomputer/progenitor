// Copyright 2021 Oxide Computer Company

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
    let client = Client::new("https://foo/bar");
    // Explicitly drop the future to avoid clippy's let_underscore_future lint.
    std::mem::drop(client.enrol(
        "auth-token",
        &types::EnrolBody {
            host: "".to_string(),
            key: "".to_string(),
        },
    ));
}
