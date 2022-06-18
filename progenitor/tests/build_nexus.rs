// Copyright 2021 Oxide Computer Company

mod positional {
    use futures::StreamExt;

    mod nexus_client {
        progenitor::generate_api!("../sample_openapi/nexus.json");
    }

    use nexus_client::{types, Client};

    fn _ignore() {
        let _ = async {
            let client = Client::new("");
            let org = types::Name("org".to_string());
            let project = types::Name("project".to_string());
            let instance = types::Name("instance".to_string());
            let stream = client.instance_disks_get_stream(
                &org, &project, &instance, None, None,
            );
            let _ = stream.collect::<Vec<_>>();
        };
    }
}

mod builder_untagged {
    use futures::StreamExt;

    mod nexus_client {
        progenitor::generate_api!(
            spec = "../sample_openapi/nexus.json",
            interface = Builder,
            tags = Merged,
        );
    }

    use nexus_client::{types, Client};

    pub fn _ignore() {
        let client = Client::new("");
        let stream = client
            .instance_disks_get()
            .organization_name(types::Name("org".to_string()))
            .project_name(types::Name("project".to_string()))
            .instance_name(types::Name("instance".to_string()))
            .stream();
        let _ = stream.collect::<Vec<_>>();
    }
}

mod builder_tagged {
    use futures::StreamExt;

    mod nexus_client {
        progenitor::generate_api!(
            spec = "../sample_openapi/nexus.json",
            interface = Builder,
            tags = Separate,
        );
    }

    use nexus_client::{types, Client, ClientInstancesExt};

    fn _ignore() {
        let client = Client::new("");
        let stream = client
            .instance_disks_get()
            .organization_name(types::Name("org".to_string()))
            .project_name(types::Name("project".to_string()))
            .instance_name(types::Name("instance".to_string()))
            .stream();
        let _ = stream.collect::<Vec<_>>();
    }
}
