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

    impl FileMetadata {
        pub fn builder() -> builder::FileMetadata {
            Default::default()
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

    impl UploadFileMultipartParts {
        pub fn builder() -> builder::UploadFileMultipartParts {
            Default::default()
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

    impl UploadMultipleFilesMultipartParts {
        pub fn builder() -> builder::UploadMultipleFilesMultipartParts {
            Default::default()
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

    impl UploadResponse {
        pub fn builder() -> builder::UploadResponse {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct FileMetadata {
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mime_type: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for FileMetadata {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    mime_type: Err("no value supplied for mime_type".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl FileMetadata {
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn mime_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.mime_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mime_type: {}", e));
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

        impl ::std::convert::TryFrom<FileMetadata> for super::FileMetadata {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FileMetadata,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    description: value.description?,
                    mime_type: value.mime_type?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::FileMetadata> for FileMetadata {
            fn from(value: super::FileMetadata) -> Self {
                Self {
                    description: Ok(value.description),
                    mime_type: Ok(value.mime_type),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UploadFileMultipartParts {
            file: ::std::result::Result<::std::vec::Vec<u8>, ::std::string::String>,
            file_content_type: ::std::result::Result<::std::string::String, ::std::string::String>,
            metadata: ::std::result::Result<super::FileMetadata, ::std::string::String>,
        }

        impl ::std::default::Default for UploadFileMultipartParts {
            fn default() -> Self {
                Self {
                    file: Err("no value supplied for file".to_string()),
                    file_content_type: Err("no value supplied for file_content_type".to_string()),
                    metadata: Err("no value supplied for metadata".to_string()),
                }
            }
        }

        impl UploadFileMultipartParts {
            pub fn file<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<u8>>,
                T::Error: ::std::fmt::Display,
            {
                self.file = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for file: {}", e));
                self
            }
            pub fn file_content_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.file_content_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for file_content_type: {}",
                        e
                    )
                });
                self
            }
            pub fn metadata<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::FileMetadata>,
                T::Error: ::std::fmt::Display,
            {
                self.metadata = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for metadata: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<UploadFileMultipartParts> for super::UploadFileMultipartParts {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UploadFileMultipartParts,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    file: value.file?,
                    file_content_type: value.file_content_type?,
                    metadata: value.metadata?,
                })
            }
        }

        impl ::std::convert::From<super::UploadFileMultipartParts> for UploadFileMultipartParts {
            fn from(value: super::UploadFileMultipartParts) -> Self {
                Self {
                    file: Ok(value.file),
                    file_content_type: Ok(value.file_content_type),
                    metadata: Ok(value.metadata),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UploadMultipleFilesMultipartParts {
            attachment: ::std::result::Result<::std::vec::Vec<u8>, ::std::string::String>,
            attachment_content_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            document: ::std::result::Result<::std::vec::Vec<u8>, ::std::string::String>,
            document_content_type:
                ::std::result::Result<::std::string::String, ::std::string::String>,
            metadata: ::std::result::Result<super::FileMetadata, ::std::string::String>,
            thumbnail: ::std::result::Result<::std::vec::Vec<u8>, ::std::string::String>,
            thumbnail_content_type:
                ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for UploadMultipleFilesMultipartParts {
            fn default() -> Self {
                Self {
                    attachment: Ok(Default::default()),
                    attachment_content_type: Ok(Default::default()),
                    document: Err("no value supplied for document".to_string()),
                    document_content_type: Err(
                        "no value supplied for document_content_type".to_string()
                    ),
                    metadata: Err("no value supplied for metadata".to_string()),
                    thumbnail: Err("no value supplied for thumbnail".to_string()),
                    thumbnail_content_type: Err(
                        "no value supplied for thumbnail_content_type".to_string()
                    ),
                }
            }
        }

        impl UploadMultipleFilesMultipartParts {
            pub fn attachment<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<u8>>,
                T::Error: ::std::fmt::Display,
            {
                self.attachment = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attachment: {}", e));
                self
            }
            pub fn attachment_content_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.attachment_content_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for attachment_content_type: {}",
                        e
                    )
                });
                self
            }
            pub fn document<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<u8>>,
                T::Error: ::std::fmt::Display,
            {
                self.document = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for document: {}", e));
                self
            }
            pub fn document_content_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.document_content_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for document_content_type: {}",
                        e
                    )
                });
                self
            }
            pub fn metadata<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::FileMetadata>,
                T::Error: ::std::fmt::Display,
            {
                self.metadata = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for metadata: {}", e));
                self
            }
            pub fn thumbnail<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<u8>>,
                T::Error: ::std::fmt::Display,
            {
                self.thumbnail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for thumbnail: {}", e));
                self
            }
            pub fn thumbnail_content_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.thumbnail_content_type = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for thumbnail_content_type: {}",
                        e
                    )
                });
                self
            }
        }

        impl ::std::convert::TryFrom<UploadMultipleFilesMultipartParts>
            for super::UploadMultipleFilesMultipartParts
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UploadMultipleFilesMultipartParts,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    attachment: value.attachment?,
                    attachment_content_type: value.attachment_content_type?,
                    document: value.document?,
                    document_content_type: value.document_content_type?,
                    metadata: value.metadata?,
                    thumbnail: value.thumbnail?,
                    thumbnail_content_type: value.thumbnail_content_type?,
                })
            }
        }

        impl ::std::convert::From<super::UploadMultipleFilesMultipartParts>
            for UploadMultipleFilesMultipartParts
        {
            fn from(value: super::UploadMultipleFilesMultipartParts) -> Self {
                Self {
                    attachment: Ok(value.attachment),
                    attachment_content_type: Ok(value.attachment_content_type),
                    document: Ok(value.document),
                    document_content_type: Ok(value.document_content_type),
                    metadata: Ok(value.metadata),
                    thumbnail: Ok(value.thumbnail),
                    thumbnail_content_type: Ok(value.thumbnail_content_type),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UploadResponse {
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            size: ::std::result::Result<i64, ::std::string::String>,
        }

        impl ::std::default::Default for UploadResponse {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    size: Err("no value supplied for size".to_string()),
                }
            }
        }

        impl UploadResponse {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
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
            pub fn size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i64>,
                T::Error: ::std::fmt::Display,
            {
                self.size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<UploadResponse> for super::UploadResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UploadResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    name: value.name?,
                    size: value.size?,
                })
            }
        }

        impl ::std::convert::From<super::UploadResponse> for UploadResponse {
            fn from(value: super::UploadResponse) -> Self {
                Self {
                    id: Ok(value.id),
                    name: Ok(value.name),
                    size: Ok(value.size),
                }
            }
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
impl Client {
    ///Upload a file with metadata using multipart/related
    ///
    ///Uploads a file along with JSON metadata in a multipart/related request
    ///
    ///Sends a `POST` request to `/upload`
    ///
    ///```ignore
    /// let response = client.upload_file()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn upload_file(&self) -> builder::UploadFile<'_> {
        builder::UploadFile::new(self)
    }

    ///Simple upload using multipart/related
    ///
    ///Sends a `POST` request to `/upload-simple`
    ///
    ///```ignore
    /// let response = client.upload_simple()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn upload_simple(&self) -> builder::UploadSimple<'_> {
        builder::UploadSimple::new(self)
    }

    ///Upload multiple files with metadata using multipart/related
    ///
    ///Uploads multiple files along with JSON metadata in a single
    /// multipart/related request
    ///
    ///Sends a `POST` request to `/upload-multiple`
    ///
    ///```ignore
    /// let response = client.upload_multiple_files()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn upload_multiple_files(&self) -> builder::UploadMultipleFiles<'_> {
        builder::UploadMultipleFiles::new(self)
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
        file: Result<Vec<u8>, String>,
        metadata: Result<types::FileMetadata, String>,
        file_content_type: Result<String, String>,
    }

    impl<'a> UploadFile<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                file: Err("file was not initialized".to_string()),
                metadata: Err("metadata was not initialized".to_string()),
                file_content_type: Err("file_content_type was not initialized".to_string()),
            }
        }

        pub fn file<V, C>(mut self, value: V, content_type: C) -> Self
        where
            V: std::convert::TryInto<Vec<u8>>,
            C: std::convert::TryInto<String>,
        {
            self.file = value
                .try_into()
                .map_err(|_| "conversion to Vec<u8> for file failed".to_string());
            self.file_content_type = content_type
                .try_into()
                .map_err(|_| "conversion to String for file_content_type failed".to_string());
            self
        }

        pub fn metadata<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::FileMetadata>,
        {
            self.metadata = value
                .try_into()
                .map_err(|_| "conversion to `FileMetadata` for metadata failed".to_string());
            self
        }

        ///Sends a `POST` request to `/upload`
        pub async fn send(self) -> Result<ResponseValue<types::UploadResponse>, Error<()>> {
            let Self {
                client,
                file,
                metadata,
                file_content_type,
            } = self;
            let file = file.map_err(Error::InvalidRequest)?;
            let metadata = metadata.map_err(Error::InvalidRequest)?;
            let file_content_type = file_content_type.map_err(Error::InvalidRequest)?;
            let body = types::UploadFileMultipartParts {
                file,
                file_content_type,
                metadata,
            };
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
                .multipart_related(&body)?
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
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::upload_simple`]
    ///
    ///[`Client::upload_simple`]: super::Client::upload_simple
    #[derive(Debug)]
    pub struct UploadSimple<'a> {
        client: &'a super::Client,
        body: Result<reqwest::Body, String>,
    }

    impl<'a> UploadSimple<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn body<B>(mut self, value: B) -> Self
        where
            B: std::convert::TryInto<reqwest::Body>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `reqwest::Body` for body failed".to_string());
            self
        }

        ///Sends a `POST` request to `/upload-simple`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client, body } = self;
            let body = body.map_err(Error::InvalidRequest)?;
            let url = format!("{}/upload-simple", client.baseurl,);
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
                    ::reqwest::header::CONTENT_TYPE,
                    ::reqwest::header::HeaderValue::from_static("multipart/related"),
                )
                .body(body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "upload_simple",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::upload_multiple_files`]
    ///
    ///[`Client::upload_multiple_files`]: super::Client::upload_multiple_files
    #[derive(Debug, Clone)]
    pub struct UploadMultipleFiles<'a> {
        client: &'a super::Client,
        attachment: Result<Vec<u8>, String>,
        document: Result<Vec<u8>, String>,
        metadata: Result<types::FileMetadata, String>,
        thumbnail: Result<Vec<u8>, String>,
        attachment_content_type: Result<String, String>,
        document_content_type: Result<String, String>,
        thumbnail_content_type: Result<String, String>,
    }

    impl<'a> UploadMultipleFiles<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                attachment: Err("attachment was not initialized".to_string()),
                document: Err("document was not initialized".to_string()),
                metadata: Err("metadata was not initialized".to_string()),
                thumbnail: Err("thumbnail was not initialized".to_string()),
                attachment_content_type: Err(
                    "attachment_content_type was not initialized".to_string()
                ),
                document_content_type: Err("document_content_type was not initialized".to_string()),
                thumbnail_content_type: Err(
                    "thumbnail_content_type was not initialized".to_string()
                ),
            }
        }

        pub fn attachment<V, C>(mut self, value: V, content_type: C) -> Self
        where
            V: std::convert::TryInto<Vec<u8>>,
            C: std::convert::TryInto<String>,
        {
            self.attachment = value
                .try_into()
                .map_err(|_| "conversion to Vec<u8> for attachment failed".to_string());
            self.attachment_content_type = content_type
                .try_into()
                .map_err(|_| "conversion to String for attachment_content_type failed".to_string());
            self
        }

        pub fn document<V, C>(mut self, value: V, content_type: C) -> Self
        where
            V: std::convert::TryInto<Vec<u8>>,
            C: std::convert::TryInto<String>,
        {
            self.document = value
                .try_into()
                .map_err(|_| "conversion to Vec<u8> for document failed".to_string());
            self.document_content_type = content_type
                .try_into()
                .map_err(|_| "conversion to String for document_content_type failed".to_string());
            self
        }

        pub fn metadata<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::FileMetadata>,
        {
            self.metadata = value
                .try_into()
                .map_err(|_| "conversion to `FileMetadata` for metadata failed".to_string());
            self
        }

        pub fn thumbnail<V, C>(mut self, value: V, content_type: C) -> Self
        where
            V: std::convert::TryInto<Vec<u8>>,
            C: std::convert::TryInto<String>,
        {
            self.thumbnail = value
                .try_into()
                .map_err(|_| "conversion to Vec<u8> for thumbnail failed".to_string());
            self.thumbnail_content_type = content_type
                .try_into()
                .map_err(|_| "conversion to String for thumbnail_content_type failed".to_string());
            self
        }

        ///Sends a `POST` request to `/upload-multiple`
        pub async fn send(self) -> Result<ResponseValue<types::UploadResponse>, Error<()>> {
            let Self {
                client,
                attachment,
                document,
                metadata,
                thumbnail,
                attachment_content_type,
                document_content_type,
                thumbnail_content_type,
            } = self;
            let attachment = attachment.map_err(Error::InvalidRequest)?;
            let document = document.map_err(Error::InvalidRequest)?;
            let metadata = metadata.map_err(Error::InvalidRequest)?;
            let thumbnail = thumbnail.map_err(Error::InvalidRequest)?;
            let attachment_content_type = attachment_content_type.map_err(Error::InvalidRequest)?;
            let document_content_type = document_content_type.map_err(Error::InvalidRequest)?;
            let thumbnail_content_type = thumbnail_content_type.map_err(Error::InvalidRequest)?;
            let body = types::UploadMultipleFilesMultipartParts {
                attachment,
                attachment_content_type,
                document,
                document_content_type,
                metadata,
                thumbnail,
                thumbnail_content_type,
            };
            let url = format!("{}/upload-multiple", client.baseurl,);
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
                .multipart_related(&body)?
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "upload_multiple_files",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
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
