mod load_yaml {
    progenitor::generate_api!("../sample_openapi/param-overrides.yaml");

    fn _ignore() {
        let client = Client::new("");
        let _future = client.key_get(None, None);
    }
}
