// Copyright 2022 Oxide Computer Company

mod positional {
    progenitor::generate_api!("../sample_openapi/keeper.json");

    fn _ignore() {
        let body = types::EnrolBody {
            host: "".to_string(),
            key: "".to_string(),
        };
        let client = Client::new("");
        let _future = client.enrol("auth token", &body);
    }
}

mod builder_untagged {
    progenitor::generate_api!(
        spec = "../sample_openapi/keeper.json",
        interface = Builder,
        tags = Merged,
    );

    fn _ignore() {
        let client = Client::new("");
        let _future = client
            .enrol()
            .authorization("")
            .body(types::EnrolBody {
                host: "".to_string(),
                key: "".to_string(),
            })
            .send();
    }
}

mod builder_tagged {
    progenitor::generate_api!(
        spec = "../sample_openapi/keeper.json",
        interface = Builder,
        tags = Separate,
    );

    fn _ignore() {
        let client = Client::new("");
        let _future = client
            .enrol()
            .authorization("")
            .body(types::EnrolBody {
                host: "".to_string(),
                key: "".to_string(),
            })
            .send();
    }
}
