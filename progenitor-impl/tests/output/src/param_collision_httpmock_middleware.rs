pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::param_collision_builder::*;
    pub struct KeyGetWhen(::httpmock::When);
    impl KeyGetWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/key/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn query(self, value: bool) -> Self {
            let re = regex::Regex::new(&format!("^/key/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn client(self, value: bool) -> Self {
            Self(self.0.query_param("client", value.to_string()))
        }

        pub fn request(self, value: bool) -> Self {
            Self(self.0.query_param("request", value.to_string()))
        }

        pub fn response(self, value: bool) -> Self {
            Self(self.0.query_param("response", value.to_string()))
        }

        pub fn result(self, value: bool) -> Self {
            Self(self.0.query_param("result", value.to_string()))
        }

        pub fn url(self, value: bool) -> Self {
            Self(self.0.query_param("url", value.to_string()))
        }
    }

    pub struct KeyGetThen(::httpmock::Then);
    impl KeyGetThen {
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
    fn key_get<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::KeyGetWhen, operations::KeyGetThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn key_get<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::KeyGetWhen, operations::KeyGetThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::KeyGetWhen::new(when),
                operations::KeyGetThen::new(then),
            )
        })
    }
}
