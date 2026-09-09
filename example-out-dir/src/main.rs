// Copyright 2026 Oxide Computer Company

//! Test that `generate_sdk!` works with `relative_to = OutDir`, where a build
//! script copies the spec into `OUT_DIR`.

use progenitor::generate_sdk;

generate_sdk!(
    spec = { path = "keeper.json", relative_to = OutDir },
);

fn main() {
    let client = Client::new("https://example.com");
    std::mem::drop(client.enrol(
        "auth-token",
        &types::EnrolBody {
            host: "".to_string(),
            key: "".to_string(),
        },
    ));
}
