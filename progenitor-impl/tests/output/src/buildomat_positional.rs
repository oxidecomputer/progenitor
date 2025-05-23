#![allow(elided_named_lifetimes)]
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

    ///`GetThingOrThingsId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "string"
    ///    },
    ///    {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum GetThingOrThingsId {
        Variant0(::std::string::String),
        Variant1(::std::vec::Vec<::std::string::String>),
    }

    impl ::std::convert::From<&Self> for GetThingOrThingsId {
        fn from(value: &GetThingOrThingsId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for GetThingOrThingsId {
        fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
            Self::Variant1(value)
        }
    }

    ///`HeaderArgAcceptLanguage`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "en",
    ///  "type": "string",
    ///  "enum": [
    ///    "de",
    ///    "en"
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
    pub enum HeaderArgAcceptLanguage {
        #[serde(rename = "de")]
        De,
        #[serde(rename = "en")]
        En,
    }

    impl ::std::convert::From<&Self> for HeaderArgAcceptLanguage {
        fn from(value: &HeaderArgAcceptLanguage) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for HeaderArgAcceptLanguage {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::De => write!(f, "de"),
                Self::En => write!(f, "en"),
            }
        }
    }

    impl ::std::str::FromStr for HeaderArgAcceptLanguage {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "de" => Ok(Self::De),
                "en" => Ok(Self::En),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for HeaderArgAcceptLanguage {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for HeaderArgAcceptLanguage {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for HeaderArgAcceptLanguage {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::default::Default for HeaderArgAcceptLanguage {
        fn default() -> Self {
            HeaderArgAcceptLanguage::En
        }
    }

    ///`ObjWithOptionArray`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "stranger-things",
    ///    "things"
    ///  ],
    ///  "properties": {
    ///    "stranger-things": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "type": "null"
    ///          },
    ///          {
    ///            "allOf": [
    ///              {
    ///                "$ref": "#/components/schemas/Task"
    ///              }
    ///            ],
    ///            "oneOf": [
    ///              {}
    ///            ]
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "things": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "type": "null"
    ///          },
    ///          {
    ///            "allOf": [
    ///              {
    ///                "$ref": "#/components/schemas/Task"
    ///              }
    ///            ]
    ///          }
    ///        ]
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ObjWithOptionArray {
        #[serde(rename = "stranger-things")]
        pub stranger_things: ::std::vec::Vec<::std::option::Option<Task>>,
        pub things: ::std::vec::Vec<::std::option::Option<Task>>,
    }

    impl ::std::convert::From<&ObjWithOptionArray> for ObjWithOptionArray {
        fn from(value: &ObjWithOptionArray) -> Self {
            value.clone()
        }
    }

    ///`Task`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name",
    ///    "output_rules",
    ///    "script",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "output_rules": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "script": {
    ///      "type": "string"
    ///    },
    ///    "state": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Task {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
        pub output_rules: ::std::vec::Vec<::std::string::String>,
        pub script: ::std::string::String,
        pub state: ::std::string::String,
    }

    impl ::std::convert::From<&Task> for Task {
        fn from(value: &Task) -> Self {
            value.clone()
        }
    }

    ///`TaskEvent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "payload",
    ///    "seq",
    ///    "stream",
    ///    "time"
    ///  ],
    ///  "properties": {
    ///    "payload": {
    ///      "type": "string"
    ///    },
    ///    "seq": {
    ///      "type": "integer",
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "stream": {
    ///      "type": "string"
    ///    },
    ///    "time": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TaskEvent {
        pub payload: ::std::string::String,
        pub seq: u32,
        pub stream: ::std::string::String,
        pub time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&TaskEvent> for TaskEvent {
        fn from(value: &TaskEvent) -> Self {
            value.clone()
        }
    }

    ///`TaskOutput`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "path",
    ///    "size"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TaskOutput {
        pub id: ::std::string::String,
        pub path: ::std::string::String,
        pub size: u64,
    }

    impl ::std::convert::From<&TaskOutput> for TaskOutput {
        fn from(value: &TaskOutput) -> Self {
            value.clone()
        }
    }

    ///`TaskSubmit`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "default",
    ///    "name",
    ///    "script"
    ///  ],
    ///  "properties": {
    ///    "default": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "output_rules": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "script": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TaskSubmit {
        pub default: bool,
        pub name: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub output_rules: ::std::vec::Vec<::std::string::String>,
        pub script: ::std::string::String,
    }

    impl ::std::convert::From<&TaskSubmit> for TaskSubmit {
        fn from(value: &TaskSubmit) -> Self {
            value.clone()
        }
    }

    ///`TaskSubmitResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TaskSubmitResult {
        pub id: ::std::string::String,
    }

    impl ::std::convert::From<&TaskSubmitResult> for TaskSubmitResult {
        fn from(value: &TaskSubmitResult) -> Self {
            value.clone()
        }
    }

    ///`UploadedChunk`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UploadedChunk {
        pub id: ::std::string::String,
    }

    impl ::std::convert::From<&UploadedChunk> for UploadedChunk {
        fn from(value: &UploadedChunk) -> Self {
            value.clone()
        }
    }

    ///`UserCreate`
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
    pub struct UserCreate {
        pub name: ::std::string::String,
    }

    impl ::std::convert::From<&UserCreate> for UserCreate {
        fn from(value: &UserCreate) -> Self {
            value.clone()
        }
    }

    ///`UserCreateResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name",
    ///    "token"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "token": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserCreateResult {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
        pub token: ::std::string::String,
    }

    impl ::std::convert::From<&UserCreateResult> for UserCreateResult {
        fn from(value: &UserCreateResult) -> Self {
            value.clone()
        }
    }

    ///`WhoamiResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WhoamiResult {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
    }

    impl ::std::convert::From<&WhoamiResult> for WhoamiResult {
        fn from(value: &WhoamiResult) -> Self {
            value.clone()
        }
    }

    ///`Worker`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "deleted",
    ///    "id",
    ///    "recycle",
    ///    "tasks"
    ///  ],
    ///  "properties": {
    ///    "deleted": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "instance_id": {
    ///      "type": "string"
    ///    },
    ///    "lastping": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "recycle": {
    ///      "type": "boolean"
    ///    },
    ///    "tasks": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/WorkerTask"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Worker {
        pub deleted: bool,
        pub id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lastping: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub recycle: bool,
        pub tasks: ::std::vec::Vec<WorkerTask>,
    }

    impl ::std::convert::From<&Worker> for Worker {
        fn from(value: &Worker) -> Self {
            value.clone()
        }
    }

    ///`WorkerAddOutput`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "chunks",
    ///    "path",
    ///    "size"
    ///  ],
    ///  "properties": {
    ///    "chunks": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "path": {
    ///      "type": "string"
    ///    },
    ///    "size": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerAddOutput {
        pub chunks: ::std::vec::Vec<::std::string::String>,
        pub path: ::std::string::String,
        pub size: i64,
    }

    impl ::std::convert::From<&WorkerAddOutput> for WorkerAddOutput {
        fn from(value: &WorkerAddOutput) -> Self {
            value.clone()
        }
    }

    ///`WorkerAppendTask`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "payload",
    ///    "stream",
    ///    "time"
    ///  ],
    ///  "properties": {
    ///    "payload": {
    ///      "type": "string"
    ///    },
    ///    "stream": {
    ///      "type": "string"
    ///    },
    ///    "time": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerAppendTask {
        pub payload: ::std::string::String,
        pub stream: ::std::string::String,
        pub time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&WorkerAppendTask> for WorkerAppendTask {
        fn from(value: &WorkerAppendTask) -> Self {
            value.clone()
        }
    }

    ///`WorkerBootstrap`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "bootstrap",
    ///    "token"
    ///  ],
    ///  "properties": {
    ///    "bootstrap": {
    ///      "type": "string"
    ///    },
    ///    "token": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerBootstrap {
        pub bootstrap: ::std::string::String,
        pub token: ::std::string::String,
    }

    impl ::std::convert::From<&WorkerBootstrap> for WorkerBootstrap {
        fn from(value: &WorkerBootstrap) -> Self {
            value.clone()
        }
    }

    ///`WorkerBootstrapResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerBootstrapResult {
        pub id: ::std::string::String,
    }

    impl ::std::convert::From<&WorkerBootstrapResult> for WorkerBootstrapResult {
        fn from(value: &WorkerBootstrapResult) -> Self {
            value.clone()
        }
    }

    ///`WorkerCompleteTask`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "failed"
    ///  ],
    ///  "properties": {
    ///    "failed": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerCompleteTask {
        pub failed: bool,
    }

    impl ::std::convert::From<&WorkerCompleteTask> for WorkerCompleteTask {
        fn from(value: &WorkerCompleteTask) -> Self {
            value.clone()
        }
    }

    ///`WorkerPingResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "poweroff"
    ///  ],
    ///  "properties": {
    ///    "poweroff": {
    ///      "type": "boolean"
    ///    },
    ///    "task": {
    ///      "$ref": "#/components/schemas/WorkerPingTask"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerPingResult {
        pub poweroff: bool,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub task: ::std::option::Option<WorkerPingTask>,
    }

    impl ::std::convert::From<&WorkerPingResult> for WorkerPingResult {
        fn from(value: &WorkerPingResult) -> Self {
            value.clone()
        }
    }

    ///`WorkerPingTask`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "output_rules",
    ///    "script"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "output_rules": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "script": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerPingTask {
        pub id: ::std::string::String,
        pub output_rules: ::std::vec::Vec<::std::string::String>,
        pub script: ::std::string::String,
    }

    impl ::std::convert::From<&WorkerPingTask> for WorkerPingTask {
        fn from(value: &WorkerPingTask) -> Self {
            value.clone()
        }
    }

    ///`WorkerTask`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "name",
    ///    "owner"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "owner": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkerTask {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
        pub owner: ::std::string::String,
    }

    impl ::std::convert::From<&WorkerTask> for WorkerTask {
        fn from(value: &WorkerTask) -> Self {
            value.clone()
        }
    }

    ///`WorkersResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "workers"
    ///  ],
    ///  "properties": {
    ///    "workers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Worker"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct WorkersResult {
        pub workers: ::std::vec::Vec<Worker>,
    }

    impl ::std::convert::From<&WorkersResult> for WorkersResult {
        fn from(value: &WorkersResult) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
///Client for Buildomat
///
///Version: 1.0
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
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.0"
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
#[allow(elided_named_lifetimes)]
impl Client {
    ///Sends a `POST` request to `/v1/control/hold`
    pub async fn control_hold<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/control/hold", self.baseurl,);
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
            operation_id: "control_hold",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/control/resume`
    pub async fn control_resume<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/control/resume", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "control_resume",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/task/{Task}`
    pub async fn task_get<'a>(
        &'a self,
        task: &'a str,
    ) -> Result<ResponseValue<types::Task>, Error<()>> {
        let url = format!(
            "{}/v1/task/{}",
            self.baseurl,
            encode_path(&task.to_string()),
        );
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
            operation_id: "task_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/tasks`
    pub async fn tasks_get<'a>(
        &'a self,
    ) -> Result<ResponseValue<::std::vec::Vec<types::Task>>, Error<()>> {
        let url = format!("{}/v1/tasks", self.baseurl,);
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
            operation_id: "tasks_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/tasks`
    pub async fn task_submit<'a>(
        &'a self,
        body: &'a types::TaskSubmit,
    ) -> Result<ResponseValue<types::TaskSubmitResult>, Error<()>> {
        let url = format!("{}/v1/tasks", self.baseurl,);
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "task_submit",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/tasks/{task}/events`
    pub async fn task_events_get<'a>(
        &'a self,
        task: &'a str,
        minseq: Option<u32>,
    ) -> Result<ResponseValue<::std::vec::Vec<types::TaskEvent>>, Error<()>> {
        let url = format!(
            "{}/v1/tasks/{}/events",
            self.baseurl,
            encode_path(&task.to_string()),
        );
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
            .query(&progenitor_client::QueryParam::new("minseq", &minseq))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "task_events_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/tasks/{task}/outputs`
    pub async fn task_outputs_get<'a>(
        &'a self,
        task: &'a str,
    ) -> Result<ResponseValue<::std::vec::Vec<types::TaskOutput>>, Error<()>> {
        let url = format!(
            "{}/v1/tasks/{}/outputs",
            self.baseurl,
            encode_path(&task.to_string()),
        );
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
            operation_id: "task_outputs_get",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/tasks/{task}/outputs/{output}`
    pub async fn task_output_download<'a>(
        &'a self,
        task: &'a str,
        output: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/v1/tasks/{}/outputs/{}",
            self.baseurl,
            encode_path(&task.to_string()),
            encode_path(&output.to_string()),
        );
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "task_output_download",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/users`
    pub async fn user_create<'a>(
        &'a self,
        body: &'a types::UserCreate,
    ) -> Result<ResponseValue<types::UserCreateResult>, Error<()>> {
        let url = format!("{}/v1/users", self.baseurl,);
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "user_create",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/whoami`
    pub async fn whoami<'a>(&'a self) -> Result<ResponseValue<types::WhoamiResult>, Error<()>> {
        let url = format!("{}/v1/whoami", self.baseurl,);
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
            operation_id: "whoami",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/v1/whoami/name`
    pub async fn whoami_put_name<'a>(
        &'a self,
        body: String,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/whoami/name", self.baseurl,);
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
                ::reqwest::header::CONTENT_TYPE,
                ::reqwest::header::HeaderValue::from_static("text/plain"),
            )
            .body(body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "whoami_put_name",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/worker/bootstrap`
    pub async fn worker_bootstrap<'a>(
        &'a self,
        body: &'a types::WorkerBootstrap,
    ) -> Result<ResponseValue<types::WorkerBootstrapResult>, Error<()>> {
        let url = format!("{}/v1/worker/bootstrap", self.baseurl,);
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "worker_bootstrap",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/worker/ping`
    pub async fn worker_ping<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::WorkerPingResult>, Error<()>> {
        let url = format!("{}/v1/worker/ping", self.baseurl,);
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
            operation_id: "worker_ping",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/worker/task/{task}/append`
    pub async fn worker_task_append<'a>(
        &'a self,
        task: &'a str,
        body: &'a types::WorkerAppendTask,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/v1/worker/task/{}/append",
            self.baseurl,
            encode_path(&task.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "worker_task_append",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/worker/task/{task}/chunk`
    pub async fn worker_task_upload_chunk<'a, B: Into<reqwest::Body>>(
        &'a self,
        task: &'a str,
        body: B,
    ) -> Result<ResponseValue<types::UploadedChunk>, Error<()>> {
        let url = format!(
            "{}/v1/worker/task/{}/chunk",
            self.baseurl,
            encode_path(&task.to_string()),
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
            .header(
                ::reqwest::header::CONTENT_TYPE,
                ::reqwest::header::HeaderValue::from_static("application/octet-stream"),
            )
            .body(body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "worker_task_upload_chunk",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/worker/task/{task}/complete`
    pub async fn worker_task_complete<'a>(
        &'a self,
        task: &'a str,
        body: &'a types::WorkerCompleteTask,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/v1/worker/task/{}/complete",
            self.baseurl,
            encode_path(&task.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "worker_task_complete",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/worker/task/{task}/output`
    pub async fn worker_task_add_output<'a>(
        &'a self,
        task: &'a str,
        body: &'a types::WorkerAddOutput,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/v1/worker/task/{}/output",
            self.baseurl,
            encode_path(&task.to_string()),
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
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "worker_task_add_output",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/workers`
    pub async fn workers_list<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::WorkersResult>, Error<()>> {
        let url = format!("{}/v1/workers", self.baseurl,);
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
            operation_id: "workers_list",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/v1/workers/recycle`
    pub async fn workers_recycle<'a>(&'a self) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/workers/recycle", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "workers_recycle",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/things`
    pub async fn get_thing_or_things<'a>(
        &'a self,
        id: Option<&'a types::GetThingOrThingsId>,
    ) -> Result<ResponseValue<::std::string::String>, Error<()>> {
        let url = format!("{}/v1/things", self.baseurl,);
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
            .query(&progenitor_client::QueryParam::new("id", &id))
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "get_thing_or_things",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/v1/header-arg`
    pub async fn header_arg<'a>(
        &'a self,
        accept_language: Option<types::HeaderArgAcceptLanguage>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/v1/header-arg", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        if let Some(value) = accept_language {
            header_map.append("accept-language", value.to_string().try_into()?);
        }

        #[allow(unused_mut)]
        let mut request = self.client.get(url).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "header_arg",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::ErrorResponse(ResponseValue::empty(response))),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
