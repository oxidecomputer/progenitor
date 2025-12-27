pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::nexus_builder::*;
    /// Apply decoded query parameter pairs to the matcher.
    fn apply_query_param_pairs(
        mut when: ::httpmock::When,
        pairs: &[(String, String)],
    ) -> ::httpmock::When {
        for (key, value) in pairs {
            when = when.query_param(key, value);
        }

        when
    }

    pub struct DiskViewByIdWhen(::httpmock::When);
    impl DiskViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/disks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/by-id/disks/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct DiskViewByIdThen(::httpmock::Then);
    impl DiskViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageViewByIdWhen(::httpmock::When);
    impl ImageViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/images/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/by-id/images/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ImageViewByIdThen(::httpmock::Then);
    impl ImageViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Image) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceViewByIdWhen(::httpmock::When);
    impl InstanceViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/instances/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/by-id/instances/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceViewByIdThen(::httpmock::Then);
    impl InstanceViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceViewByIdWhen(::httpmock::When);
    impl InstanceNetworkInterfaceViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/network-interfaces/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/by-id/network-interfaces/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceNetworkInterfaceViewByIdThen(::httpmock::Then);
    impl InstanceNetworkInterfaceViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::NetworkInterface) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationViewByIdWhen(::httpmock::When);
    impl OrganizationViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/organizations/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/by-id/organizations/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct OrganizationViewByIdThen(::httpmock::Then);
    impl OrganizationViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Organization) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectViewByIdWhen(::httpmock::When);
    impl ProjectViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/by-id/projects/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectViewByIdThen(::httpmock::Then);
    impl ProjectViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SnapshotViewByIdWhen(::httpmock::When);
    impl SnapshotViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/snapshots/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/by-id/snapshots/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SnapshotViewByIdThen(::httpmock::Then);
    impl SnapshotViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Snapshot) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteViewByIdWhen(::httpmock::When);
    impl VpcRouterRouteViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/vpc-router-routes/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/by-id/vpc-router-routes/{}$", value.to_string()))
                    .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcRouterRouteViewByIdThen(::httpmock::Then);
    impl VpcRouterRouteViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RouterRoute) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterViewByIdWhen(::httpmock::When);
    impl VpcRouterViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/vpc-routers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/by-id/vpc-routers/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcRouterViewByIdThen(::httpmock::Then);
    impl VpcRouterViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcRouter) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetViewByIdWhen(::httpmock::When);
    impl VpcSubnetViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/vpc-subnets/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/by-id/vpc-subnets/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcSubnetViewByIdThen(::httpmock::Then);
    impl VpcSubnetViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcSubnet) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcViewByIdWhen(::httpmock::When);
    impl VpcViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/by-id/vpcs/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/by-id/vpcs/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcViewByIdThen(::httpmock::Then);
    impl VpcViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Vpc) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DeviceAuthRequestWhen(::httpmock::When);
    impl DeviceAuthRequestWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/device/auth$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::DeviceAuthRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DeviceAuthRequestThen(::httpmock::Then);
    impl DeviceAuthRequestThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct DeviceAuthConfirmWhen(::httpmock::When);
    impl DeviceAuthConfirmWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/device/confirm$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::DeviceAuthVerify) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DeviceAuthConfirmThen(::httpmock::Then);
    impl DeviceAuthConfirmThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DeviceAccessTokenWhen(::httpmock::When);
    impl DeviceAccessTokenWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/device/token$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::DeviceAccessTokenRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DeviceAccessTokenThen(::httpmock::Then);
    impl DeviceAccessTokenThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct GroupListWhen(::httpmock::When);
    impl GroupListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct GroupListThen(::httpmock::Then);
    impl GroupListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::GroupResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LoginSpoofWhen(::httpmock::When);
    impl LoginSpoofWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/login$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SpoofLoginBody) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct LoginSpoofThen(::httpmock::Then);
    impl LoginSpoofThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LoginLocalWhen(::httpmock::When);
    impl LoginLocalWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/login/[^/]*/local$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/login/{}/local$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::UsernamePasswordCredentials) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct LoginLocalThen(::httpmock::Then);
    impl LoginLocalThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn see_other(self) -> Self {
            Self(self.0.status(303u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn success(self, status: u16, value: ::serde_json::Value) -> Self {
            assert_eq!(status / 100u16, 2u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct LoginSamlBeginWhen(::httpmock::When);
    impl LoginSamlBeginWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/login/[^/]*/saml/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/login/{}/saml/.*$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn provider_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/login/.*/saml/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct LoginSamlBeginThen(::httpmock::Then);
    impl LoginSamlBeginThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn found(self) -> Self {
            Self(self.0.status(302u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn success(self, status: u16, value: ::serde_json::Value) -> Self {
            assert_eq!(status / 100u16, 2u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct LoginSamlWhen(::httpmock::When);
    impl LoginSamlWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/login/[^/]*/saml/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/login/{}/saml/.*$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn provider_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/login/.*/saml/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: ::serde_json::Value) -> Self {
            Self(self.0.json_body(value))
        }
    }

    pub struct LoginSamlThen(::httpmock::Then);
    impl LoginSamlThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn see_other(self) -> Self {
            Self(self.0.status(303u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn success(self, status: u16, value: ::serde_json::Value) -> Self {
            assert_eq!(status / 100u16, 2u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct LogoutWhen(::httpmock::When);
    impl LogoutWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/logout$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct LogoutThen(::httpmock::Then);
    impl LogoutThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationListWhen(::httpmock::When);
    impl OrganizationListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/organizations$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct OrganizationListThen(::httpmock::Then);
    impl OrganizationListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::OrganizationResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationCreateWhen(::httpmock::When);
    impl OrganizationCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/organizations$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::OrganizationCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct OrganizationCreateThen(::httpmock::Then);
    impl OrganizationCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Organization) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationViewWhen(::httpmock::When);
    impl OrganizationViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/organizations/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/organizations/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct OrganizationViewThen(::httpmock::Then);
    impl OrganizationViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Organization) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationUpdateWhen(::httpmock::When);
    impl OrganizationUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/organizations/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/organizations/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::OrganizationUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct OrganizationUpdateThen(::httpmock::Then);
    impl OrganizationUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Organization) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationDeleteWhen(::httpmock::When);
    impl OrganizationDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/organizations/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/organizations/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct OrganizationDeleteThen(::httpmock::Then);
    impl OrganizationDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationPolicyViewWhen(::httpmock::When);
    impl OrganizationPolicyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/organizations/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/organizations/{}/policy$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct OrganizationPolicyViewThen(::httpmock::Then);
    impl OrganizationPolicyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::OrganizationRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationPolicyUpdateWhen(::httpmock::When);
    impl OrganizationPolicyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/organizations/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/organizations/{}/policy$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::OrganizationRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct OrganizationPolicyUpdateThen(::httpmock::Then);
    impl OrganizationPolicyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::OrganizationRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectListWhen(::httpmock::When);
    impl ProjectListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/organizations/[^/]*/projects$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/organizations/{}/projects$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct ProjectListThen(::httpmock::Then);
    impl ProjectListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProjectResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectCreateWhen(::httpmock::When);
    impl ProjectCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/organizations/[^/]*/projects$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/organizations/{}/projects$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::ProjectCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectCreateThen(::httpmock::Then);
    impl ProjectCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectViewWhen(::httpmock::When);
    impl ProjectViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/organizations/[^/]*/projects/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectViewThen(::httpmock::Then);
    impl ProjectViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectUpdateWhen(::httpmock::When);
    impl ProjectUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::PUT).path_matches(
                    regex::Regex::new("^/organizations/[^/]*/projects/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::ProjectUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectUpdateThen(::httpmock::Then);
    impl ProjectUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectDeleteWhen(::httpmock::When);
    impl ProjectDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::DELETE).path_matches(
                    regex::Regex::new("^/organizations/[^/]*/projects/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectDeleteThen(::httpmock::Then);
    impl ProjectDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskListWhen(::httpmock::When);
    impl DiskListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/disks$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct DiskListThen(::httpmock::Then);
    impl DiskListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::DiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskCreateWhen(::httpmock::When);
    impl DiskCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/disks$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::DiskCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DiskCreateThen(::httpmock::Then);
    impl DiskCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskViewWhen(::httpmock::When);
    impl DiskViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/disks/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/disks/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/disks/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn disk_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/disks/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct DiskViewThen(::httpmock::Then);
    impl DiskViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskDeleteWhen(::httpmock::When);
    impl DiskDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/disks/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/disks/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/disks/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn disk_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/disks/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct DiskDeleteThen(::httpmock::Then);
    impl DiskDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskMetricsListWhen(::httpmock::When);
    impl DiskMetricsListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/disks/[^/]*/metrics/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/disks/.*/metrics/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/disks/.*/metrics/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn disk_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/disks/{}/metrics/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn metric_name(self, value: types::DiskMetricName) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/disks/.*/metrics/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn end_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("end_time", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("end_time"))
            }
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn start_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("start_time", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("start_time"))
            }
        }
    }

    pub struct DiskMetricsListThen(::httpmock::Then);
    impl DiskMetricsListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::MeasurementResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageListWhen(::httpmock::When);
    impl ImageListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/images$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/images$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/images$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct ImageListThen(::httpmock::Then);
    impl ImageListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ImageResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageCreateWhen(::httpmock::When);
    impl ImageCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/images$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/images$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/images$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::ImageCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ImageCreateThen(::httpmock::Then);
    impl ImageCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Image) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageViewWhen(::httpmock::When);
    impl ImageViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/images/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/images/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/images/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn image_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/images/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ImageViewThen(::httpmock::Then);
    impl ImageViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Image) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageDeleteWhen(::httpmock::When);
    impl ImageDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/images/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/images/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/images/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn image_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/images/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ImageDeleteThen(::httpmock::Then);
    impl ImageDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceListWhen(::httpmock::When);
    impl InstanceListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/instances$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct InstanceListThen(::httpmock::Then);
    impl InstanceListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceCreateWhen(::httpmock::When);
    impl InstanceCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/instances$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::InstanceCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceCreateThen(::httpmock::Then);
    impl InstanceCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceViewWhen(::httpmock::When);
    impl InstanceViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/instances/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceViewThen(::httpmock::Then);
    impl InstanceViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDeleteWhen(::httpmock::When);
    impl InstanceDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/instances/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceDeleteThen(::httpmock::Then);
    impl InstanceDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDiskListWhen(::httpmock::When);
    impl InstanceDiskListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/disks$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct InstanceDiskListThen(::httpmock::Then);
    impl InstanceDiskListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::DiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDiskAttachWhen(::httpmock::When);
    impl InstanceDiskAttachWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/disks/attach$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/disks/attach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/disks/attach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/disks/attach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::DiskIdentifier) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceDiskAttachThen(::httpmock::Then);
    impl InstanceDiskAttachThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDiskDetachWhen(::httpmock::When);
    impl InstanceDiskDetachWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/disks/detach$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/disks/detach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/disks/detach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/disks/detach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::DiskIdentifier) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceDiskDetachThen(::httpmock::Then);
    impl InstanceDiskDetachThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceExternalIpListWhen(::httpmock::When);
    impl InstanceExternalIpListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/external-ips$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/external-ips$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/external-ips$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/external-ips$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceExternalIpListThen(::httpmock::Then);
    impl InstanceExternalIpListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ExternalIpResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceMigrateWhen(::httpmock::When);
    impl InstanceMigrateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/migrate$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/migrate$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/migrate$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/migrate$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::InstanceMigrate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceMigrateThen(::httpmock::Then);
    impl InstanceMigrateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceListWhen(::httpmock::When);
    impl InstanceNetworkInterfaceListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/network-interfaces$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct InstanceNetworkInterfaceListThen(::httpmock::Then);
    impl InstanceNetworkInterfaceListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::NetworkInterfaceResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceCreateWhen(::httpmock::When);
    impl InstanceNetworkInterfaceCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/network-interfaces$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::NetworkInterfaceCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceNetworkInterfaceCreateThen(::httpmock::Then);
    impl InstanceNetworkInterfaceCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::NetworkInterface) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceViewWhen(::httpmock::When);
    impl InstanceNetworkInterfaceViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/network-interfaces/\
                         [^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/network-interfaces/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/network-interfaces/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/network-interfaces/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn interface_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/.*/network-interfaces/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceNetworkInterfaceViewThen(::httpmock::Then);
    impl InstanceNetworkInterfaceViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::NetworkInterface) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceUpdateWhen(::httpmock::When);
    impl InstanceNetworkInterfaceUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::PUT).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/network-interfaces/\
                         [^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/network-interfaces/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/network-interfaces/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/network-interfaces/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn interface_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/.*/network-interfaces/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::NetworkInterfaceUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceNetworkInterfaceUpdateThen(::httpmock::Then);
    impl InstanceNetworkInterfaceUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::NetworkInterface) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceDeleteWhen(::httpmock::When);
    impl InstanceNetworkInterfaceDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::DELETE).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/network-interfaces/\
                         [^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/network-interfaces/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/network-interfaces/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/network-interfaces/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn interface_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/.*/network-interfaces/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceNetworkInterfaceDeleteThen(::httpmock::Then);
    impl InstanceNetworkInterfaceDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceRebootWhen(::httpmock::When);
    impl InstanceRebootWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/reboot$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/reboot$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/reboot$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/reboot$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceRebootThen(::httpmock::Then);
    impl InstanceRebootThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceSerialConsoleWhen(::httpmock::When);
    impl InstanceSerialConsoleWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/serial-console$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/serial-console$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/serial-console$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/serial-console$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn from_start<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("from_start", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("from_start"))
            }
        }

        pub fn max_bytes<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("max_bytes", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("max_bytes"))
            }
        }

        pub fn most_recent<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("most_recent", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("most_recent"))
            }
        }
    }

    pub struct InstanceSerialConsoleThen(::httpmock::Then);
    impl InstanceSerialConsoleThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceSerialConsoleData) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceSerialConsoleStreamWhen(::httpmock::When);
    impl InstanceSerialConsoleStreamWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/serial-console/\
                         stream$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/serial-console/stream$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/serial-console/stream$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/serial-console/stream$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceSerialConsoleStreamThen(::httpmock::Then);
    impl InstanceSerialConsoleStreamThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16) -> Self {
            Self(self.0.status(status))
        }

        pub fn switching_protocols(self) -> Self {
            Self(self.0.status(101u16))
        }
    }

    pub struct InstanceStartWhen(::httpmock::When);
    impl InstanceStartWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/start$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/start$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/start$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/start$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceStartThen(::httpmock::Then);
    impl InstanceStartThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceStopWhen(::httpmock::When);
    impl InstanceStopWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/instances/[^/]*/stop$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/instances/.*/stop$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/instances/.*/stop$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/instances/{}/stop$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceStopThen(::httpmock::Then);
    impl InstanceStopThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectPolicyViewWhen(::httpmock::When);
    impl ProjectPolicyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/policy$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/policy$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/policy$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectPolicyViewThen(::httpmock::Then);
    impl ProjectPolicyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProjectRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectPolicyUpdateWhen(::httpmock::When);
    impl ProjectPolicyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::PUT).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/policy$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/policy$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/policy$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::ProjectRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectPolicyUpdateThen(::httpmock::Then);
    impl ProjectPolicyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProjectRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SnapshotListWhen(::httpmock::When);
    impl SnapshotListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/snapshots$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/snapshots$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/snapshots$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SnapshotListThen(::httpmock::Then);
    impl SnapshotListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SnapshotResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SnapshotCreateWhen(::httpmock::When);
    impl SnapshotCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/snapshots$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/snapshots$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/snapshots$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::SnapshotCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SnapshotCreateThen(::httpmock::Then);
    impl SnapshotCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Snapshot) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SnapshotViewWhen(::httpmock::When);
    impl SnapshotViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/snapshots/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/snapshots/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/snapshots/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn snapshot_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/snapshots/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SnapshotViewThen(::httpmock::Then);
    impl SnapshotViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Snapshot) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SnapshotDeleteWhen(::httpmock::When);
    impl SnapshotDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/snapshots/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/snapshots/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/snapshots/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn snapshot_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/snapshots/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SnapshotDeleteThen(::httpmock::Then);
    impl SnapshotDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcListWhen(::httpmock::When);
    impl VpcListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/vpcs$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct VpcListThen(::httpmock::Then);
    impl VpcListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcCreateWhen(::httpmock::When);
    impl VpcCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/vpcs$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::VpcCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcCreateThen(::httpmock::Then);
    impl VpcCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Vpc) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcViewWhen(::httpmock::When);
    impl VpcViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcViewThen(::httpmock::Then);
    impl VpcViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Vpc) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcUpdateWhen(::httpmock::When);
    impl VpcUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::PUT).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::VpcUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcUpdateThen(::httpmock::Then);
    impl VpcUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Vpc) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcDeleteWhen(::httpmock::When);
    impl VpcDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcDeleteThen(::httpmock::Then);
    impl VpcDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcFirewallRulesViewWhen(::httpmock::When);
    impl VpcFirewallRulesViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/firewall/rules$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/firewall/rules$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/firewall/rules$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/firewall/rules$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcFirewallRulesViewThen(::httpmock::Then);
    impl VpcFirewallRulesViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcFirewallRules) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcFirewallRulesUpdateWhen(::httpmock::When);
    impl VpcFirewallRulesUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::PUT).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/firewall/rules$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/firewall/rules$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/firewall/rules$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/firewall/rules$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::VpcFirewallRuleUpdateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcFirewallRulesUpdateThen(::httpmock::Then);
    impl VpcFirewallRulesUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcFirewallRules) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterListWhen(::httpmock::When);
    impl VpcRouterListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct VpcRouterListThen(::httpmock::Then);
    impl VpcRouterListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcRouterResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterCreateWhen(::httpmock::When);
    impl VpcRouterCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::VpcRouterCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcRouterCreateThen(::httpmock::Then);
    impl VpcRouterCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::VpcRouter) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterViewWhen(::httpmock::When);
    impl VpcRouterViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn router_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcRouterViewThen(::httpmock::Then);
    impl VpcRouterViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcRouter) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterUpdateWhen(::httpmock::When);
    impl VpcRouterUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::PUT).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn router_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::VpcRouterUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcRouterUpdateThen(::httpmock::Then);
    impl VpcRouterUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcRouter) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterDeleteWhen(::httpmock::When);
    impl VpcRouterDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::DELETE).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn router_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcRouterDeleteThen(::httpmock::Then);
    impl VpcRouterDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteListWhen(::httpmock::When);
    impl VpcRouterRouteListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers/[^/]*/routes$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers/.*/routes$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers/.*/routes$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers/.*/routes$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn router_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/{}/routes$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct VpcRouterRouteListThen(::httpmock::Then);
    impl VpcRouterRouteListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RouterRouteResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteCreateWhen(::httpmock::When);
    impl VpcRouterRouteCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers/[^/]*/routes$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers/.*/routes$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers/.*/routes$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers/.*/routes$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn router_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/{}/routes$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::RouterRouteCreateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcRouterRouteCreateThen(::httpmock::Then);
    impl VpcRouterRouteCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::RouterRoute) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteViewWhen(::httpmock::When);
    impl VpcRouterRouteViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers/[^/]*/routes/[^/\
                         ]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers/.*/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers/.*/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers/.*/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn router_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/{}/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn route_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/.*/routes/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcRouterRouteViewThen(::httpmock::Then);
    impl VpcRouterRouteViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RouterRoute) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteUpdateWhen(::httpmock::When);
    impl VpcRouterRouteUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::PUT).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers/[^/]*/routes/[^/\
                         ]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers/.*/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers/.*/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers/.*/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn router_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/{}/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn route_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/.*/routes/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::RouterRouteUpdateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcRouterRouteUpdateThen(::httpmock::Then);
    impl VpcRouterRouteUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RouterRoute) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteDeleteWhen(::httpmock::When);
    impl VpcRouterRouteDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::DELETE).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/routers/[^/]*/routes/[^/\
                         ]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/routers/.*/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/routers/.*/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/routers/.*/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn router_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/{}/routes/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn route_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/routers/.*/routes/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcRouterRouteDeleteThen(::httpmock::Then);
    impl VpcRouterRouteDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetListWhen(::httpmock::When);
    impl VpcSubnetListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/subnets$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/subnets$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/subnets$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/subnets$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct VpcSubnetListThen(::httpmock::Then);
    impl VpcSubnetListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcSubnetResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetCreateWhen(::httpmock::When);
    impl VpcSubnetCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/subnets$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/subnets$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/subnets$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/subnets$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::VpcSubnetCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcSubnetCreateThen(::httpmock::Then);
    impl VpcSubnetCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::VpcSubnet) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetViewWhen(::httpmock::When);
    impl VpcSubnetViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/subnets/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/subnets/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/subnets/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/subnets/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn subnet_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/subnets/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcSubnetViewThen(::httpmock::Then);
    impl VpcSubnetViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcSubnet) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetUpdateWhen(::httpmock::When);
    impl VpcSubnetUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::PUT).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/subnets/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/subnets/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/subnets/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/subnets/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn subnet_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/subnets/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::VpcSubnetUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcSubnetUpdateThen(::httpmock::Then);
    impl VpcSubnetUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcSubnet) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetDeleteWhen(::httpmock::When);
    impl VpcSubnetDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::DELETE).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/subnets/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/subnets/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/subnets/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/subnets/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn subnet_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/subnets/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct VpcSubnetDeleteThen(::httpmock::Then);
    impl VpcSubnetDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetListNetworkInterfacesWhen(::httpmock::When);
    impl VpcSubnetListNetworkInterfacesWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/organizations/[^/]*/projects/[^/]*/vpcs/[^/]*/subnets/[^/]*/\
                         network-interfaces$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/{}/projects/.*/vpcs/.*/subnets/.*/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/{}/vpcs/.*/subnets/.*/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn vpc_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/{}/subnets/.*/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn subnet_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/organizations/.*/projects/.*/vpcs/.*/subnets/{}/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct VpcSubnetListNetworkInterfacesThen(::httpmock::Then);
    impl VpcSubnetListNetworkInterfacesThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::NetworkInterfaceResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct PolicyViewWhen(::httpmock::When);
    impl PolicyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct PolicyViewThen(::httpmock::Then);
    impl PolicyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct PolicyUpdateWhen(::httpmock::When);
    impl PolicyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SiloRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct PolicyUpdateThen(::httpmock::Then);
    impl PolicyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct RoleListWhen(::httpmock::When);
    impl RoleListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/roles$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }
    }

    pub struct RoleListThen(::httpmock::Then);
    impl RoleListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RoleResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct RoleViewWhen(::httpmock::When);
    impl RoleViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/roles/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn role_name(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!("^/roles/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct RoleViewThen(::httpmock::Then);
    impl RoleViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Role) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SessionMeWhen(::httpmock::When);
    impl SessionMeWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/session/me$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct SessionMeThen(::httpmock::Then);
    impl SessionMeThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::User) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SessionMeGroupsWhen(::httpmock::When);
    impl SessionMeGroupsWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/session/me/groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SessionMeGroupsThen(::httpmock::Then);
    impl SessionMeGroupsThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::GroupResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SessionSshkeyListWhen(::httpmock::When);
    impl SessionSshkeyListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/session/me/sshkeys$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SessionSshkeyListThen(::httpmock::Then);
    impl SessionSshkeyListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SshKeyResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SessionSshkeyCreateWhen(::httpmock::When);
    impl SessionSshkeyCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/session/me/sshkeys$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SshKeyCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SessionSshkeyCreateThen(::httpmock::Then);
    impl SessionSshkeyCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::SshKey) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SessionSshkeyViewWhen(::httpmock::When);
    impl SessionSshkeyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/session/me/sshkeys/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn ssh_key_name(self, value: &types::Name) -> Self {
            let re =
                regex::Regex::new(&format!("^/session/me/sshkeys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SessionSshkeyViewThen(::httpmock::Then);
    impl SessionSshkeyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SshKey) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SessionSshkeyDeleteWhen(::httpmock::When);
    impl SessionSshkeyDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/session/me/sshkeys/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn ssh_key_name(self, value: &types::Name) -> Self {
            let re =
                regex::Regex::new(&format!("^/session/me/sshkeys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SessionSshkeyDeleteThen(::httpmock::Then);
    impl SessionSshkeyDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemImageViewByIdWhen(::httpmock::When);
    impl SystemImageViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/by-id/images/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/system/by-id/images/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SystemImageViewByIdThen(::httpmock::Then);
    impl SystemImageViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::GlobalImage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolViewByIdWhen(::httpmock::When);
    impl IpPoolViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/by-id/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/system/by-id/ip-pools/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct IpPoolViewByIdThen(::httpmock::Then);
    impl IpPoolViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPool) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloViewByIdWhen(::httpmock::When);
    impl SiloViewByIdWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/by-id/silos/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/system/by-id/silos/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloViewByIdThen(::httpmock::Then);
    impl SiloViewByIdThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Silo) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CertificateListWhen(::httpmock::When);
    impl CertificateListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/certificates$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct CertificateListThen(::httpmock::Then);
    impl CertificateListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::CertificateResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CertificateCreateWhen(::httpmock::When);
    impl CertificateCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/system/certificates$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::CertificateCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct CertificateCreateThen(::httpmock::Then);
    impl CertificateCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Certificate) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CertificateViewWhen(::httpmock::When);
    impl CertificateViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/certificates/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn certificate(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/system/certificates/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CertificateViewThen(::httpmock::Then);
    impl CertificateViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Certificate) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CertificateDeleteWhen(::httpmock::When);
    impl CertificateDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/system/certificates/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn certificate(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/system/certificates/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CertificateDeleteThen(::httpmock::Then);
    impl CertificateDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct PhysicalDiskListWhen(::httpmock::When);
    impl PhysicalDiskListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/hardware/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct PhysicalDiskListThen(::httpmock::Then);
    impl PhysicalDiskListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::PhysicalDiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct RackListWhen(::httpmock::When);
    impl RackListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/hardware/racks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct RackListThen(::httpmock::Then);
    impl RackListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RackResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct RackViewWhen(::httpmock::When);
    impl RackViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/hardware/racks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn rack_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/system/hardware/racks/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct RackViewThen(::httpmock::Then);
    impl RackViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Rack) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SledListWhen(::httpmock::When);
    impl SledListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/hardware/sleds$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SledListThen(::httpmock::Then);
    impl SledListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SledResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SledViewWhen(::httpmock::When);
    impl SledViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/hardware/sleds/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn sled_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/system/hardware/sleds/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SledViewThen(::httpmock::Then);
    impl SledViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Sled) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SledPhysicalDiskListWhen(::httpmock::When);
    impl SledPhysicalDiskListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/system/hardware/sleds/[^/]*/disks$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn sled_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/hardware/sleds/{}/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SledPhysicalDiskListThen(::httpmock::Then);
    impl SledPhysicalDiskListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::PhysicalDiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemImageListWhen(::httpmock::When);
    impl SystemImageListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/images$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SystemImageListThen(::httpmock::Then);
    impl SystemImageListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::GlobalImageResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemImageCreateWhen(::httpmock::When);
    impl SystemImageCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/system/images$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::GlobalImageCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SystemImageCreateThen(::httpmock::Then);
    impl SystemImageCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::GlobalImage) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemImageViewWhen(::httpmock::When);
    impl SystemImageViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/images/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn image_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/system/images/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SystemImageViewThen(::httpmock::Then);
    impl SystemImageViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::GlobalImage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemImageDeleteWhen(::httpmock::When);
    impl SystemImageDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/system/images/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn image_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/system/images/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SystemImageDeleteThen(::httpmock::Then);
    impl SystemImageDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolListWhen(::httpmock::When);
    impl IpPoolListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct IpPoolListThen(::httpmock::Then);
    impl IpPoolListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPoolResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolCreateWhen(::httpmock::When);
    impl IpPoolCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/system/ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::IpPoolCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolCreateThen(::httpmock::Then);
    impl IpPoolCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::IpPool) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolViewWhen(::httpmock::When);
    impl IpPoolViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool_name(self, value: &types::Name) -> Self {
            let re =
                regex::Regex::new(&format!("^/system/ip-pools/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct IpPoolViewThen(::httpmock::Then);
    impl IpPoolViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPool) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolUpdateWhen(::httpmock::When);
    impl IpPoolUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/system/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool_name(self, value: &types::Name) -> Self {
            let re =
                regex::Regex::new(&format!("^/system/ip-pools/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::IpPoolUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolUpdateThen(::httpmock::Then);
    impl IpPoolUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPool) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolDeleteWhen(::httpmock::When);
    impl IpPoolDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/system/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool_name(self, value: &types::Name) -> Self {
            let re =
                regex::Regex::new(&format!("^/system/ip-pools/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct IpPoolDeleteThen(::httpmock::Then);
    impl IpPoolDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolRangeListWhen(::httpmock::When);
    impl IpPoolRangeListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/ip-pools/[^/]*/ranges$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/system/ip-pools/{}/ranges$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }
    }

    pub struct IpPoolRangeListThen(::httpmock::Then);
    impl IpPoolRangeListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPoolRangeResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolRangeAddWhen(::httpmock::When);
    impl IpPoolRangeAddWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/system/ip-pools/[^/]*/ranges/add$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/ip-pools/{}/ranges/add$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolRangeAddThen(::httpmock::Then);
    impl IpPoolRangeAddThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::IpPoolRange) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolRangeRemoveWhen(::httpmock::When);
    impl IpPoolRangeRemoveWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/system/ip-pools/[^/]*/ranges/remove$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/ip-pools/{}/ranges/remove$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolRangeRemoveThen(::httpmock::Then);
    impl IpPoolRangeRemoveThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolServiceViewWhen(::httpmock::When);
    impl IpPoolServiceViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/ip-pools-service$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct IpPoolServiceViewThen(::httpmock::Then);
    impl IpPoolServiceViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPool) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolServiceRangeListWhen(::httpmock::When);
    impl IpPoolServiceRangeListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/ip-pools-service/ranges$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }
    }

    pub struct IpPoolServiceRangeListThen(::httpmock::Then);
    impl IpPoolServiceRangeListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPoolRangeResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolServiceRangeAddWhen(::httpmock::When);
    impl IpPoolServiceRangeAddWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/system/ip-pools-service/ranges/add$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolServiceRangeAddThen(::httpmock::Then);
    impl IpPoolServiceRangeAddThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::IpPoolRange) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolServiceRangeRemoveWhen(::httpmock::When);
    impl IpPoolServiceRangeRemoveWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/system/ip-pools-service/ranges/remove$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolServiceRangeRemoveThen(::httpmock::Then);
    impl IpPoolServiceRangeRemoveThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemMetricWhen(::httpmock::When);
    impl SystemMetricWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/metrics/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn metric_name(self, value: types::SystemMetricName) -> Self {
            let re =
                regex::Regex::new(&format!("^/system/metrics/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn end_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("end_time", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("end_time"))
            }
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let expected_pairs = ::progenitor_client::query_param_pairs("id", &value)
                .expect("failed to serialize query param");
            Self(apply_query_param_pairs(self.0, &expected_pairs))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn start_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("start_time", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("start_time"))
            }
        }
    }

    pub struct SystemMetricThen(::httpmock::Then);
    impl SystemMetricThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::MeasurementResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemPolicyViewWhen(::httpmock::When);
    impl SystemPolicyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct SystemPolicyViewThen(::httpmock::Then);
    impl SystemPolicyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::FleetRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemPolicyUpdateWhen(::httpmock::When);
    impl SystemPolicyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/system/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::FleetRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SystemPolicyUpdateThen(::httpmock::Then);
    impl SystemPolicyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::FleetRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SagaListWhen(::httpmock::When);
    impl SagaListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/sagas$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SagaListThen(::httpmock::Then);
    impl SagaListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SagaResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SagaViewWhen(::httpmock::When);
    impl SagaViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/sagas/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn saga_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/system/sagas/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SagaViewThen(::httpmock::Then);
    impl SagaViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Saga) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloListWhen(::httpmock::When);
    impl SiloListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/silos$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SiloListThen(::httpmock::Then);
    impl SiloListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloCreateWhen(::httpmock::When);
    impl SiloCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/system/silos$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SiloCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SiloCreateThen(::httpmock::Then);
    impl SiloCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Silo) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloViewWhen(::httpmock::When);
    impl SiloViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/silos/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/system/silos/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloViewThen(::httpmock::Then);
    impl SiloViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Silo) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloDeleteWhen(::httpmock::When);
    impl SiloDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/system/silos/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/system/silos/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloDeleteThen(::httpmock::Then);
    impl SiloDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloIdentityProviderListWhen(::httpmock::When);
    impl SiloIdentityProviderListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/system/silos/[^/]*/identity-providers$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/{}/identity-providers$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SiloIdentityProviderListThen(::httpmock::Then);
    impl SiloIdentityProviderListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IdentityProviderResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LocalIdpUserCreateWhen(::httpmock::When);
    impl LocalIdpUserCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/system/silos/[^/]*/identity-providers/local/users$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/{}/identity-providers/local/users$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::UserCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct LocalIdpUserCreateThen(::httpmock::Then);
    impl LocalIdpUserCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::User) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LocalIdpUserDeleteWhen(::httpmock::When);
    impl LocalIdpUserDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::DELETE).path_matches(
                    regex::Regex::new("^/system/silos/[^/]*/identity-providers/local/users/[^/]*$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/{}/identity-providers/local/users/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn user_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/.*/identity-providers/local/users/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct LocalIdpUserDeleteThen(::httpmock::Then);
    impl LocalIdpUserDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LocalIdpUserSetPasswordWhen(::httpmock::When);
    impl LocalIdpUserSetPasswordWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/system/silos/[^/]*/identity-providers/local/users/[^/]*/set-password$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/{}/identity-providers/local/users/.*/set-password$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn user_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/.*/identity-providers/local/users/{}/set-password$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::UserPassword) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct LocalIdpUserSetPasswordThen(::httpmock::Then);
    impl LocalIdpUserSetPasswordThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SamlIdentityProviderCreateWhen(::httpmock::When);
    impl SamlIdentityProviderCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/system/silos/[^/]*/identity-providers/saml$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/{}/identity-providers/saml$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::SamlIdentityProviderCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SamlIdentityProviderCreateThen(::httpmock::Then);
    impl SamlIdentityProviderCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::SamlIdentityProvider) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SamlIdentityProviderViewWhen(::httpmock::When);
    impl SamlIdentityProviderViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/system/silos/[^/]*/identity-providers/saml/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/{}/identity-providers/saml/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn provider_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/.*/identity-providers/saml/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SamlIdentityProviderViewThen(::httpmock::Then);
    impl SamlIdentityProviderViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SamlIdentityProvider) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloPolicyViewWhen(::httpmock::When);
    impl SiloPolicyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/silos/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/system/silos/{}/policy$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloPolicyViewThen(::httpmock::Then);
    impl SiloPolicyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloPolicyUpdateWhen(::httpmock::When);
    impl SiloPolicyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/system/silos/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/system/silos/{}/policy$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::SiloRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SiloPolicyUpdateThen(::httpmock::Then);
    impl SiloPolicyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloUsersListWhen(::httpmock::When);
    impl SiloUsersListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/silos/[^/]*/users/all$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/system/silos/{}/users/all$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SiloUsersListThen(::httpmock::Then);
    impl SiloUsersListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UserResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloUserViewWhen(::httpmock::When);
    impl SiloUserViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/system/silos/[^/]*/users/id/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/{}/users/id/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn user_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/system/silos/.*/users/id/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloUserViewThen(::httpmock::Then);
    impl SiloUserViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::User) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUserListWhen(::httpmock::When);
    impl SystemUserListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/user$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SystemUserListThen(::httpmock::Then);
    impl SystemUserListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UserBuiltinResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUserViewWhen(::httpmock::When);
    impl SystemUserViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/system/user/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn user_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/system/user/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SystemUserViewThen(::httpmock::Then);
    impl SystemUserViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UserBuiltin) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct TimeseriesSchemaGetWhen(::httpmock::When);
    impl TimeseriesSchemaGetWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/timeseries/schema$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }
    }

    pub struct TimeseriesSchemaGetThen(::httpmock::Then);
    impl TimeseriesSchemaGetThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::TimeseriesSchemaResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct UserListWhen(::httpmock::When);
    impl UserListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/users$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct UserListThen(::httpmock::Then);
    impl UserListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UserResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskListV1When(::httpmock::When);
    impl DiskListV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct DiskListV1Then(::httpmock::Then);
    impl DiskListV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::DiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskCreateV1When(::httpmock::When);
    impl DiskCreateV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                .expect("failed to serialize query param");
            Self(apply_query_param_pairs(self.0, &expected_pairs))
        }

        pub fn body(self, value: &types::DiskCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DiskCreateV1Then(::httpmock::Then);
    impl DiskCreateV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskViewV1When(::httpmock::When);
    impl DiskViewV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/disks/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }
    }

    pub struct DiskViewV1Then(::httpmock::Then);
    impl DiskViewV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskDeleteV1When(::httpmock::When);
    impl DiskDeleteV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/disks/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }
    }

    pub struct DiskDeleteV1Then(::httpmock::Then);
    impl DiskDeleteV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceListV1When(::httpmock::When);
    impl InstanceListV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct InstanceListV1Then(::httpmock::Then);
    impl InstanceListV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceCreateV1When(::httpmock::When);
    impl InstanceCreateV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                .expect("failed to serialize query param");
            Self(apply_query_param_pairs(self.0, &expected_pairs))
        }

        pub fn body(self, value: &types::InstanceCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceCreateV1Then(::httpmock::Then);
    impl InstanceCreateV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceViewV1When(::httpmock::When);
    impl InstanceViewV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/instances/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }
    }

    pub struct InstanceViewV1Then(::httpmock::Then);
    impl InstanceViewV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDeleteV1When(::httpmock::When);
    impl InstanceDeleteV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/instances/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }
    }

    pub struct InstanceDeleteV1Then(::httpmock::Then);
    impl InstanceDeleteV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDiskListV1When(::httpmock::When);
    impl InstanceDiskListV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/instances/{}/disks$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct InstanceDiskListV1Then(::httpmock::Then);
    impl InstanceDiskListV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::DiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDiskAttachV1When(::httpmock::When);
    impl InstanceDiskAttachV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/disks/attach$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/disks/attach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }

        pub fn body(self, value: &types::DiskPath) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceDiskAttachV1Then(::httpmock::Then);
    impl InstanceDiskAttachV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDiskDetachV1When(::httpmock::When);
    impl InstanceDiskDetachV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/disks/detach$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/disks/detach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }

        pub fn body(self, value: &types::DiskPath) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceDiskDetachV1Then(::httpmock::Then);
    impl InstanceDiskDetachV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceMigrateV1When(::httpmock::When);
    impl InstanceMigrateV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/migrate$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/instances/{}/migrate$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }

        pub fn body(self, value: &types::InstanceMigrate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceMigrateV1Then(::httpmock::Then);
    impl InstanceMigrateV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceRebootV1When(::httpmock::When);
    impl InstanceRebootV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/reboot$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/instances/{}/reboot$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }
    }

    pub struct InstanceRebootV1Then(::httpmock::Then);
    impl InstanceRebootV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceSerialConsoleV1When(::httpmock::When);
    impl InstanceSerialConsoleV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/instances/[^/]*/serial-console$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/serial-console$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn from_start<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("from_start", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("from_start"))
            }
        }

        pub fn max_bytes<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("max_bytes", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("max_bytes"))
            }
        }

        pub fn most_recent<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("most_recent", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("most_recent"))
            }
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }
    }

    pub struct InstanceSerialConsoleV1Then(::httpmock::Then);
    impl InstanceSerialConsoleV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceSerialConsoleData) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceSerialConsoleStreamV1When(::httpmock::When);
    impl InstanceSerialConsoleStreamV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/instances/[^/]*/serial-console/stream$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/serial-console/stream$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }
    }

    pub struct InstanceSerialConsoleStreamV1Then(::httpmock::Then);
    impl InstanceSerialConsoleStreamV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16) -> Self {
            Self(self.0.status(status))
        }

        pub fn switching_protocols(self) -> Self {
            Self(self.0.status(101u16))
        }
    }

    pub struct InstanceStartV1When(::httpmock::When);
    impl InstanceStartV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/start$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/instances/{}/start$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }
    }

    pub struct InstanceStartV1Then(::httpmock::Then);
    impl InstanceStartV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceStopV1When(::httpmock::When);
    impl InstanceStopV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/stop$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/instances/{}/stop$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("project", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("project"))
            }
        }
    }

    pub struct InstanceStopV1Then(::httpmock::Then);
    impl InstanceStopV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationListV1When(::httpmock::When);
    impl OrganizationListV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/organizations$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct OrganizationListV1Then(::httpmock::Then);
    impl OrganizationListV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::OrganizationResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationCreateV1When(::httpmock::When);
    impl OrganizationCreateV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/organizations$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::OrganizationCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct OrganizationCreateV1Then(::httpmock::Then);
    impl OrganizationCreateV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Organization) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationViewV1When(::httpmock::When);
    impl OrganizationViewV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/organizations/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/organizations/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct OrganizationViewV1Then(::httpmock::Then);
    impl OrganizationViewV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Organization) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationUpdateV1When(::httpmock::When);
    impl OrganizationUpdateV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/organizations/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/organizations/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::OrganizationUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct OrganizationUpdateV1Then(::httpmock::Then);
    impl OrganizationUpdateV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Organization) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationDeleteV1When(::httpmock::When);
    impl OrganizationDeleteV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/organizations/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/organizations/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct OrganizationDeleteV1Then(::httpmock::Then);
    impl OrganizationDeleteV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationPolicyViewV1When(::httpmock::When);
    impl OrganizationPolicyViewV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/organizations/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/organizations/{}/policy$", value.to_string()))
                    .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct OrganizationPolicyViewV1Then(::httpmock::Then);
    impl OrganizationPolicyViewV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::OrganizationRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct OrganizationPolicyUpdateV1When(::httpmock::When);
    impl OrganizationPolicyUpdateV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/organizations/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/organizations/{}/policy$", value.to_string()))
                    .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::OrganizationRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct OrganizationPolicyUpdateV1Then(::httpmock::Then);
    impl OrganizationPolicyUpdateV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::OrganizationRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectListV1When(::httpmock::When);
    impl ProjectListV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/projects$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct ProjectListV1Then(::httpmock::Then);
    impl ProjectListV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProjectResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectCreateV1When(::httpmock::When);
    impl ProjectCreateV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/projects$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn organization(self, value: &types::NameOrId) -> Self {
            let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                .expect("failed to serialize query param");
            Self(apply_query_param_pairs(self.0, &expected_pairs))
        }

        pub fn body(self, value: &types::ProjectCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectCreateV1Then(::httpmock::Then);
    impl ProjectCreateV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectViewV1When(::httpmock::When);
    impl ProjectViewV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/projects/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }
    }

    pub struct ProjectViewV1Then(::httpmock::Then);
    impl ProjectViewV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectUpdateV1When(::httpmock::When);
    impl ProjectUpdateV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/projects/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn body(self, value: &types::ProjectUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectUpdateV1Then(::httpmock::Then);
    impl ProjectUpdateV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectDeleteV1When(::httpmock::When);
    impl ProjectDeleteV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/projects/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }
    }

    pub struct ProjectDeleteV1Then(::httpmock::Then);
    impl ProjectDeleteV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectPolicyViewV1When(::httpmock::When);
    impl ProjectPolicyViewV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/projects/{}/policy$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }
    }

    pub struct ProjectPolicyViewV1Then(::httpmock::Then);
    impl ProjectPolicyViewV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProjectRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectPolicyUpdateV1When(::httpmock::When);
    impl ProjectPolicyUpdateV1When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/projects/{}/policy$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn organization<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("organization", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("organization"))
            }
        }

        pub fn body(self, value: &types::ProjectRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectPolicyUpdateV1Then(::httpmock::Then);
    impl ProjectPolicyUpdateV1Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProjectRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemComponentVersionListWhen(::httpmock::When);
    impl SystemComponentVersionListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/components$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SystemComponentVersionListThen(::httpmock::Then);
    impl SystemComponentVersionListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UpdateableComponentResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct UpdateDeploymentsListWhen(::httpmock::When);
    impl UpdateDeploymentsListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/deployments$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct UpdateDeploymentsListThen(::httpmock::Then);
    impl UpdateDeploymentsListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UpdateDeploymentResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct UpdateDeploymentViewWhen(::httpmock::When);
    impl UpdateDeploymentViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/update/deployments/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/update/deployments/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct UpdateDeploymentViewThen(::httpmock::Then);
    impl UpdateDeploymentViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UpdateDeployment) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateRefreshWhen(::httpmock::When);
    impl SystemUpdateRefreshWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/update/refresh$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct SystemUpdateRefreshThen(::httpmock::Then);
    impl SystemUpdateRefreshThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateStartWhen(::httpmock::When);
    impl SystemUpdateStartWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/update/start$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SystemUpdateStart) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SystemUpdateStartThen(::httpmock::Then);
    impl SystemUpdateStartThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::UpdateDeployment) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateStopWhen(::httpmock::When);
    impl SystemUpdateStopWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/update/stop$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct SystemUpdateStopThen(::httpmock::Then);
    impl SystemUpdateStopThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateListWhen(::httpmock::When);
    impl SystemUpdateListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/updates$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("limit", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("limit"))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("page_token", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("page_token"))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("sort_by", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("sort_by"))
            }
        }
    }

    pub struct SystemUpdateListThen(::httpmock::Then);
    impl SystemUpdateListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SystemUpdateResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateViewWhen(::httpmock::When);
    impl SystemUpdateViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/updates/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn version(self, value: &types::SemverVersion) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/update/updates/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SystemUpdateViewThen(::httpmock::Then);
    impl SystemUpdateViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SystemUpdate) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateComponentsListWhen(::httpmock::When);
    impl SystemUpdateComponentsListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/update/updates/[^/]*/components$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn version(self, value: &types::SemverVersion) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/update/updates/{}/components$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SystemUpdateComponentsListThen(::httpmock::Then);
    impl SystemUpdateComponentsListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ComponentUpdateResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemVersionWhen(::httpmock::When);
    impl SystemVersionWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/version$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct SystemVersionThen(::httpmock::Then);
    impl SystemVersionThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SystemVersion) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
}

#[doc = r" An extension trait for [`MockServer`](::httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](::httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn disk_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskViewByIdWhen, operations::DiskViewByIdThen);
    fn image_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageViewByIdWhen, operations::ImageViewByIdThen);
    fn instance_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceViewByIdWhen, operations::InstanceViewByIdThen);
    fn instance_network_interface_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceViewByIdWhen,
            operations::InstanceNetworkInterfaceViewByIdThen,
        );
    fn organization_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationViewByIdWhen, operations::OrganizationViewByIdThen);
    fn project_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectViewByIdWhen, operations::ProjectViewByIdThen);
    fn snapshot_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotViewByIdWhen, operations::SnapshotViewByIdThen);
    fn vpc_router_route_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteViewByIdWhen, operations::VpcRouterRouteViewByIdThen);
    fn vpc_router_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterViewByIdWhen, operations::VpcRouterViewByIdThen);
    fn vpc_subnet_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetViewByIdWhen, operations::VpcSubnetViewByIdThen);
    fn vpc_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcViewByIdWhen, operations::VpcViewByIdThen);
    fn device_auth_request<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DeviceAuthRequestWhen, operations::DeviceAuthRequestThen);
    fn device_auth_confirm<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DeviceAuthConfirmWhen, operations::DeviceAuthConfirmThen);
    fn device_access_token<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DeviceAccessTokenWhen, operations::DeviceAccessTokenThen);
    fn group_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::GroupListWhen, operations::GroupListThen);
    fn login_spoof<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LoginSpoofWhen, operations::LoginSpoofThen);
    fn login_local<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LoginLocalWhen, operations::LoginLocalThen);
    fn login_saml_begin<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LoginSamlBeginWhen, operations::LoginSamlBeginThen);
    fn login_saml<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LoginSamlWhen, operations::LoginSamlThen);
    fn logout<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LogoutWhen, operations::LogoutThen);
    fn organization_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationListWhen, operations::OrganizationListThen);
    fn organization_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationCreateWhen, operations::OrganizationCreateThen);
    fn organization_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationViewWhen, operations::OrganizationViewThen);
    fn organization_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationUpdateWhen, operations::OrganizationUpdateThen);
    fn organization_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationDeleteWhen, operations::OrganizationDeleteThen);
    fn organization_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationPolicyViewWhen, operations::OrganizationPolicyViewThen);
    fn organization_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::OrganizationPolicyUpdateWhen,
            operations::OrganizationPolicyUpdateThen,
        );
    fn project_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectListWhen, operations::ProjectListThen);
    fn project_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectCreateWhen, operations::ProjectCreateThen);
    fn project_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectViewWhen, operations::ProjectViewThen);
    fn project_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectUpdateWhen, operations::ProjectUpdateThen);
    fn project_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectDeleteWhen, operations::ProjectDeleteThen);
    fn disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskListWhen, operations::DiskListThen);
    fn disk_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskCreateWhen, operations::DiskCreateThen);
    fn disk_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskViewWhen, operations::DiskViewThen);
    fn disk_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskDeleteWhen, operations::DiskDeleteThen);
    fn disk_metrics_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskMetricsListWhen, operations::DiskMetricsListThen);
    fn image_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageListWhen, operations::ImageListThen);
    fn image_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageCreateWhen, operations::ImageCreateThen);
    fn image_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageViewWhen, operations::ImageViewThen);
    fn image_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageDeleteWhen, operations::ImageDeleteThen);
    fn instance_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceListWhen, operations::InstanceListThen);
    fn instance_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceCreateWhen, operations::InstanceCreateThen);
    fn instance_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceViewWhen, operations::InstanceViewThen);
    fn instance_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDeleteWhen, operations::InstanceDeleteThen);
    fn instance_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskListWhen, operations::InstanceDiskListThen);
    fn instance_disk_attach<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskAttachWhen, operations::InstanceDiskAttachThen);
    fn instance_disk_detach<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskDetachWhen, operations::InstanceDiskDetachThen);
    fn instance_external_ip_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceExternalIpListWhen, operations::InstanceExternalIpListThen);
    fn instance_migrate<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceMigrateWhen, operations::InstanceMigrateThen);
    fn instance_network_interface_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceListWhen,
            operations::InstanceNetworkInterfaceListThen,
        );
    fn instance_network_interface_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceCreateWhen,
            operations::InstanceNetworkInterfaceCreateThen,
        );
    fn instance_network_interface_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceViewWhen,
            operations::InstanceNetworkInterfaceViewThen,
        );
    fn instance_network_interface_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceUpdateWhen,
            operations::InstanceNetworkInterfaceUpdateThen,
        );
    fn instance_network_interface_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceDeleteWhen,
            operations::InstanceNetworkInterfaceDeleteThen,
        );
    fn instance_reboot<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceRebootWhen, operations::InstanceRebootThen);
    fn instance_serial_console<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceSerialConsoleWhen, operations::InstanceSerialConsoleThen);
    fn instance_serial_console_stream<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceSerialConsoleStreamWhen,
            operations::InstanceSerialConsoleStreamThen,
        );
    fn instance_start<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStartWhen, operations::InstanceStartThen);
    fn instance_stop<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStopWhen, operations::InstanceStopThen);
    fn project_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectPolicyViewWhen, operations::ProjectPolicyViewThen);
    fn project_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectPolicyUpdateWhen, operations::ProjectPolicyUpdateThen);
    fn snapshot_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotListWhen, operations::SnapshotListThen);
    fn snapshot_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotCreateWhen, operations::SnapshotCreateThen);
    fn snapshot_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotViewWhen, operations::SnapshotViewThen);
    fn snapshot_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotDeleteWhen, operations::SnapshotDeleteThen);
    fn vpc_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcListWhen, operations::VpcListThen);
    fn vpc_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcCreateWhen, operations::VpcCreateThen);
    fn vpc_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcViewWhen, operations::VpcViewThen);
    fn vpc_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcUpdateWhen, operations::VpcUpdateThen);
    fn vpc_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcDeleteWhen, operations::VpcDeleteThen);
    fn vpc_firewall_rules_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcFirewallRulesViewWhen, operations::VpcFirewallRulesViewThen);
    fn vpc_firewall_rules_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcFirewallRulesUpdateWhen, operations::VpcFirewallRulesUpdateThen);
    fn vpc_router_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterListWhen, operations::VpcRouterListThen);
    fn vpc_router_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterCreateWhen, operations::VpcRouterCreateThen);
    fn vpc_router_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterViewWhen, operations::VpcRouterViewThen);
    fn vpc_router_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterUpdateWhen, operations::VpcRouterUpdateThen);
    fn vpc_router_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterDeleteWhen, operations::VpcRouterDeleteThen);
    fn vpc_router_route_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteListWhen, operations::VpcRouterRouteListThen);
    fn vpc_router_route_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteCreateWhen, operations::VpcRouterRouteCreateThen);
    fn vpc_router_route_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteViewWhen, operations::VpcRouterRouteViewThen);
    fn vpc_router_route_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteUpdateWhen, operations::VpcRouterRouteUpdateThen);
    fn vpc_router_route_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteDeleteWhen, operations::VpcRouterRouteDeleteThen);
    fn vpc_subnet_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetListWhen, operations::VpcSubnetListThen);
    fn vpc_subnet_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetCreateWhen, operations::VpcSubnetCreateThen);
    fn vpc_subnet_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetViewWhen, operations::VpcSubnetViewThen);
    fn vpc_subnet_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetUpdateWhen, operations::VpcSubnetUpdateThen);
    fn vpc_subnet_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetDeleteWhen, operations::VpcSubnetDeleteThen);
    fn vpc_subnet_list_network_interfaces<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::VpcSubnetListNetworkInterfacesWhen,
            operations::VpcSubnetListNetworkInterfacesThen,
        );
    fn policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::PolicyViewWhen, operations::PolicyViewThen);
    fn policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::PolicyUpdateWhen, operations::PolicyUpdateThen);
    fn role_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::RoleListWhen, operations::RoleListThen);
    fn role_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::RoleViewWhen, operations::RoleViewThen);
    fn session_me<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionMeWhen, operations::SessionMeThen);
    fn session_me_groups<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionMeGroupsWhen, operations::SessionMeGroupsThen);
    fn session_sshkey_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionSshkeyListWhen, operations::SessionSshkeyListThen);
    fn session_sshkey_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionSshkeyCreateWhen, operations::SessionSshkeyCreateThen);
    fn session_sshkey_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionSshkeyViewWhen, operations::SessionSshkeyViewThen);
    fn session_sshkey_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionSshkeyDeleteWhen, operations::SessionSshkeyDeleteThen);
    fn system_image_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageViewByIdWhen, operations::SystemImageViewByIdThen);
    fn ip_pool_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolViewByIdWhen, operations::IpPoolViewByIdThen);
    fn silo_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloViewByIdWhen, operations::SiloViewByIdThen);
    fn certificate_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::CertificateListWhen, operations::CertificateListThen);
    fn certificate_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::CertificateCreateWhen, operations::CertificateCreateThen);
    fn certificate_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::CertificateViewWhen, operations::CertificateViewThen);
    fn certificate_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::CertificateDeleteWhen, operations::CertificateDeleteThen);
    fn physical_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::PhysicalDiskListWhen, operations::PhysicalDiskListThen);
    fn rack_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::RackListWhen, operations::RackListThen);
    fn rack_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::RackViewWhen, operations::RackViewThen);
    fn sled_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SledListWhen, operations::SledListThen);
    fn sled_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SledViewWhen, operations::SledViewThen);
    fn sled_physical_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SledPhysicalDiskListWhen, operations::SledPhysicalDiskListThen);
    fn system_image_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageListWhen, operations::SystemImageListThen);
    fn system_image_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageCreateWhen, operations::SystemImageCreateThen);
    fn system_image_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageViewWhen, operations::SystemImageViewThen);
    fn system_image_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageDeleteWhen, operations::SystemImageDeleteThen);
    fn ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolListWhen, operations::IpPoolListThen);
    fn ip_pool_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolCreateWhen, operations::IpPoolCreateThen);
    fn ip_pool_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolViewWhen, operations::IpPoolViewThen);
    fn ip_pool_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolUpdateWhen, operations::IpPoolUpdateThen);
    fn ip_pool_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolDeleteWhen, operations::IpPoolDeleteThen);
    fn ip_pool_range_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolRangeListWhen, operations::IpPoolRangeListThen);
    fn ip_pool_range_add<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolRangeAddWhen, operations::IpPoolRangeAddThen);
    fn ip_pool_range_remove<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolRangeRemoveWhen, operations::IpPoolRangeRemoveThen);
    fn ip_pool_service_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolServiceViewWhen, operations::IpPoolServiceViewThen);
    fn ip_pool_service_range_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolServiceRangeListWhen, operations::IpPoolServiceRangeListThen);
    fn ip_pool_service_range_add<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolServiceRangeAddWhen, operations::IpPoolServiceRangeAddThen);
    fn ip_pool_service_range_remove<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::IpPoolServiceRangeRemoveWhen,
            operations::IpPoolServiceRangeRemoveThen,
        );
    fn system_metric<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemMetricWhen, operations::SystemMetricThen);
    fn system_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemPolicyViewWhen, operations::SystemPolicyViewThen);
    fn system_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemPolicyUpdateWhen, operations::SystemPolicyUpdateThen);
    fn saga_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SagaListWhen, operations::SagaListThen);
    fn saga_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SagaViewWhen, operations::SagaViewThen);
    fn silo_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloListWhen, operations::SiloListThen);
    fn silo_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloCreateWhen, operations::SiloCreateThen);
    fn silo_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloViewWhen, operations::SiloViewThen);
    fn silo_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloDeleteWhen, operations::SiloDeleteThen);
    fn silo_identity_provider_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SiloIdentityProviderListWhen,
            operations::SiloIdentityProviderListThen,
        );
    fn local_idp_user_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LocalIdpUserCreateWhen, operations::LocalIdpUserCreateThen);
    fn local_idp_user_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LocalIdpUserDeleteWhen, operations::LocalIdpUserDeleteThen);
    fn local_idp_user_set_password<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LocalIdpUserSetPasswordWhen, operations::LocalIdpUserSetPasswordThen);
    fn saml_identity_provider_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SamlIdentityProviderCreateWhen,
            operations::SamlIdentityProviderCreateThen,
        );
    fn saml_identity_provider_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SamlIdentityProviderViewWhen,
            operations::SamlIdentityProviderViewThen,
        );
    fn silo_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloPolicyViewWhen, operations::SiloPolicyViewThen);
    fn silo_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloPolicyUpdateWhen, operations::SiloPolicyUpdateThen);
    fn silo_users_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloUsersListWhen, operations::SiloUsersListThen);
    fn silo_user_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloUserViewWhen, operations::SiloUserViewThen);
    fn system_user_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUserListWhen, operations::SystemUserListThen);
    fn system_user_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUserViewWhen, operations::SystemUserViewThen);
    fn timeseries_schema_get<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::TimeseriesSchemaGetWhen, operations::TimeseriesSchemaGetThen);
    fn user_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UserListWhen, operations::UserListThen);
    fn disk_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskListV1When, operations::DiskListV1Then);
    fn disk_create_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskCreateV1When, operations::DiskCreateV1Then);
    fn disk_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskViewV1When, operations::DiskViewV1Then);
    fn disk_delete_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskDeleteV1When, operations::DiskDeleteV1Then);
    fn instance_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceListV1When, operations::InstanceListV1Then);
    fn instance_create_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceCreateV1When, operations::InstanceCreateV1Then);
    fn instance_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceViewV1When, operations::InstanceViewV1Then);
    fn instance_delete_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDeleteV1When, operations::InstanceDeleteV1Then);
    fn instance_disk_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskListV1When, operations::InstanceDiskListV1Then);
    fn instance_disk_attach_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskAttachV1When, operations::InstanceDiskAttachV1Then);
    fn instance_disk_detach_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskDetachV1When, operations::InstanceDiskDetachV1Then);
    fn instance_migrate_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceMigrateV1When, operations::InstanceMigrateV1Then);
    fn instance_reboot_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceRebootV1When, operations::InstanceRebootV1Then);
    fn instance_serial_console_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceSerialConsoleV1When, operations::InstanceSerialConsoleV1Then);
    fn instance_serial_console_stream_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceSerialConsoleStreamV1When,
            operations::InstanceSerialConsoleStreamV1Then,
        );
    fn instance_start_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStartV1When, operations::InstanceStartV1Then);
    fn instance_stop_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStopV1When, operations::InstanceStopV1Then);
    fn organization_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationListV1When, operations::OrganizationListV1Then);
    fn organization_create_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationCreateV1When, operations::OrganizationCreateV1Then);
    fn organization_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationViewV1When, operations::OrganizationViewV1Then);
    fn organization_update_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationUpdateV1When, operations::OrganizationUpdateV1Then);
    fn organization_delete_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationDeleteV1When, operations::OrganizationDeleteV1Then);
    fn organization_policy_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::OrganizationPolicyViewV1When,
            operations::OrganizationPolicyViewV1Then,
        );
    fn organization_policy_update_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::OrganizationPolicyUpdateV1When,
            operations::OrganizationPolicyUpdateV1Then,
        );
    fn project_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectListV1When, operations::ProjectListV1Then);
    fn project_create_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectCreateV1When, operations::ProjectCreateV1Then);
    fn project_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectViewV1When, operations::ProjectViewV1Then);
    fn project_update_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectUpdateV1When, operations::ProjectUpdateV1Then);
    fn project_delete_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectDeleteV1When, operations::ProjectDeleteV1Then);
    fn project_policy_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectPolicyViewV1When, operations::ProjectPolicyViewV1Then);
    fn project_policy_update_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectPolicyUpdateV1When, operations::ProjectPolicyUpdateV1Then);
    fn system_component_version_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SystemComponentVersionListWhen,
            operations::SystemComponentVersionListThen,
        );
    fn update_deployments_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UpdateDeploymentsListWhen, operations::UpdateDeploymentsListThen);
    fn update_deployment_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UpdateDeploymentViewWhen, operations::UpdateDeploymentViewThen);
    fn system_update_refresh<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateRefreshWhen, operations::SystemUpdateRefreshThen);
    fn system_update_start<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateStartWhen, operations::SystemUpdateStartThen);
    fn system_update_stop<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateStopWhen, operations::SystemUpdateStopThen);
    fn system_update_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateListWhen, operations::SystemUpdateListThen);
    fn system_update_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateViewWhen, operations::SystemUpdateViewThen);
    fn system_update_components_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SystemUpdateComponentsListWhen,
            operations::SystemUpdateComponentsListThen,
        );
    fn system_version<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemVersionWhen, operations::SystemVersionThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn disk_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskViewByIdWhen, operations::DiskViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskViewByIdWhen::new(when),
                operations::DiskViewByIdThen::new(then),
            )
        })
    }

    fn image_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageViewByIdWhen, operations::ImageViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageViewByIdWhen::new(when),
                operations::ImageViewByIdThen::new(then),
            )
        })
    }

    fn instance_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceViewByIdWhen, operations::InstanceViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceViewByIdWhen::new(when),
                operations::InstanceViewByIdThen::new(then),
            )
        })
    }

    fn instance_network_interface_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceViewByIdWhen,
            operations::InstanceNetworkInterfaceViewByIdThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceViewByIdWhen::new(when),
                operations::InstanceNetworkInterfaceViewByIdThen::new(then),
            )
        })
    }

    fn organization_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationViewByIdWhen, operations::OrganizationViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationViewByIdWhen::new(when),
                operations::OrganizationViewByIdThen::new(then),
            )
        })
    }

    fn project_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectViewByIdWhen, operations::ProjectViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectViewByIdWhen::new(when),
                operations::ProjectViewByIdThen::new(then),
            )
        })
    }

    fn snapshot_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotViewByIdWhen, operations::SnapshotViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SnapshotViewByIdWhen::new(when),
                operations::SnapshotViewByIdThen::new(then),
            )
        })
    }

    fn vpc_router_route_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteViewByIdWhen, operations::VpcRouterRouteViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteViewByIdWhen::new(when),
                operations::VpcRouterRouteViewByIdThen::new(then),
            )
        })
    }

    fn vpc_router_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterViewByIdWhen, operations::VpcRouterViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterViewByIdWhen::new(when),
                operations::VpcRouterViewByIdThen::new(then),
            )
        })
    }

    fn vpc_subnet_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetViewByIdWhen, operations::VpcSubnetViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetViewByIdWhen::new(when),
                operations::VpcSubnetViewByIdThen::new(then),
            )
        })
    }

    fn vpc_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcViewByIdWhen, operations::VpcViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcViewByIdWhen::new(when),
                operations::VpcViewByIdThen::new(then),
            )
        })
    }

    fn device_auth_request<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DeviceAuthRequestWhen, operations::DeviceAuthRequestThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DeviceAuthRequestWhen::new(when),
                operations::DeviceAuthRequestThen::new(then),
            )
        })
    }

    fn device_auth_confirm<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DeviceAuthConfirmWhen, operations::DeviceAuthConfirmThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DeviceAuthConfirmWhen::new(when),
                operations::DeviceAuthConfirmThen::new(then),
            )
        })
    }

    fn device_access_token<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DeviceAccessTokenWhen, operations::DeviceAccessTokenThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DeviceAccessTokenWhen::new(when),
                operations::DeviceAccessTokenThen::new(then),
            )
        })
    }

    fn group_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::GroupListWhen, operations::GroupListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::GroupListWhen::new(when),
                operations::GroupListThen::new(then),
            )
        })
    }

    fn login_spoof<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LoginSpoofWhen, operations::LoginSpoofThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LoginSpoofWhen::new(when),
                operations::LoginSpoofThen::new(then),
            )
        })
    }

    fn login_local<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LoginLocalWhen, operations::LoginLocalThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LoginLocalWhen::new(when),
                operations::LoginLocalThen::new(then),
            )
        })
    }

    fn login_saml_begin<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LoginSamlBeginWhen, operations::LoginSamlBeginThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LoginSamlBeginWhen::new(when),
                operations::LoginSamlBeginThen::new(then),
            )
        })
    }

    fn login_saml<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LoginSamlWhen, operations::LoginSamlThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LoginSamlWhen::new(when),
                operations::LoginSamlThen::new(then),
            )
        })
    }

    fn logout<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LogoutWhen, operations::LogoutThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LogoutWhen::new(when),
                operations::LogoutThen::new(then),
            )
        })
    }

    fn organization_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationListWhen, operations::OrganizationListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationListWhen::new(when),
                operations::OrganizationListThen::new(then),
            )
        })
    }

    fn organization_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationCreateWhen, operations::OrganizationCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationCreateWhen::new(when),
                operations::OrganizationCreateThen::new(then),
            )
        })
    }

    fn organization_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationViewWhen, operations::OrganizationViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationViewWhen::new(when),
                operations::OrganizationViewThen::new(then),
            )
        })
    }

    fn organization_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationUpdateWhen, operations::OrganizationUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationUpdateWhen::new(when),
                operations::OrganizationUpdateThen::new(then),
            )
        })
    }

    fn organization_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationDeleteWhen, operations::OrganizationDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationDeleteWhen::new(when),
                operations::OrganizationDeleteThen::new(then),
            )
        })
    }

    fn organization_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationPolicyViewWhen, operations::OrganizationPolicyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationPolicyViewWhen::new(when),
                operations::OrganizationPolicyViewThen::new(then),
            )
        })
    }

    fn organization_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::OrganizationPolicyUpdateWhen,
            operations::OrganizationPolicyUpdateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationPolicyUpdateWhen::new(when),
                operations::OrganizationPolicyUpdateThen::new(then),
            )
        })
    }

    fn project_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectListWhen, operations::ProjectListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectListWhen::new(when),
                operations::ProjectListThen::new(then),
            )
        })
    }

    fn project_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectCreateWhen, operations::ProjectCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectCreateWhen::new(when),
                operations::ProjectCreateThen::new(then),
            )
        })
    }

    fn project_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectViewWhen, operations::ProjectViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectViewWhen::new(when),
                operations::ProjectViewThen::new(then),
            )
        })
    }

    fn project_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectUpdateWhen, operations::ProjectUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectUpdateWhen::new(when),
                operations::ProjectUpdateThen::new(then),
            )
        })
    }

    fn project_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectDeleteWhen, operations::ProjectDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectDeleteWhen::new(when),
                operations::ProjectDeleteThen::new(then),
            )
        })
    }

    fn disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskListWhen, operations::DiskListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskListWhen::new(when),
                operations::DiskListThen::new(then),
            )
        })
    }

    fn disk_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskCreateWhen, operations::DiskCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskCreateWhen::new(when),
                operations::DiskCreateThen::new(then),
            )
        })
    }

    fn disk_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskViewWhen, operations::DiskViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskViewWhen::new(when),
                operations::DiskViewThen::new(then),
            )
        })
    }

    fn disk_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskDeleteWhen, operations::DiskDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskDeleteWhen::new(when),
                operations::DiskDeleteThen::new(then),
            )
        })
    }

    fn disk_metrics_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskMetricsListWhen, operations::DiskMetricsListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskMetricsListWhen::new(when),
                operations::DiskMetricsListThen::new(then),
            )
        })
    }

    fn image_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageListWhen, operations::ImageListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageListWhen::new(when),
                operations::ImageListThen::new(then),
            )
        })
    }

    fn image_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageCreateWhen, operations::ImageCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageCreateWhen::new(when),
                operations::ImageCreateThen::new(then),
            )
        })
    }

    fn image_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageViewWhen, operations::ImageViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageViewWhen::new(when),
                operations::ImageViewThen::new(then),
            )
        })
    }

    fn image_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ImageDeleteWhen, operations::ImageDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageDeleteWhen::new(when),
                operations::ImageDeleteThen::new(then),
            )
        })
    }

    fn instance_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceListWhen, operations::InstanceListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceListWhen::new(when),
                operations::InstanceListThen::new(then),
            )
        })
    }

    fn instance_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceCreateWhen, operations::InstanceCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceCreateWhen::new(when),
                operations::InstanceCreateThen::new(then),
            )
        })
    }

    fn instance_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceViewWhen, operations::InstanceViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceViewWhen::new(when),
                operations::InstanceViewThen::new(then),
            )
        })
    }

    fn instance_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDeleteWhen, operations::InstanceDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDeleteWhen::new(when),
                operations::InstanceDeleteThen::new(then),
            )
        })
    }

    fn instance_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskListWhen, operations::InstanceDiskListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDiskListWhen::new(when),
                operations::InstanceDiskListThen::new(then),
            )
        })
    }

    fn instance_disk_attach<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskAttachWhen, operations::InstanceDiskAttachThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDiskAttachWhen::new(when),
                operations::InstanceDiskAttachThen::new(then),
            )
        })
    }

    fn instance_disk_detach<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskDetachWhen, operations::InstanceDiskDetachThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDiskDetachWhen::new(when),
                operations::InstanceDiskDetachThen::new(then),
            )
        })
    }

    fn instance_external_ip_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceExternalIpListWhen, operations::InstanceExternalIpListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceExternalIpListWhen::new(when),
                operations::InstanceExternalIpListThen::new(then),
            )
        })
    }

    fn instance_migrate<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceMigrateWhen, operations::InstanceMigrateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceMigrateWhen::new(when),
                operations::InstanceMigrateThen::new(then),
            )
        })
    }

    fn instance_network_interface_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceListWhen,
            operations::InstanceNetworkInterfaceListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceListWhen::new(when),
                operations::InstanceNetworkInterfaceListThen::new(then),
            )
        })
    }

    fn instance_network_interface_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceCreateWhen,
            operations::InstanceNetworkInterfaceCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceCreateWhen::new(when),
                operations::InstanceNetworkInterfaceCreateThen::new(then),
            )
        })
    }

    fn instance_network_interface_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceViewWhen,
            operations::InstanceNetworkInterfaceViewThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceViewWhen::new(when),
                operations::InstanceNetworkInterfaceViewThen::new(then),
            )
        })
    }

    fn instance_network_interface_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceUpdateWhen,
            operations::InstanceNetworkInterfaceUpdateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceUpdateWhen::new(when),
                operations::InstanceNetworkInterfaceUpdateThen::new(then),
            )
        })
    }

    fn instance_network_interface_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceDeleteWhen,
            operations::InstanceNetworkInterfaceDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceDeleteWhen::new(when),
                operations::InstanceNetworkInterfaceDeleteThen::new(then),
            )
        })
    }

    fn instance_reboot<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceRebootWhen, operations::InstanceRebootThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceRebootWhen::new(when),
                operations::InstanceRebootThen::new(then),
            )
        })
    }

    fn instance_serial_console<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceSerialConsoleWhen, operations::InstanceSerialConsoleThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceSerialConsoleWhen::new(when),
                operations::InstanceSerialConsoleThen::new(then),
            )
        })
    }

    fn instance_serial_console_stream<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceSerialConsoleStreamWhen,
            operations::InstanceSerialConsoleStreamThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceSerialConsoleStreamWhen::new(when),
                operations::InstanceSerialConsoleStreamThen::new(then),
            )
        })
    }

    fn instance_start<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStartWhen, operations::InstanceStartThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceStartWhen::new(when),
                operations::InstanceStartThen::new(then),
            )
        })
    }

    fn instance_stop<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStopWhen, operations::InstanceStopThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceStopWhen::new(when),
                operations::InstanceStopThen::new(then),
            )
        })
    }

    fn project_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectPolicyViewWhen, operations::ProjectPolicyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectPolicyViewWhen::new(when),
                operations::ProjectPolicyViewThen::new(then),
            )
        })
    }

    fn project_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectPolicyUpdateWhen, operations::ProjectPolicyUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectPolicyUpdateWhen::new(when),
                operations::ProjectPolicyUpdateThen::new(then),
            )
        })
    }

    fn snapshot_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotListWhen, operations::SnapshotListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SnapshotListWhen::new(when),
                operations::SnapshotListThen::new(then),
            )
        })
    }

    fn snapshot_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotCreateWhen, operations::SnapshotCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SnapshotCreateWhen::new(when),
                operations::SnapshotCreateThen::new(then),
            )
        })
    }

    fn snapshot_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotViewWhen, operations::SnapshotViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SnapshotViewWhen::new(when),
                operations::SnapshotViewThen::new(then),
            )
        })
    }

    fn snapshot_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SnapshotDeleteWhen, operations::SnapshotDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SnapshotDeleteWhen::new(when),
                operations::SnapshotDeleteThen::new(then),
            )
        })
    }

    fn vpc_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcListWhen, operations::VpcListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcListWhen::new(when),
                operations::VpcListThen::new(then),
            )
        })
    }

    fn vpc_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcCreateWhen, operations::VpcCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcCreateWhen::new(when),
                operations::VpcCreateThen::new(then),
            )
        })
    }

    fn vpc_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcViewWhen, operations::VpcViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcViewWhen::new(when),
                operations::VpcViewThen::new(then),
            )
        })
    }

    fn vpc_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcUpdateWhen, operations::VpcUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcUpdateWhen::new(when),
                operations::VpcUpdateThen::new(then),
            )
        })
    }

    fn vpc_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcDeleteWhen, operations::VpcDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcDeleteWhen::new(when),
                operations::VpcDeleteThen::new(then),
            )
        })
    }

    fn vpc_firewall_rules_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcFirewallRulesViewWhen, operations::VpcFirewallRulesViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcFirewallRulesViewWhen::new(when),
                operations::VpcFirewallRulesViewThen::new(then),
            )
        })
    }

    fn vpc_firewall_rules_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcFirewallRulesUpdateWhen, operations::VpcFirewallRulesUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcFirewallRulesUpdateWhen::new(when),
                operations::VpcFirewallRulesUpdateThen::new(then),
            )
        })
    }

    fn vpc_router_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterListWhen, operations::VpcRouterListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterListWhen::new(when),
                operations::VpcRouterListThen::new(then),
            )
        })
    }

    fn vpc_router_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterCreateWhen, operations::VpcRouterCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterCreateWhen::new(when),
                operations::VpcRouterCreateThen::new(then),
            )
        })
    }

    fn vpc_router_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterViewWhen, operations::VpcRouterViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterViewWhen::new(when),
                operations::VpcRouterViewThen::new(then),
            )
        })
    }

    fn vpc_router_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterUpdateWhen, operations::VpcRouterUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterUpdateWhen::new(when),
                operations::VpcRouterUpdateThen::new(then),
            )
        })
    }

    fn vpc_router_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterDeleteWhen, operations::VpcRouterDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterDeleteWhen::new(when),
                operations::VpcRouterDeleteThen::new(then),
            )
        })
    }

    fn vpc_router_route_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteListWhen, operations::VpcRouterRouteListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteListWhen::new(when),
                operations::VpcRouterRouteListThen::new(then),
            )
        })
    }

    fn vpc_router_route_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteCreateWhen, operations::VpcRouterRouteCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteCreateWhen::new(when),
                operations::VpcRouterRouteCreateThen::new(then),
            )
        })
    }

    fn vpc_router_route_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteViewWhen, operations::VpcRouterRouteViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteViewWhen::new(when),
                operations::VpcRouterRouteViewThen::new(then),
            )
        })
    }

    fn vpc_router_route_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteUpdateWhen, operations::VpcRouterRouteUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteUpdateWhen::new(when),
                operations::VpcRouterRouteUpdateThen::new(then),
            )
        })
    }

    fn vpc_router_route_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcRouterRouteDeleteWhen, operations::VpcRouterRouteDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteDeleteWhen::new(when),
                operations::VpcRouterRouteDeleteThen::new(then),
            )
        })
    }

    fn vpc_subnet_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetListWhen, operations::VpcSubnetListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetListWhen::new(when),
                operations::VpcSubnetListThen::new(then),
            )
        })
    }

    fn vpc_subnet_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetCreateWhen, operations::VpcSubnetCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetCreateWhen::new(when),
                operations::VpcSubnetCreateThen::new(then),
            )
        })
    }

    fn vpc_subnet_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetViewWhen, operations::VpcSubnetViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetViewWhen::new(when),
                operations::VpcSubnetViewThen::new(then),
            )
        })
    }

    fn vpc_subnet_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetUpdateWhen, operations::VpcSubnetUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetUpdateWhen::new(when),
                operations::VpcSubnetUpdateThen::new(then),
            )
        })
    }

    fn vpc_subnet_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::VpcSubnetDeleteWhen, operations::VpcSubnetDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetDeleteWhen::new(when),
                operations::VpcSubnetDeleteThen::new(then),
            )
        })
    }

    fn vpc_subnet_list_network_interfaces<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::VpcSubnetListNetworkInterfacesWhen,
            operations::VpcSubnetListNetworkInterfacesThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetListNetworkInterfacesWhen::new(when),
                operations::VpcSubnetListNetworkInterfacesThen::new(then),
            )
        })
    }

    fn policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::PolicyViewWhen, operations::PolicyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PolicyViewWhen::new(when),
                operations::PolicyViewThen::new(then),
            )
        })
    }

    fn policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::PolicyUpdateWhen, operations::PolicyUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PolicyUpdateWhen::new(when),
                operations::PolicyUpdateThen::new(then),
            )
        })
    }

    fn role_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::RoleListWhen, operations::RoleListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::RoleListWhen::new(when),
                operations::RoleListThen::new(then),
            )
        })
    }

    fn role_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::RoleViewWhen, operations::RoleViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::RoleViewWhen::new(when),
                operations::RoleViewThen::new(then),
            )
        })
    }

    fn session_me<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionMeWhen, operations::SessionMeThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SessionMeWhen::new(when),
                operations::SessionMeThen::new(then),
            )
        })
    }

    fn session_me_groups<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionMeGroupsWhen, operations::SessionMeGroupsThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SessionMeGroupsWhen::new(when),
                operations::SessionMeGroupsThen::new(then),
            )
        })
    }

    fn session_sshkey_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionSshkeyListWhen, operations::SessionSshkeyListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SessionSshkeyListWhen::new(when),
                operations::SessionSshkeyListThen::new(then),
            )
        })
    }

    fn session_sshkey_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionSshkeyCreateWhen, operations::SessionSshkeyCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SessionSshkeyCreateWhen::new(when),
                operations::SessionSshkeyCreateThen::new(then),
            )
        })
    }

    fn session_sshkey_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionSshkeyViewWhen, operations::SessionSshkeyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SessionSshkeyViewWhen::new(when),
                operations::SessionSshkeyViewThen::new(then),
            )
        })
    }

    fn session_sshkey_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SessionSshkeyDeleteWhen, operations::SessionSshkeyDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SessionSshkeyDeleteWhen::new(when),
                operations::SessionSshkeyDeleteThen::new(then),
            )
        })
    }

    fn system_image_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageViewByIdWhen, operations::SystemImageViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemImageViewByIdWhen::new(when),
                operations::SystemImageViewByIdThen::new(then),
            )
        })
    }

    fn ip_pool_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolViewByIdWhen, operations::IpPoolViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolViewByIdWhen::new(when),
                operations::IpPoolViewByIdThen::new(then),
            )
        })
    }

    fn silo_view_by_id<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloViewByIdWhen, operations::SiloViewByIdThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloViewByIdWhen::new(when),
                operations::SiloViewByIdThen::new(then),
            )
        })
    }

    fn certificate_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::CertificateListWhen, operations::CertificateListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CertificateListWhen::new(when),
                operations::CertificateListThen::new(then),
            )
        })
    }

    fn certificate_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::CertificateCreateWhen, operations::CertificateCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CertificateCreateWhen::new(when),
                operations::CertificateCreateThen::new(then),
            )
        })
    }

    fn certificate_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::CertificateViewWhen, operations::CertificateViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CertificateViewWhen::new(when),
                operations::CertificateViewThen::new(then),
            )
        })
    }

    fn certificate_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::CertificateDeleteWhen, operations::CertificateDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CertificateDeleteWhen::new(when),
                operations::CertificateDeleteThen::new(then),
            )
        })
    }

    fn physical_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::PhysicalDiskListWhen, operations::PhysicalDiskListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PhysicalDiskListWhen::new(when),
                operations::PhysicalDiskListThen::new(then),
            )
        })
    }

    fn rack_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::RackListWhen, operations::RackListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::RackListWhen::new(when),
                operations::RackListThen::new(then),
            )
        })
    }

    fn rack_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::RackViewWhen, operations::RackViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::RackViewWhen::new(when),
                operations::RackViewThen::new(then),
            )
        })
    }

    fn sled_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SledListWhen, operations::SledListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledListWhen::new(when),
                operations::SledListThen::new(then),
            )
        })
    }

    fn sled_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SledViewWhen, operations::SledViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledViewWhen::new(when),
                operations::SledViewThen::new(then),
            )
        })
    }

    fn sled_physical_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SledPhysicalDiskListWhen, operations::SledPhysicalDiskListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledPhysicalDiskListWhen::new(when),
                operations::SledPhysicalDiskListThen::new(then),
            )
        })
    }

    fn system_image_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageListWhen, operations::SystemImageListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemImageListWhen::new(when),
                operations::SystemImageListThen::new(then),
            )
        })
    }

    fn system_image_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageCreateWhen, operations::SystemImageCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemImageCreateWhen::new(when),
                operations::SystemImageCreateThen::new(then),
            )
        })
    }

    fn system_image_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageViewWhen, operations::SystemImageViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemImageViewWhen::new(when),
                operations::SystemImageViewThen::new(then),
            )
        })
    }

    fn system_image_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemImageDeleteWhen, operations::SystemImageDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemImageDeleteWhen::new(when),
                operations::SystemImageDeleteThen::new(then),
            )
        })
    }

    fn ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolListWhen, operations::IpPoolListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolListWhen::new(when),
                operations::IpPoolListThen::new(then),
            )
        })
    }

    fn ip_pool_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolCreateWhen, operations::IpPoolCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolCreateWhen::new(when),
                operations::IpPoolCreateThen::new(then),
            )
        })
    }

    fn ip_pool_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolViewWhen, operations::IpPoolViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolViewWhen::new(when),
                operations::IpPoolViewThen::new(then),
            )
        })
    }

    fn ip_pool_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolUpdateWhen, operations::IpPoolUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolUpdateWhen::new(when),
                operations::IpPoolUpdateThen::new(then),
            )
        })
    }

    fn ip_pool_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolDeleteWhen, operations::IpPoolDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolDeleteWhen::new(when),
                operations::IpPoolDeleteThen::new(then),
            )
        })
    }

    fn ip_pool_range_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolRangeListWhen, operations::IpPoolRangeListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolRangeListWhen::new(when),
                operations::IpPoolRangeListThen::new(then),
            )
        })
    }

    fn ip_pool_range_add<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolRangeAddWhen, operations::IpPoolRangeAddThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolRangeAddWhen::new(when),
                operations::IpPoolRangeAddThen::new(then),
            )
        })
    }

    fn ip_pool_range_remove<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolRangeRemoveWhen, operations::IpPoolRangeRemoveThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolRangeRemoveWhen::new(when),
                operations::IpPoolRangeRemoveThen::new(then),
            )
        })
    }

    fn ip_pool_service_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolServiceViewWhen, operations::IpPoolServiceViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolServiceViewWhen::new(when),
                operations::IpPoolServiceViewThen::new(then),
            )
        })
    }

    fn ip_pool_service_range_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolServiceRangeListWhen, operations::IpPoolServiceRangeListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolServiceRangeListWhen::new(when),
                operations::IpPoolServiceRangeListThen::new(then),
            )
        })
    }

    fn ip_pool_service_range_add<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::IpPoolServiceRangeAddWhen, operations::IpPoolServiceRangeAddThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolServiceRangeAddWhen::new(when),
                operations::IpPoolServiceRangeAddThen::new(then),
            )
        })
    }

    fn ip_pool_service_range_remove<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::IpPoolServiceRangeRemoveWhen,
            operations::IpPoolServiceRangeRemoveThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolServiceRangeRemoveWhen::new(when),
                operations::IpPoolServiceRangeRemoveThen::new(then),
            )
        })
    }

    fn system_metric<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemMetricWhen, operations::SystemMetricThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemMetricWhen::new(when),
                operations::SystemMetricThen::new(then),
            )
        })
    }

    fn system_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemPolicyViewWhen, operations::SystemPolicyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemPolicyViewWhen::new(when),
                operations::SystemPolicyViewThen::new(then),
            )
        })
    }

    fn system_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemPolicyUpdateWhen, operations::SystemPolicyUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemPolicyUpdateWhen::new(when),
                operations::SystemPolicyUpdateThen::new(then),
            )
        })
    }

    fn saga_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SagaListWhen, operations::SagaListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SagaListWhen::new(when),
                operations::SagaListThen::new(then),
            )
        })
    }

    fn saga_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SagaViewWhen, operations::SagaViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SagaViewWhen::new(when),
                operations::SagaViewThen::new(then),
            )
        })
    }

    fn silo_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloListWhen, operations::SiloListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloListWhen::new(when),
                operations::SiloListThen::new(then),
            )
        })
    }

    fn silo_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloCreateWhen, operations::SiloCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloCreateWhen::new(when),
                operations::SiloCreateThen::new(then),
            )
        })
    }

    fn silo_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloViewWhen, operations::SiloViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloViewWhen::new(when),
                operations::SiloViewThen::new(then),
            )
        })
    }

    fn silo_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloDeleteWhen, operations::SiloDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloDeleteWhen::new(when),
                operations::SiloDeleteThen::new(then),
            )
        })
    }

    fn silo_identity_provider_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SiloIdentityProviderListWhen,
            operations::SiloIdentityProviderListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloIdentityProviderListWhen::new(when),
                operations::SiloIdentityProviderListThen::new(then),
            )
        })
    }

    fn local_idp_user_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LocalIdpUserCreateWhen, operations::LocalIdpUserCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LocalIdpUserCreateWhen::new(when),
                operations::LocalIdpUserCreateThen::new(then),
            )
        })
    }

    fn local_idp_user_delete<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LocalIdpUserDeleteWhen, operations::LocalIdpUserDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LocalIdpUserDeleteWhen::new(when),
                operations::LocalIdpUserDeleteThen::new(then),
            )
        })
    }

    fn local_idp_user_set_password<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::LocalIdpUserSetPasswordWhen, operations::LocalIdpUserSetPasswordThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LocalIdpUserSetPasswordWhen::new(when),
                operations::LocalIdpUserSetPasswordThen::new(then),
            )
        })
    }

    fn saml_identity_provider_create<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SamlIdentityProviderCreateWhen,
            operations::SamlIdentityProviderCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SamlIdentityProviderCreateWhen::new(when),
                operations::SamlIdentityProviderCreateThen::new(then),
            )
        })
    }

    fn saml_identity_provider_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SamlIdentityProviderViewWhen,
            operations::SamlIdentityProviderViewThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SamlIdentityProviderViewWhen::new(when),
                operations::SamlIdentityProviderViewThen::new(then),
            )
        })
    }

    fn silo_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloPolicyViewWhen, operations::SiloPolicyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloPolicyViewWhen::new(when),
                operations::SiloPolicyViewThen::new(then),
            )
        })
    }

    fn silo_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloPolicyUpdateWhen, operations::SiloPolicyUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloPolicyUpdateWhen::new(when),
                operations::SiloPolicyUpdateThen::new(then),
            )
        })
    }

    fn silo_users_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloUsersListWhen, operations::SiloUsersListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloUsersListWhen::new(when),
                operations::SiloUsersListThen::new(then),
            )
        })
    }

    fn silo_user_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SiloUserViewWhen, operations::SiloUserViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloUserViewWhen::new(when),
                operations::SiloUserViewThen::new(then),
            )
        })
    }

    fn system_user_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUserListWhen, operations::SystemUserListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUserListWhen::new(when),
                operations::SystemUserListThen::new(then),
            )
        })
    }

    fn system_user_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUserViewWhen, operations::SystemUserViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUserViewWhen::new(when),
                operations::SystemUserViewThen::new(then),
            )
        })
    }

    fn timeseries_schema_get<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::TimeseriesSchemaGetWhen, operations::TimeseriesSchemaGetThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TimeseriesSchemaGetWhen::new(when),
                operations::TimeseriesSchemaGetThen::new(then),
            )
        })
    }

    fn user_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UserListWhen, operations::UserListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserListWhen::new(when),
                operations::UserListThen::new(then),
            )
        })
    }

    fn disk_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskListV1When, operations::DiskListV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskListV1When::new(when),
                operations::DiskListV1Then::new(then),
            )
        })
    }

    fn disk_create_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskCreateV1When, operations::DiskCreateV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskCreateV1When::new(when),
                operations::DiskCreateV1Then::new(then),
            )
        })
    }

    fn disk_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskViewV1When, operations::DiskViewV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskViewV1When::new(when),
                operations::DiskViewV1Then::new(then),
            )
        })
    }

    fn disk_delete_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DiskDeleteV1When, operations::DiskDeleteV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskDeleteV1When::new(when),
                operations::DiskDeleteV1Then::new(then),
            )
        })
    }

    fn instance_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceListV1When, operations::InstanceListV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceListV1When::new(when),
                operations::InstanceListV1Then::new(then),
            )
        })
    }

    fn instance_create_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceCreateV1When, operations::InstanceCreateV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceCreateV1When::new(when),
                operations::InstanceCreateV1Then::new(then),
            )
        })
    }

    fn instance_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceViewV1When, operations::InstanceViewV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceViewV1When::new(when),
                operations::InstanceViewV1Then::new(then),
            )
        })
    }

    fn instance_delete_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDeleteV1When, operations::InstanceDeleteV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDeleteV1When::new(when),
                operations::InstanceDeleteV1Then::new(then),
            )
        })
    }

    fn instance_disk_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskListV1When, operations::InstanceDiskListV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDiskListV1When::new(when),
                operations::InstanceDiskListV1Then::new(then),
            )
        })
    }

    fn instance_disk_attach_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskAttachV1When, operations::InstanceDiskAttachV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDiskAttachV1When::new(when),
                operations::InstanceDiskAttachV1Then::new(then),
            )
        })
    }

    fn instance_disk_detach_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceDiskDetachV1When, operations::InstanceDiskDetachV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDiskDetachV1When::new(when),
                operations::InstanceDiskDetachV1Then::new(then),
            )
        })
    }

    fn instance_migrate_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceMigrateV1When, operations::InstanceMigrateV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceMigrateV1When::new(when),
                operations::InstanceMigrateV1Then::new(then),
            )
        })
    }

    fn instance_reboot_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceRebootV1When, operations::InstanceRebootV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceRebootV1When::new(when),
                operations::InstanceRebootV1Then::new(then),
            )
        })
    }

    fn instance_serial_console_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceSerialConsoleV1When, operations::InstanceSerialConsoleV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceSerialConsoleV1When::new(when),
                operations::InstanceSerialConsoleV1Then::new(then),
            )
        })
    }

    fn instance_serial_console_stream_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceSerialConsoleStreamV1When,
            operations::InstanceSerialConsoleStreamV1Then,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceSerialConsoleStreamV1When::new(when),
                operations::InstanceSerialConsoleStreamV1Then::new(then),
            )
        })
    }

    fn instance_start_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStartV1When, operations::InstanceStartV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceStartV1When::new(when),
                operations::InstanceStartV1Then::new(then),
            )
        })
    }

    fn instance_stop_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStopV1When, operations::InstanceStopV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceStopV1When::new(when),
                operations::InstanceStopV1Then::new(then),
            )
        })
    }

    fn organization_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationListV1When, operations::OrganizationListV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationListV1When::new(when),
                operations::OrganizationListV1Then::new(then),
            )
        })
    }

    fn organization_create_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationCreateV1When, operations::OrganizationCreateV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationCreateV1When::new(when),
                operations::OrganizationCreateV1Then::new(then),
            )
        })
    }

    fn organization_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationViewV1When, operations::OrganizationViewV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationViewV1When::new(when),
                operations::OrganizationViewV1Then::new(then),
            )
        })
    }

    fn organization_update_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationUpdateV1When, operations::OrganizationUpdateV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationUpdateV1When::new(when),
                operations::OrganizationUpdateV1Then::new(then),
            )
        })
    }

    fn organization_delete_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::OrganizationDeleteV1When, operations::OrganizationDeleteV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationDeleteV1When::new(when),
                operations::OrganizationDeleteV1Then::new(then),
            )
        })
    }

    fn organization_policy_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::OrganizationPolicyViewV1When,
            operations::OrganizationPolicyViewV1Then,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationPolicyViewV1When::new(when),
                operations::OrganizationPolicyViewV1Then::new(then),
            )
        })
    }

    fn organization_policy_update_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::OrganizationPolicyUpdateV1When,
            operations::OrganizationPolicyUpdateV1Then,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::OrganizationPolicyUpdateV1When::new(when),
                operations::OrganizationPolicyUpdateV1Then::new(then),
            )
        })
    }

    fn project_list_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectListV1When, operations::ProjectListV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectListV1When::new(when),
                operations::ProjectListV1Then::new(then),
            )
        })
    }

    fn project_create_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectCreateV1When, operations::ProjectCreateV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectCreateV1When::new(when),
                operations::ProjectCreateV1Then::new(then),
            )
        })
    }

    fn project_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectViewV1When, operations::ProjectViewV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectViewV1When::new(when),
                operations::ProjectViewV1Then::new(then),
            )
        })
    }

    fn project_update_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectUpdateV1When, operations::ProjectUpdateV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectUpdateV1When::new(when),
                operations::ProjectUpdateV1Then::new(then),
            )
        })
    }

    fn project_delete_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectDeleteV1When, operations::ProjectDeleteV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectDeleteV1When::new(when),
                operations::ProjectDeleteV1Then::new(then),
            )
        })
    }

    fn project_policy_view_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectPolicyViewV1When, operations::ProjectPolicyViewV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectPolicyViewV1When::new(when),
                operations::ProjectPolicyViewV1Then::new(then),
            )
        })
    }

    fn project_policy_update_v1<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ProjectPolicyUpdateV1When, operations::ProjectPolicyUpdateV1Then),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectPolicyUpdateV1When::new(when),
                operations::ProjectPolicyUpdateV1Then::new(then),
            )
        })
    }

    fn system_component_version_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SystemComponentVersionListWhen,
            operations::SystemComponentVersionListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemComponentVersionListWhen::new(when),
                operations::SystemComponentVersionListThen::new(then),
            )
        })
    }

    fn update_deployments_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UpdateDeploymentsListWhen, operations::UpdateDeploymentsListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UpdateDeploymentsListWhen::new(when),
                operations::UpdateDeploymentsListThen::new(then),
            )
        })
    }

    fn update_deployment_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UpdateDeploymentViewWhen, operations::UpdateDeploymentViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UpdateDeploymentViewWhen::new(when),
                operations::UpdateDeploymentViewThen::new(then),
            )
        })
    }

    fn system_update_refresh<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateRefreshWhen, operations::SystemUpdateRefreshThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateRefreshWhen::new(when),
                operations::SystemUpdateRefreshThen::new(then),
            )
        })
    }

    fn system_update_start<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateStartWhen, operations::SystemUpdateStartThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateStartWhen::new(when),
                operations::SystemUpdateStartThen::new(then),
            )
        })
    }

    fn system_update_stop<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateStopWhen, operations::SystemUpdateStopThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateStopWhen::new(when),
                operations::SystemUpdateStopThen::new(then),
            )
        })
    }

    fn system_update_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateListWhen, operations::SystemUpdateListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateListWhen::new(when),
                operations::SystemUpdateListThen::new(then),
            )
        })
    }

    fn system_update_view<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemUpdateViewWhen, operations::SystemUpdateViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateViewWhen::new(when),
                operations::SystemUpdateViewThen::new(then),
            )
        })
    }

    fn system_update_components_list<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::SystemUpdateComponentsListWhen,
            operations::SystemUpdateComponentsListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateComponentsListWhen::new(when),
                operations::SystemUpdateComponentsListThen::new(then),
            )
        })
    }

    fn system_version<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SystemVersionWhen, operations::SystemVersionThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemVersionWhen::new(when),
                operations::SystemVersionThen::new(then),
            )
        })
    }
}
