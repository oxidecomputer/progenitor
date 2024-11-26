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
        /// Error from a TryFrom or FromStr implementation.
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

    ///ObjWithOptionArray
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
        pub stranger_things: Vec<Option<Task>>,
        pub things: Vec<Option<Task>>,
    }

    impl From<&ObjWithOptionArray> for ObjWithOptionArray {
        fn from(value: &ObjWithOptionArray) -> Self {
            value.clone()
        }
    }

    impl ObjWithOptionArray {
        pub fn builder() -> builder::ObjWithOptionArray {
            Default::default()
        }
    }

    ///Task
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
        pub id: String,
        pub name: String,
        pub output_rules: Vec<String>,
        pub script: String,
        pub state: String,
    }

    impl From<&Task> for Task {
        fn from(value: &Task) -> Self {
            value.clone()
        }
    }

    impl Task {
        pub fn builder() -> builder::Task {
            Default::default()
        }
    }

    ///TaskEvent
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
        pub payload: String,
        pub seq: u32,
        pub stream: String,
        pub time: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&TaskEvent> for TaskEvent {
        fn from(value: &TaskEvent) -> Self {
            value.clone()
        }
    }

    impl TaskEvent {
        pub fn builder() -> builder::TaskEvent {
            Default::default()
        }
    }

    ///TaskOutput
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
        pub id: String,
        pub path: String,
        pub size: u64,
    }

    impl From<&TaskOutput> for TaskOutput {
        fn from(value: &TaskOutput) -> Self {
            value.clone()
        }
    }

    impl TaskOutput {
        pub fn builder() -> builder::TaskOutput {
            Default::default()
        }
    }

    ///TaskSubmit
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "script"
    ///  ],
    ///  "properties": {
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
        pub name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub output_rules: Vec<String>,
        pub script: String,
    }

    impl From<&TaskSubmit> for TaskSubmit {
        fn from(value: &TaskSubmit) -> Self {
            value.clone()
        }
    }

    impl TaskSubmit {
        pub fn builder() -> builder::TaskSubmit {
            Default::default()
        }
    }

    ///TaskSubmitResult
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
        pub id: String,
    }

    impl From<&TaskSubmitResult> for TaskSubmitResult {
        fn from(value: &TaskSubmitResult) -> Self {
            value.clone()
        }
    }

    impl TaskSubmitResult {
        pub fn builder() -> builder::TaskSubmitResult {
            Default::default()
        }
    }

    ///UploadedChunk
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
        pub id: String,
    }

    impl From<&UploadedChunk> for UploadedChunk {
        fn from(value: &UploadedChunk) -> Self {
            value.clone()
        }
    }

    impl UploadedChunk {
        pub fn builder() -> builder::UploadedChunk {
            Default::default()
        }
    }

    ///UserCreate
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
        pub name: String,
    }

    impl From<&UserCreate> for UserCreate {
        fn from(value: &UserCreate) -> Self {
            value.clone()
        }
    }

    impl UserCreate {
        pub fn builder() -> builder::UserCreate {
            Default::default()
        }
    }

    ///UserCreateResult
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
        pub id: String,
        pub name: String,
        pub token: String,
    }

    impl From<&UserCreateResult> for UserCreateResult {
        fn from(value: &UserCreateResult) -> Self {
            value.clone()
        }
    }

    impl UserCreateResult {
        pub fn builder() -> builder::UserCreateResult {
            Default::default()
        }
    }

    ///WhoamiResult
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
        pub id: String,
        pub name: String,
    }

    impl From<&WhoamiResult> for WhoamiResult {
        fn from(value: &WhoamiResult) -> Self {
            value.clone()
        }
    }

    impl WhoamiResult {
        pub fn builder() -> builder::WhoamiResult {
            Default::default()
        }
    }

    ///Worker
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
        pub id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub instance_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lastping: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub recycle: bool,
        pub tasks: Vec<WorkerTask>,
    }

    impl From<&Worker> for Worker {
        fn from(value: &Worker) -> Self {
            value.clone()
        }
    }

    impl Worker {
        pub fn builder() -> builder::Worker {
            Default::default()
        }
    }

    ///WorkerAddOutput
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
        pub chunks: Vec<String>,
        pub path: String,
        pub size: i64,
    }

    impl From<&WorkerAddOutput> for WorkerAddOutput {
        fn from(value: &WorkerAddOutput) -> Self {
            value.clone()
        }
    }

    impl WorkerAddOutput {
        pub fn builder() -> builder::WorkerAddOutput {
            Default::default()
        }
    }

    ///WorkerAppendTask
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
        pub payload: String,
        pub stream: String,
        pub time: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&WorkerAppendTask> for WorkerAppendTask {
        fn from(value: &WorkerAppendTask) -> Self {
            value.clone()
        }
    }

    impl WorkerAppendTask {
        pub fn builder() -> builder::WorkerAppendTask {
            Default::default()
        }
    }

    ///WorkerBootstrap
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
        pub bootstrap: String,
        pub token: String,
    }

    impl From<&WorkerBootstrap> for WorkerBootstrap {
        fn from(value: &WorkerBootstrap) -> Self {
            value.clone()
        }
    }

    impl WorkerBootstrap {
        pub fn builder() -> builder::WorkerBootstrap {
            Default::default()
        }
    }

    ///WorkerBootstrapResult
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
        pub id: String,
    }

    impl From<&WorkerBootstrapResult> for WorkerBootstrapResult {
        fn from(value: &WorkerBootstrapResult) -> Self {
            value.clone()
        }
    }

    impl WorkerBootstrapResult {
        pub fn builder() -> builder::WorkerBootstrapResult {
            Default::default()
        }
    }

    ///WorkerCompleteTask
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

    impl From<&WorkerCompleteTask> for WorkerCompleteTask {
        fn from(value: &WorkerCompleteTask) -> Self {
            value.clone()
        }
    }

    impl WorkerCompleteTask {
        pub fn builder() -> builder::WorkerCompleteTask {
            Default::default()
        }
    }

    ///WorkerPingResult
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub task: Option<WorkerPingTask>,
    }

    impl From<&WorkerPingResult> for WorkerPingResult {
        fn from(value: &WorkerPingResult) -> Self {
            value.clone()
        }
    }

    impl WorkerPingResult {
        pub fn builder() -> builder::WorkerPingResult {
            Default::default()
        }
    }

    ///WorkerPingTask
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
        pub id: String,
        pub output_rules: Vec<String>,
        pub script: String,
    }

    impl From<&WorkerPingTask> for WorkerPingTask {
        fn from(value: &WorkerPingTask) -> Self {
            value.clone()
        }
    }

    impl WorkerPingTask {
        pub fn builder() -> builder::WorkerPingTask {
            Default::default()
        }
    }

    ///WorkerTask
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
        pub id: String,
        pub name: String,
        pub owner: String,
    }

    impl From<&WorkerTask> for WorkerTask {
        fn from(value: &WorkerTask) -> Self {
            value.clone()
        }
    }

    impl WorkerTask {
        pub fn builder() -> builder::WorkerTask {
            Default::default()
        }
    }

    ///WorkersResult
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
        pub workers: Vec<Worker>,
    }

    impl From<&WorkersResult> for WorkersResult {
        fn from(value: &WorkersResult) -> Self {
            value.clone()
        }
    }

    impl WorkersResult {
        pub fn builder() -> builder::WorkersResult {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct ObjWithOptionArray {
            stranger_things: Result<Vec<Option<super::Task>>, String>,
            things: Result<Vec<Option<super::Task>>, String>,
        }

        impl Default for ObjWithOptionArray {
            fn default() -> Self {
                Self {
                    stranger_things: Err("no value supplied for stranger_things".to_string()),
                    things: Err("no value supplied for things".to_string()),
                }
            }
        }

        impl ObjWithOptionArray {
            pub fn stranger_things<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<Option<super::Task>>>,
                T::Error: std::fmt::Display,
            {
                self.stranger_things = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stranger_things: {}", e)
                });
                self
            }
            pub fn things<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<Option<super::Task>>>,
                T::Error: std::fmt::Display,
            {
                self.things = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for things: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ObjWithOptionArray> for super::ObjWithOptionArray {
            type Error = super::error::ConversionError;
            fn try_from(value: ObjWithOptionArray) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    stranger_things: value.stranger_things?,
                    things: value.things?,
                })
            }
        }

        impl From<super::ObjWithOptionArray> for ObjWithOptionArray {
            fn from(value: super::ObjWithOptionArray) -> Self {
                Self {
                    stranger_things: Ok(value.stranger_things),
                    things: Ok(value.things),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Task {
            id: Result<String, String>,
            name: Result<String, String>,
            output_rules: Result<Vec<String>, String>,
            script: Result<String, String>,
            state: Result<String, String>,
        }

        impl Default for Task {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    output_rules: Err("no value supplied for output_rules".to_string()),
                    script: Err("no value supplied for script".to_string()),
                    state: Err("no value supplied for state".to_string()),
                }
            }
        }

        impl Task {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn output_rules<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.output_rules = value.try_into().map_err(|e| {
                    format!("error converting supplied value for output_rules: {}", e)
                });
                self
            }
            pub fn script<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.script = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for script: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Task> for super::Task {
            type Error = super::error::ConversionError;
            fn try_from(value: Task) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    name: value.name?,
                    output_rules: value.output_rules?,
                    script: value.script?,
                    state: value.state?,
                })
            }
        }

        impl From<super::Task> for Task {
            fn from(value: super::Task) -> Self {
                Self {
                    id: Ok(value.id),
                    name: Ok(value.name),
                    output_rules: Ok(value.output_rules),
                    script: Ok(value.script),
                    state: Ok(value.state),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TaskEvent {
            payload: Result<String, String>,
            seq: Result<u32, String>,
            stream: Result<String, String>,
            time: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for TaskEvent {
            fn default() -> Self {
                Self {
                    payload: Err("no value supplied for payload".to_string()),
                    seq: Err("no value supplied for seq".to_string()),
                    stream: Err("no value supplied for stream".to_string()),
                    time: Err("no value supplied for time".to_string()),
                }
            }
        }

        impl TaskEvent {
            pub fn payload<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.payload = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for payload: {}", e));
                self
            }
            pub fn seq<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u32>,
                T::Error: std::fmt::Display,
            {
                self.seq = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for seq: {}", e));
                self
            }
            pub fn stream<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.stream = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stream: {}", e));
                self
            }
            pub fn time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<TaskEvent> for super::TaskEvent {
            type Error = super::error::ConversionError;
            fn try_from(value: TaskEvent) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    payload: value.payload?,
                    seq: value.seq?,
                    stream: value.stream?,
                    time: value.time?,
                })
            }
        }

        impl From<super::TaskEvent> for TaskEvent {
            fn from(value: super::TaskEvent) -> Self {
                Self {
                    payload: Ok(value.payload),
                    seq: Ok(value.seq),
                    stream: Ok(value.stream),
                    time: Ok(value.time),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TaskOutput {
            id: Result<String, String>,
            path: Result<String, String>,
            size: Result<u64, String>,
        }

        impl Default for TaskOutput {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    path: Err("no value supplied for path".to_string()),
                    size: Err("no value supplied for size".to_string()),
                }
            }
        }

        impl TaskOutput {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u64>,
                T::Error: std::fmt::Display,
            {
                self.size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<TaskOutput> for super::TaskOutput {
            type Error = super::error::ConversionError;
            fn try_from(value: TaskOutput) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    path: value.path?,
                    size: value.size?,
                })
            }
        }

        impl From<super::TaskOutput> for TaskOutput {
            fn from(value: super::TaskOutput) -> Self {
                Self {
                    id: Ok(value.id),
                    path: Ok(value.path),
                    size: Ok(value.size),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TaskSubmit {
            name: Result<String, String>,
            output_rules: Result<Vec<String>, String>,
            script: Result<String, String>,
        }

        impl Default for TaskSubmit {
            fn default() -> Self {
                Self {
                    name: Err("no value supplied for name".to_string()),
                    output_rules: Ok(Default::default()),
                    script: Err("no value supplied for script".to_string()),
                }
            }
        }

        impl TaskSubmit {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn output_rules<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.output_rules = value.try_into().map_err(|e| {
                    format!("error converting supplied value for output_rules: {}", e)
                });
                self
            }
            pub fn script<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.script = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for script: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<TaskSubmit> for super::TaskSubmit {
            type Error = super::error::ConversionError;
            fn try_from(value: TaskSubmit) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    output_rules: value.output_rules?,
                    script: value.script?,
                })
            }
        }

        impl From<super::TaskSubmit> for TaskSubmit {
            fn from(value: super::TaskSubmit) -> Self {
                Self {
                    name: Ok(value.name),
                    output_rules: Ok(value.output_rules),
                    script: Ok(value.script),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TaskSubmitResult {
            id: Result<String, String>,
        }

        impl Default for TaskSubmitResult {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                }
            }
        }

        impl TaskSubmitResult {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<TaskSubmitResult> for super::TaskSubmitResult {
            type Error = super::error::ConversionError;
            fn try_from(value: TaskSubmitResult) -> Result<Self, super::error::ConversionError> {
                Ok(Self { id: value.id? })
            }
        }

        impl From<super::TaskSubmitResult> for TaskSubmitResult {
            fn from(value: super::TaskSubmitResult) -> Self {
                Self { id: Ok(value.id) }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UploadedChunk {
            id: Result<String, String>,
        }

        impl Default for UploadedChunk {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                }
            }
        }

        impl UploadedChunk {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UploadedChunk> for super::UploadedChunk {
            type Error = super::error::ConversionError;
            fn try_from(value: UploadedChunk) -> Result<Self, super::error::ConversionError> {
                Ok(Self { id: value.id? })
            }
        }

        impl From<super::UploadedChunk> for UploadedChunk {
            fn from(value: super::UploadedChunk) -> Self {
                Self { id: Ok(value.id) }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UserCreate {
            name: Result<String, String>,
        }

        impl Default for UserCreate {
            fn default() -> Self {
                Self {
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl UserCreate {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UserCreate> for super::UserCreate {
            type Error = super::error::ConversionError;
            fn try_from(value: UserCreate) -> Result<Self, super::error::ConversionError> {
                Ok(Self { name: value.name? })
            }
        }

        impl From<super::UserCreate> for UserCreate {
            fn from(value: super::UserCreate) -> Self {
                Self {
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UserCreateResult {
            id: Result<String, String>,
            name: Result<String, String>,
            token: Result<String, String>,
        }

        impl Default for UserCreateResult {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl UserCreateResult {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn token<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UserCreateResult> for super::UserCreateResult {
            type Error = super::error::ConversionError;
            fn try_from(value: UserCreateResult) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    name: value.name?,
                    token: value.token?,
                })
            }
        }

        impl From<super::UserCreateResult> for UserCreateResult {
            fn from(value: super::UserCreateResult) -> Self {
                Self {
                    id: Ok(value.id),
                    name: Ok(value.name),
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WhoamiResult {
            id: Result<String, String>,
            name: Result<String, String>,
        }

        impl Default for WhoamiResult {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl WhoamiResult {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WhoamiResult> for super::WhoamiResult {
            type Error = super::error::ConversionError;
            fn try_from(value: WhoamiResult) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    name: value.name?,
                })
            }
        }

        impl From<super::WhoamiResult> for WhoamiResult {
            fn from(value: super::WhoamiResult) -> Self {
                Self {
                    id: Ok(value.id),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Worker {
            deleted: Result<bool, String>,
            id: Result<String, String>,
            instance_id: Result<Option<String>, String>,
            lastping: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            recycle: Result<bool, String>,
            tasks: Result<Vec<super::WorkerTask>, String>,
        }

        impl Default for Worker {
            fn default() -> Self {
                Self {
                    deleted: Err("no value supplied for deleted".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    instance_id: Ok(Default::default()),
                    lastping: Ok(Default::default()),
                    recycle: Err("no value supplied for recycle".to_string()),
                    tasks: Err("no value supplied for tasks".to_string()),
                }
            }
        }

        impl Worker {
            pub fn deleted<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.deleted = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn instance_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.instance_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instance_id: {}", e));
                self
            }
            pub fn lastping<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.lastping = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for lastping: {}", e));
                self
            }
            pub fn recycle<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.recycle = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for recycle: {}", e));
                self
            }
            pub fn tasks<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::WorkerTask>>,
                T::Error: std::fmt::Display,
            {
                self.tasks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tasks: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Worker> for super::Worker {
            type Error = super::error::ConversionError;
            fn try_from(value: Worker) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    deleted: value.deleted?,
                    id: value.id?,
                    instance_id: value.instance_id?,
                    lastping: value.lastping?,
                    recycle: value.recycle?,
                    tasks: value.tasks?,
                })
            }
        }

        impl From<super::Worker> for Worker {
            fn from(value: super::Worker) -> Self {
                Self {
                    deleted: Ok(value.deleted),
                    id: Ok(value.id),
                    instance_id: Ok(value.instance_id),
                    lastping: Ok(value.lastping),
                    recycle: Ok(value.recycle),
                    tasks: Ok(value.tasks),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WorkerAddOutput {
            chunks: Result<Vec<String>, String>,
            path: Result<String, String>,
            size: Result<i64, String>,
        }

        impl Default for WorkerAddOutput {
            fn default() -> Self {
                Self {
                    chunks: Err("no value supplied for chunks".to_string()),
                    path: Err("no value supplied for path".to_string()),
                    size: Err("no value supplied for size".to_string()),
                }
            }
        }

        impl WorkerAddOutput {
            pub fn chunks<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.chunks = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for chunks: {}", e));
                self
            }
            pub fn path<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.path = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for path: {}", e));
                self
            }
            pub fn size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i64>,
                T::Error: std::fmt::Display,
            {
                self.size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WorkerAddOutput> for super::WorkerAddOutput {
            type Error = super::error::ConversionError;
            fn try_from(value: WorkerAddOutput) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    chunks: value.chunks?,
                    path: value.path?,
                    size: value.size?,
                })
            }
        }

        impl From<super::WorkerAddOutput> for WorkerAddOutput {
            fn from(value: super::WorkerAddOutput) -> Self {
                Self {
                    chunks: Ok(value.chunks),
                    path: Ok(value.path),
                    size: Ok(value.size),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WorkerAppendTask {
            payload: Result<String, String>,
            stream: Result<String, String>,
            time: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for WorkerAppendTask {
            fn default() -> Self {
                Self {
                    payload: Err("no value supplied for payload".to_string()),
                    stream: Err("no value supplied for stream".to_string()),
                    time: Err("no value supplied for time".to_string()),
                }
            }
        }

        impl WorkerAppendTask {
            pub fn payload<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.payload = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for payload: {}", e));
                self
            }
            pub fn stream<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.stream = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stream: {}", e));
                self
            }
            pub fn time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WorkerAppendTask> for super::WorkerAppendTask {
            type Error = super::error::ConversionError;
            fn try_from(value: WorkerAppendTask) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    payload: value.payload?,
                    stream: value.stream?,
                    time: value.time?,
                })
            }
        }

        impl From<super::WorkerAppendTask> for WorkerAppendTask {
            fn from(value: super::WorkerAppendTask) -> Self {
                Self {
                    payload: Ok(value.payload),
                    stream: Ok(value.stream),
                    time: Ok(value.time),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WorkerBootstrap {
            bootstrap: Result<String, String>,
            token: Result<String, String>,
        }

        impl Default for WorkerBootstrap {
            fn default() -> Self {
                Self {
                    bootstrap: Err("no value supplied for bootstrap".to_string()),
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl WorkerBootstrap {
            pub fn bootstrap<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.bootstrap = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bootstrap: {}", e));
                self
            }
            pub fn token<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WorkerBootstrap> for super::WorkerBootstrap {
            type Error = super::error::ConversionError;
            fn try_from(value: WorkerBootstrap) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    bootstrap: value.bootstrap?,
                    token: value.token?,
                })
            }
        }

        impl From<super::WorkerBootstrap> for WorkerBootstrap {
            fn from(value: super::WorkerBootstrap) -> Self {
                Self {
                    bootstrap: Ok(value.bootstrap),
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WorkerBootstrapResult {
            id: Result<String, String>,
        }

        impl Default for WorkerBootstrapResult {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                }
            }
        }

        impl WorkerBootstrapResult {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WorkerBootstrapResult> for super::WorkerBootstrapResult {
            type Error = super::error::ConversionError;
            fn try_from(
                value: WorkerBootstrapResult,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self { id: value.id? })
            }
        }

        impl From<super::WorkerBootstrapResult> for WorkerBootstrapResult {
            fn from(value: super::WorkerBootstrapResult) -> Self {
                Self { id: Ok(value.id) }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WorkerCompleteTask {
            failed: Result<bool, String>,
        }

        impl Default for WorkerCompleteTask {
            fn default() -> Self {
                Self {
                    failed: Err("no value supplied for failed".to_string()),
                }
            }
        }

        impl WorkerCompleteTask {
            pub fn failed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.failed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for failed: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WorkerCompleteTask> for super::WorkerCompleteTask {
            type Error = super::error::ConversionError;
            fn try_from(value: WorkerCompleteTask) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    failed: value.failed?,
                })
            }
        }

        impl From<super::WorkerCompleteTask> for WorkerCompleteTask {
            fn from(value: super::WorkerCompleteTask) -> Self {
                Self {
                    failed: Ok(value.failed),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WorkerPingResult {
            poweroff: Result<bool, String>,
            task: Result<Option<super::WorkerPingTask>, String>,
        }

        impl Default for WorkerPingResult {
            fn default() -> Self {
                Self {
                    poweroff: Err("no value supplied for poweroff".to_string()),
                    task: Ok(Default::default()),
                }
            }
        }

        impl WorkerPingResult {
            pub fn poweroff<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.poweroff = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for poweroff: {}", e));
                self
            }
            pub fn task<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::WorkerPingTask>>,
                T::Error: std::fmt::Display,
            {
                self.task = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for task: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WorkerPingResult> for super::WorkerPingResult {
            type Error = super::error::ConversionError;
            fn try_from(value: WorkerPingResult) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    poweroff: value.poweroff?,
                    task: value.task?,
                })
            }
        }

        impl From<super::WorkerPingResult> for WorkerPingResult {
            fn from(value: super::WorkerPingResult) -> Self {
                Self {
                    poweroff: Ok(value.poweroff),
                    task: Ok(value.task),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WorkerPingTask {
            id: Result<String, String>,
            output_rules: Result<Vec<String>, String>,
            script: Result<String, String>,
        }

        impl Default for WorkerPingTask {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    output_rules: Err("no value supplied for output_rules".to_string()),
                    script: Err("no value supplied for script".to_string()),
                }
            }
        }

        impl WorkerPingTask {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn output_rules<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.output_rules = value.try_into().map_err(|e| {
                    format!("error converting supplied value for output_rules: {}", e)
                });
                self
            }
            pub fn script<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.script = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for script: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WorkerPingTask> for super::WorkerPingTask {
            type Error = super::error::ConversionError;
            fn try_from(value: WorkerPingTask) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    output_rules: value.output_rules?,
                    script: value.script?,
                })
            }
        }

        impl From<super::WorkerPingTask> for WorkerPingTask {
            fn from(value: super::WorkerPingTask) -> Self {
                Self {
                    id: Ok(value.id),
                    output_rules: Ok(value.output_rules),
                    script: Ok(value.script),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WorkerTask {
            id: Result<String, String>,
            name: Result<String, String>,
            owner: Result<String, String>,
        }

        impl Default for WorkerTask {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    owner: Err("no value supplied for owner".to_string()),
                }
            }
        }

        impl WorkerTask {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn owner<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.owner = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for owner: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WorkerTask> for super::WorkerTask {
            type Error = super::error::ConversionError;
            fn try_from(value: WorkerTask) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    name: value.name?,
                    owner: value.owner?,
                })
            }
        }

        impl From<super::WorkerTask> for WorkerTask {
            fn from(value: super::WorkerTask) -> Self {
                Self {
                    id: Ok(value.id),
                    name: Ok(value.name),
                    owner: Ok(value.owner),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct WorkersResult {
            workers: Result<Vec<super::Worker>, String>,
        }

        impl Default for WorkersResult {
            fn default() -> Self {
                Self {
                    workers: Err("no value supplied for workers".to_string()),
                }
            }
        }

        impl WorkersResult {
            pub fn workers<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::Worker>>,
                T::Error: std::fmt::Display,
            {
                self.workers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for workers: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<WorkersResult> for super::WorkersResult {
            type Error = super::error::ConversionError;
            fn try_from(value: WorkersResult) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    workers: value.workers?,
                })
            }
        }

        impl From<super::WorkersResult> for WorkersResult {
            fn from(value: super::WorkersResult) -> Self {
                Self {
                    workers: Ok(value.workers),
                }
            }
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
        "1.0"
    }
}

impl Client {
    ///Sends a `POST` request to `/v1/control/hold`
    ///
    ///```ignore
    /// let response = client.control_hold()
    ///    .send()
    ///    .await;
    /// ```
    pub fn control_hold(&self) -> builder::ControlHold {
        builder::ControlHold::new(self)
    }

    ///Sends a `POST` request to `/v1/control/resume`
    ///
    ///```ignore
    /// let response = client.control_resume()
    ///    .send()
    ///    .await;
    /// ```
    pub fn control_resume(&self) -> builder::ControlResume {
        builder::ControlResume::new(self)
    }

    ///Sends a `GET` request to `/v1/task/{Task}`
    ///
    ///```ignore
    /// let response = client.task_get()
    ///    .task(task)
    ///    .send()
    ///    .await;
    /// ```
    pub fn task_get(&self) -> builder::TaskGet {
        builder::TaskGet::new(self)
    }

    ///Sends a `GET` request to `/v1/tasks`
    ///
    ///```ignore
    /// let response = client.tasks_get()
    ///    .send()
    ///    .await;
    /// ```
    pub fn tasks_get(&self) -> builder::TasksGet {
        builder::TasksGet::new(self)
    }

    ///Sends a `POST` request to `/v1/tasks`
    ///
    ///```ignore
    /// let response = client.task_submit()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn task_submit(&self) -> builder::TaskSubmit {
        builder::TaskSubmit::new(self)
    }

    ///Sends a `GET` request to `/v1/tasks/{task}/events`
    ///
    ///```ignore
    /// let response = client.task_events_get()
    ///    .task(task)
    ///    .minseq(minseq)
    ///    .send()
    ///    .await;
    /// ```
    pub fn task_events_get(&self) -> builder::TaskEventsGet {
        builder::TaskEventsGet::new(self)
    }

    ///Sends a `GET` request to `/v1/tasks/{task}/outputs`
    ///
    ///```ignore
    /// let response = client.task_outputs_get()
    ///    .task(task)
    ///    .send()
    ///    .await;
    /// ```
    pub fn task_outputs_get(&self) -> builder::TaskOutputsGet {
        builder::TaskOutputsGet::new(self)
    }

    ///Sends a `GET` request to `/v1/tasks/{task}/outputs/{output}`
    ///
    ///```ignore
    /// let response = client.task_output_download()
    ///    .task(task)
    ///    .output(output)
    ///    .send()
    ///    .await;
    /// ```
    pub fn task_output_download(&self) -> builder::TaskOutputDownload {
        builder::TaskOutputDownload::new(self)
    }

    ///Sends a `POST` request to `/v1/users`
    ///
    ///```ignore
    /// let response = client.user_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_create(&self) -> builder::UserCreate {
        builder::UserCreate::new(self)
    }

    ///Sends a `GET` request to `/v1/whoami`
    ///
    ///```ignore
    /// let response = client.whoami()
    ///    .send()
    ///    .await;
    /// ```
    pub fn whoami(&self) -> builder::Whoami {
        builder::Whoami::new(self)
    }

    ///Sends a `PUT` request to `/v1/whoami/name`
    ///
    ///```ignore
    /// let response = client.whoami_put_name()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn whoami_put_name(&self) -> builder::WhoamiPutName {
        builder::WhoamiPutName::new(self)
    }

    ///Sends a `POST` request to `/v1/worker/bootstrap`
    ///
    ///```ignore
    /// let response = client.worker_bootstrap()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn worker_bootstrap(&self) -> builder::WorkerBootstrap {
        builder::WorkerBootstrap::new(self)
    }

    ///Sends a `GET` request to `/v1/worker/ping`
    ///
    ///```ignore
    /// let response = client.worker_ping()
    ///    .send()
    ///    .await;
    /// ```
    pub fn worker_ping(&self) -> builder::WorkerPing {
        builder::WorkerPing::new(self)
    }

    ///Sends a `POST` request to `/v1/worker/task/{task}/append`
    ///
    ///```ignore
    /// let response = client.worker_task_append()
    ///    .task(task)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn worker_task_append(&self) -> builder::WorkerTaskAppend {
        builder::WorkerTaskAppend::new(self)
    }

    ///Sends a `POST` request to `/v1/worker/task/{task}/chunk`
    ///
    ///```ignore
    /// let response = client.worker_task_upload_chunk()
    ///    .task(task)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn worker_task_upload_chunk(&self) -> builder::WorkerTaskUploadChunk {
        builder::WorkerTaskUploadChunk::new(self)
    }

    ///Sends a `POST` request to `/v1/worker/task/{task}/complete`
    ///
    ///```ignore
    /// let response = client.worker_task_complete()
    ///    .task(task)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn worker_task_complete(&self) -> builder::WorkerTaskComplete {
        builder::WorkerTaskComplete::new(self)
    }

    ///Sends a `POST` request to `/v1/worker/task/{task}/output`
    ///
    ///```ignore
    /// let response = client.worker_task_add_output()
    ///    .task(task)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn worker_task_add_output(&self) -> builder::WorkerTaskAddOutput {
        builder::WorkerTaskAddOutput::new(self)
    }

    ///Sends a `GET` request to `/v1/workers`
    ///
    ///```ignore
    /// let response = client.workers_list()
    ///    .send()
    ///    .await;
    /// ```
    pub fn workers_list(&self) -> builder::WorkersList {
        builder::WorkersList::new(self)
    }

    ///Sends a `POST` request to `/v1/workers/recycle`
    ///
    ///```ignore
    /// let response = client.workers_recycle()
    ///    .send()
    ///    .await;
    /// ```
    pub fn workers_recycle(&self) -> builder::WorkersRecycle {
        builder::WorkersRecycle::new(self)
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
    ///Builder for [`Client::control_hold`]
    ///
    ///[`Client::control_hold`]: super::Client::control_hold
    #[derive(Debug, Clone)]
    pub struct ControlHold<'a> {
        client: &'a super::Client,
    }

    impl<'a> ControlHold<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `POST` request to `/v1/control/hold`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/v1/control/hold", client.baseurl,);
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::control_resume`]
    ///
    ///[`Client::control_resume`]: super::Client::control_resume
    #[derive(Debug, Clone)]
    pub struct ControlResume<'a> {
        client: &'a super::Client,
    }

    impl<'a> ControlResume<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `POST` request to `/v1/control/resume`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/v1/control/resume", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client.client.post(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::task_get`]
    ///
    ///[`Client::task_get`]: super::Client::task_get
    #[derive(Debug, Clone)]
    pub struct TaskGet<'a> {
        client: &'a super::Client,
        task: Result<String, String>,
    }

    impl<'a> TaskGet<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                task: Err("task was not initialized".to_string()),
            }
        }

        pub fn task<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.task = value
                .try_into()
                .map_err(|_| "conversion to `String` for task failed".to_string());
            self
        }

        ///Sends a `GET` request to `/v1/task/{Task}`
        pub async fn send(self) -> Result<ResponseValue<types::Task>, Error<()>> {
            let Self { client, task } = self;
            let task = task.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/v1/task/{}",
                client.baseurl,
                encode_path(&task.to_string()),
            );
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::tasks_get`]
    ///
    ///[`Client::tasks_get`]: super::Client::tasks_get
    #[derive(Debug, Clone)]
    pub struct TasksGet<'a> {
        client: &'a super::Client,
    }

    impl<'a> TasksGet<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `GET` request to `/v1/tasks`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::Task>>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/v1/tasks", client.baseurl,);
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::task_submit`]
    ///
    ///[`Client::task_submit`]: super::Client::task_submit
    #[derive(Debug, Clone)]
    pub struct TaskSubmit<'a> {
        client: &'a super::Client,
        body: Result<types::builder::TaskSubmit, String>,
    }

    impl<'a> TaskSubmit<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::TaskSubmit::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TaskSubmit>,
            <V as std::convert::TryInto<types::TaskSubmit>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `TaskSubmit` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::TaskSubmit) -> types::builder::TaskSubmit,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/v1/tasks`
        pub async fn send(self) -> Result<ResponseValue<types::TaskSubmitResult>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::TaskSubmit::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/v1/tasks", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::task_events_get`]
    ///
    ///[`Client::task_events_get`]: super::Client::task_events_get
    #[derive(Debug, Clone)]
    pub struct TaskEventsGet<'a> {
        client: &'a super::Client,
        task: Result<String, String>,
        minseq: Result<Option<u32>, String>,
    }

    impl<'a> TaskEventsGet<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                task: Err("task was not initialized".to_string()),
                minseq: Ok(None),
            }
        }

        pub fn task<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.task = value
                .try_into()
                .map_err(|_| "conversion to `String` for task failed".to_string());
            self
        }

        pub fn minseq<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<u32>,
        {
            self.minseq = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `u32` for minseq failed".to_string());
            self
        }

        ///Sends a `GET` request to `/v1/tasks/{task}/events`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::TaskEvent>>, Error<()>> {
            let Self {
                client,
                task,
                minseq,
            } = self;
            let task = task.map_err(Error::InvalidRequest)?;
            let minseq = minseq.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/v1/tasks/{}/events",
                client.baseurl,
                encode_path(&task.to_string()),
            );
            let mut query = Vec::with_capacity(1usize);
            if let Some(minseq) = &minseq {
                query.push(("minseq", minseq.to_string()));
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::task_outputs_get`]
    ///
    ///[`Client::task_outputs_get`]: super::Client::task_outputs_get
    #[derive(Debug, Clone)]
    pub struct TaskOutputsGet<'a> {
        client: &'a super::Client,
        task: Result<String, String>,
    }

    impl<'a> TaskOutputsGet<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                task: Err("task was not initialized".to_string()),
            }
        }

        pub fn task<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.task = value
                .try_into()
                .map_err(|_| "conversion to `String` for task failed".to_string());
            self
        }

        ///Sends a `GET` request to `/v1/tasks/{task}/outputs`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::TaskOutput>>, Error<()>> {
            let Self { client, task } = self;
            let task = task.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/v1/tasks/{}/outputs",
                client.baseurl,
                encode_path(&task.to_string()),
            );
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::task_output_download`]
    ///
    ///[`Client::task_output_download`]: super::Client::task_output_download
    #[derive(Debug, Clone)]
    pub struct TaskOutputDownload<'a> {
        client: &'a super::Client,
        task: Result<String, String>,
        output: Result<String, String>,
    }

    impl<'a> TaskOutputDownload<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                task: Err("task was not initialized".to_string()),
                output: Err("output was not initialized".to_string()),
            }
        }

        pub fn task<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.task = value
                .try_into()
                .map_err(|_| "conversion to `String` for task failed".to_string());
            self
        }

        pub fn output<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.output = value
                .try_into()
                .map_err(|_| "conversion to `String` for output failed".to_string());
            self
        }

        ///Sends a `GET` request to `/v1/tasks/{task}/outputs/{output}`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self {
                client,
                task,
                output,
            } = self;
            let task = task.map_err(Error::InvalidRequest)?;
            let output = output.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/v1/tasks/{}/outputs/{}",
                client.baseurl,
                encode_path(&task.to_string()),
                encode_path(&output.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.get(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::user_create`]
    ///
    ///[`Client::user_create`]: super::Client::user_create
    #[derive(Debug, Clone)]
    pub struct UserCreate<'a> {
        client: &'a super::Client,
        body: Result<types::builder::UserCreate, String>,
    }

    impl<'a> UserCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::UserCreate::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UserCreate>,
            <V as std::convert::TryInto<types::UserCreate>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `UserCreate` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::UserCreate) -> types::builder::UserCreate,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/v1/users`
        pub async fn send(self) -> Result<ResponseValue<types::UserCreateResult>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::UserCreate::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/v1/users", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::whoami`]
    ///
    ///[`Client::whoami`]: super::Client::whoami
    #[derive(Debug, Clone)]
    pub struct Whoami<'a> {
        client: &'a super::Client,
    }

    impl<'a> Whoami<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `GET` request to `/v1/whoami`
        pub async fn send(self) -> Result<ResponseValue<types::WhoamiResult>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/v1/whoami", client.baseurl,);
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::whoami_put_name`]
    ///
    ///[`Client::whoami_put_name`]: super::Client::whoami_put_name
    #[derive(Debug)]
    pub struct WhoamiPutName<'a> {
        client: &'a super::Client,
        body: Result<reqwest::Body, String>,
    }

    impl<'a> WhoamiPutName<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `String` for body failed".to_string())
                .map(|v| v.into());
            self
        }

        ///Sends a `PUT` request to `/v1/whoami/name`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client, body } = self;
            let body = body.map_err(Error::InvalidRequest)?;
            let url = format!("{}/v1/whoami/name", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("text/plain"),
                )
                .body(body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::worker_bootstrap`]
    ///
    ///[`Client::worker_bootstrap`]: super::Client::worker_bootstrap
    #[derive(Debug, Clone)]
    pub struct WorkerBootstrap<'a> {
        client: &'a super::Client,
        body: Result<types::builder::WorkerBootstrap, String>,
    }

    impl<'a> WorkerBootstrap<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(types::builder::WorkerBootstrap::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::WorkerBootstrap>,
            <V as std::convert::TryInto<types::WorkerBootstrap>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `WorkerBootstrap` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::WorkerBootstrap) -> types::builder::WorkerBootstrap,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/v1/worker/bootstrap`
        pub async fn send(self) -> Result<ResponseValue<types::WorkerBootstrapResult>, Error<()>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::WorkerBootstrap::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/v1/worker/bootstrap", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::worker_ping`]
    ///
    ///[`Client::worker_ping`]: super::Client::worker_ping
    #[derive(Debug, Clone)]
    pub struct WorkerPing<'a> {
        client: &'a super::Client,
    }

    impl<'a> WorkerPing<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `GET` request to `/v1/worker/ping`
        pub async fn send(self) -> Result<ResponseValue<types::WorkerPingResult>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/v1/worker/ping", client.baseurl,);
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::worker_task_append`]
    ///
    ///[`Client::worker_task_append`]: super::Client::worker_task_append
    #[derive(Debug, Clone)]
    pub struct WorkerTaskAppend<'a> {
        client: &'a super::Client,
        task: Result<String, String>,
        body: Result<types::builder::WorkerAppendTask, String>,
    }

    impl<'a> WorkerTaskAppend<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                task: Err("task was not initialized".to_string()),
                body: Ok(types::builder::WorkerAppendTask::default()),
            }
        }

        pub fn task<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.task = value
                .try_into()
                .map_err(|_| "conversion to `String` for task failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::WorkerAppendTask>,
            <V as std::convert::TryInto<types::WorkerAppendTask>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `WorkerAppendTask` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::WorkerAppendTask,
            ) -> types::builder::WorkerAppendTask,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/v1/worker/task/{task}/append`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client, task, body } = self;
            let task = task.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::WorkerAppendTask::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/v1/worker/task/{}/append",
                client.baseurl,
                encode_path(&task.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).json(&body).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => Ok(ResponseValue::empty(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::worker_task_upload_chunk`]
    ///
    ///[`Client::worker_task_upload_chunk`]: super::Client::worker_task_upload_chunk
    #[derive(Debug)]
    pub struct WorkerTaskUploadChunk<'a> {
        client: &'a super::Client,
        task: Result<String, String>,
        body: Result<reqwest::Body, String>,
    }

    impl<'a> WorkerTaskUploadChunk<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                task: Err("task was not initialized".to_string()),
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn task<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.task = value
                .try_into()
                .map_err(|_| "conversion to `String` for task failed".to_string());
            self
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

        ///Sends a `POST` request to `/v1/worker/task/{task}/chunk`
        pub async fn send(self) -> Result<ResponseValue<types::UploadedChunk>, Error<()>> {
            let Self { client, task, body } = self;
            let task = task.map_err(Error::InvalidRequest)?;
            let body = body.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/v1/worker/task/{}/chunk",
                client.baseurl,
                encode_path(&task.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .header(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/octet-stream"),
                )
                .body(body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::worker_task_complete`]
    ///
    ///[`Client::worker_task_complete`]: super::Client::worker_task_complete
    #[derive(Debug, Clone)]
    pub struct WorkerTaskComplete<'a> {
        client: &'a super::Client,
        task: Result<String, String>,
        body: Result<types::builder::WorkerCompleteTask, String>,
    }

    impl<'a> WorkerTaskComplete<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                task: Err("task was not initialized".to_string()),
                body: Ok(types::builder::WorkerCompleteTask::default()),
            }
        }

        pub fn task<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.task = value
                .try_into()
                .map_err(|_| "conversion to `String` for task failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::WorkerCompleteTask>,
            <V as std::convert::TryInto<types::WorkerCompleteTask>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `WorkerCompleteTask` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::WorkerCompleteTask,
            ) -> types::builder::WorkerCompleteTask,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/v1/worker/task/{task}/complete`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client, task, body } = self;
            let task = task.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::WorkerCompleteTask::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/v1/worker/task/{}/complete",
                client.baseurl,
                encode_path(&task.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).json(&body).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::worker_task_add_output`]
    ///
    ///[`Client::worker_task_add_output`]: super::Client::worker_task_add_output
    #[derive(Debug, Clone)]
    pub struct WorkerTaskAddOutput<'a> {
        client: &'a super::Client,
        task: Result<String, String>,
        body: Result<types::builder::WorkerAddOutput, String>,
    }

    impl<'a> WorkerTaskAddOutput<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                task: Err("task was not initialized".to_string()),
                body: Ok(types::builder::WorkerAddOutput::default()),
            }
        }

        pub fn task<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.task = value
                .try_into()
                .map_err(|_| "conversion to `String` for task failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::WorkerAddOutput>,
            <V as std::convert::TryInto<types::WorkerAddOutput>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `WorkerAddOutput` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::WorkerAddOutput) -> types::builder::WorkerAddOutput,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/v1/worker/task/{task}/output`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client, task, body } = self;
            let task = task.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::WorkerAddOutput::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/v1/worker/task/{}/output",
                client.baseurl,
                encode_path(&task.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client.client.post(url).json(&body).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => Ok(ResponseValue::empty(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::workers_list`]
    ///
    ///[`Client::workers_list`]: super::Client::workers_list
    #[derive(Debug, Clone)]
    pub struct WorkersList<'a> {
        client: &'a super::Client,
    }

    impl<'a> WorkersList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `GET` request to `/v1/workers`
        pub async fn send(self) -> Result<ResponseValue<types::WorkersResult>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/v1/workers", client.baseurl,);
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
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::workers_recycle`]
    ///
    ///[`Client::workers_recycle`]: super::Client::workers_recycle
    #[derive(Debug, Clone)]
    pub struct WorkersRecycle<'a> {
        client: &'a super::Client,
    }

    impl<'a> WorkersRecycle<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `POST` request to `/v1/workers/recycle`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/v1/workers/recycle", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client.client.post(url).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
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
