pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::missing_operation_id_builder::*;
    pub struct DoPingWhen(::httpmock::When);
    impl DoPingWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct DoPingThen(::httpmock::Then);
    impl DoPingThen {
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
    fn do_ping<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DoPingWhen, operations::DoPingThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn do_ping<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::DoPingWhen, operations::DoPingThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DoPingWhen::new(when),
                operations::DoPingThen::new(then),
            )
        })
    }
}
