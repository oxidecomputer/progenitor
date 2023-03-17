// Copyright 2022 Oxide Computer Company

mod positional {
    use futures::StreamExt;

    mod nexus_client {
        progenitor::generate_api!("../sample_openapi/nexus.json");
    }

    use nexus_client::{types, Client};

    fn _ignore() {
        let _ = async {
            let client = Client::new("");
            let org = types::Name::try_from("org").unwrap();
            let project = types::Name::try_from("project").unwrap();
            let instance = types::Name::try_from("instance").unwrap();
            let stream = client.instance_disk_list_stream(
                &org, &project, &instance, None, None,
            );
            let _ = stream.collect::<Vec<_>>();
        };
    }
}

mod builder_untagged {
    use futures::StreamExt;

    mod nexus_client {
        #[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
        pub struct MyIpv4Net(String);
        impl std::str::FromStr for MyIpv4Net {
            type Err = std::convert::Infallible;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self(value.to_string()))
            }
        }
        impl ToString for MyIpv4Net {
            fn to_string(&self) -> String {
                self.0.clone()
            }
        }
        progenitor::generate_api!(
            spec = "../sample_openapi/nexus.json",
            interface = Builder,
            tags = Merged,
            patch = {
                Name = {
                    derives = [Hash],
                }
            },
            replace = {
                Ipv4Net = crate::builder_untagged::nexus_client::MyIpv4Net,
            }
        );
    }

    use nexus_client::Client;

    pub fn _ignore() {
        // Verify the replacement above.
        let _ignore = nexus_client::types::IpNet::V4("".parse().unwrap());

        let client = Client::new("");
        let stream = client
            .instance_disk_list()
            .organization_name("org")
            .project_name("project")
            .instance_name("instance")
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

    use nexus_client::prelude::*;

    async fn _ignore() {
        let client = Client::new("");
        let stream = client
            .instance_disk_list()
            .organization_name("org")
            .project_name("project")
            .instance_name("instance")
            .stream();
        let _ = stream.collect::<Vec<_>>();

        let _ = client
            .instance_create()
            .organization_name("org")
            .project_name("project")
            .body(self::nexus_client::types::InstanceCreate::builder())
            .send()
            .await;
    }
}
