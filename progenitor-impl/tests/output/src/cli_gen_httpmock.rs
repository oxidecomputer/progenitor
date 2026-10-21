pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::cli_gen_builder::*;
    #[doc = r" Match the query parameter `name` against exactly the pairs"]
    #[doc = r" that the client would send for `value`. Each pair must"]
    #[doc = r" appear as many times as the client would send it, and no"]
    #[doc = r" other values may be present for those keys. If the client"]
    #[doc = r" would send nothing, the parameter must be absent."]
    fn match_query_param<T: ::serde::Serialize>(
        mut when: ::httpmock::When,
        name: &str,
        value: &T,
    ) -> ::httpmock::When {
        let pairs = ::progenitor_client::query_param_pairs(name, value)
            .expect("failed to serialize query param");
        if pairs.is_empty() {
            return when.query_param_missing(name);
        }

        let exact = |s: &str| format!("^{}$", regex::escape(s));
        let mut key_counts = ::std::collections::BTreeMap::<&str, usize>::new();
        let mut pair_counts = ::std::collections::BTreeMap::<(&str, &str), usize>::new();
        for (key, value) in &pairs {
            *key_counts.entry(key.as_str()).or_default() += 1;
            *pair_counts
                .entry((key.as_str(), value.as_str()))
                .or_default() += 1;
        }

        for ((key, value), count) in pair_counts {
            when = when.query_param_count(exact(key), exact(value), count);
        }

        for (key, count) in key_counts {
            when = when.query_param_count(exact(key), "(?s).*", count);
        }

        when
    }

    pub struct UnoWhen(::httpmock::When);
    impl UnoWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/uno$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn gateway(self, value: &str) -> Self {
            Self(match_query_param(self.0, "gateway", &value))
        }

        pub fn body(self, value: &types::UnoBody) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct UnoThen(::httpmock::Then);
    impl UnoThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn success(self, status: u16, value: ::serde_json::Value) -> Self {
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

#[doc = r" An extension trait for [`MockServer`](::httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](::httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn uno<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::UnoWhen, operations::UnoThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn uno<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
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
