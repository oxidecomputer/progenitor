pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::multipart_related_test_builder::*;
    pub struct UploadFileWhen(::httpmock::When);
    impl UploadFileWhen {
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

        pub fn body(self, value: &types::UploadFileMultipartParts) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct UploadFileThen(::httpmock::Then);
    impl UploadFileThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UploadResponse) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn bad_request(self) -> Self {
            Self(self.0.status(400u16))
        }
    }

    pub struct UploadSimpleWhen(::httpmock::When);
    impl UploadSimpleWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/upload-simple$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: ::serde_json::Value) -> Self {
            Self(self.0.json_body(value))
        }
    }

    pub struct UploadSimpleThen(::httpmock::Then);
    impl UploadSimpleThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }
    }

    pub struct UploadMultipleFilesWhen(::httpmock::When);
    impl UploadMultipleFilesWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/upload-multiple$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::UploadMultipleFilesMultipartParts) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct UploadMultipleFilesThen(::httpmock::Then);
    impl UploadMultipleFilesThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UploadResponse) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn bad_request(self) -> Self {
            Self(self.0.status(400u16))
        }
    }
}

#[doc = r" An extension trait for [`MockServer`](::httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](::httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn upload_file<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UploadFileWhen, operations::UploadFileThen);
    fn upload_simple<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UploadSimpleWhen, operations::UploadSimpleThen);
    fn upload_multiple_files<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UploadMultipleFilesWhen, operations::UploadMultipleFilesThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn upload_file<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UploadFileWhen, operations::UploadFileThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UploadFileWhen::new(when),
                operations::UploadFileThen::new(then),
            )
        })
    }

    fn upload_simple<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UploadSimpleWhen, operations::UploadSimpleThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UploadSimpleWhen::new(when),
                operations::UploadSimpleThen::new(then),
            )
        })
    }

    fn upload_multiple_files<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UploadMultipleFilesWhen, operations::UploadMultipleFilesThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UploadMultipleFilesWhen::new(when),
                operations::UploadMultipleFilesThen::new(then),
            )
        })
    }
}
