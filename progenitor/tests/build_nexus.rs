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
            let stream = client.instance_disk_list_stream(&org, &project, &instance, None, None);
            let _ = stream.collect::<Vec<_>>();
        };
    }
}

mod builder_untagged {
    use futures::StreamExt;

    mod nexus_client {
        use std::fmt::Display;

        #[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
        pub struct MyIpv4Net(String);
        impl std::str::FromStr for MyIpv4Net {
            type Err = std::convert::Infallible;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self(value.to_string()))
            }
        }
        impl Display for MyIpv4Net {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
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

    use self::nexus_client::types;

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

    #[tokio::test]
    #[should_panic = "called `Result::unwrap()` on an `Err` value: Invalid Request: conversion to `SiloCreate` for body failed: no value supplied for description"]
    async fn test_decent_error_from_body_builder() {
        let _ = Client::new("")
            .silo_create()
            .body(types::SiloCreate::builder())
            .send()
            .await
            .unwrap();
    }
}
