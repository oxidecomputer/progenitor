// Copyright 2024 Oxide Computer Company

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test_client_new() {
    let client = Client::new("http://foo/bar");
    assert!(client.baseurl == "http://foo/bar");
}
