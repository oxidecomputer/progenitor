pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::example_multipart_builder::*;
    pub struct UploadWhen(::httpmock::When);
    impl UploadWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/upload$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::UploadForm) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct UploadThen(::httpmock::Then);
    impl UploadThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self) -> Self {
            Self(self.0.status(200u16))
        }
    }
}

#[doc = r" An extension trait for [`MockServer`](::httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](::httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn upload<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UploadWhen, operations::UploadThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn upload<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UploadWhen, operations::UploadThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UploadWhen::new(when),
                operations::UploadThen::new(then),
            )
        })
    }
}
