pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::httpmock_string_body_builder::*;
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

    pub struct GetMessageWhen(::httpmock::When);
    impl GetMessageWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/message$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct GetMessageThen(::httpmock::Then);
    impl GetMessageThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &str) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SendMessageWhen(::httpmock::When);
    impl SendMessageWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/message$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &str) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SendMessageThen(::httpmock::Then);
    impl SendMessageThen {
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
    fn get_message<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::GetMessageWhen, operations::GetMessageThen);
    fn send_message<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SendMessageWhen, operations::SendMessageThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn get_message<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::GetMessageWhen, operations::GetMessageThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::GetMessageWhen::new(when),
                operations::GetMessageThen::new(then),
            )
        })
    }

    fn send_message<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::SendMessageWhen, operations::SendMessageThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SendMessageWhen::new(when),
                operations::SendMessageThen::new(then),
            )
        })
    }
}
