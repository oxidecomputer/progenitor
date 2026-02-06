// Copyright 2022 Oxide Computer Company

mod positional {
    progenitor::generate_api!("../sample_openapi/buildomat.json");

    fn _ignore() {
        std::mem::drop(Client::new("").worker_task_upload_chunk("task", vec![0]));
    }
}

mod builder_untagged {
    progenitor::generate_api!(
        spec = "../sample_openapi/buildomat.json",
        interface = Builder,
        tags = Merged,
    );

    fn _ignore() {
        std::mem::drop(
            Client::new("")
                .worker_task_upload_chunk()
                .task("task")
                .body(vec![0])
                .send(),
        );
    }
}

mod builder_tagged {
    progenitor::generate_api!(
        spec = "../sample_openapi/buildomat.json",
        interface = Builder,
        tags = Separate,
    );

    fn _ignore() {
        std::mem::drop(
            Client::new("")
                .worker_task_upload_chunk()
                .task("task")
                .body(vec![0])
                .send(),
        );
    }
}
