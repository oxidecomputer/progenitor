pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::query_param_defaults_builder::*;
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

        pub fn required_param(self, value: &str) -> Self {
            Self(self.0.query_param("requiredParam", value.to_string()))
        }

        pub fn supports_all_drives<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("supportsAllDrives", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "supportsAllDrives"))
                        .is_none()
                }))
            }
        }

        pub fn upload_type<T>(self, value: T) -> Self
        where
            T: Into<Option<types::UploadFileUploadType>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("uploadType", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "uploadType"))
                        .is_none()
                }))
            }
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

        pub fn ok(self, value: &types::UploadFileResponse) -> Self {
            Self(
                self.0
                    .status(200u16)
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
    fn upload_file<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UploadFileWhen, operations::UploadFileThen);
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
}
