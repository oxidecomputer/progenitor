pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::propolis_server_builder::*;
    #[doc = r" Apply decoded query parameter pairs to the matcher."]
    fn apply_query_param_pairs(
        mut when: ::httpmock::When,
        pairs: &[(String, String)],
    ) -> ::httpmock::When {
        for (key, value) in pairs {
            when = when.query_param(key, value);
        }

        when
    }

    pub struct InstanceGetWhen(::httpmock::When);
    impl InstanceGetWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/instance$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct InstanceGetThen(::httpmock::Then);
    impl InstanceGetThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceGetResponse) -> Self {
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

    pub struct InstanceEnsureWhen(::httpmock::When);
    impl InstanceEnsureWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/instance$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::InstanceEnsureRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceEnsureThen(::httpmock::Then);
    impl InstanceEnsureThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::InstanceEnsureResponse) -> Self {
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

    pub struct InstanceIssueCrucibleSnapshotRequestWhen(::httpmock::When);
    impl InstanceIssueCrucibleSnapshotRequestWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/instance/disk/[^/]*/snapshot/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/instance/disk/{}/snapshot/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn snapshot_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/instance/disk/.*/snapshot/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct InstanceIssueCrucibleSnapshotRequestThen(::httpmock::Then);
    impl InstanceIssueCrucibleSnapshotRequestThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: ()) -> Self {
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

    pub struct InstanceMigrateStatusWhen(::httpmock::When);
    impl InstanceMigrateStatusWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/instance/migrate/status$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::InstanceMigrateStatusRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceMigrateStatusThen(::httpmock::Then);
    impl InstanceMigrateStatusThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceMigrateStatusResponse) -> Self {
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

    pub struct InstanceSerialWhen(::httpmock::When);
    impl InstanceSerialWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/instance/serial$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct InstanceSerialThen(::httpmock::Then);
    impl InstanceSerialThen {
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

    pub struct InstanceStatePutWhen(::httpmock::When);
    impl InstanceStatePutWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/instance/state$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: types::InstanceStateRequested) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceStatePutThen(::httpmock::Then);
    impl InstanceStatePutThen {
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

    pub struct InstanceStateMonitorWhen(::httpmock::When);
    impl InstanceStateMonitorWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/instance/state-monitor$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::InstanceStateMonitorRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceStateMonitorThen(::httpmock::Then);
    impl InstanceStateMonitorThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceStateMonitorResponse) -> Self {
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
    fn instance_get<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceGetWhen, operations::InstanceGetThen);
    fn instance_ensure<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceEnsureWhen, operations::InstanceEnsureThen);
    fn instance_issue_crucible_snapshot_request<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceIssueCrucibleSnapshotRequestWhen,
            operations::InstanceIssueCrucibleSnapshotRequestThen,
        );
    fn instance_migrate_status<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceMigrateStatusWhen, operations::InstanceMigrateStatusThen);
    fn instance_serial<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceSerialWhen, operations::InstanceSerialThen);
    fn instance_state_put<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStatePutWhen, operations::InstanceStatePutThen);
    fn instance_state_monitor<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStateMonitorWhen, operations::InstanceStateMonitorThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn instance_get<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceGetWhen, operations::InstanceGetThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceGetWhen::new(when),
                operations::InstanceGetThen::new(then),
            )
        })
    }

    fn instance_ensure<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceEnsureWhen, operations::InstanceEnsureThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceEnsureWhen::new(when),
                operations::InstanceEnsureThen::new(then),
            )
        })
    }

    fn instance_issue_crucible_snapshot_request<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::InstanceIssueCrucibleSnapshotRequestWhen,
            operations::InstanceIssueCrucibleSnapshotRequestThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceIssueCrucibleSnapshotRequestWhen::new(when),
                operations::InstanceIssueCrucibleSnapshotRequestThen::new(then),
            )
        })
    }

    fn instance_migrate_status<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceMigrateStatusWhen, operations::InstanceMigrateStatusThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceMigrateStatusWhen::new(when),
                operations::InstanceMigrateStatusThen::new(then),
            )
        })
    }

    fn instance_serial<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceSerialWhen, operations::InstanceSerialThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceSerialWhen::new(when),
                operations::InstanceSerialThen::new(then),
            )
        })
    }

    fn instance_state_put<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStatePutWhen, operations::InstanceStatePutThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceStatePutWhen::new(when),
                operations::InstanceStatePutThen::new(then),
            )
        })
    }

    fn instance_state_monitor<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::InstanceStateMonitorWhen, operations::InstanceStateMonitorThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceStateMonitorWhen::new(when),
                operations::InstanceStateMonitorThen::new(then),
            )
        })
    }
}
