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

    ///BodyWithDefaults
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "s"
    ///  ],
    ///  "properties": {
    ///    "forty-two": {
    ///      "default": 42,
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "s": {
    ///      "type": "string"
    ///    },
    ///    "something": {
    ///      "default": true,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "yes": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BodyWithDefaults {
        #[serde(rename = "forty-two", default = "defaults::default_u64::<u32, 42>")]
        pub forty_two: u32,
        pub s: String,
        #[serde(default = "defaults::body_with_defaults_something")]
        pub something: Option<bool>,
        #[serde(default)]
        pub yes: bool,
    }

    impl From<&BodyWithDefaults> for BodyWithDefaults {
        fn from(value: &BodyWithDefaults) -> Self {
            value.clone()
        }
    }

    impl BodyWithDefaults {
        pub fn builder() -> builder::BodyWithDefaults {
            Default::default()
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
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

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct BodyWithDefaults {
            forty_two: Result<u32, String>,
            s: Result<String, String>,
            something: Result<Option<bool>, String>,
            yes: Result<bool, String>,
        }

        impl Default for BodyWithDefaults {
            fn default() -> Self {
                Self {
                    forty_two: Ok(super::defaults::default_u64::<u32, 42>()),
                    s: Err("no value supplied for s".to_string()),
                    something: Ok(super::defaults::body_with_defaults_something()),
                    yes: Ok(Default::default()),
                }
            }
        }

        impl BodyWithDefaults {
            pub fn forty_two<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u32>,
                T::Error: std::fmt::Display,
            {
                self.forty_two = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for forty_two: {}", e));
                self
            }
            pub fn s<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.s = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for s: {}", e));
                self
            }
            pub fn something<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.something = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for something: {}", e));
                self
            }
            pub fn yes<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.yes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for yes: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<BodyWithDefaults> for super::BodyWithDefaults {
            type Error = super::error::ConversionError;
            fn try_from(value: BodyWithDefaults) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    forty_two: value.forty_two?,
                    s: value.s?,
                    something: value.something?,
                    yes: value.yes?,
                })
            }
        }

        impl From<super::BodyWithDefaults> for BodyWithDefaults {
            fn from(value: super::BodyWithDefaults) -> Self {
                Self {
                    forty_two: Ok(value.forty_two),
                    s: Ok(value.s),
                    something: Ok(value.something),
                    yes: Ok(value.yes),
                }
            }
        }

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
    }

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn default_u64<T, const V: u64>() -> T
        where
            T: std::convert::TryFrom<u64>,
            <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
        {
            T::try_from(V).unwrap()
        }

        pub(super) fn body_with_defaults_something() -> Option<bool> {
            Some(true)
        }
    }
}

#[derive(Clone, Debug)]
///Client for pagination-demo
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
    ///Sends a `POST` request to `/`
    ///
    ///```ignore
    /// let response = client.default_params()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn default_params(&self) -> builder::DefaultParams {
        builder::DefaultParams::new(self)
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
    ///Builder for [`Client::default_params`]
    ///
    ///[`Client::default_params`]: super::Client::default_params
    #[derive(Debug, Clone)]
    pub struct DefaultParams<'a> {
        client: &'a super::Client,
        body: Result<types::builder::BodyWithDefaults, String>,
    }

    impl<'a> DefaultParams<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::BodyWithDefaults::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::BodyWithDefaults>,
            <V as std::convert::TryInto<types::BodyWithDefaults>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `BodyWithDefaults` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::BodyWithDefaults,
            ) -> types::builder::BodyWithDefaults,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<ByteStream>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::BodyWithDefaults::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client.client.post(url).json(&body).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
            }
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
