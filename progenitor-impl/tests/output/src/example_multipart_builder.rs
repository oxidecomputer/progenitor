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

    ///`UploadForm`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "file",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "file": {
    ///      "type": "string",
    ///      "format": "binary"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct UploadForm {
        pub file: ::std::string::String,
        pub name: ::std::string::String,
    }

    impl ::std::convert::From<&UploadForm> for UploadForm {
        fn from(value: &UploadForm) -> Self {
            value.clone()
        }
    }

    impl UploadForm {
        pub fn builder() -> builder::UploadForm {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct UploadForm {
            file: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for UploadForm {
            fn default() -> Self {
                Self {
                    file: Err("no value supplied for file".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl UploadForm {
            pub fn file<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.file = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for file: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<UploadForm> for super::UploadForm {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UploadForm,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    file: value.file?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::UploadForm> for UploadForm {
            fn from(value: super::UploadForm) -> Self {
                Self {
                    file: Ok(value.file),
                    name: Ok(value.name),
                }
            }
        }
    }

    impl UploadForm {
        /// Convert this form into an iterator of (field_name, field_value)
        /// pairs
        /// suitable for multipart/form-data encoding.
        pub fn as_form(&self) -> Vec<(&'static str, progenitor_client::FormPart)> {
            let mut parts = Vec::new();
            if let Some(ref val) = self.file {
                parts.push((
                    "file",
                    progenitor_client::FormPart::Binary(progenitor_client::BinaryFormPart {
                        data: val.clone().into(),
                        filename: None,
                        content_type: None,
                    }),
                ));
            }
            if let Some(ref val) = self.name {
                parts.push((
                    "name",
                    progenitor_client::FormPart::Text(progenitor_client::TextFormPart {
                        value: val.to_string(),
                        content_type: None,
                    }),
                ));
            }
            parts
        }
    }
}

#[derive(Clone, Debug)]
///Client for Multipart Example
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
    ///Sends a `POST` request to `/upload`
    ///
    ///```ignore
    /// let response = client.upload()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn upload(&self) -> builder::Upload<'_> {
        builder::Upload::new(self)
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
    ///Builder for [`Client::upload`]
    ///
    ///[`Client::upload`]: super::Client::upload
    #[derive(Debug, Clone)]
    pub struct Upload<'a> {
        client: &'a super::Client,
        body: Result<types::builder::UploadForm, String>,
    }

    impl<'a> Upload<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UploadForm>,
            <V as std::convert::TryInto<types::UploadForm>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `UploadForm` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::UploadForm) -> types::builder::UploadForm,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/upload`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::UploadForm::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
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
                .form_from_parts(body.as_form())?
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "upload",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
