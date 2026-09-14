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

    ///`UploadFileResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct UploadFileResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&UploadFileResponse> for UploadFileResponse {
        fn from(value: &UploadFileResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for UploadFileResponse {
        fn default() -> Self {
            Self {
                status: Default::default(),
            }
        }
    }

    impl UploadFileResponse {
        pub fn builder() -> builder::UploadFileResponse {
            Default::default()
        }
    }

    ///`UploadFileUploadType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "multipart",
    ///  "type": "string",
    ///  "enum": [
    ///    "multipart"
    ///  ]
    ///}
    /// ```
    /// </details>
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
        schemars :: JsonSchema,
    )]
    pub enum UploadFileUploadType {
        #[serde(rename = "multipart")]
        Multipart,
    }

    impl ::std::convert::From<&Self> for UploadFileUploadType {
        fn from(value: &UploadFileUploadType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for UploadFileUploadType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Multipart => f.write_str("multipart"),
            }
        }
    }

    impl ::std::str::FromStr for UploadFileUploadType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "multipart" => Ok(Self::Multipart),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UploadFileUploadType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UploadFileUploadType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UploadFileUploadType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::default::Default for UploadFileUploadType {
        fn default() -> Self {
            UploadFileUploadType::Multipart
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct UploadFileResponse {
            status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for UploadFileResponse {
            fn default() -> Self {
                Self {
                    status: Ok(Default::default()),
                }
            }
        }

        impl UploadFileResponse {
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<UploadFileResponse> for super::UploadFileResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UploadFileResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::UploadFileResponse> for UploadFileResponse {
            fn from(value: super::UploadFileResponse) -> Self {
                Self {
                    status: Ok(value.status),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for Default Parameter Test API
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
impl Client {
    ///Upload a file
    ///
    ///Sends a `POST` request to `/upload`
    ///
    ///```ignore
    /// let response = client.upload_file()
    ///    .required_param(required_param)
    ///    .supports_all_drives(supports_all_drives)
    ///    .upload_type(upload_type)
    ///    .send()
    ///    .await;
    /// ```
    pub fn upload_file(&self) -> builder::UploadFile<'_> {
        builder::UploadFile::new(self)
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
    ///Builder for [`Client::upload_file`]
    ///
    ///[`Client::upload_file`]: super::Client::upload_file
    #[derive(Debug, Clone)]
    pub struct UploadFile<'a> {
        client: &'a super::Client,
        required_param: Result<::std::string::String, String>,
        supports_all_drives: Result<Option<bool>, String>,
        upload_type: Result<Option<types::UploadFileUploadType>, String>,
    }

    impl<'a> UploadFile<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                required_param: Err("required_param was not initialized".to_string()),
                supports_all_drives: Ok(None),
                upload_type: Ok(Some(::std::default::Default::default())),
            }
        }

        pub fn required_param<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.required_param = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for required_param failed".to_string()
            });
            self
        }

        pub fn supports_all_drives<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.supports_all_drives = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for supports_all_drives failed".to_string());
            self
        }

        pub fn upload_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UploadFileUploadType>,
        {
            self.upload_type = value.try_into().map(Some).map_err(|_| {
                "conversion to `UploadFileUploadType` for upload_type failed".to_string()
            });
            self
        }

        ///Sends a `POST` request to `/upload`
        pub async fn send(self) -> Result<ResponseValue<types::UploadFileResponse>, Error<()>> {
            let Self {
                client,
                required_param,
                supports_all_drives,
                upload_type,
            } = self;
            let required_param = required_param.map_err(Error::InvalidRequest)?;
            let supports_all_drives = supports_all_drives.map_err(Error::InvalidRequest)?;
            let upload_type = upload_type.map_err(Error::InvalidRequest)?;
            let url = format!("{}/upload", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new(
                    "requiredParam",
                    &required_param,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "supportsAllDrives",
                    &supports_all_drives,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "uploadType",
                    &upload_type,
                ))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "upload_file",
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

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
