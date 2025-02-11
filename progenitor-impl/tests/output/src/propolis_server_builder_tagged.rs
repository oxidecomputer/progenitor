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

    ///`CrucibleOpts`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "lossy",
    ///    "read_only",
    ///    "target"
    ///  ],
    ///  "properties": {
    ///    "cert_pem": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "control": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "flush_timeout": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "key": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "key_pem": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "lossy": {
    ///      "type": "boolean"
    ///    },
    ///    "read_only": {
    ///      "type": "boolean"
    ///    },
    ///    "root_cert_pem": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "target": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CrucibleOpts {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cert_pem: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub control: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub flush_timeout: ::std::option::Option<u32>,
        pub id: ::uuid::Uuid,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub key_pem: ::std::option::Option<::std::string::String>,
        pub lossy: bool,
        pub read_only: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub root_cert_pem: ::std::option::Option<::std::string::String>,
        pub target: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::convert::From<&CrucibleOpts> for CrucibleOpts {
        fn from(value: &CrucibleOpts) -> Self {
            value.clone()
        }
    }

    impl CrucibleOpts {
        pub fn builder() -> builder::CrucibleOpts {
            Default::default()
        }
    }

    ///`DiskAttachment`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "disk_id",
    ///    "generation_id",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "disk_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "generation_id": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/DiskAttachmentState"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskAttachment {
        pub disk_id: ::uuid::Uuid,
        pub generation_id: u64,
        pub state: DiskAttachmentState,
    }

    impl ::std::convert::From<&DiskAttachment> for DiskAttachment {
        fn from(value: &DiskAttachment) -> Self {
            value.clone()
        }
    }

    impl DiskAttachment {
        pub fn builder() -> builder::DiskAttachment {
            Default::default()
        }
    }

    ///`DiskAttachmentState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "Detached",
    ///        "Destroyed",
    ///        "Faulted"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Attached"
    ///      ],
    ///      "properties": {
    ///        "Attached": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum DiskAttachmentState {
        Detached,
        Destroyed,
        Faulted,
        Attached(::uuid::Uuid),
    }

    impl ::std::convert::From<&Self> for DiskAttachmentState {
        fn from(value: &DiskAttachmentState) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for DiskAttachmentState {
        fn from(value: ::uuid::Uuid) -> Self {
            Self::Attached(value)
        }
    }

    ///`DiskRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "device",
    ///    "gen",
    ///    "name",
    ///    "read_only",
    ///    "slot",
    ///    "volume_construction_request"
    ///  ],
    ///  "properties": {
    ///    "device": {
    ///      "type": "string"
    ///    },
    ///    "gen": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "read_only": {
    ///      "type": "boolean"
    ///    },
    ///    "slot": {
    ///      "$ref": "#/components/schemas/Slot"
    ///    },
    ///    "volume_construction_request": {
    ///      "$ref": "#/components/schemas/VolumeConstructionRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskRequest {
        pub device: ::std::string::String,
        pub gen: u64,
        pub name: ::std::string::String,
        pub read_only: bool,
        pub slot: Slot,
        pub volume_construction_request: VolumeConstructionRequest,
    }

    impl ::std::convert::From<&DiskRequest> for DiskRequest {
        fn from(value: &DiskRequest) -> Self {
            value.clone()
        }
    }

    impl DiskRequest {
        pub fn builder() -> builder::DiskRequest {
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error_code: ::std::option::Option<::std::string::String>,
        pub message: ::std::string::String,
        pub request_id: ::std::string::String,
    }

    impl ::std::convert::From<&Error> for Error {
        fn from(value: &Error) -> Self {
            value.clone()
        }
    }

    impl Error {
        pub fn builder() -> builder::Error {
            Default::default()
        }
    }

    ///`Instance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "disks",
    ///    "nics",
    ///    "properties",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "disks": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DiskAttachment"
    ///      }
    ///    },
    ///    "nics": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/NetworkInterface"
    ///      }
    ///    },
    ///    "properties": {
    ///      "$ref": "#/components/schemas/InstanceProperties"
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/InstanceState"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Instance {
        pub disks: ::std::vec::Vec<DiskAttachment>,
        pub nics: ::std::vec::Vec<NetworkInterface>,
        pub properties: InstanceProperties,
        pub state: InstanceState,
    }

    impl ::std::convert::From<&Instance> for Instance {
        fn from(value: &Instance) -> Self {
            value.clone()
        }
    }

    impl Instance {
        pub fn builder() -> builder::Instance {
            Default::default()
        }
    }

    ///`InstanceEnsureRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "properties"
    ///  ],
    ///  "properties": {
    ///    "cloud_init_bytes": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "disks": {
    ///      "default": [],
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DiskRequest"
    ///      }
    ///    },
    ///    "migrate": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/InstanceMigrateInitiateRequest"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "nics": {
    ///      "default": [],
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/NetworkInterfaceRequest"
    ///      }
    ///    },
    ///    "properties": {
    ///      "$ref": "#/components/schemas/InstanceProperties"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceEnsureRequest {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cloud_init_bytes: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub disks: ::std::vec::Vec<DiskRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub migrate: ::std::option::Option<InstanceMigrateInitiateRequest>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub nics: ::std::vec::Vec<NetworkInterfaceRequest>,
        pub properties: InstanceProperties,
    }

    impl ::std::convert::From<&InstanceEnsureRequest> for InstanceEnsureRequest {
        fn from(value: &InstanceEnsureRequest) -> Self {
            value.clone()
        }
    }

    impl InstanceEnsureRequest {
        pub fn builder() -> builder::InstanceEnsureRequest {
            Default::default()
        }
    }

    ///`InstanceEnsureResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "migrate": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/InstanceMigrateInitiateResponse"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceEnsureResponse {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub migrate: ::std::option::Option<InstanceMigrateInitiateResponse>,
    }

    impl ::std::convert::From<&InstanceEnsureResponse> for InstanceEnsureResponse {
        fn from(value: &InstanceEnsureResponse) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for InstanceEnsureResponse {
        fn default() -> Self {
            Self {
                migrate: Default::default(),
            }
        }
    }

    impl InstanceEnsureResponse {
        pub fn builder() -> builder::InstanceEnsureResponse {
            Default::default()
        }
    }

    ///`InstanceGetResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "instance"
    ///  ],
    ///  "properties": {
    ///    "instance": {
    ///      "$ref": "#/components/schemas/Instance"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceGetResponse {
        pub instance: Instance,
    }

    impl ::std::convert::From<&InstanceGetResponse> for InstanceGetResponse {
        fn from(value: &InstanceGetResponse) -> Self {
            value.clone()
        }
    }

    impl InstanceGetResponse {
        pub fn builder() -> builder::InstanceGetResponse {
            Default::default()
        }
    }

    ///`InstanceMigrateInitiateRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "migration_id",
    ///    "src_addr",
    ///    "src_uuid"
    ///  ],
    ///  "properties": {
    ///    "migration_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "src_addr": {
    ///      "type": "string"
    ///    },
    ///    "src_uuid": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrateInitiateRequest {
        pub migration_id: ::uuid::Uuid,
        pub src_addr: ::std::string::String,
        pub src_uuid: ::uuid::Uuid,
    }

    impl ::std::convert::From<&InstanceMigrateInitiateRequest> for InstanceMigrateInitiateRequest {
        fn from(value: &InstanceMigrateInitiateRequest) -> Self {
            value.clone()
        }
    }

    impl InstanceMigrateInitiateRequest {
        pub fn builder() -> builder::InstanceMigrateInitiateRequest {
            Default::default()
        }
    }

    ///`InstanceMigrateInitiateResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "migration_id"
    ///  ],
    ///  "properties": {
    ///    "migration_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrateInitiateResponse {
        pub migration_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&InstanceMigrateInitiateResponse> for InstanceMigrateInitiateResponse {
        fn from(value: &InstanceMigrateInitiateResponse) -> Self {
            value.clone()
        }
    }

    impl InstanceMigrateInitiateResponse {
        pub fn builder() -> builder::InstanceMigrateInitiateResponse {
            Default::default()
        }
    }

    ///`InstanceMigrateStatusRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "migration_id"
    ///  ],
    ///  "properties": {
    ///    "migration_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrateStatusRequest {
        pub migration_id: ::uuid::Uuid,
    }

    impl ::std::convert::From<&InstanceMigrateStatusRequest> for InstanceMigrateStatusRequest {
        fn from(value: &InstanceMigrateStatusRequest) -> Self {
            value.clone()
        }
    }

    impl InstanceMigrateStatusRequest {
        pub fn builder() -> builder::InstanceMigrateStatusRequest {
            Default::default()
        }
    }

    ///`InstanceMigrateStatusResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "state": {
    ///      "$ref": "#/components/schemas/MigrationState"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrateStatusResponse {
        pub state: MigrationState,
    }

    impl ::std::convert::From<&InstanceMigrateStatusResponse> for InstanceMigrateStatusResponse {
        fn from(value: &InstanceMigrateStatusResponse) -> Self {
            value.clone()
        }
    }

    impl InstanceMigrateStatusResponse {
        pub fn builder() -> builder::InstanceMigrateStatusResponse {
            Default::default()
        }
    }

    ///`InstanceProperties`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "bootrom_id",
    ///    "description",
    ///    "id",
    ///    "image_id",
    ///    "memory",
    ///    "name",
    ///    "vcpus"
    ///  ],
    ///  "properties": {
    ///    "bootrom_id": {
    ///      "description": "ID of the bootrom used to initialize this
    /// Instance.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "description": {
    ///      "description": "Free-form text description of an Instance.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Unique identifier for this Instance.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "image_id": {
    ///      "description": "ID of the image used to initialize this Instance.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "memory": {
    ///      "description": "Size of memory allocated to the Instance, in MiB.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "name": {
    ///      "description": "Human-readable name of the Instance.",
    ///      "type": "string"
    ///    },
    ///    "vcpus": {
    ///      "description": "Number of vCPUs to be allocated to the Instance.",
    ///      "type": "integer",
    ///      "format": "uint8",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceProperties {
        ///ID of the bootrom used to initialize this Instance.
        pub bootrom_id: ::uuid::Uuid,
        ///Free-form text description of an Instance.
        pub description: ::std::string::String,
        ///Unique identifier for this Instance.
        pub id: ::uuid::Uuid,
        ///ID of the image used to initialize this Instance.
        pub image_id: ::uuid::Uuid,
        ///Size of memory allocated to the Instance, in MiB.
        pub memory: u64,
        ///Human-readable name of the Instance.
        pub name: ::std::string::String,
        ///Number of vCPUs to be allocated to the Instance.
        pub vcpus: u8,
    }

    impl ::std::convert::From<&InstanceProperties> for InstanceProperties {
        fn from(value: &InstanceProperties) -> Self {
            value.clone()
        }
    }

    impl InstanceProperties {
        pub fn builder() -> builder::InstanceProperties {
            Default::default()
        }
    }

    ///Current state of an Instance.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Current state of an Instance.",
    ///  "type": "string",
    ///  "enum": [
    ///    "Creating",
    ///    "Starting",
    ///    "Running",
    ///    "Stopping",
    ///    "Stopped",
    ///    "Rebooting",
    ///    "Migrating",
    ///    "Repairing",
    ///    "Failed",
    ///    "Destroyed"
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
    )]
    pub enum InstanceState {
        Creating,
        Starting,
        Running,
        Stopping,
        Stopped,
        Rebooting,
        Migrating,
        Repairing,
        Failed,
        Destroyed,
    }

    impl ::std::convert::From<&Self> for InstanceState {
        fn from(value: &InstanceState) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for InstanceState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Creating => write!(f, "Creating"),
                Self::Starting => write!(f, "Starting"),
                Self::Running => write!(f, "Running"),
                Self::Stopping => write!(f, "Stopping"),
                Self::Stopped => write!(f, "Stopped"),
                Self::Rebooting => write!(f, "Rebooting"),
                Self::Migrating => write!(f, "Migrating"),
                Self::Repairing => write!(f, "Repairing"),
                Self::Failed => write!(f, "Failed"),
                Self::Destroyed => write!(f, "Destroyed"),
            }
        }
    }

    impl ::std::str::FromStr for InstanceState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Creating" => Ok(Self::Creating),
                "Starting" => Ok(Self::Starting),
                "Running" => Ok(Self::Running),
                "Stopping" => Ok(Self::Stopping),
                "Stopped" => Ok(Self::Stopped),
                "Rebooting" => Ok(Self::Rebooting),
                "Migrating" => Ok(Self::Migrating),
                "Repairing" => Ok(Self::Repairing),
                "Failed" => Ok(Self::Failed),
                "Destroyed" => Ok(Self::Destroyed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstanceState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstanceState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstanceState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`InstanceStateMonitorRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "gen"
    ///  ],
    ///  "properties": {
    ///    "gen": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceStateMonitorRequest {
        pub gen: u64,
    }

    impl ::std::convert::From<&InstanceStateMonitorRequest> for InstanceStateMonitorRequest {
        fn from(value: &InstanceStateMonitorRequest) -> Self {
            value.clone()
        }
    }

    impl InstanceStateMonitorRequest {
        pub fn builder() -> builder::InstanceStateMonitorRequest {
            Default::default()
        }
    }

    ///`InstanceStateMonitorResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "gen",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "gen": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/InstanceState"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceStateMonitorResponse {
        pub gen: u64,
        pub state: InstanceState,
    }

    impl ::std::convert::From<&InstanceStateMonitorResponse> for InstanceStateMonitorResponse {
        fn from(value: &InstanceStateMonitorResponse) -> Self {
            value.clone()
        }
    }

    impl InstanceStateMonitorResponse {
        pub fn builder() -> builder::InstanceStateMonitorResponse {
            Default::default()
        }
    }

    ///`InstanceStateRequested`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "Run",
    ///    "Stop",
    ///    "Reboot",
    ///    "MigrateStart"
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
    )]
    pub enum InstanceStateRequested {
        Run,
        Stop,
        Reboot,
        MigrateStart,
    }

    impl ::std::convert::From<&Self> for InstanceStateRequested {
        fn from(value: &InstanceStateRequested) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for InstanceStateRequested {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Run => write!(f, "Run"),
                Self::Stop => write!(f, "Stop"),
                Self::Reboot => write!(f, "Reboot"),
                Self::MigrateStart => write!(f, "MigrateStart"),
            }
        }
    }

    impl ::std::str::FromStr for InstanceStateRequested {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Run" => Ok(Self::Run),
                "Stop" => Ok(Self::Stop),
                "Reboot" => Ok(Self::Reboot),
                "MigrateStart" => Ok(Self::MigrateStart),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstanceStateRequested {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstanceStateRequested {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstanceStateRequested {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`MigrationState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "Sync",
    ///    "RamPush",
    ///    "Pause",
    ///    "RamPushDirty",
    ///    "Device",
    ///    "Arch",
    ///    "Resume",
    ///    "RamPull",
    ///    "Finish",
    ///    "Error"
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
    )]
    pub enum MigrationState {
        Sync,
        RamPush,
        Pause,
        RamPushDirty,
        Device,
        Arch,
        Resume,
        RamPull,
        Finish,
        Error,
    }

    impl ::std::convert::From<&Self> for MigrationState {
        fn from(value: &MigrationState) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for MigrationState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Sync => write!(f, "Sync"),
                Self::RamPush => write!(f, "RamPush"),
                Self::Pause => write!(f, "Pause"),
                Self::RamPushDirty => write!(f, "RamPushDirty"),
                Self::Device => write!(f, "Device"),
                Self::Arch => write!(f, "Arch"),
                Self::Resume => write!(f, "Resume"),
                Self::RamPull => write!(f, "RamPull"),
                Self::Finish => write!(f, "Finish"),
                Self::Error => write!(f, "Error"),
            }
        }
    }

    impl ::std::str::FromStr for MigrationState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Sync" => Ok(Self::Sync),
                "RamPush" => Ok(Self::RamPush),
                "Pause" => Ok(Self::Pause),
                "RamPushDirty" => Ok(Self::RamPushDirty),
                "Device" => Ok(Self::Device),
                "Arch" => Ok(Self::Arch),
                "Resume" => Ok(Self::Resume),
                "RamPull" => Ok(Self::RamPull),
                "Finish" => Ok(Self::Finish),
                "Error" => Ok(Self::Error),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MigrationState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MigrationState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MigrationState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`NetworkInterface`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "attachment",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "attachment": {
    ///      "$ref": "#/components/schemas/NetworkInterfaceAttachmentState"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterface {
        pub attachment: NetworkInterfaceAttachmentState,
        pub name: ::std::string::String,
    }

    impl ::std::convert::From<&NetworkInterface> for NetworkInterface {
        fn from(value: &NetworkInterface) -> Self {
            value.clone()
        }
    }

    impl NetworkInterface {
        pub fn builder() -> builder::NetworkInterface {
            Default::default()
        }
    }

    ///`NetworkInterfaceAttachmentState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "Detached",
    ///        "Faulted"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Attached"
    ///      ],
    ///      "properties": {
    ///        "Attached": {
    ///          "$ref": "#/components/schemas/Slot"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum NetworkInterfaceAttachmentState {
        Detached,
        Faulted,
        Attached(Slot),
    }

    impl ::std::convert::From<&Self> for NetworkInterfaceAttachmentState {
        fn from(value: &NetworkInterfaceAttachmentState) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<Slot> for NetworkInterfaceAttachmentState {
        fn from(value: Slot) -> Self {
            Self::Attached(value)
        }
    }

    ///`NetworkInterfaceRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "slot"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "slot": {
    ///      "$ref": "#/components/schemas/Slot"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterfaceRequest {
        pub name: ::std::string::String,
        pub slot: Slot,
    }

    impl ::std::convert::From<&NetworkInterfaceRequest> for NetworkInterfaceRequest {
        fn from(value: &NetworkInterfaceRequest) -> Self {
            value.clone()
        }
    }

    impl NetworkInterfaceRequest {
        pub fn builder() -> builder::NetworkInterfaceRequest {
            Default::default()
        }
    }

    ///A stable index which is translated by Propolis into a PCI BDF, visible
    /// to the guest.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A stable index which is translated by Propolis into a
    /// PCI BDF, visible to the guest.",
    ///  "type": "integer",
    ///  "format": "uint8",
    ///  "minimum": 0.0
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct Slot(pub u8);
    impl ::std::ops::Deref for Slot {
        type Target = u8;
        fn deref(&self) -> &u8 {
            &self.0
        }
    }

    impl ::std::convert::From<Slot> for u8 {
        fn from(value: Slot) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Slot> for Slot {
        fn from(value: &Slot) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<u8> for Slot {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for Slot {
        type Err = <u8 as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for Slot {
        type Error = <u8 as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Slot {
        type Error = <u8 as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Slot {
        type Error = <u8 as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for Slot {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///`VolumeConstructionRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "block_size",
    ///        "id",
    ///        "sub_volumes",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "block_size": {
    ///          "type": "integer",
    ///          "format": "uint64",
    ///          "minimum": 0.0
    ///        },
    ///        "id": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "read_only_parent": {
    ///          "oneOf": [
    ///            {
    ///              "type": "null"
    ///            },
    ///            {
    ///              "allOf": [
    ///                {
    ///                  "$ref":
    /// "#/components/schemas/VolumeConstructionRequest"
    ///                }
    ///              ]
    ///            }
    ///          ]
    ///        },
    ///        "sub_volumes": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/VolumeConstructionRequest"
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "volume"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "block_size",
    ///        "id",
    ///        "type",
    ///        "url"
    ///      ],
    ///      "properties": {
    ///        "block_size": {
    ///          "type": "integer",
    ///          "format": "uint64",
    ///          "minimum": 0.0
    ///        },
    ///        "id": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "url"
    ///          ]
    ///        },
    ///        "url": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "block_size",
    ///        "gen",
    ///        "opts",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "block_size": {
    ///          "type": "integer",
    ///          "format": "uint64",
    ///          "minimum": 0.0
    ///        },
    ///        "gen": {
    ///          "type": "integer",
    ///          "format": "uint64",
    ///          "minimum": 0.0
    ///        },
    ///        "opts": {
    ///          "$ref": "#/components/schemas/CrucibleOpts"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "region"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "block_size",
    ///        "id",
    ///        "path",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "block_size": {
    ///          "type": "integer",
    ///          "format": "uint64",
    ///          "minimum": 0.0
    ///        },
    ///        "id": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "path": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "file"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type")]
    pub enum VolumeConstructionRequest {
        #[serde(rename = "volume")]
        Volume {
            block_size: u64,
            id: ::uuid::Uuid,
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            read_only_parent: ::std::option::Option<::std::boxed::Box<VolumeConstructionRequest>>,
            sub_volumes: ::std::vec::Vec<VolumeConstructionRequest>,
        },
        #[serde(rename = "url")]
        Url {
            block_size: u64,
            id: ::uuid::Uuid,
            url: ::std::string::String,
        },
        #[serde(rename = "region")]
        Region {
            block_size: u64,
            gen: u64,
            opts: CrucibleOpts,
        },
        #[serde(rename = "file")]
        File {
            block_size: u64,
            id: ::uuid::Uuid,
            path: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for VolumeConstructionRequest {
        fn from(value: &VolumeConstructionRequest) -> Self {
            value.clone()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct CrucibleOpts {
            cert_pem: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            control: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            flush_timeout: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            key: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            key_pem: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            lossy: ::std::result::Result<bool, ::std::string::String>,
            read_only: ::std::result::Result<bool, ::std::string::String>,
            root_cert_pem: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            target: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for CrucibleOpts {
            fn default() -> Self {
                Self {
                    cert_pem: Ok(Default::default()),
                    control: Ok(Default::default()),
                    flush_timeout: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    key: Ok(Default::default()),
                    key_pem: Ok(Default::default()),
                    lossy: Err("no value supplied for lossy".to_string()),
                    read_only: Err("no value supplied for read_only".to_string()),
                    root_cert_pem: Ok(Default::default()),
                    target: Err("no value supplied for target".to_string()),
                }
            }
        }

        impl CrucibleOpts {
            pub fn cert_pem<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cert_pem = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cert_pem: {}", e));
                self
            }
            pub fn control<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.control = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for control: {}", e));
                self
            }
            pub fn flush_timeout<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<u32>>,
                T::Error: ::std::fmt::Display,
            {
                self.flush_timeout = value.try_into().map_err(|e| {
                    format!("error converting supplied value for flush_timeout: {}", e)
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
            pub fn key_pem<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.key_pem = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key_pem: {}", e));
                self
            }
            pub fn lossy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.lossy = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lossy: {}", e));
                self
            }
            pub fn read_only<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.read_only = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for read_only: {}", e));
                self
            }
            pub fn root_cert_pem<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.root_cert_pem = value.try_into().map_err(|e| {
                    format!("error converting supplied value for root_cert_pem: {}", e)
                });
                self
            }
            pub fn target<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.target = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for target: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<CrucibleOpts> for super::CrucibleOpts {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CrucibleOpts,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cert_pem: value.cert_pem?,
                    control: value.control?,
                    flush_timeout: value.flush_timeout?,
                    id: value.id?,
                    key: value.key?,
                    key_pem: value.key_pem?,
                    lossy: value.lossy?,
                    read_only: value.read_only?,
                    root_cert_pem: value.root_cert_pem?,
                    target: value.target?,
                })
            }
        }

        impl ::std::convert::From<super::CrucibleOpts> for CrucibleOpts {
            fn from(value: super::CrucibleOpts) -> Self {
                Self {
                    cert_pem: Ok(value.cert_pem),
                    control: Ok(value.control),
                    flush_timeout: Ok(value.flush_timeout),
                    id: Ok(value.id),
                    key: Ok(value.key),
                    key_pem: Ok(value.key_pem),
                    lossy: Ok(value.lossy),
                    read_only: Ok(value.read_only),
                    root_cert_pem: Ok(value.root_cert_pem),
                    target: Ok(value.target),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DiskAttachment {
            disk_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            generation_id: ::std::result::Result<u64, ::std::string::String>,
            state: ::std::result::Result<super::DiskAttachmentState, ::std::string::String>,
        }

        impl ::std::default::Default for DiskAttachment {
            fn default() -> Self {
                Self {
                    disk_id: Err("no value supplied for disk_id".to_string()),
                    generation_id: Err("no value supplied for generation_id".to_string()),
                    state: Err("no value supplied for state".to_string()),
                }
            }
        }

        impl DiskAttachment {
            pub fn disk_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.disk_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for disk_id: {}", e));
                self
            }
            pub fn generation_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.generation_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for generation_id: {}", e)
                });
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DiskAttachmentState>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<DiskAttachment> for super::DiskAttachment {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DiskAttachment,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    disk_id: value.disk_id?,
                    generation_id: value.generation_id?,
                    state: value.state?,
                })
            }
        }

        impl ::std::convert::From<super::DiskAttachment> for DiskAttachment {
            fn from(value: super::DiskAttachment) -> Self {
                Self {
                    disk_id: Ok(value.disk_id),
                    generation_id: Ok(value.generation_id),
                    state: Ok(value.state),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DiskRequest {
            device: ::std::result::Result<::std::string::String, ::std::string::String>,
            gen: ::std::result::Result<u64, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            read_only: ::std::result::Result<bool, ::std::string::String>,
            slot: ::std::result::Result<super::Slot, ::std::string::String>,
            volume_construction_request:
                ::std::result::Result<super::VolumeConstructionRequest, ::std::string::String>,
        }

        impl ::std::default::Default for DiskRequest {
            fn default() -> Self {
                Self {
                    device: Err("no value supplied for device".to_string()),
                    gen: Err("no value supplied for gen".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    read_only: Err("no value supplied for read_only".to_string()),
                    slot: Err("no value supplied for slot".to_string()),
                    volume_construction_request: Err("no value supplied for \
                                                      volume_construction_request"
                        .to_string()),
                }
            }
        }

        impl DiskRequest {
            pub fn device<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.device = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for device: {}", e));
                self
            }
            pub fn gen<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.gen = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gen: {}", e));
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
            pub fn read_only<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.read_only = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for read_only: {}", e));
                self
            }
            pub fn slot<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Slot>,
                T::Error: ::std::fmt::Display,
            {
                self.slot = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for slot: {}", e));
                self
            }
            pub fn volume_construction_request<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::VolumeConstructionRequest>,
                T::Error: ::std::fmt::Display,
            {
                self.volume_construction_request = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for volume_construction_request: {}",
                        e
                    )
                });
                self
            }
        }

        impl ::std::convert::TryFrom<DiskRequest> for super::DiskRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DiskRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    device: value.device?,
                    gen: value.gen?,
                    name: value.name?,
                    read_only: value.read_only?,
                    slot: value.slot?,
                    volume_construction_request: value.volume_construction_request?,
                })
            }
        }

        impl ::std::convert::From<super::DiskRequest> for DiskRequest {
            fn from(value: super::DiskRequest) -> Self {
                Self {
                    device: Ok(value.device),
                    gen: Ok(value.gen),
                    name: Ok(value.name),
                    read_only: Ok(value.read_only),
                    slot: Ok(value.slot),
                    volume_construction_request: Ok(value.volume_construction_request),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Error {
            error_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
            request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for Error {
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
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.error_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error_code: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn request_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.request_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for request_id: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Error> for super::Error {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Error,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    error_code: value.error_code?,
                    message: value.message?,
                    request_id: value.request_id?,
                })
            }
        }

        impl ::std::convert::From<super::Error> for Error {
            fn from(value: super::Error) -> Self {
                Self {
                    error_code: Ok(value.error_code),
                    message: Ok(value.message),
                    request_id: Ok(value.request_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Instance {
            disks: ::std::result::Result<
                ::std::vec::Vec<super::DiskAttachment>,
                ::std::string::String,
            >,
            nics: ::std::result::Result<
                ::std::vec::Vec<super::NetworkInterface>,
                ::std::string::String,
            >,
            properties: ::std::result::Result<super::InstanceProperties, ::std::string::String>,
            state: ::std::result::Result<super::InstanceState, ::std::string::String>,
        }

        impl ::std::default::Default for Instance {
            fn default() -> Self {
                Self {
                    disks: Err("no value supplied for disks".to_string()),
                    nics: Err("no value supplied for nics".to_string()),
                    properties: Err("no value supplied for properties".to_string()),
                    state: Err("no value supplied for state".to_string()),
                }
            }
        }

        impl Instance {
            pub fn disks<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::DiskAttachment>>,
                T::Error: ::std::fmt::Display,
            {
                self.disks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for disks: {}", e));
                self
            }
            pub fn nics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::NetworkInterface>>,
                T::Error: ::std::fmt::Display,
            {
                self.nics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for nics: {}", e));
                self
            }
            pub fn properties<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::InstanceProperties>,
                T::Error: ::std::fmt::Display,
            {
                self.properties = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for properties: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::InstanceState>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Instance> for super::Instance {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Instance,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    disks: value.disks?,
                    nics: value.nics?,
                    properties: value.properties?,
                    state: value.state?,
                })
            }
        }

        impl ::std::convert::From<super::Instance> for Instance {
            fn from(value: super::Instance) -> Self {
                Self {
                    disks: Ok(value.disks),
                    nics: Ok(value.nics),
                    properties: Ok(value.properties),
                    state: Ok(value.state),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceEnsureRequest {
            cloud_init_bytes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            disks:
                ::std::result::Result<::std::vec::Vec<super::DiskRequest>, ::std::string::String>,
            migrate: ::std::result::Result<
                ::std::option::Option<super::InstanceMigrateInitiateRequest>,
                ::std::string::String,
            >,
            nics: ::std::result::Result<
                ::std::vec::Vec<super::NetworkInterfaceRequest>,
                ::std::string::String,
            >,
            properties: ::std::result::Result<super::InstanceProperties, ::std::string::String>,
        }

        impl ::std::default::Default for InstanceEnsureRequest {
            fn default() -> Self {
                Self {
                    cloud_init_bytes: Ok(Default::default()),
                    disks: Ok(Default::default()),
                    migrate: Ok(Default::default()),
                    nics: Ok(Default::default()),
                    properties: Err("no value supplied for properties".to_string()),
                }
            }
        }

        impl InstanceEnsureRequest {
            pub fn cloud_init_bytes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cloud_init_bytes = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for cloud_init_bytes: {}",
                        e
                    )
                });
                self
            }
            pub fn disks<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::DiskRequest>>,
                T::Error: ::std::fmt::Display,
            {
                self.disks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for disks: {}", e));
                self
            }
            pub fn migrate<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::InstanceMigrateInitiateRequest>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.migrate = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for migrate: {}", e));
                self
            }
            pub fn nics<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::NetworkInterfaceRequest>>,
                T::Error: ::std::fmt::Display,
            {
                self.nics = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for nics: {}", e));
                self
            }
            pub fn properties<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::InstanceProperties>,
                T::Error: ::std::fmt::Display,
            {
                self.properties = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for properties: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceEnsureRequest> for super::InstanceEnsureRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceEnsureRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cloud_init_bytes: value.cloud_init_bytes?,
                    disks: value.disks?,
                    migrate: value.migrate?,
                    nics: value.nics?,
                    properties: value.properties?,
                })
            }
        }

        impl ::std::convert::From<super::InstanceEnsureRequest> for InstanceEnsureRequest {
            fn from(value: super::InstanceEnsureRequest) -> Self {
                Self {
                    cloud_init_bytes: Ok(value.cloud_init_bytes),
                    disks: Ok(value.disks),
                    migrate: Ok(value.migrate),
                    nics: Ok(value.nics),
                    properties: Ok(value.properties),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceEnsureResponse {
            migrate: ::std::result::Result<
                ::std::option::Option<super::InstanceMigrateInitiateResponse>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for InstanceEnsureResponse {
            fn default() -> Self {
                Self {
                    migrate: Ok(Default::default()),
                }
            }
        }

        impl InstanceEnsureResponse {
            pub fn migrate<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::InstanceMigrateInitiateResponse>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.migrate = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for migrate: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceEnsureResponse> for super::InstanceEnsureResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceEnsureResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    migrate: value.migrate?,
                })
            }
        }

        impl ::std::convert::From<super::InstanceEnsureResponse> for InstanceEnsureResponse {
            fn from(value: super::InstanceEnsureResponse) -> Self {
                Self {
                    migrate: Ok(value.migrate),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceGetResponse {
            instance: ::std::result::Result<super::Instance, ::std::string::String>,
        }

        impl ::std::default::Default for InstanceGetResponse {
            fn default() -> Self {
                Self {
                    instance: Err("no value supplied for instance".to_string()),
                }
            }
        }

        impl InstanceGetResponse {
            pub fn instance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Instance>,
                T::Error: ::std::fmt::Display,
            {
                self.instance = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instance: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceGetResponse> for super::InstanceGetResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceGetResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    instance: value.instance?,
                })
            }
        }

        impl ::std::convert::From<super::InstanceGetResponse> for InstanceGetResponse {
            fn from(value: super::InstanceGetResponse) -> Self {
                Self {
                    instance: Ok(value.instance),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceMigrateInitiateRequest {
            migration_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            src_addr: ::std::result::Result<::std::string::String, ::std::string::String>,
            src_uuid: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        }

        impl ::std::default::Default for InstanceMigrateInitiateRequest {
            fn default() -> Self {
                Self {
                    migration_id: Err("no value supplied for migration_id".to_string()),
                    src_addr: Err("no value supplied for src_addr".to_string()),
                    src_uuid: Err("no value supplied for src_uuid".to_string()),
                }
            }
        }

        impl InstanceMigrateInitiateRequest {
            pub fn migration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.migration_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for migration_id: {}", e)
                });
                self
            }
            pub fn src_addr<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.src_addr = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for src_addr: {}", e));
                self
            }
            pub fn src_uuid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.src_uuid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for src_uuid: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceMigrateInitiateRequest>
            for super::InstanceMigrateInitiateRequest
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceMigrateInitiateRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    migration_id: value.migration_id?,
                    src_addr: value.src_addr?,
                    src_uuid: value.src_uuid?,
                })
            }
        }

        impl ::std::convert::From<super::InstanceMigrateInitiateRequest>
            for InstanceMigrateInitiateRequest
        {
            fn from(value: super::InstanceMigrateInitiateRequest) -> Self {
                Self {
                    migration_id: Ok(value.migration_id),
                    src_addr: Ok(value.src_addr),
                    src_uuid: Ok(value.src_uuid),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceMigrateInitiateResponse {
            migration_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        }

        impl ::std::default::Default for InstanceMigrateInitiateResponse {
            fn default() -> Self {
                Self {
                    migration_id: Err("no value supplied for migration_id".to_string()),
                }
            }
        }

        impl InstanceMigrateInitiateResponse {
            pub fn migration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.migration_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for migration_id: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceMigrateInitiateResponse>
            for super::InstanceMigrateInitiateResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceMigrateInitiateResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    migration_id: value.migration_id?,
                })
            }
        }

        impl ::std::convert::From<super::InstanceMigrateInitiateResponse>
            for InstanceMigrateInitiateResponse
        {
            fn from(value: super::InstanceMigrateInitiateResponse) -> Self {
                Self {
                    migration_id: Ok(value.migration_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceMigrateStatusRequest {
            migration_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        }

        impl ::std::default::Default for InstanceMigrateStatusRequest {
            fn default() -> Self {
                Self {
                    migration_id: Err("no value supplied for migration_id".to_string()),
                }
            }
        }

        impl InstanceMigrateStatusRequest {
            pub fn migration_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.migration_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for migration_id: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceMigrateStatusRequest> for super::InstanceMigrateStatusRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceMigrateStatusRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    migration_id: value.migration_id?,
                })
            }
        }

        impl ::std::convert::From<super::InstanceMigrateStatusRequest> for InstanceMigrateStatusRequest {
            fn from(value: super::InstanceMigrateStatusRequest) -> Self {
                Self {
                    migration_id: Ok(value.migration_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceMigrateStatusResponse {
            state: ::std::result::Result<super::MigrationState, ::std::string::String>,
        }

        impl ::std::default::Default for InstanceMigrateStatusResponse {
            fn default() -> Self {
                Self {
                    state: Err("no value supplied for state".to_string()),
                }
            }
        }

        impl InstanceMigrateStatusResponse {
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MigrationState>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceMigrateStatusResponse>
            for super::InstanceMigrateStatusResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceMigrateStatusResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    state: value.state?,
                })
            }
        }

        impl ::std::convert::From<super::InstanceMigrateStatusResponse> for InstanceMigrateStatusResponse {
            fn from(value: super::InstanceMigrateStatusResponse) -> Self {
                Self {
                    state: Ok(value.state),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceProperties {
            bootrom_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            description: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            image_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            memory: ::std::result::Result<u64, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            vcpus: ::std::result::Result<u8, ::std::string::String>,
        }

        impl ::std::default::Default for InstanceProperties {
            fn default() -> Self {
                Self {
                    bootrom_id: Err("no value supplied for bootrom_id".to_string()),
                    description: Err("no value supplied for description".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    image_id: Err("no value supplied for image_id".to_string()),
                    memory: Err("no value supplied for memory".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    vcpus: Err("no value supplied for vcpus".to_string()),
                }
            }
        }

        impl InstanceProperties {
            pub fn bootrom_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.bootrom_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bootrom_id: {}", e));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn image_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.image_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for image_id: {}", e));
                self
            }
            pub fn memory<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.memory = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for memory: {}", e));
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
            pub fn vcpus<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u8>,
                T::Error: ::std::fmt::Display,
            {
                self.vcpus = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for vcpus: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceProperties> for super::InstanceProperties {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceProperties,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    bootrom_id: value.bootrom_id?,
                    description: value.description?,
                    id: value.id?,
                    image_id: value.image_id?,
                    memory: value.memory?,
                    name: value.name?,
                    vcpus: value.vcpus?,
                })
            }
        }

        impl ::std::convert::From<super::InstanceProperties> for InstanceProperties {
            fn from(value: super::InstanceProperties) -> Self {
                Self {
                    bootrom_id: Ok(value.bootrom_id),
                    description: Ok(value.description),
                    id: Ok(value.id),
                    image_id: Ok(value.image_id),
                    memory: Ok(value.memory),
                    name: Ok(value.name),
                    vcpus: Ok(value.vcpus),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceStateMonitorRequest {
            gen: ::std::result::Result<u64, ::std::string::String>,
        }

        impl ::std::default::Default for InstanceStateMonitorRequest {
            fn default() -> Self {
                Self {
                    gen: Err("no value supplied for gen".to_string()),
                }
            }
        }

        impl InstanceStateMonitorRequest {
            pub fn gen<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.gen = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gen: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceStateMonitorRequest> for super::InstanceStateMonitorRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceStateMonitorRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self { gen: value.gen? })
            }
        }

        impl ::std::convert::From<super::InstanceStateMonitorRequest> for InstanceStateMonitorRequest {
            fn from(value: super::InstanceStateMonitorRequest) -> Self {
                Self { gen: Ok(value.gen) }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstanceStateMonitorResponse {
            gen: ::std::result::Result<u64, ::std::string::String>,
            state: ::std::result::Result<super::InstanceState, ::std::string::String>,
        }

        impl ::std::default::Default for InstanceStateMonitorResponse {
            fn default() -> Self {
                Self {
                    gen: Err("no value supplied for gen".to_string()),
                    state: Err("no value supplied for state".to_string()),
                }
            }
        }

        impl InstanceStateMonitorResponse {
            pub fn gen<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.gen = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gen: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::InstanceState>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InstanceStateMonitorResponse> for super::InstanceStateMonitorResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceStateMonitorResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    gen: value.gen?,
                    state: value.state?,
                })
            }
        }

        impl ::std::convert::From<super::InstanceStateMonitorResponse> for InstanceStateMonitorResponse {
            fn from(value: super::InstanceStateMonitorResponse) -> Self {
                Self {
                    gen: Ok(value.gen),
                    state: Ok(value.state),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct NetworkInterface {
            attachment: ::std::result::Result<
                super::NetworkInterfaceAttachmentState,
                ::std::string::String,
            >,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for NetworkInterface {
            fn default() -> Self {
                Self {
                    attachment: Err("no value supplied for attachment".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl NetworkInterface {
            pub fn attachment<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::NetworkInterfaceAttachmentState>,
                T::Error: ::std::fmt::Display,
            {
                self.attachment = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attachment: {}", e));
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

        impl ::std::convert::TryFrom<NetworkInterface> for super::NetworkInterface {
            type Error = super::error::ConversionError;
            fn try_from(
                value: NetworkInterface,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    attachment: value.attachment?,
                    name: value.name?,
                })
            }
        }

        impl ::std::convert::From<super::NetworkInterface> for NetworkInterface {
            fn from(value: super::NetworkInterface) -> Self {
                Self {
                    attachment: Ok(value.attachment),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct NetworkInterfaceRequest {
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            slot: ::std::result::Result<super::Slot, ::std::string::String>,
        }

        impl ::std::default::Default for NetworkInterfaceRequest {
            fn default() -> Self {
                Self {
                    name: Err("no value supplied for name".to_string()),
                    slot: Err("no value supplied for slot".to_string()),
                }
            }
        }

        impl NetworkInterfaceRequest {
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
            pub fn slot<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Slot>,
                T::Error: ::std::fmt::Display,
            {
                self.slot = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for slot: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<NetworkInterfaceRequest> for super::NetworkInterfaceRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: NetworkInterfaceRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    slot: value.slot?,
                })
            }
        }

        impl ::std::convert::From<super::NetworkInterfaceRequest> for NetworkInterfaceRequest {
            fn from(value: super::NetworkInterfaceRequest) -> Self {
                Self {
                    name: Ok(value.name),
                    slot: Ok(value.slot),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for Oxide Propolis Server API
///
///API for interacting with the Propolis hypervisor frontend.
///
///Version: 0.0.1
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
        "0.0.1"
    }
}

impl Client {
    ///Sends a `GET` request to `/instance`
    ///
    ///```ignore
    /// let response = client.instance_get()
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_get(&self) -> builder::InstanceGet {
        builder::InstanceGet::new(self)
    }

    ///Sends a `PUT` request to `/instance`
    ///
    ///```ignore
    /// let response = client.instance_ensure()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_ensure(&self) -> builder::InstanceEnsure {
        builder::InstanceEnsure::new(self)
    }

    ///Issue a snapshot request to a crucible backend
    ///
    ///Sends a `POST` request to `/instance/disk/{id}/snapshot/{snapshot_id}`
    ///
    ///```ignore
    /// let response = client.instance_issue_crucible_snapshot_request()
    ///    .id(id)
    ///    .snapshot_id(snapshot_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_issue_crucible_snapshot_request(
        &self,
    ) -> builder::InstanceIssueCrucibleSnapshotRequest {
        builder::InstanceIssueCrucibleSnapshotRequest::new(self)
    }

    ///Sends a `GET` request to `/instance/migrate/status`
    ///
    ///```ignore
    /// let response = client.instance_migrate_status()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_migrate_status(&self) -> builder::InstanceMigrateStatus {
        builder::InstanceMigrateStatus::new(self)
    }

    ///Sends a `GET` request to `/instance/serial`
    ///
    ///```ignore
    /// let response = client.instance_serial()
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_serial(&self) -> builder::InstanceSerial {
        builder::InstanceSerial::new(self)
    }

    ///Sends a `PUT` request to `/instance/state`
    ///
    ///```ignore
    /// let response = client.instance_state_put()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_state_put(&self) -> builder::InstanceStatePut {
        builder::InstanceStatePut::new(self)
    }

    ///Sends a `GET` request to `/instance/state-monitor`
    ///
    ///```ignore
    /// let response = client.instance_state_monitor()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_state_monitor(&self) -> builder::InstanceStateMonitor {
        builder::InstanceStateMonitor::new(self)
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
    ///Builder for [`Client::instance_get`]
    ///
    ///[`Client::instance_get`]: super::Client::instance_get
    #[derive(Debug, Clone)]
    pub struct InstanceGet<'a> {
        client: &'a super::Client,
    }

    impl<'a> InstanceGet<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `GET` request to `/instance`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InstanceGetResponse>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/instance", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::instance_ensure`]
    ///
    ///[`Client::instance_ensure`]: super::Client::instance_ensure
    #[derive(Debug, Clone)]
    pub struct InstanceEnsure<'a> {
        client: &'a super::Client,
        body: Result<types::builder::InstanceEnsureRequest, String>,
    }

    impl<'a> InstanceEnsure<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::InstanceEnsureRequest>,
            <V as std::convert::TryInto<types::InstanceEnsureRequest>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `InstanceEnsureRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::InstanceEnsureRequest,
            ) -> types::builder::InstanceEnsureRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/instance`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InstanceEnsureResponse>, Error<types::Error>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::InstanceEnsureRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/instance", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::instance_issue_crucible_snapshot_request`]
    ///
    ///[`Client::instance_issue_crucible_snapshot_request`]: super::Client::instance_issue_crucible_snapshot_request
    #[derive(Debug, Clone)]
    pub struct InstanceIssueCrucibleSnapshotRequest<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
        snapshot_id: Result<::uuid::Uuid, String>,
    }

    impl<'a> InstanceIssueCrucibleSnapshotRequest<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
                snapshot_id: Err("snapshot_id was not initialized".to_string()),
            }
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }

        pub fn snapshot_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.snapshot_id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for snapshot_id failed".to_string());
            self
        }

        ///Sends a `POST` request to
        /// `/instance/disk/{id}/snapshot/{snapshot_id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self {
                client,
                id,
                snapshot_id,
            } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let snapshot_id = snapshot_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/instance/disk/{}/snapshot/{}",
                client.baseurl,
                encode_path(&id.to_string()),
                encode_path(&snapshot_id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::instance_migrate_status`]
    ///
    ///[`Client::instance_migrate_status`]: super::Client::instance_migrate_status
    #[derive(Debug, Clone)]
    pub struct InstanceMigrateStatus<'a> {
        client: &'a super::Client,
        body: Result<types::builder::InstanceMigrateStatusRequest, String>,
    }

    impl<'a> InstanceMigrateStatus<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::InstanceMigrateStatusRequest>,
            <V as std::convert::TryInto<types::InstanceMigrateStatusRequest>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `InstanceMigrateStatusRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::InstanceMigrateStatusRequest,
            ) -> types::builder::InstanceMigrateStatusRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `GET` request to `/instance/migrate/status`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InstanceMigrateStatusResponse>, Error<types::Error>>
        {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| {
                    types::InstanceMigrateStatusRequest::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/instance/migrate/status", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::instance_serial`]
    ///
    ///[`Client::instance_serial`]: super::Client::instance_serial
    #[derive(Debug, Clone)]
    pub struct InstanceSerial<'a> {
        client: &'a super::Client,
    }

    impl<'a> InstanceSerial<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `GET` request to `/instance/serial`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<reqwest::Upgraded>, Error<reqwest::Upgraded>> {
            let Self { client } = self;
            let url = format!("{}/instance/serial", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(reqwest::header::CONNECTION, "Upgrade")
                .header(reqwest::header::UPGRADE, "websocket")
                .header(reqwest::header::SEC_WEBSOCKET_VERSION, "13")
                .header(
                    reqwest::header::SEC_WEBSOCKET_KEY,
                    base64::Engine::encode(
                        &base64::engine::general_purpose::STANDARD,
                        rand::random::<[u8; 16]>(),
                    ),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                101u16 => ResponseValue::upgrade(response).await,
                200..=299 => ResponseValue::upgrade(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::instance_state_put`]
    ///
    ///[`Client::instance_state_put`]: super::Client::instance_state_put
    #[derive(Debug, Clone)]
    pub struct InstanceStatePut<'a> {
        client: &'a super::Client,
        body: Result<types::InstanceStateRequested, String>,
    }

    impl<'a> InstanceStatePut<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::InstanceStateRequested>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `InstanceStateRequested` for body failed".to_string());
            self
        }

        ///Sends a `PUT` request to `/instance/state`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self { client, body } = self;
            let body = body.map_err(Error::InvalidRequest)?;
            let url = format!("{}/instance/state", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::instance_state_monitor`]
    ///
    ///[`Client::instance_state_monitor`]: super::Client::instance_state_monitor
    #[derive(Debug, Clone)]
    pub struct InstanceStateMonitor<'a> {
        client: &'a super::Client,
        body: Result<types::builder::InstanceStateMonitorRequest, String>,
    }

    impl<'a> InstanceStateMonitor<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::InstanceStateMonitorRequest>,
            <V as std::convert::TryInto<types::InstanceStateMonitorRequest>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `InstanceStateMonitorRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::InstanceStateMonitorRequest,
            ) -> types::builder::InstanceStateMonitorRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `GET` request to `/instance/state-monitor`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InstanceStateMonitorResponse>, Error<types::Error>>
        {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| {
                    types::InstanceStateMonitorRequest::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/instance/state-monitor", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
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
