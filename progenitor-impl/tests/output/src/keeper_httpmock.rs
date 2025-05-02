pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::keeper_builder::*;
    pub struct EnrolWhen(::httpmock::When);
    impl EnrolWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/enrol$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn authorization(self, value: &str) -> Self {
            todo!()
        }

        pub fn body(self, value: &types::EnrolBody) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct EnrolThen(::httpmock::Then);
    impl EnrolThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self) -> Self {
            Self(self.0.status(201u16))
        }
    }

    pub struct GlobalJobsWhen(::httpmock::When);
    impl GlobalJobsWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/global/jobs$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn authorization(self, value: &str) -> Self {
            todo!()
        }
    }

    pub struct GlobalJobsThen(::httpmock::Then);
    impl GlobalJobsThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::GlobalJobsResult) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct PingWhen(::httpmock::When);
    impl PingWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/ping$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn authorization(self, value: &str) -> Self {
            todo!()
        }
    }

    pub struct PingThen(::httpmock::Then);
    impl PingThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::PingResult) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ReportFinishWhen(::httpmock::When);
    impl ReportFinishWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/report/finish$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn authorization(self, value: &str) -> Self {
            todo!()
        }

        pub fn body(self, value: &types::ReportFinishBody) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ReportFinishThen(::httpmock::Then);
    impl ReportFinishThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::ReportResult) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ReportOutputWhen(::httpmock::When);
    impl ReportOutputWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/report/output$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn authorization(self, value: &str) -> Self {
            todo!()
        }

        pub fn body(self, value: &types::ReportOutputBody) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ReportOutputThen(::httpmock::Then);
    impl ReportOutputThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::ReportResult) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ReportStartWhen(::httpmock::When);
    impl ReportStartWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/report/start$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn authorization(self, value: &str) -> Self {
            todo!()
        }

        pub fn body(self, value: &types::ReportStartBody) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ReportStartThen(::httpmock::Then);
    impl ReportStartThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::ReportResult) -> Self {
            Self(
                self.0
                    .status(201u16)
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
    fn enrol<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::EnrolWhen, operations::EnrolThen);
    fn global_jobs<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::GlobalJobsWhen, operations::GlobalJobsThen);
    fn ping<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PingWhen, operations::PingThen);
    fn report_finish<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ReportFinishWhen, operations::ReportFinishThen);
    fn report_output<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ReportOutputWhen, operations::ReportOutputThen);
    fn report_start<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ReportStartWhen, operations::ReportStartThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn enrol<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::EnrolWhen, operations::EnrolThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::EnrolWhen::new(when),
                operations::EnrolThen::new(then),
            )
        })
    }

    fn global_jobs<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::GlobalJobsWhen, operations::GlobalJobsThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::GlobalJobsWhen::new(when),
                operations::GlobalJobsThen::new(then),
            )
        })
    }

    fn ping<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PingWhen, operations::PingThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PingWhen::new(when),
                operations::PingThen::new(then),
            )
        })
    }

    fn report_finish<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ReportFinishWhen, operations::ReportFinishThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ReportFinishWhen::new(when),
                operations::ReportFinishThen::new(then),
            )
        })
    }

    fn report_output<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ReportOutputWhen, operations::ReportOutputThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ReportOutputWhen::new(when),
                operations::ReportOutputThen::new(then),
            )
        })
    }

    fn report_start<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ReportStartWhen, operations::ReportStartThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ReportStartWhen::new(when),
                operations::ReportStartThen::new(then),
            )
        })
    }
}
