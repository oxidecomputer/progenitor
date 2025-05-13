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
}

#[derive(Clone, Debug)]
///Client for Parameter name collision test
///
///Minimal API for testing collision between parameter names and generated code
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
    ///Gets a key
    ///
    ///Sends a `GET` request to `/key/{query}`
    ///
    ///Arguments:
    /// - `query`: Parameter name that was previously colliding
    /// - `client`: Parameter name that was previously colliding
    /// - `request`: Parameter name that was previously colliding
    /// - `response`: Parameter name that was previously colliding
    /// - `result`: Parameter name that was previously colliding
    /// - `url`: Parameter name that was previously colliding
    ///```ignore
    /// let response = client.key_get()
    ///    .query(query)
    ///    .client(client)
    ///    .request(request)
    ///    .response(response)
    ///    .result(result)
    ///    .url(url)
    ///    .send()
    ///    .await;
    /// ```
    pub fn key_get(&self) -> builder::KeyGet {
        builder::KeyGet::new(self)
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
    pub mod built {
        use super::super::types;
        #[allow(unused_imports)]
        use super::super::{
            encode_path, ByteStream, ClientHooks, ClientInfo, Error, OperationInfo,
            RequestBuilderExt, ResponseValue,
        };
        pub struct KeyGet<'a> {
            pub(crate) client: &'a super::super::Client,
            pub(crate) request: reqwest::RequestBuilder,
        }

        impl<'a> KeyGet<'a> {
            pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
                let Self { client, request } = self;
                #[allow(unused_mut)]
                let mut request = request.build()?;
                let info = OperationInfo {
                    operation_id: "key_get",
                };
                client.pre(&mut request, &info).await?;
                let _result = client.exec(request, &info).await;
                client.post(&_result, &info).await?;
                let _response = _result?;
                match _response.status().as_u16() {
                    200u16 => Ok(ResponseValue::empty(_response)),
                    _ => Err(Error::UnexpectedResponse(_response)),
                }
            }
            pub fn map_request<F>(self, f: F) -> Self
            where
                F: Fn(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
            {
                Self {
                    client: self.client,
                    request: f(self.request),
                }
            }
        }
    }

    ///Builder for [`Client::key_get`]
    ///
    ///[`Client::key_get`]: super::Client::key_get
    #[derive(Debug, Clone)]
    pub struct KeyGet<'a> {
        _client: &'a super::Client,
        query: Result<bool, String>,
        client: Result<bool, String>,
        request: Result<bool, String>,
        response: Result<bool, String>,
        result: Result<bool, String>,
        url: Result<bool, String>,
    }

    impl<'a> KeyGet<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                _client: client,
                query: Err("query was not initialized".to_string()),
                client: Err("client was not initialized".to_string()),
                request: Err("request was not initialized".to_string()),
                response: Err("response was not initialized".to_string()),
                result: Err("result was not initialized".to_string()),
                url: Err("url was not initialized".to_string()),
            }
        }

        pub fn query<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.query = value
                .try_into()
                .map_err(|_| "conversion to `bool` for query failed".to_string());
            self
        }

        pub fn client<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.client = value
                .try_into()
                .map_err(|_| "conversion to `bool` for client failed".to_string());
            self
        }

        pub fn request<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.request = value
                .try_into()
                .map_err(|_| "conversion to `bool` for request failed".to_string());
            self
        }

        pub fn response<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.response = value
                .try_into()
                .map_err(|_| "conversion to `bool` for response failed".to_string());
            self
        }

        pub fn result<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.result = value
                .try_into()
                .map_err(|_| "conversion to `bool` for result failed".to_string());
            self
        }

        pub fn url<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.url = value
                .try_into()
                .map_err(|_| "conversion to `bool` for url failed".to_string());
            self
        }

        ///Sends a `GET` request to `/key/{query}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            self.build()?.send().await
        }

        pub fn build(self) -> Result<built::KeyGet<'a>, Error<()>> {
            let Self {
                _client,
                query,
                client,
                request,
                response,
                result,
                url,
            } = self;
            let query = query.map_err(Error::InvalidRequest)?;
            let client = client.map_err(Error::InvalidRequest)?;
            let request = request.map_err(Error::InvalidRequest)?;
            let response = response.map_err(Error::InvalidRequest)?;
            let result = result.map_err(Error::InvalidRequest)?;
            let url = url.map_err(Error::InvalidRequest)?;
            let request = {
                let _url = format!("{}/key/{}", client.baseurl, encode_path(&query.to_string()),);
                let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
                header_map.append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
                );
                client
                    .client
                    .get(_url)
                    .query(&progenitor_client::QueryParam::new("client", &client))
                    .query(&progenitor_client::QueryParam::new("request", &request))
                    .query(&progenitor_client::QueryParam::new("response", &response))
                    .query(&progenitor_client::QueryParam::new("result", &result))
                    .query(&progenitor_client::QueryParam::new("url", &url))
                    .headers(header_map)
            };
            Ok(built::KeyGet {
                client: _client,
                request,
            })
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
