#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///`Annotation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "note"
    ///  ],
    ///  "properties": {
    ///    "note": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Annotation {
        pub note: ::std::string::String,
    }

    impl Annotation {
        pub fn builder() -> builder::Annotation {
            Default::default()
        }
    }

    ///`Problem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "status",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "detail": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "integer",
    ///      "format": "uint16",
    ///      "minimum": 0.0
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Problem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub detail: ::std::option::Option<::std::string::String>,
        pub status: u16,
        pub title: ::std::string::String,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }

    impl Problem {
        pub fn builder() -> builder::Problem {
            Default::default()
        }
    }

    ///`Thing`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "notes"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "notes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Thing {
        pub id: ::std::string::String,
        pub notes: ::std::vec::Vec<::std::string::String>,
    }

    impl Thing {
        pub fn builder() -> builder::Thing {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct Annotation {
            note: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for Annotation {
            fn default() -> Self {
                Self {
                    note: Err("no value supplied for note".to_string()),
                }
            }
        }

        impl Annotation {
            pub fn note<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.note = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for note: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Annotation> for super::Annotation {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Annotation,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self { note: value.note? })
            }
        }

        impl ::std::convert::From<super::Annotation> for Annotation {
            fn from(value: super::Annotation) -> Self {
                Self {
                    note: Ok(value.note),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Problem {
            detail: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            status: ::std::result::Result<u16, ::std::string::String>,
            title: ::std::result::Result<::std::string::String, ::std::string::String>,
            type_: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Problem {
            fn default() -> Self {
                Self {
                    detail: Ok(Default::default()),
                    status: Err("no value supplied for status".to_string()),
                    title: Err("no value supplied for title".to_string()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl Problem {
            pub fn detail<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.detail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for detail: {e}"));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u16>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {e}"));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Problem> for super::Problem {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Problem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    detail: value.detail?,
                    status: value.status?,
                    title: value.title?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::Problem> for Problem {
            fn from(value: super::Problem) -> Self {
                Self {
                    detail: Ok(value.detail),
                    status: Ok(value.status),
                    title: Ok(value.title),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Thing {
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            notes: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Thing {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    notes: Err("no value supplied for notes".to_string()),
                }
            }
        }

        impl Thing {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn notes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.notes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for notes: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Thing> for super::Thing {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Thing,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    notes: value.notes?,
                })
            }
        }

        impl ::std::convert::From<super::Thing> for Thing {
            fn from(value: super::Thing) -> Self {
                Self {
                    id: Ok(value.id),
                    notes: Ok(value.notes),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for JSON structured syntax suffix test
///
///Minimal API for testing RFC 6839 "+json" structured syntax suffix media
/// types on request bodies, responses, and errors
///
///Version: v1
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "v1"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
impl Client {
    ///Sends a `POST` request to `/things/{id}/annotate`
    ///
    ///```ignore
    /// let response = client.annotate_thing()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn annotate_thing(&self) -> builder::AnnotateThing<'_> {
        builder::AnnotateThing::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, ClientHooks, ClientInfo, Error, OperationInfo, RequestBuilderExt,
        ResponseValue,
    };
    ///Builder for [`Client::annotate_thing`]
    ///
    ///[`Client::annotate_thing`]: super::Client::annotate_thing
    #[derive(Debug, Clone)]
    pub struct AnnotateThing<'a> {
        client: &'a super::Client,
        id: Result<::std::string::String, String>,
        body: Result<types::builder::Annotation, String>,
    }

    impl<'a> AnnotateThing<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::Annotation>,
            <V as std::convert::TryInto<types::Annotation>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `Annotation` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::Annotation) -> types::builder::Annotation,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/things/{id}/annotate`
        pub async fn send(self) -> Result<ResponseValue<types::Thing>, Error<types::Problem>> {
            let Self { client, id, body } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::Annotation::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/things/{}/annotate",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .header(
                    ::reqwest::header::CONTENT_TYPE,
                    ::reqwest::header::HeaderValue::from_static(
                        "application/vnd.example.annotation+json",
                    ),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "annotate_thing",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => ResponseValue::from_response(response).await,
                _ => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client and
/// extension traits.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
