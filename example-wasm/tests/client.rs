use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[wasm_bindgen_test]
fn test_client_new() {
    let client = Client::new("http://foo/bar");
    assert!(client.baseurl == "http://foo/bar");
}
