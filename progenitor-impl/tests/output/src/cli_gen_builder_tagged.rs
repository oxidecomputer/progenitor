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

    ///UnoBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "required"
    ///  ],
    ///  "properties": {
    ///    "gateway": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UnoBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gateway: Option<String>,
        pub required: ::serde_json::Value,
    }

    impl From<&UnoBody> for UnoBody {
        fn from(value: &UnoBody) -> Self {
            value.clone()
        }
    }

    impl UnoBody {
        pub fn builder() -> builder::UnoBody {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct UnoBody {
            gateway: Result<Option<String>, String>,
            required: Result<::serde_json::Value, String>,
        }

        impl Default for UnoBody {
            fn default() -> Self {
                Self {
                    gateway: Ok(Default::default()),
                    required: Err("no value supplied for required".to_string()),
                }
            }
        }

        impl UnoBody {
            pub fn gateway<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.gateway = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gateway: {}", e));
                self
            }
            pub fn required<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<::serde_json::Value>,
                T::Error: std::fmt::Display,
            {
                self.required = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for required: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UnoBody> for super::UnoBody {
            type Error = super::error::ConversionError;
            fn try_from(value: UnoBody) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    gateway: value.gateway?,
                    required: value.required?,
                })
            }
        }

        impl From<super::UnoBody> for UnoBody {
            fn from(value: super::UnoBody) -> Self {
                Self {
                    gateway: Ok(value.gateway),
                    required: Ok(value.required),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for CLI gen test
///
///Test case to exercise CLI generation
///
///Version: 9000
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
        "9000"
    }
}

impl Client {
    ///Sends a `GET` request to `/uno`
    ///
    ///```ignore
    /// let response = client.uno()
    ///    .gateway(gateway)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn uno(&self) -> builder::Uno {
        builder::Uno::new(self)
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
    ///Builder for [`Client::uno`]
    ///
    ///[`Client::uno`]: super::Client::uno
    #[derive(Debug, Clone)]
    pub struct Uno<'a> {
        client: &'a super::Client,
        gateway: Result<String, String>,
        body: Result<types::builder::UnoBody, String>,
    }

    impl<'a> Uno<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                gateway: Err("gateway was not initialized".to_string()),
                body: Ok(types::builder::UnoBody::default()),
            }
        }

        pub fn gateway<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.gateway = value
                .try_into()
                .map_err(|_| "conversion to `String` for gateway failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UnoBody>,
            <V as std::convert::TryInto<types::UnoBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `UnoBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::UnoBody) -> types::builder::UnoBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `GET` request to `/uno`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self {
                client,
                gateway,
                body,
            } = self;
            let gateway = gateway.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::UnoBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/uno", client.baseurl,);
            let mut query = Vec::with_capacity(1usize);
            query.push(("gateway", gateway.to_string()));
            #[allow(unused_mut)]
            let mut request = client.client.get(url).json(&body).query(&query).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
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
