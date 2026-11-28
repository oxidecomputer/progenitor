pub mod operations {
    #![doc = r" [`When`](::httpmock::When) and [`Then`](::httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::param_arrays_builder::*;
    pub struct ListThingsWhen(::httpmock::When);
    impl ListThingsWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/things$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn kinds(self, value: &::std::vec::Vec<types::ThingKind>) -> Self {
            Self(self.0.query_param("kinds", value.to_string()))
        }

        pub fn tags<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::std::vec::Vec<::std::string::String>>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("tags", value.to_string()))
            } else {
                Self(self.0.query_param_missing("tags"))
            }
        }

        pub fn x_codes<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::std::vec::Vec<types::ListThingsXCodesItem>>>,
        {
            if let Some(value) = value.into() {
                Self(
                    self.0.header(
                        "x-codes",
                        value
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<::std::vec::Vec<_>>()
                            .join(","),
                    ),
                )
            } else {
                Self(self.0.header_missing("x-codes"))
            }
        }

        pub fn x_labels<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::std::vec::Vec<::std::string::String>>>,
        {
            if let Some(value) = value.into() {
                Self(
                    self.0.header(
                        "x-labels",
                        value
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<::std::vec::Vec<_>>()
                            .join(","),
                    ),
                )
            } else {
                Self(self.0.header_missing("x-labels"))
            }
        }

        pub fn x_shard_ids(self, value: &::std::vec::Vec<u32>) -> Self {
            Self(
                self.0.header(
                    "x-shard-ids",
                    value
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<::std::vec::Vec<_>>()
                        .join(","),
                ),
            )
        }
    }

    pub struct ListThingsThen(::httpmock::Then);
    impl ListThingsThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &::std::vec::Vec<::std::string::String>) -> Self {
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
    fn list_things<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ListThingsWhen, operations::ListThingsThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn list_things<F>(&self, config_fn: F) -> ::httpmock::Mock<'_>
    where
        F: FnOnce(operations::ListThingsWhen, operations::ListThingsThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ListThingsWhen::new(when),
                operations::ListThingsThen::new(then),
            )
        })
    }
}
