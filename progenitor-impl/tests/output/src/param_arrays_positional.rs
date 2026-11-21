#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, QueryParam, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    ///`ThingKind`
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ThingKind {
        #[serde(rename = "widget")]
        Widget,
        #[serde(rename = "gadget")]
        Gadget,
    }

    impl ::std::fmt::Display for ThingKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Widget => f.write_str("widget"),
                Self::Gadget => f.write_str("gadget"),
            }
        }
    }

    impl ::std::str::FromStr for ThingKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "widget" => Ok(Self::Widget),
                "gadget" => Ok(Self::Gadget),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ThingKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ThingKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

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
///Client for Parameter Arrays
///
///Header and query parameters whose schema is an array
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
    ///
    /// Returns an error if the `reqwest::Client` cannot be built,
    /// for example when the TLS backend fails to initialize.
    pub fn new(baseurl: &str) -> Result<Self, reqwest::Error> {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Ok(Self::new_with_client(baseurl, client.build()?))
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
#[allow(clippy::all)]
impl Client {
    ///Sends a `GET` request to `/things`
    ///
    ///Arguments:
    /// - `kinds`: kinds of thing to include
    /// - `tags`: tags that each thing must have
    /// - `x_labels`: labels for the request
    /// - `x_shard_ids`: shards to read from
    pub async fn list_things<'a>(
        &'a self,
        kinds: &'a ::std::vec::Vec<types::ThingKind>,
        tags: Option<&'a ::std::vec::Vec<::std::string::String>>,
        x_labels: Option<&'a ::std::vec::Vec<::std::string::String>>,
        x_shard_ids: &'a ::std::vec::Vec<u32>,
    ) -> Result<ResponseValue<::std::vec::Vec<::std::string::String>>, Error<()>> {
        let url = format!("{}/things", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(3usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        if let Some(value) = x_labels {
            header_map.append(
                "x-labels",
                value
                    .iter()
                    .map(::std::string::ToString::to_string)
                    .collect::<::std::vec::Vec<_>>()
                    .join(",")
                    .try_into()?,
            );
        }

        header_map.append(
            "x-shard-ids",
            x_shard_ids
                .iter()
                .map(::std::string::ToString::to_string)
                .collect::<::std::vec::Vec<_>>()
                .join(",")
                .try_into()?,
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&QueryParam::new("kinds", &kinds))
            .query(&QueryParam::new("tags", &tags))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "list_things",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
