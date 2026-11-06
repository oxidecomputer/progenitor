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

    ///`Accepted`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "job"
    ///  ],
    ///  "properties": {
    ///    "job": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Accepted {
        pub job: ::std::string::String,
    }

    ///`Error`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Error {
        pub message: ::std::string::String,
    }

    ///`Readiness`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "ready"
    ///  ],
    ///  "properties": {
    ///    "ready": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Readiness {
        pub ready: bool,
    }

    ///`Thing`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Thing {
        pub name: ::std::string::String,
    }
}

pub mod response {
    ///Response types of `get_ready`.
    pub mod get_ready {
        #[allow(unused_imports)]
        use super::super::types;
        ///The error responses of `get_ready`, which carry more than one type.
        #[derive(Clone, Debug, serde :: Serialize)]
        #[serde(untagged)]
        pub enum Error {
            ///Answered with `500`.
            Status500(types::Error),
            ///Answered with `503`.
            Status503(types::Readiness),
        }
    }

    ///Response types of `put_thing`.
    pub mod put_thing {
        #[allow(unused_imports)]
        use super::super::types;
        ///The successful responses of `put_thing`, which carry more than one
        /// type.
        #[derive(Clone, Debug, serde :: Serialize)]
        #[serde(untagged)]
        pub enum Success {
            ///Answered with `200`.
            Status200(types::Thing),
            ///Answered with `202`.
            Status202(types::Accepted),
            ///Answered with `204`.
            Status204,
        }
    }
}

#[derive(Clone, Debug)]
///Client for multiple response types
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
            let dur = ::std::time::Duration::from_secs(15u64);
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
        "1.0.0"
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
    ///Sends a `GET` request to `/ready`
    pub async fn get_ready<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::Readiness>, Error<response::get_ready::Error>> {
        let url = format!("{}/ready", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_ready",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(
                ResponseValue::<types::Error>::from_response(response)
                    .await?
                    .map(response::get_ready::Error::Status500),
            )),
            503u16 => Err(Error::ErrorResponse(
                ResponseValue::<types::Readiness>::from_response(response)
                    .await?
                    .map(response::get_ready::Error::Status503),
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/thing`
    pub async fn put_thing<'a>(
        &'a self,
    ) -> Result<ResponseValue<response::put_thing::Success>, Error<()>> {
        let url = format!("{}/thing", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "put_thing",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::<types::Thing>::from_response(response)
                .await?
                .map(response::put_thing::Success::Status200)),
            202u16 => Ok(ResponseValue::<types::Accepted>::from_response(response)
                .await?
                .map(response::put_thing::Success::Status202)),
            204u16 => Ok(
                ResponseValue::empty(response).map(|()| response::put_thing::Success::Status204)
            ),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
