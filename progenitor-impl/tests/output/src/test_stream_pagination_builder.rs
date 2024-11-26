#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
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

    ///Error information from a response.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Error information from a response.",
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "request_id"
    ///  ],
    ///  "properties": {
    ///    "error_code": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "request_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error_code: Option<String>,
        pub message: String,
        pub request_id: String,
    }

    impl From<&Error> for Error {
        fn from(value: &Error) -> Self {
            value.clone()
        }
    }

    impl Error {
        pub fn builder() -> builder::Error {
            Default::default()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint32",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Uint32ResultsPage {
        ///list of items on this page of results
        pub items: Vec<u32>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&Uint32ResultsPage> for Uint32ResultsPage {
        fn from(value: &Uint32ResultsPage) -> Self {
            value.clone()
        }
    }

    impl Uint32ResultsPage {
        pub fn builder() -> builder::Uint32ResultsPage {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct Error {
            error_code: Result<Option<String>, String>,
            message: Result<String, String>,
            request_id: Result<String, String>,
        }

        impl Default for Error {
            fn default() -> Self {
                Self {
                    error_code: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                    request_id: Err("no value supplied for request_id".to_string()),
                }
            }
        }

        impl Error {
            pub fn error_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.error_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error_code: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn request_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.request_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for request_id: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Error> for super::Error {
            type Error = super::error::ConversionError;
            fn try_from(value: Error) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    error_code: value.error_code?,
                    message: value.message?,
                    request_id: value.request_id?,
                })
            }
        }

        impl From<super::Error> for Error {
            fn from(value: super::Error) -> Self {
                Self {
                    error_code: Ok(value.error_code),
                    message: Ok(value.message),
                    request_id: Ok(value.request_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Uint32ResultsPage {
            items: Result<Vec<u32>, String>,
            next_page: Result<Option<String>, String>,
        }

        impl Default for Uint32ResultsPage {
            fn default() -> Self {
                Self {
                    items: Err("no value supplied for items".to_string()),
                    next_page: Ok(Default::default()),
                }
            }
        }

        impl Uint32ResultsPage {
            pub fn items<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<u32>>,
                T::Error: std::fmt::Display,
            {
                self.items = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for items: {}", e));
                self
            }
            pub fn next_page<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.next_page = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for next_page: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Uint32ResultsPage> for super::Uint32ResultsPage {
            type Error = super::error::ConversionError;
            fn try_from(value: Uint32ResultsPage) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    items: value.items?,
                    next_page: value.next_page?,
                })
            }
        }

        impl From<super::Uint32ResultsPage> for Uint32ResultsPage {
            fn from(value: super::Uint32ResultsPage) -> Self {
                Self {
                    items: Ok(value.items),
                    next_page: Ok(value.next_page),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for test_stream_pagination
///
///Version: 1.0.0
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
            let dur = std::time::Duration::from_secs(15);
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

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0.0"
    }
}

impl Client {
    ///Sends a `GET` request to `/`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    ///```ignore
    /// let response = client.paginated_u32s()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .send()
    ///    .await;
    /// ```
    pub fn paginated_u32s(&self) -> builder::PaginatedU32s {
        builder::PaginatedU32s::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    ///Builder for [`Client::paginated_u32s`]
    ///
    ///[`Client::paginated_u32s`]: super::Client::paginated_u32s
    #[derive(Debug, Clone)]
    pub struct PaginatedU32s<'a> {
        client: &'a super::Client,
        limit: Result<Option<std::num::NonZeroU32>, String>,
        page_token: Result<Option<String>, String>,
    }

    impl<'a> PaginatedU32s<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                limit: Ok(None),
                page_token: Ok(None),
            }
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<std::num::NonZeroU32>,
        {
            self.limit = value.try_into().map(Some).map_err(|_| {
                "conversion to `std :: num :: NonZeroU32` for limit failed".to_string()
            });
            self
        }

        pub fn page_token<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.page_token = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for page_token failed".to_string());
            self
        }

        ///Sends a `GET` request to `/`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::Uint32ResultsPage>, Error<types::Error>> {
            let Self {
                client,
                limit,
                page_token,
            } = self;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let page_token = page_token.map_err(Error::InvalidRequest)?;
            let url = format!("{}/", client.baseurl,);
            let mut query = Vec::with_capacity(2usize);
            if let Some(limit) = &limit {
                query.push(("limit", limit.to_string()));
            }
            if let Some(page_token) = &page_token {
                query.push(("page_token", page_token.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }

        ///Streams `GET` requests to `/`
        pub fn stream(
            self,
        ) -> impl futures::Stream<Item = Result<u32, Error<types::Error>>> + Unpin + 'a {
            use futures::StreamExt;
            use futures::TryFutureExt;
            use futures::TryStreamExt;
            let next = Self {
                page_token: Ok(None),
                ..self.clone()
            };
            self.send()
                .map_ok(move |page| {
                    let page = page.into_inner();
                    let first = futures::stream::iter(page.items).map(Ok);
                    let rest = futures::stream::try_unfold(
                        (page.next_page, next),
                        |(next_page, next)| async {
                            if next_page.is_none() {
                                Ok(None)
                            } else {
                                Self {
                                    page_token: Ok(next_page),
                                    ..next.clone()
                                }
                                .send()
                                .map_ok(|page| {
                                    let page = page.into_inner();
                                    Some((
                                        futures::stream::iter(page.items).map(Ok),
                                        (page.next_page, next),
                                    ))
                                })
                                .await
                            }
                        },
                    )
                    .try_flatten();
                    first.chain(rest)
                })
                .try_flatten_stream()
                .boxed()
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
