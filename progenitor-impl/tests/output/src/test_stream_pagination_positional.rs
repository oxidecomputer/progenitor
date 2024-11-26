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

#[allow(clippy::all)]
impl Client {
    ///Sends a `GET` request to `/`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    pub async fn paginated_u32s<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::Uint32ResultsPage>, Error<types::Error>> {
        let url = format!("{}/", self.baseurl,);
        let mut query = Vec::with_capacity(2usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
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

    ///Sends repeated `GET` requests to `/` until there are no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    pub fn paginated_u32s_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
    ) -> impl futures::Stream<Item = Result<u32, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.paginated_u32s(limit, None)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.paginated_u32s(limit, state.as_deref())
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
