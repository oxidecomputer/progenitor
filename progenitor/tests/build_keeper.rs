// Copyright 2022 Oxide Computer Company

mod positional {
    progenitor::generate_sdk!("../sample_openapi/keeper.json");

    fn _ignore() {
        let _ = Client::new("").enrol(
            "auth token",
            &types::EnrolBody {
                host: "".to_string(),
                key: "".to_string(),
            },
        );
    }
}

mod builder_untagged {
    progenitor::generate_sdk!(
        spec = "../sample_openapi/keeper.json",
        interface = Builder,
        tags = Merged,
    );

    fn _ignore() {
        let _ = Client::new("")
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
    progenitor::generate_sdk!(
        spec = "../sample_openapi/keeper.json",
        interface = Builder,
        tags = Separate,
    );

    fn _ignore() {
        let _ = Client::new("")
            .enrol()
            .authorization("")
            .body(types::EnrolBody {
                host: "".to_string(),
                key: "".to_string(),
            })
            .send();
    }
}
