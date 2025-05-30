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

    ///`EnrolBody`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct EnrolBody {
        pub host: ::std::string::String,
        pub key: ::std::string::String,
    }

    impl ::std::convert::From<&EnrolBody> for EnrolBody {
        fn from(value: &EnrolBody) -> Self {
            value.clone()
        }
    }

    ///`GlobalJobsResult`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GlobalJobsResult {
        pub summary: ::std::vec::Vec<ReportSummary>,
    }

    impl ::std::convert::From<&GlobalJobsResult> for GlobalJobsResult {
        fn from(value: &GlobalJobsResult) -> Self {
            value.clone()
        }
    }

    ///`OutputRecord`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OutputRecord {
        pub msg: ::std::string::String,
        pub stream: ::std::string::String,
        pub time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&OutputRecord> for OutputRecord {
        fn from(value: &OutputRecord) -> Self {
            value.clone()
        }
    }

    ///`PingResult`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PingResult {
        pub host: ::std::string::String,
        pub ok: bool,
    }

    impl ::std::convert::From<&PingResult> for PingResult {
        fn from(value: &PingResult) -> Self {
            value.clone()
        }
    }

    ///`ReportFinishBody`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportFinishBody {
        pub duration_millis: i32,
        pub end_time: ::chrono::DateTime<::chrono::offset::Utc>,
        pub exit_status: i32,
        pub id: ReportId,
    }

    impl ::std::convert::From<&ReportFinishBody> for ReportFinishBody {
        fn from(value: &ReportFinishBody) -> Self {
            value.clone()
        }
    }

    ///`ReportId`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportId {
        pub host: ::std::string::String,
        pub job: ::std::string::String,
        pub pid: u64,
        pub time: ::chrono::DateTime<::chrono::offset::Utc>,
        pub uuid: ::std::string::String,
    }

    impl ::std::convert::From<&ReportId> for ReportId {
        fn from(value: &ReportId) -> Self {
            value.clone()
        }
    }

    ///`ReportOutputBody`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportOutputBody {
        pub id: ReportId,
        pub record: OutputRecord,
    }

    impl ::std::convert::From<&ReportOutputBody> for ReportOutputBody {
        fn from(value: &ReportOutputBody) -> Self {
            value.clone()
        }
    }

    ///`ReportResult`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportResult {
        pub existed_already: bool,
    }

    impl ::std::convert::From<&ReportResult> for ReportResult {
        fn from(value: &ReportResult) -> Self {
            value.clone()
        }
    }

    ///`ReportStartBody`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportStartBody {
        pub id: ReportId,
        pub script: ::std::string::String,
        pub start_time: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&ReportStartBody> for ReportStartBody {
        fn from(value: &ReportStartBody) -> Self {
            value.clone()
        }
    }

    ///`ReportSummary`
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReportSummary {
        pub age_seconds: i32,
        pub duration_seconds: i32,
        pub host: ::std::string::String,
        pub job: ::std::string::String,
        pub status: i32,
        pub when: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&ReportSummary> for ReportSummary {
        fn from(value: &ReportSummary) -> Self {
            value.clone()
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
    ///Sends a `POST` request to `/enrol`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    /// - `body`
    pub async fn enrol<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::EnrolBody,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/enrol", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "enrol",
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

    ///Sends a `GET` request to `/global/jobs`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    pub async fn global_jobs<'a>(
        &'a self,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::GlobalJobsResult>, Error<()>> {
        let url = format!("{}/global/jobs", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "global_jobs",
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

    ///Sends a `GET` request to `/ping`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    pub async fn ping<'a>(
        &'a self,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::PingResult>, Error<()>> {
        let url = format!("{}/ping", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "ping",
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

    ///Sends a `POST` request to `/report/finish`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    /// - `body`
    pub async fn report_finish<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::ReportFinishBody,
    ) -> Result<ResponseValue<types::ReportResult>, Error<()>> {
        let url = format!("{}/report/finish", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "report_finish",
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

    ///Sends a `POST` request to `/report/output`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    /// - `body`
    pub async fn report_output<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::ReportOutputBody,
    ) -> Result<ResponseValue<types::ReportResult>, Error<()>> {
        let url = format!("{}/report/output", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "report_output",
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

    ///Sends a `POST` request to `/report/start`
    ///
    ///Arguments:
    /// - `authorization`: Authorization header (bearer token)
    /// - `body`
    pub async fn report_start<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::ReportStartBody,
    ) -> Result<ResponseValue<types::ReportResult>, Error<()>> {
        let url = format!("{}/report/start", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        header_map.append("Authorization", authorization.to_string().try_into()?);
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
            operation_id: "report_start",
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
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
