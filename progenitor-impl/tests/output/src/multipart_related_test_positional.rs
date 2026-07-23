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

    ///`FileMetadata`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "mimeType",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "Optional description of the file",
    ///      "type": "string"
    ///    },
    ///    "mimeType": {
    ///      "description": "The MIME type of the file",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The name of the file",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FileMetadata {
        ///Optional description of the file
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///The MIME type of the file
        #[serde(rename = "mimeType")]
        pub mime_type: ::std::string::String,
        ///The name of the file
        pub name: ::std::string::String,
    }

    impl ::std::convert::From<&FileMetadata> for FileMetadata {
        fn from(value: &FileMetadata) -> Self {
            value.clone()
        }
    }

    ///`UploadFileMultipartParts`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "file",
    ///    "file_content_type",
    ///    "metadata"
    ///  ],
    ///  "properties": {
    ///    "file": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint8",
    ///        "maximum": 255.0,
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "file_content_type": {
    ///      "description": "MIME type for the file field",
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "$ref": "#/components/schemas/FileMetadata"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UploadFileMultipartParts {
        pub file: ::std::vec::Vec<u8>,
        ///MIME type for the file field
        pub file_content_type: ::std::string::String,
        pub metadata: FileMetadata,
    }

    impl ::std::convert::From<&UploadFileMultipartParts> for UploadFileMultipartParts {
        fn from(value: &UploadFileMultipartParts) -> Self {
            value.clone()
        }
    }

    ///`UploadMultipleFilesMultipartParts`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "document",
    ///    "document_content_type",
    ///    "metadata",
    ///    "thumbnail",
    ///    "thumbnail_content_type"
    ///  ],
    ///  "properties": {
    ///    "attachment": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint8",
    ///        "maximum": 255.0,
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "attachment_content_type": {
    ///      "description": "MIME type for the attachment field",
    ///      "type": "string"
    ///    },
    ///    "document": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint8",
    ///        "maximum": 255.0,
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "document_content_type": {
    ///      "description": "MIME type for the document field",
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "$ref": "#/components/schemas/FileMetadata"
    ///    },
    ///    "thumbnail": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint8",
    ///        "maximum": 255.0,
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "thumbnail_content_type": {
    ///      "description": "MIME type for the thumbnail field",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UploadMultipleFilesMultipartParts {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub attachment: ::std::vec::Vec<u8>,
        ///MIME type for the attachment field
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub attachment_content_type: ::std::option::Option<::std::string::String>,
        pub document: ::std::vec::Vec<u8>,
        ///MIME type for the document field
        pub document_content_type: ::std::string::String,
        pub metadata: FileMetadata,
        pub thumbnail: ::std::vec::Vec<u8>,
        ///MIME type for the thumbnail field
        pub thumbnail_content_type: ::std::string::String,
    }

    impl ::std::convert::From<&UploadMultipleFilesMultipartParts>
        for UploadMultipleFilesMultipartParts
    {
        fn from(value: &UploadMultipleFilesMultipartParts) -> Self {
            value.clone()
        }
    }

    ///`UploadResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name",
    ///    "size"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "The ID of the uploaded file",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "The name of the uploaded file",
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "description": "The size of the uploaded file in bytes",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UploadResponse {
        ///The ID of the uploaded file
        pub id: ::std::string::String,
        ///The name of the uploaded file
        pub name: ::std::string::String,
        ///The size of the uploaded file in bytes
        pub size: i64,
    }

    impl ::std::convert::From<&UploadResponse> for UploadResponse {
        fn from(value: &UploadResponse) -> Self {
            value.clone()
        }
    }

    impl crate::progenitor_client::MultipartRelatedBody for UploadFileMultipartParts {
        fn as_multipart_parts(&self) -> Vec<crate::progenitor_client::MultipartPart> {
            vec![
                Some(crate::progenitor_client::MultipartPart {
                    content_type: "application/json",
                    content_id: "metadata",
                    bytes: ::std::borrow::Cow::Owned(
                        ::serde_json::to_vec(&self.metadata).expect("failed to serialize field"),
                    ),
                }),
                Some(crate::progenitor_client::MultipartPart {
                    content_type: &self.file_content_type,
                    content_id: "file",
                    bytes: ::std::borrow::Cow::Borrowed(&self.file),
                }),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }

    impl crate::progenitor_client::MultipartRelatedBody for UploadMultipleFilesMultipartParts {
        fn as_multipart_parts(&self) -> Vec<crate::progenitor_client::MultipartPart> {
            vec![
                Some(crate::progenitor_client::MultipartPart {
                    content_type: "application/json",
                    content_id: "metadata",
                    bytes: ::std::borrow::Cow::Owned(
                        ::serde_json::to_vec(&self.metadata).expect("failed to serialize field"),
                    ),
                }),
                Some(crate::progenitor_client::MultipartPart {
                    content_type: &self.document_content_type,
                    content_id: "document",
                    bytes: ::std::borrow::Cow::Borrowed(&self.document),
                }),
                Some(crate::progenitor_client::MultipartPart {
                    content_type: &self.thumbnail_content_type,
                    content_id: "thumbnail",
                    bytes: ::std::borrow::Cow::Borrowed(&self.thumbnail),
                }),
                if let Some(ref content_type) = self.attachment_content_type {
                    if !self.attachment.is_empty() {
                        Some(crate::progenitor_client::MultipartPart {
                            content_type: content_type.as_str(),
                            content_id: "attachment",
                            bytes: ::std::borrow::Cow::Borrowed(&self.attachment),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                },
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }
}

#[derive(Clone, Debug)]
///Client for Multipart Related Test API
///
///Test API for multipart/related content type support
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
    ///Upload a file with metadata using multipart/related
    ///
    ///Uploads a file along with JSON metadata in a multipart/related request
    ///
    ///Sends a `POST` request to `/upload`
    pub async fn upload_file<'a>(
        &'a self,
        body: &'a types::UploadFileMultipartParts,
    ) -> Result<ResponseValue<types::UploadResponse>, Error<()>> {
        let url = format!("{}/upload", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .multipart_related(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "upload_file",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Simple upload using multipart/related
    ///
    ///Sends a `POST` request to `/upload-simple`
    pub async fn upload_simple<'a, B: Into<reqwest::Body>>(
        &'a self,
        body: B,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/upload-simple", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::CONTENT_TYPE,
                ::reqwest::header::HeaderValue::from_static("multipart/related"),
            )
            .body(body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "upload_simple",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Upload multiple files with metadata using multipart/related
    ///
    ///Uploads multiple files along with JSON metadata in a single
    /// multipart/related request
    ///
    ///Sends a `POST` request to `/upload-multiple`
    pub async fn upload_multiple_files<'a>(
        &'a self,
        body: &'a types::UploadMultipleFilesMultipartParts,
    ) -> Result<ResponseValue<types::UploadResponse>, Error<()>> {
        let url = format!("{}/upload-multiple", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .multipart_related(&body)?
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "upload_multiple_files",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
