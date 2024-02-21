#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
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
        "v1"
    }
}

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
        let mut _query = Vec::with_capacity(5usize);
        _query.push(("client", client.to_string()));
        _query.push(("request", request.to_string()));
        _query.push(("response", response.to_string()));
        _query.push(("result", result.to_string()));
        _query.push(("url", url.to_string()));
        #[allow(unused_mut)]
        let mut _request = self.client.get(_url).query(&_query).build()?;
        let _result = self.client.execute(_request).await;
        let _response = _result?;
        match _response.status().as_u16() {
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
