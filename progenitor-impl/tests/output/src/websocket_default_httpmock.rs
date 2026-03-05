pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::websocket_default_builder::*;
    pub struct WebsocketWithoutExplicit101When(::httpmock::When);
    impl WebsocketWithoutExplicit101When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/websocket/default$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct WebsocketWithoutExplicit101Then(::httpmock::Then);
    impl WebsocketWithoutExplicit101Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn switching_protocols(self) -> Self {
            Self(self.0.status(101u16))
        }
    }
}

#[doc = r" An extension trait for [`MockServer`](::httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](::httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn websocket_without_explicit_101<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::WebsocketWithoutExplicit101When,
            operations::WebsocketWithoutExplicit101Then,
        );
}

impl MockServerExt for ::httpmock::MockServer {
    fn websocket_without_explicit_101<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(
            operations::WebsocketWithoutExplicit101When,
            operations::WebsocketWithoutExplicit101Then,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WebsocketWithoutExplicit101When::new(when),
                operations::WebsocketWithoutExplicit101Then::new(then),
            )
        })
    }
}
