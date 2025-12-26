// Copyright 2022 Oxide Computer Company

mod positional {
    progenitor::generate_api!("../sample_openapi/keeper.json");

    fn _ignore() {
        let _ = Client::new("").enrol(
            "auth token",
            &types::EnrolBody {
                host: String::new(),
                key: String::new(),
            },
        );
    }
}

mod builder_untagged {
    progenitor::generate_api!(
        spec = "../sample_openapi/keeper.json",
        interface = Builder,
        tags = Merged,
    );

    fn _ignore() {
        let _ = Client::new("")
            .enrol()
            .authorization("")
            .body(types::EnrolBody {
                host: String::new(),
                key: String::new(),
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
        let _ = Client::new("")
            .enrol()
            .authorization("")
            .body(types::EnrolBody {
                host: String::new(),
                key: String::new(),
            })
            .send();
    }
}
