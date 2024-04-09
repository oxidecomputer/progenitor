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

    ///EnrolBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "EnrolBody",
    ///  "type": "object",
    ///  "required": [
    ///    "host",
    ///    "key"
    ///  ],
    ///  "properties": {
    ///    "host": {
    ///      "type": "string"
    ///    },
    ///    "key": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct EnrolBody {
        pub host: String,
        pub key: String,
    }

    impl From<&EnrolBody> for EnrolBody {
        fn from(value: &EnrolBody) -> Self {
            value.clone()
        }
    }

    impl EnrolBody {
        pub fn builder() -> builder::EnrolBody {
            Default::default()
        }
    }

    ///GlobalJobsResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "GlobalJobsResult",
    ///  "type": "object",
    ///  "required": [
    ///    "summary"
    ///  ],
    ///  "properties": {
    ///    "summary": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReportSummary"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct GlobalJobsResult {
        pub summary: Vec<ReportSummary>,
    }

    impl From<&GlobalJobsResult> for GlobalJobsResult {
        fn from(value: &GlobalJobsResult) -> Self {
            value.clone()
        }
    }

    impl GlobalJobsResult {
        pub fn builder() -> builder::GlobalJobsResult {
            Default::default()
        }
    }

    ///OutputRecord
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "msg",
    ///    "stream",
    ///    "time"
    ///  ],
    ///  "properties": {
    ///    "msg": {
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
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct OutputRecord {
        pub msg: String,
        pub stream: String,
        pub time: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&OutputRecord> for OutputRecord {
        fn from(value: &OutputRecord) -> Self {
            value.clone()
        }
    }

    impl OutputRecord {
        pub fn builder() -> builder::OutputRecord {
            Default::default()
        }
    }

    ///PingResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "PingResult",
    ///  "type": "object",
    ///  "required": [
    ///    "host",
    ///    "ok"
    ///  ],
    ///  "properties": {
    ///    "host": {
    ///      "type": "string"
    ///    },
    ///    "ok": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct PingResult {
        pub host: String,
        pub ok: bool,
    }

    impl From<&PingResult> for PingResult {
        fn from(value: &PingResult) -> Self {
            value.clone()
        }
    }

    impl PingResult {
        pub fn builder() -> builder::PingResult {
            Default::default()
        }
    }

    ///ReportFinishBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ReportFinishBody",
    ///  "type": "object",
    ///  "required": [
    ///    "duration_millis",
    ///    "end_time",
    ///    "exit_status",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "duration_millis": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "end_time": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "exit_status": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/ReportId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ReportFinishBody {
        pub duration_millis: usize,
        pub end_time: chrono::DateTime<chrono::offset::Utc>,
        pub exit_status: usize,
        pub id: ReportId,
    }

    impl From<&ReportFinishBody> for ReportFinishBody {
        fn from(value: &ReportFinishBody) -> Self {
            value.clone()
        }
    }

    impl ReportFinishBody {
        pub fn builder() -> builder::ReportFinishBody {
            Default::default()
        }
    }

    ///ReportId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "host",
    ///    "job",
    ///    "pid",
    ///    "time",
    ///    "uuid"
    ///  ],
    ///  "properties": {
    ///    "host": {
    ///      "type": "string"
    ///    },
    ///    "job": {
    ///      "type": "string"
    ///    },
    ///    "pid": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "time": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "uuid": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ReportId {
        pub host: String,
        pub job: String,
        pub pid: u64,
        pub time: chrono::DateTime<chrono::offset::Utc>,
        pub uuid: String,
    }

    impl From<&ReportId> for ReportId {
        fn from(value: &ReportId) -> Self {
            value.clone()
        }
    }

    impl ReportId {
        pub fn builder() -> builder::ReportId {
            Default::default()
        }
    }

    ///ReportOutputBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ReportOutputBody",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "record"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "$ref": "#/components/schemas/ReportId"
    ///    },
    ///    "record": {
    ///      "$ref": "#/components/schemas/OutputRecord"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ReportOutputBody {
        pub id: ReportId,
        pub record: OutputRecord,
    }

    impl From<&ReportOutputBody> for ReportOutputBody {
        fn from(value: &ReportOutputBody) -> Self {
            value.clone()
        }
    }

    impl ReportOutputBody {
        pub fn builder() -> builder::ReportOutputBody {
            Default::default()
        }
    }

    ///ReportResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ReportResult",
    ///  "type": "object",
    ///  "required": [
    ///    "existed_already"
    ///  ],
    ///  "properties": {
    ///    "existed_already": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ReportResult {
        pub existed_already: bool,
    }

    impl From<&ReportResult> for ReportResult {
        fn from(value: &ReportResult) -> Self {
            value.clone()
        }
    }

    impl ReportResult {
        pub fn builder() -> builder::ReportResult {
            Default::default()
        }
    }

    ///ReportStartBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "ReportStartBody",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "script",
    ///    "start_time"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "$ref": "#/components/schemas/ReportId"
    ///    },
    ///    "script": {
    ///      "type": "string"
    ///    },
    ///    "start_time": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ReportStartBody {
        pub id: ReportId,
        pub script: String,
        pub start_time: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&ReportStartBody> for ReportStartBody {
        fn from(value: &ReportStartBody) -> Self {
            value.clone()
        }
    }

    impl ReportStartBody {
        pub fn builder() -> builder::ReportStartBody {
            Default::default()
        }
    }

    ///ReportSummary
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "age_seconds",
    ///    "duration_seconds",
    ///    "host",
    ///    "job",
    ///    "status",
    ///    "when"
    ///  ],
    ///  "properties": {
    ///    "age_seconds": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "duration_seconds": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "host": {
    ///      "type": "string"
    ///    },
    ///    "job": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "when": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ReportSummary {
        pub age_seconds: usize,
        pub duration_seconds: usize,
        pub host: String,
        pub job: String,
        pub status: usize,
        pub when: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&ReportSummary> for ReportSummary {
        fn from(value: &ReportSummary) -> Self {
            value.clone()
        }
    }

    impl ReportSummary {
        pub fn builder() -> builder::ReportSummary {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct EnrolBody {
            host: Result<String, String>,
            key: Result<String, String>,
        }

        impl Default for EnrolBody {
            fn default() -> Self {
                Self {
                    host: Err("no value supplied for host".to_string()),
                    key: Err("no value supplied for key".to_string()),
                }
            }
        }

        impl EnrolBody {
            pub fn host<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.host = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for host: {}", e));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<EnrolBody> for super::EnrolBody {
            type Error = super::error::ConversionError;
            fn try_from(value: EnrolBody) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    host: value.host?,
                    key: value.key?,
                })
            }
        }

        impl From<super::EnrolBody> for EnrolBody {
            fn from(value: super::EnrolBody) -> Self {
                Self {
                    host: Ok(value.host),
                    key: Ok(value.key),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GlobalJobsResult {
            summary: Result<Vec<super::ReportSummary>, String>,
        }

        impl Default for GlobalJobsResult {
            fn default() -> Self {
                Self {
                    summary: Err("no value supplied for summary".to_string()),
                }
            }
        }

        impl GlobalJobsResult {
            pub fn summary<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::ReportSummary>>,
                T::Error: std::fmt::Display,
            {
                self.summary = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for summary: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GlobalJobsResult> for super::GlobalJobsResult {
            type Error = super::error::ConversionError;
            fn try_from(value: GlobalJobsResult) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    summary: value.summary?,
                })
            }
        }

        impl From<super::GlobalJobsResult> for GlobalJobsResult {
            fn from(value: super::GlobalJobsResult) -> Self {
                Self {
                    summary: Ok(value.summary),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OutputRecord {
            msg: Result<String, String>,
            stream: Result<String, String>,
            time: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for OutputRecord {
            fn default() -> Self {
                Self {
                    msg: Err("no value supplied for msg".to_string()),
                    stream: Err("no value supplied for stream".to_string()),
                    time: Err("no value supplied for time".to_string()),
                }
            }
        }

        impl OutputRecord {
            pub fn msg<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.msg = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for msg: {}", e));
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

        impl std::convert::TryFrom<OutputRecord> for super::OutputRecord {
            type Error = super::error::ConversionError;
            fn try_from(value: OutputRecord) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    msg: value.msg?,
                    stream: value.stream?,
                    time: value.time?,
                })
            }
        }

        impl From<super::OutputRecord> for OutputRecord {
            fn from(value: super::OutputRecord) -> Self {
                Self {
                    msg: Ok(value.msg),
                    stream: Ok(value.stream),
                    time: Ok(value.time),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PingResult {
            host: Result<String, String>,
            ok: Result<bool, String>,
        }

        impl Default for PingResult {
            fn default() -> Self {
                Self {
                    host: Err("no value supplied for host".to_string()),
                    ok: Err("no value supplied for ok".to_string()),
                }
            }
        }

        impl PingResult {
            pub fn host<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.host = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for host: {}", e));
                self
            }
            pub fn ok<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.ok = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ok: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<PingResult> for super::PingResult {
            type Error = super::error::ConversionError;
            fn try_from(value: PingResult) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    host: value.host?,
                    ok: value.ok?,
                })
            }
        }

        impl From<super::PingResult> for PingResult {
            fn from(value: super::PingResult) -> Self {
                Self {
                    host: Ok(value.host),
                    ok: Ok(value.ok),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReportFinishBody {
            duration_millis: Result<usize, String>,
            end_time: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            exit_status: Result<usize, String>,
            id: Result<super::ReportId, String>,
        }

        impl Default for ReportFinishBody {
            fn default() -> Self {
                Self {
                    duration_millis: Err("no value supplied for duration_millis".to_string()),
                    end_time: Err("no value supplied for end_time".to_string()),
                    exit_status: Err("no value supplied for exit_status".to_string()),
                    id: Err("no value supplied for id".to_string()),
                }
            }
        }

        impl ReportFinishBody {
            pub fn duration_millis<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<usize>,
                T::Error: std::fmt::Display,
            {
                self.duration_millis = value.try_into().map_err(|e| {
                    format!("error converting supplied value for duration_millis: {}", e)
                });
                self
            }
            pub fn end_time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.end_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for end_time: {}", e));
                self
            }
            pub fn exit_status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<usize>,
                T::Error: std::fmt::Display,
            {
                self.exit_status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exit_status: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::ReportId>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ReportFinishBody> for super::ReportFinishBody {
            type Error = super::error::ConversionError;
            fn try_from(value: ReportFinishBody) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    duration_millis: value.duration_millis?,
                    end_time: value.end_time?,
                    exit_status: value.exit_status?,
                    id: value.id?,
                })
            }
        }

        impl From<super::ReportFinishBody> for ReportFinishBody {
            fn from(value: super::ReportFinishBody) -> Self {
                Self {
                    duration_millis: Ok(value.duration_millis),
                    end_time: Ok(value.end_time),
                    exit_status: Ok(value.exit_status),
                    id: Ok(value.id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReportId {
            host: Result<String, String>,
            job: Result<String, String>,
            pid: Result<u64, String>,
            time: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            uuid: Result<String, String>,
        }

        impl Default for ReportId {
            fn default() -> Self {
                Self {
                    host: Err("no value supplied for host".to_string()),
                    job: Err("no value supplied for job".to_string()),
                    pid: Err("no value supplied for pid".to_string()),
                    time: Err("no value supplied for time".to_string()),
                    uuid: Err("no value supplied for uuid".to_string()),
                }
            }
        }

        impl ReportId {
            pub fn host<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.host = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for host: {}", e));
                self
            }
            pub fn job<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.job = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for job: {}", e));
                self
            }
            pub fn pid<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u64>,
                T::Error: std::fmt::Display,
            {
                self.pid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pid: {}", e));
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
            pub fn uuid<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.uuid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for uuid: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ReportId> for super::ReportId {
            type Error = super::error::ConversionError;
            fn try_from(value: ReportId) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    host: value.host?,
                    job: value.job?,
                    pid: value.pid?,
                    time: value.time?,
                    uuid: value.uuid?,
                })
            }
        }

        impl From<super::ReportId> for ReportId {
            fn from(value: super::ReportId) -> Self {
                Self {
                    host: Ok(value.host),
                    job: Ok(value.job),
                    pid: Ok(value.pid),
                    time: Ok(value.time),
                    uuid: Ok(value.uuid),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReportOutputBody {
            id: Result<super::ReportId, String>,
            record: Result<super::OutputRecord, String>,
        }

        impl Default for ReportOutputBody {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    record: Err("no value supplied for record".to_string()),
                }
            }
        }

        impl ReportOutputBody {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::ReportId>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn record<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::OutputRecord>,
                T::Error: std::fmt::Display,
            {
                self.record = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for record: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ReportOutputBody> for super::ReportOutputBody {
            type Error = super::error::ConversionError;
            fn try_from(value: ReportOutputBody) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    record: value.record?,
                })
            }
        }

        impl From<super::ReportOutputBody> for ReportOutputBody {
            fn from(value: super::ReportOutputBody) -> Self {
                Self {
                    id: Ok(value.id),
                    record: Ok(value.record),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReportResult {
            existed_already: Result<bool, String>,
        }

        impl Default for ReportResult {
            fn default() -> Self {
                Self {
                    existed_already: Err("no value supplied for existed_already".to_string()),
                }
            }
        }

        impl ReportResult {
            pub fn existed_already<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.existed_already = value.try_into().map_err(|e| {
                    format!("error converting supplied value for existed_already: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<ReportResult> for super::ReportResult {
            type Error = super::error::ConversionError;
            fn try_from(value: ReportResult) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    existed_already: value.existed_already?,
                })
            }
        }

        impl From<super::ReportResult> for ReportResult {
            fn from(value: super::ReportResult) -> Self {
                Self {
                    existed_already: Ok(value.existed_already),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReportStartBody {
            id: Result<super::ReportId, String>,
            script: Result<String, String>,
            start_time: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for ReportStartBody {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    script: Err("no value supplied for script".to_string()),
                    start_time: Err("no value supplied for start_time".to_string()),
                }
            }
        }

        impl ReportStartBody {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::ReportId>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
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
            pub fn start_time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.start_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start_time: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ReportStartBody> for super::ReportStartBody {
            type Error = super::error::ConversionError;
            fn try_from(value: ReportStartBody) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    script: value.script?,
                    start_time: value.start_time?,
                })
            }
        }

        impl From<super::ReportStartBody> for ReportStartBody {
            fn from(value: super::ReportStartBody) -> Self {
                Self {
                    id: Ok(value.id),
                    script: Ok(value.script),
                    start_time: Ok(value.start_time),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReportSummary {
            age_seconds: Result<usize, String>,
            duration_seconds: Result<usize, String>,
            host: Result<String, String>,
            job: Result<String, String>,
            status: Result<usize, String>,
            when: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for ReportSummary {
            fn default() -> Self {
                Self {
                    age_seconds: Err("no value supplied for age_seconds".to_string()),
                    duration_seconds: Err("no value supplied for duration_seconds".to_string()),
                    host: Err("no value supplied for host".to_string()),
                    job: Err("no value supplied for job".to_string()),
                    status: Err("no value supplied for status".to_string()),
                    when: Err("no value supplied for when".to_string()),
                }
            }
        }

        impl ReportSummary {
            pub fn age_seconds<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<usize>,
                T::Error: std::fmt::Display,
            {
                self.age_seconds = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for age_seconds: {}", e));
                self
            }
            pub fn duration_seconds<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<usize>,
                T::Error: std::fmt::Display,
            {
                self.duration_seconds = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for duration_seconds: {}",
                        e
                    )
                });
                self
            }
            pub fn host<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.host = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for host: {}", e));
                self
            }
            pub fn job<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.job = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for job: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<usize>,
                T::Error: std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn when<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.when = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for when: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ReportSummary> for super::ReportSummary {
            type Error = super::error::ConversionError;
            fn try_from(value: ReportSummary) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    age_seconds: value.age_seconds?,
                    duration_seconds: value.duration_seconds?,
                    host: value.host?,
                    job: value.job?,
                    status: value.status?,
                    when: value.when?,
                })
            }
        }

        impl From<super::ReportSummary> for ReportSummary {
            fn from(value: super::ReportSummary) -> Self {
                Self {
                    age_seconds: Ok(value.age_seconds),
                    duration_seconds: Ok(value.duration_seconds),
                    host: Ok(value.host),
                    job: Ok(value.job),
                    status: Ok(value.status),
                    when: Ok(value.when),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for Keeper API
///
///report execution of cron jobs through a mechanism other than mail
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
    ///Sends a `POST` request to `/enrol`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    /// - `body`
    ///```ignore
    /// let response = client.enrol()
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn enrol(&self) -> builder::Enrol {
        builder::Enrol::new(self)
    }

    ///Sends a `GET` request to `/global/jobs`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    ///```ignore
    /// let response = client.global_jobs()
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    pub fn global_jobs(&self) -> builder::GlobalJobs {
        builder::GlobalJobs::new(self)
    }

    ///Sends a `GET` request to `/ping`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    ///```ignore
    /// let response = client.ping()
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ping(&self) -> builder::Ping {
        builder::Ping::new(self)
    }

    ///Sends a `POST` request to `/report/finish`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    /// - `body`
    ///```ignore
    /// let response = client.report_finish()
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn report_finish(&self) -> builder::ReportFinish {
        builder::ReportFinish::new(self)
    }

    ///Sends a `POST` request to `/report/output`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    /// - `body`
    ///```ignore
    /// let response = client.report_output()
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn report_output(&self) -> builder::ReportOutput {
        builder::ReportOutput::new(self)
    }

    ///Sends a `POST` request to `/report/start`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    /// - `body`
    ///```ignore
    /// let response = client.report_start()
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn report_start(&self) -> builder::ReportStart {
        builder::ReportStart::new(self)
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
    ///Builder for [`Client::enrol`]
    ///
    ///[`Client::enrol`]: super::Client::enrol
    #[derive(Debug, Clone)]
    pub struct Enrol<'a> {
        client: &'a super::Client,
        authorization: Result<String, String>,
        body: Result<types::builder::EnrolBody, String>,
    }

    impl<'a> Enrol<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                authorization: Err("authorization was not initialized".to_string()),
                body: Ok(types::builder::EnrolBody::default()),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::EnrolBody>,
            <V as std::convert::TryInto<types::EnrolBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `EnrolBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::EnrolBody) -> types::builder::EnrolBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/enrol`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<()>> {
            let Self {
                client,
                authorization,
                body,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::EnrolBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/enrol", client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            header_map.append("Authorization", HeaderValue::try_from(authorization)?);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .json(&body)
                .headers(header_map)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => Ok(ResponseValue::empty(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::global_jobs`]
    ///
    ///[`Client::global_jobs`]: super::Client::global_jobs
    #[derive(Debug, Clone)]
    pub struct GlobalJobs<'a> {
        client: &'a super::Client,
        authorization: Result<String, String>,
    }

    impl<'a> GlobalJobs<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                authorization: Err("authorization was not initialized".to_string()),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/global/jobs`
        pub async fn send(self) -> Result<ResponseValue<types::GlobalJobsResult>, Error<()>> {
            let Self {
                client,
                authorization,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let url = format!("{}/global/jobs", client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            header_map.append("Authorization", HeaderValue::try_from(authorization)?);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::ping`]
    ///
    ///[`Client::ping`]: super::Client::ping
    #[derive(Debug, Clone)]
    pub struct Ping<'a> {
        client: &'a super::Client,
        authorization: Result<String, String>,
    }

    impl<'a> Ping<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                authorization: Err("authorization was not initialized".to_string()),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ping`
        pub async fn send(self) -> Result<ResponseValue<types::PingResult>, Error<()>> {
            let Self {
                client,
                authorization,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let url = format!("{}/ping", client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            header_map.append("Authorization", HeaderValue::try_from(authorization)?);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::report_finish`]
    ///
    ///[`Client::report_finish`]: super::Client::report_finish
    #[derive(Debug, Clone)]
    pub struct ReportFinish<'a> {
        client: &'a super::Client,
        authorization: Result<String, String>,
        body: Result<types::builder::ReportFinishBody, String>,
    }

    impl<'a> ReportFinish<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                authorization: Err("authorization was not initialized".to_string()),
                body: Ok(types::builder::ReportFinishBody::default()),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ReportFinishBody>,
            <V as std::convert::TryInto<types::ReportFinishBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `ReportFinishBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ReportFinishBody,
            ) -> types::builder::ReportFinishBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/report/finish`
        pub async fn send(self) -> Result<ResponseValue<types::ReportResult>, Error<()>> {
            let Self {
                client,
                authorization,
                body,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::ReportFinishBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/report/finish", client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            header_map.append("Authorization", HeaderValue::try_from(authorization)?);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::report_output`]
    ///
    ///[`Client::report_output`]: super::Client::report_output
    #[derive(Debug, Clone)]
    pub struct ReportOutput<'a> {
        client: &'a super::Client,
        authorization: Result<String, String>,
        body: Result<types::builder::ReportOutputBody, String>,
    }

    impl<'a> ReportOutput<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                authorization: Err("authorization was not initialized".to_string()),
                body: Ok(types::builder::ReportOutputBody::default()),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ReportOutputBody>,
            <V as std::convert::TryInto<types::ReportOutputBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `ReportOutputBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ReportOutputBody,
            ) -> types::builder::ReportOutputBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/report/output`
        pub async fn send(self) -> Result<ResponseValue<types::ReportResult>, Error<()>> {
            let Self {
                client,
                authorization,
                body,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::ReportOutputBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/report/output", client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            header_map.append("Authorization", HeaderValue::try_from(authorization)?);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::report_start`]
    ///
    ///[`Client::report_start`]: super::Client::report_start
    #[derive(Debug, Clone)]
    pub struct ReportStart<'a> {
        client: &'a super::Client,
        authorization: Result<String, String>,
        body: Result<types::builder::ReportStartBody, String>,
    }

    impl<'a> ReportStart<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                authorization: Err("authorization was not initialized".to_string()),
                body: Ok(types::builder::ReportStartBody::default()),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ReportStartBody>,
            <V as std::convert::TryInto<types::ReportStartBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `ReportStartBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::ReportStartBody) -> types::builder::ReportStartBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/report/start`
        pub async fn send(self) -> Result<ResponseValue<types::ReportResult>, Error<()>> {
            let Self {
                client,
                authorization,
                body,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::ReportStartBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/report/start", client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            header_map.append("Authorization", HeaderValue::try_from(authorization)?);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
