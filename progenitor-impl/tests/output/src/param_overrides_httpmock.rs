pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::param_overrides_builder::*;
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

    pub struct KeyGetWhen(::httpmock::When);
    impl KeyGetWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/key$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn key<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("key", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("key"))
            }
        }

        pub fn unique_key<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                let expected_pairs = ::progenitor_client::query_param_pairs("uniqueKey", &value)
                    .expect("failed to serialize query param");
                Self(apply_query_param_pairs(self.0, &expected_pairs))
            } else {
                Self(self.0.query_param_missing("uniqueKey"))
            }
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
    fn key_get<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::KeyGetWhen, operations::KeyGetThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn key_get<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
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
