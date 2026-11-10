pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::json_suffix_builder::*;
    pub struct AnnotateThingWhen(::httpmock::When);
    impl AnnotateThingWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/things/[^/]*/annotate$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn id(self, value: &str) -> Self {
            let re =
                regex::Regex::new(&format!("^/things/{}/annotate$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::Annotation) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct AnnotateThingThen(::httpmock::Then);
    impl AnnotateThingThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: &types::Problem) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn success(self, status: u16, value: &types::Thing) -> Self {
            assert_eq!(status / 100u16, 2u16);
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
    fn annotate_thing<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::AnnotateThingWhen, operations::AnnotateThingThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn annotate_thing<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::AnnotateThingWhen, operations::AnnotateThingThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AnnotateThingWhen::new(when),
                operations::AnnotateThingThen::new(then),
            )
        })
    }
}
