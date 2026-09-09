mod load_yaml {
    progenitor::generate_sdk!("../sample_openapi/param-overrides.yaml");

    fn _ignore() {
        let _ = Client::new("").key_get(None, None);
    }
}
