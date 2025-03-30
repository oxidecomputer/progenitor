mod no_operation_id_client {
    progenitor::generate_api!("../sample_openapi/no-operation-id.yaml");

    fn _ignore() {
        let _ = Client::new("").bazooka();
        let _ = Client::new("").foo_bar_get();
        let _ = Client::new("").foo_bar_post();
    }
}
