// Copyright 2026 Oxide Computer Company

//! Test that `generate_api!` works with `relative_to = OutDir`, where a build
//! script copies the spec into `OUT_DIR`.

use progenitor::generate_api;

generate_api!(
    spec = { path = "keeper.json", relative_to = OutDir },
);

fn main() {
    let client = Client::new("https://example.com");
    let body = types::EnrolBody {
        host: String::new(),
        key: String::new(),
    };
    let _future = client.enrol("auth-token", &body);
}
