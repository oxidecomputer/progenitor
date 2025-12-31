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
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL to which requests are made.
    pub fn new(baseurl: &str) -> Self {
        Self {
            baseurl: baseurl.to_string(),
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

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
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
    pub async fn key_get<'a>(
        &'a self,
        query: bool,
        client: bool,
        request: bool,
        response: bool,
        result: bool,
        url: bool,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let _url = format!("{}/key/{}", self.baseurl, encode_path(&query.to_string()),);
        let _request = ::gloo_net::http::Request::get(&_url)
            .query(
                ::serde_urlencoded::to_string([("client", &client)])
                    .map_err(|e| Error::InvalidRequest(e.to_string()))?
                    .split('&')
                    .filter_map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        Some((parts.next()?, parts.next().unwrap_or("")))
                    }),
            )
            .query(
                ::serde_urlencoded::to_string([("request", &request)])
                    .map_err(|e| Error::InvalidRequest(e.to_string()))?
                    .split('&')
                    .filter_map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        Some((parts.next()?, parts.next().unwrap_or("")))
                    }),
            )
            .query(
                ::serde_urlencoded::to_string([("response", &response)])
                    .map_err(|e| Error::InvalidRequest(e.to_string()))?
                    .split('&')
                    .filter_map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        Some((parts.next()?, parts.next().unwrap_or("")))
                    }),
            )
            .query(
                ::serde_urlencoded::to_string([("result", &result)])
                    .map_err(|e| Error::InvalidRequest(e.to_string()))?
                    .split('&')
                    .filter_map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        Some((parts.next()?, parts.next().unwrap_or("")))
                    }),
            )
            .query(
                ::serde_urlencoded::to_string([("url", &url)])
                    .map_err(|e| Error::InvalidRequest(e.to_string()))?
                    .split('&')
                    .filter_map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        Some((parts.next()?, parts.next().unwrap_or("")))
                    }),
            )
            .header("api-version", Self::api_version())
            .build()?;
        let info = OperationInfo {
            operation_id: "key_get",
        };
        let _result = self.exec(_request, &info).await;
        self.post(&_result, &info).await?;
        let _response = _result?;
        match _response.status() {
            200u16 => Ok(ResponseValue::empty(_response)),
            _ => Err(Error::UnexpectedResponse(_response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
