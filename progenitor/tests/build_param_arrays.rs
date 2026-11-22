// Copyright 2026 Oxide Computer Company

mod positional {
    progenitor::generate_api!("../sample_openapi/param-arrays.json");

    fn _ignore() {
        let _ = Client::new("").unwrap().list_things(
            &vec![types::ThingKind::Widget],
            Some(&vec!["tag".to_string()]),
            None,
            Some(&vec!["label".to_string()]),
            &vec![1, 2],
        );
    }
}

mod builder_untagged {
    progenitor::generate_api!(
        spec = "../sample_openapi/param-arrays.json",
        interface = Builder,
        tags = Merged,
    );

    fn _ignore() {
        let _ = Client::new("")
            .unwrap()
            .list_things()
            .kinds(vec![types::ThingKind::Widget])
            .tags(vec!["tag".to_string()])
            .x_labels(vec!["label".to_string()])
            .x_shard_ids(vec![1, 2])
            .send();
    }
}

mod builder_tagged {
    progenitor::generate_api!(
        spec = "../sample_openapi/param-arrays.json",
        interface = Builder,
        tags = Separate,
    );

    fn _ignore() {
        let _ = Client::new("")
            .unwrap()
            .list_things()
            .kinds(vec![types::ThingKind::Widget])
            .tags(vec!["tag".to_string()])
            .x_labels(vec!["label".to_string()])
            .x_shard_ids(vec![1, 2])
            .send();
    }
}
