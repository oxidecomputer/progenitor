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

    ///`UnoBody`
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
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gateway: ::std::option::Option<::std::string::String>,
        pub required: ::serde_json::Value,
    }

    impl ::std::convert::From<&UnoBody> for UnoBody {
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
            gateway: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            required: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        }

        impl ::std::default::Default for UnoBody {
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
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.gateway = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gateway: {}", e));
                self
            }
            pub fn required<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.required = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for required: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<UnoBody> for super::UnoBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UnoBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    gateway: value.gateway?,
                    required: value.required?,
                })
            }
        }

        impl ::std::convert::From<super::UnoBody> for UnoBody {
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
        "9000"
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
    ///Sends a `GET` request to `/uno`
    ///
    ///```ignore
    /// let response = client.uno()
    ///    .gateway(gateway)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn uno(&self) -> builder::Uno<'_> {
        builder::Uno::new(self)
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
    ///Builder for [`Client::uno`]
    ///
    ///[`Client::uno`]: super::Client::uno
    #[derive(Debug, Clone)]
    pub struct Uno<'a> {
        client: &'a super::Client,
        gateway: Result<::std::string::String, String>,
        body: Result<types::builder::UnoBody, String>,
    }

    impl<'a> Uno<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                gateway: Err("gateway was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn gateway<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.gateway = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for gateway failed".to_string()
            });
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
            let request = ::gloo_net::http::Request::get(&url)
                .query(
                    ::serde_urlencoded::to_string([("gateway", &gateway)])
                        .map_err(|e| Error::InvalidRequest(e.to_string()))?
                        .split('&')
                        .filter_map(|pair| {
                            let mut parts = pair.splitn(2, '=');
                            Some((parts.next()?, parts.next().unwrap_or("")))
                        }),
                )
                .header("api-version", super::Client::api_version())
                .json(&body)?;
            let info = OperationInfo {
                operation_id: "uno",
            };
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status() {
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
