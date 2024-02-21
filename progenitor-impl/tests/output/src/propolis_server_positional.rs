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

    ///CrucibleOpts
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CrucibleOpts {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cert_pem: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub control: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub flush_timeout: Option<u32>,
        pub id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_pem: Option<String>,
        pub lossy: bool,
        pub read_only: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub root_cert_pem: Option<String>,
        pub target: Vec<String>,
    }

    impl From<&CrucibleOpts> for CrucibleOpts {
        fn from(value: &CrucibleOpts) -> Self {
            value.clone()
        }
    }

    ///DiskAttachment
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DiskAttachment {
        pub disk_id: uuid::Uuid,
        pub generation_id: u64,
        pub state: DiskAttachmentState,
    }

    impl From<&DiskAttachment> for DiskAttachment {
        fn from(value: &DiskAttachment) -> Self {
            value.clone()
        }
    }

    ///DiskAttachmentState
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DiskAttachmentState {
        Detached,
        Destroyed,
        Faulted,
        Attached(uuid::Uuid),
    }

    impl From<&DiskAttachmentState> for DiskAttachmentState {
        fn from(value: &DiskAttachmentState) -> Self {
            value.clone()
        }
    }

    impl From<uuid::Uuid> for DiskAttachmentState {
        fn from(value: uuid::Uuid) -> Self {
            Self::Attached(value)
        }
    }

    ///DiskRequest
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DiskRequest {
        pub device: String,
        pub gen: u64,
        pub name: String,
        pub read_only: bool,
        pub slot: Slot,
        pub volume_construction_request: VolumeConstructionRequest,
    }

    impl From<&DiskRequest> for DiskRequest {
        fn from(value: &DiskRequest) -> Self {
            value.clone()
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

    ///Instance
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Instance {
        pub disks: Vec<DiskAttachment>,
        pub nics: Vec<NetworkInterface>,
        pub properties: InstanceProperties,
        pub state: InstanceState,
    }

    impl From<&Instance> for Instance {
        fn from(value: &Instance) -> Self {
            value.clone()
        }
    }

    ///InstanceEnsureRequest
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
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/InstanceMigrateInitiateRequest"
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceEnsureRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cloud_init_bytes: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub disks: Vec<DiskRequest>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub migrate: Option<InstanceMigrateInitiateRequest>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub nics: Vec<NetworkInterfaceRequest>,
        pub properties: InstanceProperties,
    }

    impl From<&InstanceEnsureRequest> for InstanceEnsureRequest {
        fn from(value: &InstanceEnsureRequest) -> Self {
            value.clone()
        }
    }

    ///InstanceEnsureResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "migrate": {
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/InstanceMigrateInitiateResponse"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceEnsureResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub migrate: Option<InstanceMigrateInitiateResponse>,
    }

    impl From<&InstanceEnsureResponse> for InstanceEnsureResponse {
        fn from(value: &InstanceEnsureResponse) -> Self {
            value.clone()
        }
    }

    ///InstanceGetResponse
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceGetResponse {
        pub instance: Instance,
    }

    impl From<&InstanceGetResponse> for InstanceGetResponse {
        fn from(value: &InstanceGetResponse) -> Self {
            value.clone()
        }
    }

    ///InstanceMigrateInitiateRequest
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceMigrateInitiateRequest {
        pub migration_id: uuid::Uuid,
        pub src_addr: String,
        pub src_uuid: uuid::Uuid,
    }

    impl From<&InstanceMigrateInitiateRequest> for InstanceMigrateInitiateRequest {
        fn from(value: &InstanceMigrateInitiateRequest) -> Self {
            value.clone()
        }
    }

    ///InstanceMigrateInitiateResponse
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceMigrateInitiateResponse {
        pub migration_id: uuid::Uuid,
    }

    impl From<&InstanceMigrateInitiateResponse> for InstanceMigrateInitiateResponse {
        fn from(value: &InstanceMigrateInitiateResponse) -> Self {
            value.clone()
        }
    }

    ///InstanceMigrateStatusRequest
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceMigrateStatusRequest {
        pub migration_id: uuid::Uuid,
    }

    impl From<&InstanceMigrateStatusRequest> for InstanceMigrateStatusRequest {
        fn from(value: &InstanceMigrateStatusRequest) -> Self {
            value.clone()
        }
    }

    ///InstanceMigrateStatusResponse
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceMigrateStatusResponse {
        pub state: MigrationState,
    }

    impl From<&InstanceMigrateStatusResponse> for InstanceMigrateStatusResponse {
        fn from(value: &InstanceMigrateStatusResponse) -> Self {
            value.clone()
        }
    }

    ///InstanceProperties
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceProperties {
        ///ID of the bootrom used to initialize this Instance.
        pub bootrom_id: uuid::Uuid,
        ///Free-form text description of an Instance.
        pub description: String,
        ///Unique identifier for this Instance.
        pub id: uuid::Uuid,
        ///ID of the image used to initialize this Instance.
        pub image_id: uuid::Uuid,
        ///Size of memory allocated to the Instance, in MiB.
        pub memory: u64,
        ///Human-readable name of the Instance.
        pub name: String,
        ///Number of vCPUs to be allocated to the Instance.
        pub vcpus: u8,
    }

    impl From<&InstanceProperties> for InstanceProperties {
        fn from(value: &InstanceProperties) -> Self {
            value.clone()
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

    impl From<&InstanceState> for InstanceState {
        fn from(value: &InstanceState) -> Self {
            value.clone()
        }
    }

    impl ToString for InstanceState {
        fn to_string(&self) -> String {
            match *self {
                Self::Creating => "Creating".to_string(),
                Self::Starting => "Starting".to_string(),
                Self::Running => "Running".to_string(),
                Self::Stopping => "Stopping".to_string(),
                Self::Stopped => "Stopped".to_string(),
                Self::Rebooting => "Rebooting".to_string(),
                Self::Migrating => "Migrating".to_string(),
                Self::Repairing => "Repairing".to_string(),
                Self::Failed => "Failed".to_string(),
                Self::Destroyed => "Destroyed".to_string(),
            }
        }
    }

    impl std::str::FromStr for InstanceState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
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

    impl std::convert::TryFrom<&str> for InstanceState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for InstanceState {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for InstanceState {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///InstanceStateMonitorRequest
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceStateMonitorRequest {
        pub gen: u64,
    }

    impl From<&InstanceStateMonitorRequest> for InstanceStateMonitorRequest {
        fn from(value: &InstanceStateMonitorRequest) -> Self {
            value.clone()
        }
    }

    ///InstanceStateMonitorResponse
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstanceStateMonitorResponse {
        pub gen: u64,
        pub state: InstanceState,
    }

    impl From<&InstanceStateMonitorResponse> for InstanceStateMonitorResponse {
        fn from(value: &InstanceStateMonitorResponse) -> Self {
            value.clone()
        }
    }

    ///InstanceStateRequested
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum InstanceStateRequested {
        Run,
        Stop,
        Reboot,
        MigrateStart,
    }

    impl From<&InstanceStateRequested> for InstanceStateRequested {
        fn from(value: &InstanceStateRequested) -> Self {
            value.clone()
        }
    }

    impl ToString for InstanceStateRequested {
        fn to_string(&self) -> String {
            match *self {
                Self::Run => "Run".to_string(),
                Self::Stop => "Stop".to_string(),
                Self::Reboot => "Reboot".to_string(),
                Self::MigrateStart => "MigrateStart".to_string(),
            }
        }
    }

    impl std::str::FromStr for InstanceStateRequested {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "Run" => Ok(Self::Run),
                "Stop" => Ok(Self::Stop),
                "Reboot" => Ok(Self::Reboot),
                "MigrateStart" => Ok(Self::MigrateStart),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for InstanceStateRequested {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for InstanceStateRequested {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for InstanceStateRequested {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///MigrationState
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
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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

    impl From<&MigrationState> for MigrationState {
        fn from(value: &MigrationState) -> Self {
            value.clone()
        }
    }

    impl ToString for MigrationState {
        fn to_string(&self) -> String {
            match *self {
                Self::Sync => "Sync".to_string(),
                Self::RamPush => "RamPush".to_string(),
                Self::Pause => "Pause".to_string(),
                Self::RamPushDirty => "RamPushDirty".to_string(),
                Self::Device => "Device".to_string(),
                Self::Arch => "Arch".to_string(),
                Self::Resume => "Resume".to_string(),
                Self::RamPull => "RamPull".to_string(),
                Self::Finish => "Finish".to_string(),
                Self::Error => "Error".to_string(),
            }
        }
    }

    impl std::str::FromStr for MigrationState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
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

    impl std::convert::TryFrom<&str> for MigrationState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for MigrationState {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for MigrationState {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///NetworkInterface
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NetworkInterface {
        pub attachment: NetworkInterfaceAttachmentState,
        pub name: String,
    }

    impl From<&NetworkInterface> for NetworkInterface {
        fn from(value: &NetworkInterface) -> Self {
            value.clone()
        }
    }

    ///NetworkInterfaceAttachmentState
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum NetworkInterfaceAttachmentState {
        Detached,
        Faulted,
        Attached(Slot),
    }

    impl From<&NetworkInterfaceAttachmentState> for NetworkInterfaceAttachmentState {
        fn from(value: &NetworkInterfaceAttachmentState) -> Self {
            value.clone()
        }
    }

    impl From<Slot> for NetworkInterfaceAttachmentState {
        fn from(value: Slot) -> Self {
            Self::Attached(value)
        }
    }

    ///NetworkInterfaceRequest
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NetworkInterfaceRequest {
        pub name: String,
        pub slot: Slot,
    }

    impl From<&NetworkInterfaceRequest> for NetworkInterfaceRequest {
        fn from(value: &NetworkInterfaceRequest) -> Self {
            value.clone()
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Slot(pub u8);
    impl std::ops::Deref for Slot {
        type Target = u8;
        fn deref(&self) -> &u8 {
            &self.0
        }
    }

    impl From<Slot> for u8 {
        fn from(value: Slot) -> Self {
            value.0
        }
    }

    impl From<&Slot> for Slot {
        fn from(value: &Slot) -> Self {
            value.clone()
        }
    }

    impl From<u8> for Slot {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Slot {
        type Err = <u8 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Slot {
        type Error = <u8 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Slot {
        type Error = <u8 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Slot {
        type Error = <u8 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ToString for Slot {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    ///VolumeConstructionRequest
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
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/VolumeConstructionRequest"
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
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type")]
    pub enum VolumeConstructionRequest {
        #[serde(rename = "volume")]
        Volume {
            block_size: u64,
            id: uuid::Uuid,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            read_only_parent: Option<Box<VolumeConstructionRequest>>,
            sub_volumes: Vec<VolumeConstructionRequest>,
        },
        #[serde(rename = "url")]
        Url {
            block_size: u64,
            id: uuid::Uuid,
            url: String,
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
            id: uuid::Uuid,
            path: String,
        },
    }

    impl From<&VolumeConstructionRequest> for VolumeConstructionRequest {
        fn from(value: &VolumeConstructionRequest) -> Self {
            value.clone()
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

#[allow(clippy::all)]
impl Client {
    ///Sends a `GET` request to `/instance`
    pub async fn instance_get<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::InstanceGetResponse>, Error<types::Error>> {
        let url = format!("{}/instance", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
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

    ///Sends a `PUT` request to `/instance`
    pub async fn instance_ensure<'a>(
        &'a self,
        body: &'a types::InstanceEnsureRequest,
    ) -> Result<ResponseValue<types::InstanceEnsureResponse>, Error<types::Error>> {
        let url = format!("{}/instance", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
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

    ///Issue a snapshot request to a crucible backend
    ///
    ///Sends a `POST` request to `/instance/disk/{id}/snapshot/{snapshot_id}`
    pub async fn instance_issue_crucible_snapshot_request<'a>(
        &'a self,
        id: &'a uuid::Uuid,
        snapshot_id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/instance/disk/{}/snapshot/{}",
            self.baseurl,
            encode_path(&id.to_string()),
            encode_path(&snapshot_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
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

    ///Sends a `GET` request to `/instance/migrate/status`
    pub async fn instance_migrate_status<'a>(
        &'a self,
        body: &'a types::InstanceMigrateStatusRequest,
    ) -> Result<ResponseValue<types::InstanceMigrateStatusResponse>, Error<types::Error>> {
        let url = format!("{}/instance/migrate/status", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
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

    ///Sends a `GET` request to `/instance/serial`
    pub async fn instance_serial<'a>(
        &'a self,
    ) -> Result<ResponseValue<reqwest::Upgraded>, Error<reqwest::Upgraded>> {
        let url = format!("{}/instance/serial", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
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
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            101u16 => ResponseValue::upgrade(response).await,
            200..=299 => ResponseValue::upgrade(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/instance/state`
    pub async fn instance_state_put<'a>(
        &'a self,
        body: types::InstanceStateRequested,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/instance/state", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
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

    ///Sends a `GET` request to `/instance/state-monitor`
    pub async fn instance_state_monitor<'a>(
        &'a self,
        body: &'a types::InstanceStateMonitorRequest,
    ) -> Result<ResponseValue<types::InstanceStateMonitorResponse>, Error<types::Error>> {
        let url = format!("{}/instance/state-monitor", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
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

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
