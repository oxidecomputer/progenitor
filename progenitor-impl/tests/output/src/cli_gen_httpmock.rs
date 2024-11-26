pub mod operations {
    #![doc = r" [`When`](httpmock::When) and [`Then`](httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::cli_gen_builder::*;
    pub struct UnoWhen(httpmock::When);
    impl UnoWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/uno$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn gateway(self, value: &str) -> Self {
            Self(self.0.query_param("gateway", value.to_string()))
        }

        pub fn body(self, value: &types::UnoBody) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct UnoThen(httpmock::Then);
    impl UnoThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn success(self, status: u16, value: serde_json::Value) -> Self {
            assert_eq!(status / 100u16, 2u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }
}

#[doc = r" An extension trait for [`MockServer`](httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn uno<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UnoWhen, operations::UnoThen);
}

impl MockServerExt for httpmock::MockServer {
    fn uno<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UnoWhen, operations::UnoThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UnoWhen::new(when),
                operations::UnoThen::new(then),
            )
        })
    }
}
