#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, QueryParam, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Items consumers will typically use such as the Client and
/// extension traits.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    ///`ListThingsXCodesItem`
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ListThingsXCodesItem(::std::string::String);
    impl ::std::ops::Deref for ListThingsXCodesItem {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ListThingsXCodesItem> for ::std::string::String {
        fn from(value: ListThingsXCodesItem) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ListThingsXCodesItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            ::std::convert::TryFrom::try_from(value)
        }
    }

    impl ::std::convert::TryFrom<&str> for ListThingsXCodesItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
                ::std::sync::LazyLock::new(|| {
                    ::regress::Regex::with_flags("^[a-z]+$", "u").unwrap()
                });
            if PATTERN.find(value).is_none() {
                return Err("doesn't match pattern \"^[a-z]+$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ListThingsXCodesItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            ::std::convert::TryFrom::try_from(value.as_str())
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ListThingsXCodesItem {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::convert::TryFrom::try_from(::std::string::String::deserialize(deserializer)?)
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

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
impl Client {
    ///Sends a `GET` request to `/things`
    ///
    ///Arguments:
    /// - `kinds`: kinds of thing to include
    /// - `tags`: tags that each thing must have
    /// - `x_codes`: codes whose items are constrained strings
    /// - `x_labels`: labels for the request
    /// - `x_shard_ids`: shards to read from
    ///```ignore
    /// let response = client.list_things()
    ///    .kinds(kinds)
    ///    .tags(tags)
    ///    .x_codes(x_codes)
    ///    .x_labels(x_labels)
    ///    .x_shard_ids(x_shard_ids)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_things(&self) -> builder::ListThings<'_> {
        builder::ListThings::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, ClientHooks, ClientInfo, Error, OperationInfo, QueryParam,
        RequestBuilderExt, ResponseValue,
    };
    ///Builder for [`Client::list_things`]
    ///
    ///[`Client::list_things`]: super::Client::list_things
    #[derive(Debug, Clone)]
    pub struct ListThings<'a> {
        client: &'a super::Client,
        kinds: Result<::std::vec::Vec<types::ThingKind>, String>,
        tags: Result<Option<::std::vec::Vec<::std::string::String>>, String>,
        x_codes: Result<Option<::std::vec::Vec<types::ListThingsXCodesItem>>, String>,
        x_labels: Result<Option<::std::vec::Vec<::std::string::String>>, String>,
        x_shard_ids: Result<::std::vec::Vec<u32>, String>,
    }

    impl<'a> ListThings<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                kinds: Err("kinds was not initialized".to_string()),
                tags: Ok(None),
                x_codes: Ok(None),
                x_labels: Ok(None),
                x_shard_ids: Err("x_shard_ids was not initialized".to_string()),
            }
        }

        pub fn kinds<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<types::ThingKind>>,
        {
            self.kinds = value.try_into().map_err(|_| {
                "conversion to `:: std :: vec :: Vec < ThingKind >` for kinds failed".to_string()
            });
            self
        }

        pub fn tags<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.tags = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for tags \
                 failed"
                    .to_string()
            });
            self
        }

        pub fn x_codes<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<types::ListThingsXCodesItem>>,
        {
            self.x_codes = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: vec :: Vec < ListThingsXCodesItem >` for x_codes failed"
                    .to_string()
            });
            self
        }

        pub fn x_labels<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.x_labels = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for x_labels \
                 failed"
                    .to_string()
            });
            self
        }

        pub fn x_shard_ids<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<u32>>,
        {
            self.x_shard_ids = value.try_into().map_err(|_| {
                "conversion to `:: std :: vec :: Vec < u32 >` for x_shard_ids failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/things`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<::std::string::String>>, Error<()>> {
            let Self {
                client,
                kinds,
                tags,
                x_codes,
                x_labels,
                x_shard_ids,
            } = self;
            let kinds = kinds.map_err(Error::InvalidRequest)?;
            let tags = tags.map_err(Error::InvalidRequest)?;
            let x_codes = x_codes.map_err(Error::InvalidRequest)?;
            let x_labels = x_labels.map_err(Error::InvalidRequest)?;
            let x_shard_ids = x_shard_ids.map_err(Error::InvalidRequest)?;
            let url = format!("{}/things", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(4usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            if let Some(value) = x_codes {
                header_map.append(
                    "x-codes",
                    value
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<::std::vec::Vec<_>>()
                        .join(",")
                        .try_into()?,
                );
            }
            if let Some(value) = x_labels {
                header_map.append(
                    "x-labels",
                    value
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<::std::vec::Vec<_>>()
                        .join(",")
                        .try_into()?,
                );
            }
            header_map.append(
                "x-shard-ids",
                x_shard_ids
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<::std::vec::Vec<_>>()
                    .join(",")
                    .try_into()?,
            );
            #[allow(unused_mut)]
            let mut request = client
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
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}
