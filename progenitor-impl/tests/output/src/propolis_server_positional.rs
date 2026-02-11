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
        #[serde(rename = "gen")]
        pub gen_: u64,
        pub name: ::std::string::String,
        pub read_only: bool,
        pub slot: Slot,
        pub volume_construction_request: VolumeConstructionRequest,
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

    impl ::std::default::Default for InstanceEnsureResponse {
        fn default() -> Self {
            Self {
                migrate: Default::default(),
            }
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

    impl ::std::fmt::Display for InstanceState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Creating => f.write_str("Creating"),
                Self::Starting => f.write_str("Starting"),
                Self::Running => f.write_str("Running"),
                Self::Stopping => f.write_str("Stopping"),
                Self::Stopped => f.write_str("Stopped"),
                Self::Rebooting => f.write_str("Rebooting"),
                Self::Migrating => f.write_str("Migrating"),
                Self::Repairing => f.write_str("Repairing"),
                Self::Failed => f.write_str("Failed"),
                Self::Destroyed => f.write_str("Destroyed"),
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
        #[serde(rename = "gen")]
        pub gen_: u64,
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
        #[serde(rename = "gen")]
        pub gen_: u64,
        pub state: InstanceState,
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

    impl ::std::fmt::Display for InstanceStateRequested {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Run => f.write_str("Run"),
                Self::Stop => f.write_str("Stop"),
                Self::Reboot => f.write_str("Reboot"),
                Self::MigrateStart => f.write_str("MigrateStart"),
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

    impl ::std::fmt::Display for MigrationState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Sync => f.write_str("Sync"),
                Self::RamPush => f.write_str("RamPush"),
                Self::Pause => f.write_str("Pause"),
                Self::RamPushDirty => f.write_str("RamPushDirty"),
                Self::Device => f.write_str("Device"),
                Self::Arch => f.write_str("Arch"),
                Self::Resume => f.write_str("Resume"),
                Self::RamPull => f.write_str("RamPull"),
                Self::Finish => f.write_str("Finish"),
                Self::Error => f.write_str("Error"),
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
            #[serde(rename = "gen")]
            gen_: u64,
            opts: CrucibleOpts,
        },
        #[serde(rename = "file")]
        File {
            block_size: u64,
            id: ::uuid::Uuid,
            path: ::std::string::String,
        },
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
        "0.0.1"
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
    ///Sends a `GET` request to `/instance`
    pub async fn instance_get<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::InstanceGetResponse>, Error<types::Error>> {
        let url = format!("{}/instance", self.baseurl,);
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
            operation_id: "instance_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_ensure",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
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
        id: &'a ::uuid::Uuid,
        snapshot_id: &'a ::uuid::Uuid,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/instance/disk/{}/snapshot/{}",
            self.baseurl,
            encode_path(&id.to_string()),
            encode_path(&snapshot_id.to_string()),
        );
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
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_issue_crucible_snapshot_request",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_migrate_status",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .headers(header_map)
            .header(::reqwest::header::CONNECTION, "Upgrade")
            .header(::reqwest::header::UPGRADE, "websocket")
            .header(::reqwest::header::SEC_WEBSOCKET_VERSION, "13")
            .header(
                ::reqwest::header::SEC_WEBSOCKET_KEY,
                ::base64::Engine::encode(
                    &::base64::engine::general_purpose::STANDARD,
                    ::rand::random::<[u8; 16]>(),
                ),
            )
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_serial",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
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
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_state_put",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "instance_state_monitor",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
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
