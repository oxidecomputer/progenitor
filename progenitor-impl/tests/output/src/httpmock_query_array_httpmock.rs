pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::httpmock_query_array_builder::*;
    /// Apply decoded query parameter pairs to the matcher.
    fn apply_query_param_pairs(
        mut when: ::httpmock::When,
        pairs: &[(String, String)],
    ) -> ::httpmock::When {
        for (key, value) in pairs {
            when = when.query_param(key, value);
        }

        when
    }

    pub struct ListWidgetsWhen(::httpmock::When);
    impl ListWidgetsWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/widgets$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn tags(self, value: &::std::vec::Vec<::std::string::String>) -> Self {
            let expected_pairs = ::progenitor_client::query_param_pairs("tags", &value)
                .expect("failed to serialize query param");
            Self(apply_query_param_pairs(self.0, &expected_pairs))
        }
    }

    pub struct ListWidgetsThen(::httpmock::Then);
    impl ListWidgetsThen {
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
}

#[doc = r" An extension trait for [`MockServer`](::httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](::httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn list_widgets<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ListWidgetsWhen, operations::ListWidgetsThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn list_widgets<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ListWidgetsWhen, operations::ListWidgetsThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ListWidgetsWhen::new(when),
                operations::ListWidgetsThen::new(then),
            )
        })
    }
}
