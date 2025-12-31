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
///Client for Parameter override test
///
///Minimal API for testing parameter overrides
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
impl Client {
    ///Gets a key
    ///
    ///Sends a `GET` request to `/key`
    ///
    ///Arguments:
    /// - `key`: The same key parameter that overlaps with the path level
    ///   parameter
    /// - `unique_key`: A key parameter that will not be overridden by the path
    ///   spec
    ///```ignore
    /// let response = client.key_get()
    ///    .key(key)
    ///    .unique_key(unique_key)
    ///    .send()
    ///    .await;
    /// ```
    pub fn key_get(&self) -> builder::KeyGet<'_> {
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
    ///Builder for [`Client::key_get`]
    ///
    ///[`Client::key_get`]: super::Client::key_get
    #[derive(Debug, Clone)]
    pub struct KeyGet<'a> {
        client: &'a super::Client,
        key: Result<Option<bool>, String>,
        unique_key: Result<Option<::std::string::String>, String>,
    }

    impl<'a> KeyGet<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                key: Ok(None),
                unique_key: Ok(None),
            }
        }

        pub fn key<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.key = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for key failed".to_string());
            self
        }

        pub fn unique_key<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.unique_key = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for unique_key failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/key`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                key,
                unique_key,
            } = self;
            let key = key.map_err(Error::InvalidRequest)?;
            let unique_key = unique_key.map_err(Error::InvalidRequest)?;
            let url = format!("{}/key", client.baseurl,);
            let request = ::gloo_net::http::Request::get(&url)
                .query(
                    ::serde_urlencoded::to_string([("key", &key)])
                        .map_err(|e| Error::InvalidRequest(e.to_string()))?
                        .split('&')
                        .filter_map(|pair| {
                            let mut parts = pair.splitn(2, '=');
                            Some((parts.next()?, parts.next().unwrap_or("")))
                        }),
                )
                .query(
                    ::serde_urlencoded::to_string([("uniqueKey", &unique_key)])
                        .map_err(|e| Error::InvalidRequest(e.to_string()))?
                        .split('&')
                        .filter_map(|pair| {
                            let mut parts = pair.splitn(2, '=');
                            Some((parts.next()?, parts.next().unwrap_or("")))
                        }),
                )
                .header("api-version", super::Client::api_version())
                .build()?;
            let info = OperationInfo {
                operation_id: "key_get",
            };
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status() {
                200u16 => Ok(ResponseValue::empty(response)),
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
