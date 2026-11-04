pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::multiple_responses_builder::*;
    pub struct GetThingWhen(::httpmock::When);
    impl GetThingWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/things/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!("^/things/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct GetThingThen(::httpmock::Then);
    impl GetThingThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Thing) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn accepted(
            self,
            value: &::serde_json::Map<::std::string::String, ::serde_json::Value>,
        ) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn bad_request(self, value: &types::BadRequest) -> Self {
            Self(
                self.0
                    .status(400u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn not_found(self) -> Self {
            Self(self.0.status(404u16))
        }

        pub fn internal_server_error(self, value: &types::InternalError) -> Self {
            Self(
                self.0
                    .status(500u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn service_unavailable(self, value: &types::InternalError) -> Self {
            Self(
                self.0
                    .status(503u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct PutThingWhen(::httpmock::When);
    impl PutThingWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/things/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!("^/things/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::Thing) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct PutThingThen(::httpmock::Then);
    impl PutThingThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: &types::InternalError) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn created(self, value: &types::Thing) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }
    }

    pub struct GetThingContentsWhen(::httpmock::When);
    impl GetThingContentsWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/things/[^/]*/contents$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &str) -> Self {
            let re =
                regex::Regex::new(&format!("^/things/{}/contents$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct GetThingContentsThen(::httpmock::Then);
    impl GetThingContentsThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }

        pub fn accepted(self, value: &types::Progress) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn not_found(self) -> Self {
            Self(self.0.status(404u16))
        }
    }
}

#[doc = r" An extension trait for [`MockServer`](::httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](::httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn get_thing<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::GetThingWhen, operations::GetThingThen);
    fn put_thing<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::PutThingWhen, operations::PutThingThen);
    fn get_thing_contents<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::GetThingContentsWhen, operations::GetThingContentsThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn get_thing<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::GetThingWhen, operations::GetThingThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::GetThingWhen::new(when),
                operations::GetThingThen::new(then),
            )
        })
    }

    fn put_thing<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::PutThingWhen, operations::PutThingThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PutThingWhen::new(when),
                operations::PutThingThen::new(then),
            )
        })
    }

    fn get_thing_contents<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::GetThingContentsWhen, operations::GetThingContentsThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::GetThingContentsWhen::new(when),
                operations::GetThingContentsThen::new(then),
            )
        })
    }
}
