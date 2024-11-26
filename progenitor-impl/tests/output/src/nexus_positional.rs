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

    ///Describes properties that should uniquely identify a Gimlet.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes properties that should uniquely identify a
    /// Gimlet.",
    ///  "type": "object",
    ///  "required": [
    ///    "part",
    ///    "revision",
    ///    "serial"
    ///  ],
    ///  "properties": {
    ///    "part": {
    ///      "type": "string"
    ///    },
    ///    "revision": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "serial": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Baseboard {
        pub part: String,
        pub revision: i64,
        pub serial: String,
    }

    impl From<&Baseboard> for Baseboard {
        fn from(value: &Baseboard) -> Self {
            value.clone()
        }
    }

    ///A type storing a range over `T`.
    ///
    ///This type supports ranges similar to the `RangeTo`, `Range` and
    /// `RangeFrom` types in the standard library. Those cover `(..end)`,
    /// `(start..end)`, and `(start..)` respectively.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A type storing a range over `T`.\n\nThis type supports
    /// ranges similar to the `RangeTo`, `Range` and `RangeFrom` types in the
    /// standard library. Those cover `(..end)`, `(start..end)`, and `(start..)`
    /// respectively.",
    ///  "oneOf": [
    ///    {
    ///      "description": "A range unbounded below and exclusively above,
    /// `..end`.",
    ///      "type": "object",
    ///      "required": [
    ///        "end",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "end": {
    ///          "type": "number",
    ///          "format": "double"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "range_to"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "A range bounded inclusively below and exclusively
    /// above, `start..end`.",
    ///      "type": "object",
    ///      "required": [
    ///        "end",
    ///        "start",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "end": {
    ///          "type": "number",
    ///          "format": "double"
    ///        },
    ///        "start": {
    ///          "type": "number",
    ///          "format": "double"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "range"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "A range bounded inclusively below and unbounded
    /// above, `start..`.",
    ///      "type": "object",
    ///      "required": [
    ///        "start",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "start": {
    ///          "type": "number",
    ///          "format": "double"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "range_from"
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
    pub enum BinRangedouble {
        ///A range unbounded below and exclusively above, `..end`.
        #[serde(rename = "range_to")]
        RangeTo { end: f64 },
        ///A range bounded inclusively below and exclusively above,
        /// `start..end`.
        #[serde(rename = "range")]
        Range { end: f64, start: f64 },
        ///A range bounded inclusively below and unbounded above, `start..`.
        #[serde(rename = "range_from")]
        RangeFrom { start: f64 },
    }

    impl From<&BinRangedouble> for BinRangedouble {
        fn from(value: &BinRangedouble) -> Self {
            value.clone()
        }
    }

    ///A type storing a range over `T`.
    ///
    ///This type supports ranges similar to the `RangeTo`, `Range` and
    /// `RangeFrom` types in the standard library. Those cover `(..end)`,
    /// `(start..end)`, and `(start..)` respectively.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A type storing a range over `T`.\n\nThis type supports
    /// ranges similar to the `RangeTo`, `Range` and `RangeFrom` types in the
    /// standard library. Those cover `(..end)`, `(start..end)`, and `(start..)`
    /// respectively.",
    ///  "oneOf": [
    ///    {
    ///      "description": "A range unbounded below and exclusively above,
    /// `..end`.",
    ///      "type": "object",
    ///      "required": [
    ///        "end",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "end": {
    ///          "type": "integer",
    ///          "format": "int64"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "range_to"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "A range bounded inclusively below and exclusively
    /// above, `start..end`.",
    ///      "type": "object",
    ///      "required": [
    ///        "end",
    ///        "start",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "end": {
    ///          "type": "integer",
    ///          "format": "int64"
    ///        },
    ///        "start": {
    ///          "type": "integer",
    ///          "format": "int64"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "range"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "A range bounded inclusively below and unbounded
    /// above, `start..`.",
    ///      "type": "object",
    ///      "required": [
    ///        "start",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "start": {
    ///          "type": "integer",
    ///          "format": "int64"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "range_from"
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
    pub enum BinRangeint64 {
        ///A range unbounded below and exclusively above, `..end`.
        #[serde(rename = "range_to")]
        RangeTo { end: i64 },
        ///A range bounded inclusively below and exclusively above,
        /// `start..end`.
        #[serde(rename = "range")]
        Range { end: i64, start: i64 },
        ///A range bounded inclusively below and unbounded above, `start..`.
        #[serde(rename = "range_from")]
        RangeFrom { start: i64 },
    }

    impl From<&BinRangeint64> for BinRangeint64 {
        fn from(value: &BinRangeint64) -> Self {
            value.clone()
        }
    }

    ///Type storing bin edges and a count of samples within it.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Type storing bin edges and a count of samples within
    /// it.",
    ///  "type": "object",
    ///  "required": [
    ///    "count",
    ///    "range"
    ///  ],
    ///  "properties": {
    ///    "count": {
    ///      "description": "The total count of samples in this bin.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "range": {
    ///      "description": "The range of the support covered by this bin.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BinRangedouble"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Bindouble {
        ///The total count of samples in this bin.
        pub count: u64,
        ///The range of the support covered by this bin.
        pub range: BinRangedouble,
    }

    impl From<&Bindouble> for Bindouble {
        fn from(value: &Bindouble) -> Self {
            value.clone()
        }
    }

    ///Type storing bin edges and a count of samples within it.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Type storing bin edges and a count of samples within
    /// it.",
    ///  "type": "object",
    ///  "required": [
    ///    "count",
    ///    "range"
    ///  ],
    ///  "properties": {
    ///    "count": {
    ///      "description": "The total count of samples in this bin.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "range": {
    ///      "description": "The range of the support covered by this bin.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BinRangeint64"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Binint64 {
        ///The total count of samples in this bin.
        pub count: u64,
        ///The range of the support covered by this bin.
        pub range: BinRangeint64,
    }

    impl From<&Binint64> for Binint64 {
        fn from(value: &Binint64) -> Self {
            value.clone()
        }
    }

    ///BlockSize
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "disk block size in bytes",
    ///  "type": "integer",
    ///  "enum": [
    ///    512,
    ///    2048,
    ///    4096
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct BlockSize(i64);
    impl ::std::ops::Deref for BlockSize {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<BlockSize> for i64 {
        fn from(value: BlockSize) -> Self {
            value.0
        }
    }

    impl From<&BlockSize> for BlockSize {
        fn from(value: &BlockSize) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<i64> for BlockSize {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
            if ![512_i64, 2048_i64, 4096_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for BlockSize {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }

    ///A count of bytes, typically used either for memory or storage capacity
    ///
    ///The maximum supported byte count is [`i64::MAX`].  This makes it
    /// somewhat inconvenient to define constructors: a u32 constructor can be
    /// infallible, but an i64 constructor can fail (if the value is negative)
    /// and a u64 constructor can fail (if the value is larger than i64::MAX).
    /// We provide all of these for consumers' convenience.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A count of bytes, typically used either for memory or
    /// storage capacity\n\nThe maximum supported byte count is [`i64::MAX`].
    /// This makes it somewhat inconvenient to define constructors: a u32
    /// constructor can be infallible, but an i64 constructor can fail (if the
    /// value is negative) and a u64 constructor can fail (if the value is
    /// larger than i64::MAX).  We provide all of these for consumers'
    /// convenience.",
    ///  "type": "integer",
    ///  "format": "uint64",
    ///  "minimum": 0.0
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ByteCount(pub u64);
    impl ::std::ops::Deref for ByteCount {
        type Target = u64;
        fn deref(&self) -> &u64 {
            &self.0
        }
    }

    impl From<ByteCount> for u64 {
        fn from(value: ByteCount) -> Self {
            value.0
        }
    }

    impl From<&ByteCount> for ByteCount {
        fn from(value: &ByteCount) -> Self {
            value.clone()
        }
    }

    impl From<u64> for ByteCount {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for ByteCount {
        type Err = <u64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for ByteCount {
        type Error = <u64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ByteCount {
        type Error = <u64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ByteCount {
        type Error = <u64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for ByteCount {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///Client view of a [`Certificate`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Certificate`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "name",
    ///    "service",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "service": {
    ///      "$ref": "#/components/schemas/ServiceUsingCertificate"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Certificate {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        pub service: ServiceUsingCertificate,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Certificate> for Certificate {
        fn from(value: &Certificate) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a
    /// [`Certificate`](crate::external_api::views::Certificate)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a
    /// [`Certificate`](crate::external_api::views::Certificate)",
    ///  "type": "object",
    ///  "required": [
    ///    "cert",
    ///    "description",
    ///    "key",
    ///    "name",
    ///    "service"
    ///  ],
    ///  "properties": {
    ///    "cert": {
    ///      "description": "PEM file containing public certificate chain",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint8",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "key": {
    ///      "description": "PEM file containing private key",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint8",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "service": {
    ///      "description": "The service using this certificate",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ServiceUsingCertificate"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CertificateCreate {
        ///PEM file containing public certificate chain
        pub cert: Vec<u8>,
        pub description: String,
        ///PEM file containing private key
        pub key: Vec<u8>,
        pub name: Name,
        ///The service using this certificate
        pub service: ServiceUsingCertificate,
    }

    impl From<&CertificateCreate> for CertificateCreate {
        fn from(value: &CertificateCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Certificate"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CertificateResultsPage {
        ///list of items on this page of results
        pub items: Vec<Certificate>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&CertificateResultsPage> for CertificateResultsPage {
        fn from(value: &CertificateResultsPage) -> Self {
            value.clone()
        }
    }

    ///Identity-related metadata that's included in "asset" public API objects
    /// (which generally have no name or description)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Identity-related metadata that's included in \"asset\"
    /// public API objects (which generally have no name or description)",
    ///  "type": "object",
    ///  "required": [
    ///    "component_type",
    ///    "id",
    ///    "time_created",
    ///    "time_modified",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "component_type": {
    ///      "$ref": "#/components/schemas/UpdateableComponentType"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "version": {
    ///      "$ref": "#/components/schemas/SemverVersion"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ComponentUpdate {
        pub component_type: UpdateableComponentType,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        pub version: SemverVersion,
    }

    impl From<&ComponentUpdate> for ComponentUpdate {
        fn from(value: &ComponentUpdate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ComponentUpdate"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ComponentUpdateResultsPage {
        ///list of items on this page of results
        pub items: Vec<ComponentUpdate>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&ComponentUpdateResultsPage> for ComponentUpdateResultsPage {
        fn from(value: &ComponentUpdateResultsPage) -> Self {
            value.clone()
        }
    }

    ///A cumulative or counter data type.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A cumulative or counter data type.",
    ///  "type": "object",
    ///  "required": [
    ///    "start_time",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "start_time": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "value": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Cumulativedouble {
        pub start_time: chrono::DateTime<chrono::offset::Utc>,
        pub value: f64,
    }

    impl From<&Cumulativedouble> for Cumulativedouble {
        fn from(value: &Cumulativedouble) -> Self {
            value.clone()
        }
    }

    ///A cumulative or counter data type.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A cumulative or counter data type.",
    ///  "type": "object",
    ///  "required": [
    ///    "start_time",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "start_time": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "value": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Cumulativeint64 {
        pub start_time: chrono::DateTime<chrono::offset::Utc>,
        pub value: i64,
    }

    impl From<&Cumulativeint64> for Cumulativeint64 {
        fn from(value: &Cumulativeint64) -> Self {
            value.clone()
        }
    }

    ///A `Datum` is a single sampled data point from a metric.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A `Datum` is a single sampled data point from a
    /// metric.",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "datum",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "datum": {
    ///          "type": "boolean"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "bool"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "datum",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "datum": {
    ///          "type": "integer",
    ///          "format": "int64"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "i64"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "datum",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "datum": {
    ///          "type": "number",
    ///          "format": "double"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "f64"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "datum",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "datum": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "string"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "datum",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "datum": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "integer",
    ///            "format": "uint8",
    ///            "minimum": 0.0
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "bytes"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "datum",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "datum": {
    ///          "$ref": "#/components/schemas/Cumulativeint64"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "cumulative_i64"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "datum",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "datum": {
    ///          "$ref": "#/components/schemas/Cumulativedouble"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "cumulative_f64"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "datum",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "datum": {
    ///          "$ref": "#/components/schemas/Histogramint64"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "histogram_i64"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "datum",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "datum": {
    ///          "$ref": "#/components/schemas/Histogramdouble"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "histogram_f64"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "datum")]
    pub enum Datum {
        #[serde(rename = "bool")]
        Bool(bool),
        #[serde(rename = "i64")]
        I64(i64),
        #[serde(rename = "f64")]
        F64(f64),
        #[serde(rename = "string")]
        String(String),
        #[serde(rename = "bytes")]
        Bytes(Vec<u8>),
        #[serde(rename = "cumulative_i64")]
        CumulativeI64(Cumulativeint64),
        #[serde(rename = "cumulative_f64")]
        CumulativeF64(Cumulativedouble),
        #[serde(rename = "histogram_i64")]
        HistogramI64(Histogramint64),
        #[serde(rename = "histogram_f64")]
        HistogramF64(Histogramdouble),
    }

    impl From<&Datum> for Datum {
        fn from(value: &Datum) -> Self {
            value.clone()
        }
    }

    impl From<bool> for Datum {
        fn from(value: bool) -> Self {
            Self::Bool(value)
        }
    }

    impl From<i64> for Datum {
        fn from(value: i64) -> Self {
            Self::I64(value)
        }
    }

    impl From<f64> for Datum {
        fn from(value: f64) -> Self {
            Self::F64(value)
        }
    }

    impl From<Vec<u8>> for Datum {
        fn from(value: Vec<u8>) -> Self {
            Self::Bytes(value)
        }
    }

    impl From<Cumulativeint64> for Datum {
        fn from(value: Cumulativeint64) -> Self {
            Self::CumulativeI64(value)
        }
    }

    impl From<Cumulativedouble> for Datum {
        fn from(value: Cumulativedouble) -> Self {
            Self::CumulativeF64(value)
        }
    }

    impl From<Histogramint64> for Datum {
        fn from(value: Histogramint64) -> Self {
            Self::HistogramI64(value)
        }
    }

    impl From<Histogramdouble> for Datum {
        fn from(value: Histogramdouble) -> Self {
            Self::HistogramF64(value)
        }
    }

    ///The type of an individual datum of a metric.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The type of an individual datum of a metric.",
    ///  "type": "string",
    ///  "enum": [
    ///    "bool",
    ///    "i64",
    ///    "f64",
    ///    "string",
    ///    "bytes",
    ///    "cumulative_i64",
    ///    "cumulative_f64",
    ///    "histogram_i64",
    ///    "histogram_f64"
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
    pub enum DatumType {
        #[serde(rename = "bool")]
        Bool,
        #[serde(rename = "i64")]
        I64,
        #[serde(rename = "f64")]
        F64,
        #[serde(rename = "string")]
        String,
        #[serde(rename = "bytes")]
        Bytes,
        #[serde(rename = "cumulative_i64")]
        CumulativeI64,
        #[serde(rename = "cumulative_f64")]
        CumulativeF64,
        #[serde(rename = "histogram_i64")]
        HistogramI64,
        #[serde(rename = "histogram_f64")]
        HistogramF64,
    }

    impl From<&DatumType> for DatumType {
        fn from(value: &DatumType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for DatumType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bool => write!(f, "bool"),
                Self::I64 => write!(f, "i64"),
                Self::F64 => write!(f, "f64"),
                Self::String => write!(f, "string"),
                Self::Bytes => write!(f, "bytes"),
                Self::CumulativeI64 => write!(f, "cumulative_i64"),
                Self::CumulativeF64 => write!(f, "cumulative_f64"),
                Self::HistogramI64 => write!(f, "histogram_i64"),
                Self::HistogramF64 => write!(f, "histogram_f64"),
            }
        }
    }

    impl std::str::FromStr for DatumType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "bool" => Ok(Self::Bool),
                "i64" => Ok(Self::I64),
                "f64" => Ok(Self::F64),
                "string" => Ok(Self::String),
                "bytes" => Ok(Self::Bytes),
                "cumulative_i64" => Ok(Self::CumulativeI64),
                "cumulative_f64" => Ok(Self::CumulativeF64),
                "histogram_i64" => Ok(Self::HistogramI64),
                "histogram_f64" => Ok(Self::HistogramF64),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for DatumType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for DatumType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for DatumType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///DerEncodedKeyPair
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "private_key",
    ///    "public_cert"
    ///  ],
    ///  "properties": {
    ///    "private_key": {
    ///      "description": "request signing private key (base64 encoded der
    /// file)",
    ///      "type": "string"
    ///    },
    ///    "public_cert": {
    ///      "description": "request signing public certificate (base64 encoded
    /// der file)",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DerEncodedKeyPair {
        ///request signing private key (base64 encoded der file)
        pub private_key: String,
        ///request signing public certificate (base64 encoded der file)
        pub public_cert: String,
    }

    impl From<&DerEncodedKeyPair> for DerEncodedKeyPair {
        fn from(value: &DerEncodedKeyPair) -> Self {
            value.clone()
        }
    }

    ///DeviceAccessTokenRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "client_id",
    ///    "device_code",
    ///    "grant_type"
    ///  ],
    ///  "properties": {
    ///    "client_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "device_code": {
    ///      "type": "string"
    ///    },
    ///    "grant_type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeviceAccessTokenRequest {
        pub client_id: uuid::Uuid,
        pub device_code: String,
        pub grant_type: String,
    }

    impl From<&DeviceAccessTokenRequest> for DeviceAccessTokenRequest {
        fn from(value: &DeviceAccessTokenRequest) -> Self {
            value.clone()
        }
    }

    ///DeviceAuthRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "client_id"
    ///  ],
    ///  "properties": {
    ///    "client_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeviceAuthRequest {
        pub client_id: uuid::Uuid,
    }

    impl From<&DeviceAuthRequest> for DeviceAuthRequest {
        fn from(value: &DeviceAuthRequest) -> Self {
            value.clone()
        }
    }

    ///DeviceAuthVerify
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "user_code"
    ///  ],
    ///  "properties": {
    ///    "user_code": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeviceAuthVerify {
        pub user_code: String,
    }

    impl From<&DeviceAuthVerify> for DeviceAuthVerify {
        fn from(value: &DeviceAuthVerify) -> Self {
            value.clone()
        }
    }

    ///Digest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "sha256"
    ///          ]
    ///        },
    ///        "value": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum Digest {
        #[serde(rename = "sha256")]
        Sha256(String),
    }

    impl From<&Digest> for Digest {
        fn from(value: &Digest) -> Self {
            value.clone()
        }
    }

    ///Client view of a [`Disk`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Disk`]",
    ///  "type": "object",
    ///  "required": [
    ///    "block_size",
    ///    "description",
    ///    "device_path",
    ///    "id",
    ///    "name",
    ///    "project_id",
    ///    "size",
    ///    "state",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "block_size": {
    ///      "$ref": "#/components/schemas/ByteCount"
    ///    },
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "device_path": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "image_id": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "project_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "size": {
    ///      "$ref": "#/components/schemas/ByteCount"
    ///    },
    ///    "snapshot_id": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uuid"
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/DiskState"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Disk {
        pub block_size: ByteCount,
        ///human-readable free-form text about a resource
        pub description: String,
        pub device_path: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image_id: Option<uuid::Uuid>,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        pub project_id: uuid::Uuid,
        pub size: ByteCount,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot_id: Option<uuid::Uuid>,
        pub state: DiskState,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Disk> for Disk {
        fn from(value: &Disk) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a
    /// [`Disk`](omicron_common::api::external::Disk)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a
    /// [`Disk`](omicron_common::api::external::Disk)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "disk_source",
    ///    "name",
    ///    "size"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "disk_source": {
    ///      "description": "initial source for this disk",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/DiskSource"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "size": {
    ///      "description": "total size of the Disk in bytes",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ByteCount"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskCreate {
        pub description: String,
        ///initial source for this disk
        pub disk_source: DiskSource,
        pub name: Name,
        ///total size of the Disk in bytes
        pub size: ByteCount,
    }

    impl From<&DiskCreate> for DiskCreate {
        fn from(value: &DiskCreate) -> Self {
            value.clone()
        }
    }

    ///TODO-v1: Delete this Parameters for the
    /// [`Disk`](omicron_common::api::external::Disk) to be attached or detached
    /// to an instance
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "TODO-v1: Delete this Parameters for the
    /// [`Disk`](omicron_common::api::external::Disk) to be attached or detached
    /// to an instance",
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskIdentifier {
        pub name: Name,
    }

    impl From<&DiskIdentifier> for DiskIdentifier {
        fn from(value: &DiskIdentifier) -> Self {
            value.clone()
        }
    }

    ///DiskMetricName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "activated",
    ///    "flush",
    ///    "read",
    ///    "read_bytes",
    ///    "write",
    ///    "write_bytes"
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
    pub enum DiskMetricName {
        #[serde(rename = "activated")]
        Activated,
        #[serde(rename = "flush")]
        Flush,
        #[serde(rename = "read")]
        Read,
        #[serde(rename = "read_bytes")]
        ReadBytes,
        #[serde(rename = "write")]
        Write,
        #[serde(rename = "write_bytes")]
        WriteBytes,
    }

    impl From<&DiskMetricName> for DiskMetricName {
        fn from(value: &DiskMetricName) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for DiskMetricName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Activated => write!(f, "activated"),
                Self::Flush => write!(f, "flush"),
                Self::Read => write!(f, "read"),
                Self::ReadBytes => write!(f, "read_bytes"),
                Self::Write => write!(f, "write"),
                Self::WriteBytes => write!(f, "write_bytes"),
            }
        }
    }

    impl std::str::FromStr for DiskMetricName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "activated" => Ok(Self::Activated),
                "flush" => Ok(Self::Flush),
                "read" => Ok(Self::Read),
                "read_bytes" => Ok(Self::ReadBytes),
                "write" => Ok(Self::Write),
                "write_bytes" => Ok(Self::WriteBytes),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for DiskMetricName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for DiskMetricName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for DiskMetricName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///DiskPath
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "disk"
    ///  ],
    ///  "properties": {
    ///    "disk": {
    ///      "$ref": "#/components/schemas/NameOrId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskPath {
        pub disk: NameOrId,
    }

    impl From<&DiskPath> for DiskPath {
        fn from(value: &DiskPath) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Disk"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DiskResultsPage {
        ///list of items on this page of results
        pub items: Vec<Disk>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&DiskResultsPage> for DiskResultsPage {
        fn from(value: &DiskResultsPage) -> Self {
            value.clone()
        }
    }

    ///Different sources for a disk
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Different sources for a disk",
    ///  "oneOf": [
    ///    {
    ///      "description": "Create a blank disk",
    ///      "type": "object",
    ///      "required": [
    ///        "block_size",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "block_size": {
    ///          "description": "size of blocks for this Disk. valid values are:
    /// 512, 2048, or 4096",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/BlockSize"
    ///            }
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "blank"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Create a disk from a disk snapshot",
    ///      "type": "object",
    ///      "required": [
    ///        "snapshot_id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "snapshot_id": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "snapshot"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Create a disk from a project image",
    ///      "type": "object",
    ///      "required": [
    ///        "image_id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "image_id": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "image"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Create a disk from a global image",
    ///      "type": "object",
    ///      "required": [
    ///        "image_id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "image_id": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "global_image"
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
    pub enum DiskSource {
        ///Create a blank disk
        #[serde(rename = "blank")]
        Blank {
            ///size of blocks for this Disk. valid values are: 512, 2048, or
            /// 4096
            block_size: BlockSize,
        },
        ///Create a disk from a disk snapshot
        #[serde(rename = "snapshot")]
        Snapshot { snapshot_id: uuid::Uuid },
        ///Create a disk from a project image
        #[serde(rename = "image")]
        Image { image_id: uuid::Uuid },
        ///Create a disk from a global image
        #[serde(rename = "global_image")]
        GlobalImage { image_id: uuid::Uuid },
    }

    impl From<&DiskSource> for DiskSource {
        fn from(value: &DiskSource) -> Self {
            value.clone()
        }
    }

    ///State of a Disk (primarily: attached or not)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "State of a Disk (primarily: attached or not)",
    ///  "oneOf": [
    ///    {
    ///      "description": "Disk is being initialized",
    ///      "type": "object",
    ///      "required": [
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "creating"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Disk is ready but detached from any Instance",
    ///      "type": "object",
    ///      "required": [
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "detached"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Disk is being attached to the given Instance",
    ///      "type": "object",
    ///      "required": [
    ///        "instance",
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "instance": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "attaching"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Disk is attached to the given Instance",
    ///      "type": "object",
    ///      "required": [
    ///        "instance",
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "instance": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "attached"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Disk is being detached from the given Instance",
    ///      "type": "object",
    ///      "required": [
    ///        "instance",
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "instance": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "detaching"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Disk has been destroyed",
    ///      "type": "object",
    ///      "required": [
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "destroyed"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Disk is unavailable",
    ///      "type": "object",
    ///      "required": [
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "faulted"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "state", content = "instance")]
    pub enum DiskState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "detached")]
        Detached,
        ///Disk is being attached to the given Instance
        #[serde(rename = "attaching")]
        Attaching(uuid::Uuid),
        ///Disk is attached to the given Instance
        #[serde(rename = "attached")]
        Attached(uuid::Uuid),
        ///Disk is being detached from the given Instance
        #[serde(rename = "detaching")]
        Detaching(uuid::Uuid),
        #[serde(rename = "destroyed")]
        Destroyed,
        #[serde(rename = "faulted")]
        Faulted,
    }

    impl From<&DiskState> for DiskState {
        fn from(value: &DiskState) -> Self {
            value.clone()
        }
    }

    ///OS image distribution
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "OS image distribution",
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "The name of the distribution (e.g. \"alpine\" or
    /// \"ubuntu\")",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "version": {
    ///      "description": "The version of the distribution (e.g. \"3.10\" or
    /// \"18.04\")",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Distribution {
        ///The name of the distribution (e.g. "alpine" or "ubuntu")
        pub name: Name,
        ///The version of the distribution (e.g. "3.10" or "18.04")
        pub version: String,
    }

    impl From<&Distribution> for Distribution {
        fn from(value: &Distribution) -> Self {
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
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
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

    ///ExternalIp
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "ip",
    ///    "kind"
    ///  ],
    ///  "properties": {
    ///    "ip": {
    ///      "type": "string",
    ///      "format": "ip"
    ///    },
    ///    "kind": {
    ///      "$ref": "#/components/schemas/IpKind"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExternalIp {
        pub ip: std::net::IpAddr,
        pub kind: IpKind,
    }

    impl From<&ExternalIp> for ExternalIp {
        fn from(value: &ExternalIp) -> Self {
            value.clone()
        }
    }

    ///Parameters for creating an external IP address for instances.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Parameters for creating an external IP address for
    /// instances.",
    ///  "oneOf": [
    ///    {
    ///      "description": "An IP address providing both inbound and outbound
    /// access. The address is automatically-assigned from the provided IP Pool,
    /// or all available pools if not specified.",
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "pool_name": {
    ///          "oneOf": [
    ///            {
    ///              "type": "null"
    ///            },
    ///            {
    ///              "allOf": [
    ///                {
    ///                  "$ref": "#/components/schemas/Name"
    ///                }
    ///              ]
    ///            }
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ephemeral"
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
    pub enum ExternalIpCreate {
        ///An IP address providing both inbound and outbound access. The
        /// address is automatically-assigned from the provided IP Pool, or all
        /// available pools if not specified.
        #[serde(rename = "ephemeral")]
        Ephemeral {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pool_name: Option<Name>,
        },
    }

    impl From<&ExternalIpCreate> for ExternalIpCreate {
        fn from(value: &ExternalIpCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ExternalIp"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExternalIpResultsPage {
        ///list of items on this page of results
        pub items: Vec<ExternalIp>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&ExternalIpResultsPage> for ExternalIpResultsPage {
        fn from(value: &ExternalIpResultsPage) -> Self {
            value.clone()
        }
    }

    ///The name and type information for a field of a timeseries schema.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name and type information for a field of a
    /// timeseries schema.",
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "source",
    ///    "ty"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/FieldSource"
    ///    },
    ///    "ty": {
    ///      "$ref": "#/components/schemas/FieldType"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FieldSchema {
        pub name: String,
        pub source: FieldSource,
        pub ty: FieldType,
    }

    impl From<&FieldSchema> for FieldSchema {
        fn from(value: &FieldSchema) -> Self {
            value.clone()
        }
    }

    ///The source from which a field is derived, the target or metric.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The source from which a field is derived, the target or
    /// metric.",
    ///  "type": "string",
    ///  "enum": [
    ///    "target",
    ///    "metric"
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
    pub enum FieldSource {
        #[serde(rename = "target")]
        Target,
        #[serde(rename = "metric")]
        Metric,
    }

    impl From<&FieldSource> for FieldSource {
        fn from(value: &FieldSource) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for FieldSource {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Target => write!(f, "target"),
                Self::Metric => write!(f, "metric"),
            }
        }
    }

    impl std::str::FromStr for FieldSource {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "target" => Ok(Self::Target),
                "metric" => Ok(Self::Metric),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FieldSource {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FieldSource {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FieldSource {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///The `FieldType` identifies the data type of a target or metric field.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The `FieldType` identifies the data type of a target or
    /// metric field.",
    ///  "type": "string",
    ///  "enum": [
    ///    "string",
    ///    "i64",
    ///    "ip_addr",
    ///    "uuid",
    ///    "bool"
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
    pub enum FieldType {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "i64")]
        I64,
        #[serde(rename = "ip_addr")]
        IpAddr,
        #[serde(rename = "uuid")]
        Uuid,
        #[serde(rename = "bool")]
        Bool,
    }

    impl From<&FieldType> for FieldType {
        fn from(value: &FieldType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for FieldType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::String => write!(f, "string"),
                Self::I64 => write!(f, "i64"),
                Self::IpAddr => write!(f, "ip_addr"),
                Self::Uuid => write!(f, "uuid"),
                Self::Bool => write!(f, "bool"),
            }
        }
    }

    impl std::str::FromStr for FieldType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "string" => Ok(Self::String),
                "i64" => Ok(Self::I64),
                "ip_addr" => Ok(Self::IpAddr),
                "uuid" => Ok(Self::Uuid),
                "bool" => Ok(Self::Bool),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FieldType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FieldType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FieldType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///FleetRole
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "admin",
    ///    "collaborator",
    ///    "viewer"
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
    pub enum FleetRole {
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "collaborator")]
        Collaborator,
        #[serde(rename = "viewer")]
        Viewer,
    }

    impl From<&FleetRole> for FleetRole {
        fn from(value: &FleetRole) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for FleetRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Admin => write!(f, "admin"),
                Self::Collaborator => write!(f, "collaborator"),
                Self::Viewer => write!(f, "viewer"),
            }
        }
    }

    impl std::str::FromStr for FleetRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "admin" => Ok(Self::Admin),
                "collaborator" => Ok(Self::Collaborator),
                "viewer" => Ok(Self::Viewer),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FleetRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FleetRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FleetRole {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Client view of a [`Policy`], which describes how this resource may be
    /// accessed
    ///
    ///Note that the Policy only describes access granted explicitly for this
    /// resource.  The policies of parent resources can also cause a user to
    /// have access to this resource.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Policy`], which describes how this
    /// resource may be accessed\n\nNote that the Policy only describes access
    /// granted explicitly for this resource.  The policies of parent resources
    /// can also cause a user to have access to this resource.",
    ///  "type": "object",
    ///  "required": [
    ///    "role_assignments"
    ///  ],
    ///  "properties": {
    ///    "role_assignments": {
    ///      "description": "Roles directly assigned on this resource",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FleetRoleRoleAssignment"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FleetRolePolicy {
        ///Roles directly assigned on this resource
        pub role_assignments: Vec<FleetRoleRoleAssignment>,
    }

    impl From<&FleetRolePolicy> for FleetRolePolicy {
        fn from(value: &FleetRolePolicy) -> Self {
            value.clone()
        }
    }

    ///Describes the assignment of a particular role on a particular resource
    /// to a particular identity (user, group, etc.)
    ///
    ///The resource is not part of this structure.  Rather, [`RoleAssignment`]s
    /// are put into a [`Policy`] and that Policy is applied to a particular
    /// resource.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes the assignment of a particular role on a
    /// particular resource to a particular identity (user, group, etc.)\n\nThe
    /// resource is not part of this structure.  Rather, [`RoleAssignment`]s are
    /// put into a [`Policy`] and that Policy is applied to a particular
    /// resource.",
    ///  "type": "object",
    ///  "required": [
    ///    "identity_id",
    ///    "identity_type",
    ///    "role_name"
    ///  ],
    ///  "properties": {
    ///    "identity_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "identity_type": {
    ///      "$ref": "#/components/schemas/IdentityType"
    ///    },
    ///    "role_name": {
    ///      "$ref": "#/components/schemas/FleetRole"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FleetRoleRoleAssignment {
        pub identity_id: uuid::Uuid,
        pub identity_type: IdentityType,
        pub role_name: FleetRole,
    }

    impl From<&FleetRoleRoleAssignment> for FleetRoleRoleAssignment {
        fn from(value: &FleetRoleRoleAssignment) -> Self {
            value.clone()
        }
    }

    ///Client view of global Images
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of global Images",
    ///  "type": "object",
    ///  "required": [
    ///    "block_size",
    ///    "description",
    ///    "distribution",
    ///    "id",
    ///    "name",
    ///    "size",
    ///    "time_created",
    ///    "time_modified",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "block_size": {
    ///      "description": "size of blocks in bytes",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ByteCount"
    ///        }
    ///      ]
    ///    },
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "digest": {
    ///      "description": "Hash of the image contents, if applicable",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Digest"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "distribution": {
    ///      "description": "Image distribution",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "size": {
    ///      "description": "total size in bytes",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ByteCount"
    ///        }
    ///      ]
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "url": {
    ///      "description": "URL source of this image, if any",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "version": {
    ///      "description": "Image version",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GlobalImage {
        ///size of blocks in bytes
        pub block_size: ByteCount,
        ///human-readable free-form text about a resource
        pub description: String,
        ///Hash of the image contents, if applicable
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub digest: Option<Digest>,
        ///Image distribution
        pub distribution: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///total size in bytes
        pub size: ByteCount,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        ///URL source of this image, if any
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        ///Image version
        pub version: String,
    }

    impl From<&GlobalImage> for GlobalImage {
        fn from(value: &GlobalImage) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for an
    /// [`GlobalImage`](crate::external_api::views::GlobalImage)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for an
    /// [`GlobalImage`](crate::external_api::views::GlobalImage)",
    ///  "type": "object",
    ///  "required": [
    ///    "block_size",
    ///    "description",
    ///    "distribution",
    ///    "name",
    ///    "source"
    ///  ],
    ///  "properties": {
    ///    "block_size": {
    ///      "description": "block size in bytes",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BlockSize"
    ///        }
    ///      ]
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "distribution": {
    ///      "description": "OS image distribution",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Distribution"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "source": {
    ///      "description": "The source of the image's contents.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ImageSource"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GlobalImageCreate {
        ///block size in bytes
        pub block_size: BlockSize,
        pub description: String,
        ///OS image distribution
        pub distribution: Distribution,
        pub name: Name,
        ///The source of the image's contents.
        pub source: ImageSource,
    }

    impl From<&GlobalImageCreate> for GlobalImageCreate {
        fn from(value: &GlobalImageCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/GlobalImage"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GlobalImageResultsPage {
        ///list of items on this page of results
        pub items: Vec<GlobalImage>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&GlobalImageResultsPage> for GlobalImageResultsPage {
        fn from(value: &GlobalImageResultsPage) -> Self {
            value.clone()
        }
    }

    ///Client view of a [`Group`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Group`]",
    ///  "type": "object",
    ///  "required": [
    ///    "display_name",
    ///    "id",
    ///    "silo_id"
    ///  ],
    ///  "properties": {
    ///    "display_name": {
    ///      "description": "Human-readable name that can identify the group",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "silo_id": {
    ///      "description": "Uuid of the silo to which this group belongs",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Group {
        ///Human-readable name that can identify the group
        pub display_name: String,
        pub id: uuid::Uuid,
        ///Uuid of the silo to which this group belongs
        pub silo_id: uuid::Uuid,
    }

    impl From<&Group> for Group {
        fn from(value: &Group) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Group"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GroupResultsPage {
        ///list of items on this page of results
        pub items: Vec<Group>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&GroupResultsPage> for GroupResultsPage {
        fn from(value: &GroupResultsPage) -> Self {
            value.clone()
        }
    }

    ///A simple type for managing a histogram metric.
    ///
    ///A histogram maintains the count of any number of samples, over a set of
    /// bins. Bins are specified on construction via their _left_ edges,
    /// inclusive. There can't be any "gaps" in the bins, and an additional bin
    /// may be added to the left, right, or both so that the bins extend to the
    /// entire range of the support.
    ///
    ///Note that any gaps, unsorted bins, or non-finite values will result in
    /// an error.
    ///
    ///Example ------- ```rust use oximeter::histogram::{BinRange, Histogram};
    ///
    ///let edges = [0i64, 10, 20]; let mut hist =
    /// Histogram::new(&edges).unwrap(); assert_eq!(hist.n_bins(), 4); // One
    /// additional bin for the range (20..) assert_eq!(hist.n_samples(), 0);
    /// hist.sample(4); hist.sample(100); assert_eq!(hist.n_samples(), 2);
    ///
    ///let data = hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range,
    /// BinRange::range(i64::MIN, 0)); // An additional bin for `..0`
    /// assert_eq!(data[0].count, 0); // Nothing is in this bin
    ///
    ///assert_eq!(data[1].range, BinRange::range(0, 10)); // The range `0..10`
    /// assert_eq!(data[1].count, 1); // 4 is sampled into this bin ```
    ///
    ///Notes -----
    ///
    ///Histograms may be constructed either from their left bin edges, or from
    /// a sequence of ranges. In either case, the left-most bin may be converted
    /// upon construction. In particular, if the left-most value is not equal to
    /// the minimum of the support, a new bin will be added from the minimum to
    /// that provided value. If the left-most value _is_ the support's minimum,
    /// because the provided bin was unbounded below, such as `(..0)`, then that
    /// bin will be converted into one bounded below, `(MIN..0)` in this case.
    ///
    ///The short of this is that, most of the time, it shouldn't matter. If one
    /// specifies the extremes of the support as their bins, be aware that the
    /// left-most may be converted from a `BinRange::RangeTo` into a
    /// `BinRange::Range`. In other words, the first bin of a histogram is
    /// _always_ a `Bin::Range` or a `Bin::RangeFrom` after construction. In
    /// fact, every bin is one of those variants, the `BinRange::RangeTo` is
    /// only provided as a convenience during construction.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A simple type for managing a histogram metric.\n\nA
    /// histogram maintains the count of any number of samples, over a set of
    /// bins. Bins are specified on construction via their _left_ edges,
    /// inclusive. There can't be any \"gaps\" in the bins, and an additional
    /// bin may be added to the left, right, or both so that the bins extend to
    /// the entire range of the support.\n\nNote that any gaps, unsorted bins,
    /// or non-finite values will result in an error.\n\nExample ------- ```rust
    /// use oximeter::histogram::{BinRange, Histogram};\n\nlet edges = [0i64,
    /// 10, 20]; let mut hist = Histogram::new(&edges).unwrap();
    /// assert_eq!(hist.n_bins(), 4); // One additional bin for the range (20..)
    /// assert_eq!(hist.n_samples(), 0); hist.sample(4); hist.sample(100);
    /// assert_eq!(hist.n_samples(), 2);\n\nlet data =
    /// hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range,
    /// BinRange::range(i64::MIN, 0)); // An additional bin for `..0`
    /// assert_eq!(data[0].count, 0); // Nothing is in this
    /// bin\n\nassert_eq!(data[1].range, BinRange::range(0, 10)); // The range
    /// `0..10` assert_eq!(data[1].count, 1); // 4 is sampled into this bin
    /// ```\n\nNotes -----\n\nHistograms may be constructed either from their
    /// left bin edges, or from a sequence of ranges. In either case, the
    /// left-most bin may be converted upon construction. In particular, if the
    /// left-most value is not equal to the minimum of the support, a new bin
    /// will be added from the minimum to that provided value. If the left-most
    /// value _is_ the support's minimum, because the provided bin was unbounded
    /// below, such as `(..0)`, then that bin will be converted into one bounded
    /// below, `(MIN..0)` in this case.\n\nThe short of this is that, most of
    /// the time, it shouldn't matter. If one specifies the extremes of the
    /// support as their bins, be aware that the left-most may be converted from
    /// a `BinRange::RangeTo` into a `BinRange::Range`. In other words, the
    /// first bin of a histogram is _always_ a `Bin::Range` or a
    /// `Bin::RangeFrom` after construction. In fact, every bin is one of those
    /// variants, the `BinRange::RangeTo` is only provided as a convenience
    /// during construction.",
    ///  "type": "object",
    ///  "required": [
    ///    "bins",
    ///    "n_samples",
    ///    "start_time"
    ///  ],
    ///  "properties": {
    ///    "bins": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Bindouble"
    ///      }
    ///    },
    ///    "n_samples": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
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
    pub struct Histogramdouble {
        pub bins: Vec<Bindouble>,
        pub n_samples: u64,
        pub start_time: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Histogramdouble> for Histogramdouble {
        fn from(value: &Histogramdouble) -> Self {
            value.clone()
        }
    }

    ///A simple type for managing a histogram metric.
    ///
    ///A histogram maintains the count of any number of samples, over a set of
    /// bins. Bins are specified on construction via their _left_ edges,
    /// inclusive. There can't be any "gaps" in the bins, and an additional bin
    /// may be added to the left, right, or both so that the bins extend to the
    /// entire range of the support.
    ///
    ///Note that any gaps, unsorted bins, or non-finite values will result in
    /// an error.
    ///
    ///Example ------- ```rust use oximeter::histogram::{BinRange, Histogram};
    ///
    ///let edges = [0i64, 10, 20]; let mut hist =
    /// Histogram::new(&edges).unwrap(); assert_eq!(hist.n_bins(), 4); // One
    /// additional bin for the range (20..) assert_eq!(hist.n_samples(), 0);
    /// hist.sample(4); hist.sample(100); assert_eq!(hist.n_samples(), 2);
    ///
    ///let data = hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range,
    /// BinRange::range(i64::MIN, 0)); // An additional bin for `..0`
    /// assert_eq!(data[0].count, 0); // Nothing is in this bin
    ///
    ///assert_eq!(data[1].range, BinRange::range(0, 10)); // The range `0..10`
    /// assert_eq!(data[1].count, 1); // 4 is sampled into this bin ```
    ///
    ///Notes -----
    ///
    ///Histograms may be constructed either from their left bin edges, or from
    /// a sequence of ranges. In either case, the left-most bin may be converted
    /// upon construction. In particular, if the left-most value is not equal to
    /// the minimum of the support, a new bin will be added from the minimum to
    /// that provided value. If the left-most value _is_ the support's minimum,
    /// because the provided bin was unbounded below, such as `(..0)`, then that
    /// bin will be converted into one bounded below, `(MIN..0)` in this case.
    ///
    ///The short of this is that, most of the time, it shouldn't matter. If one
    /// specifies the extremes of the support as their bins, be aware that the
    /// left-most may be converted from a `BinRange::RangeTo` into a
    /// `BinRange::Range`. In other words, the first bin of a histogram is
    /// _always_ a `Bin::Range` or a `Bin::RangeFrom` after construction. In
    /// fact, every bin is one of those variants, the `BinRange::RangeTo` is
    /// only provided as a convenience during construction.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A simple type for managing a histogram metric.\n\nA
    /// histogram maintains the count of any number of samples, over a set of
    /// bins. Bins are specified on construction via their _left_ edges,
    /// inclusive. There can't be any \"gaps\" in the bins, and an additional
    /// bin may be added to the left, right, or both so that the bins extend to
    /// the entire range of the support.\n\nNote that any gaps, unsorted bins,
    /// or non-finite values will result in an error.\n\nExample ------- ```rust
    /// use oximeter::histogram::{BinRange, Histogram};\n\nlet edges = [0i64,
    /// 10, 20]; let mut hist = Histogram::new(&edges).unwrap();
    /// assert_eq!(hist.n_bins(), 4); // One additional bin for the range (20..)
    /// assert_eq!(hist.n_samples(), 0); hist.sample(4); hist.sample(100);
    /// assert_eq!(hist.n_samples(), 2);\n\nlet data =
    /// hist.iter().collect::<Vec<_>>(); assert_eq!(data[0].range,
    /// BinRange::range(i64::MIN, 0)); // An additional bin for `..0`
    /// assert_eq!(data[0].count, 0); // Nothing is in this
    /// bin\n\nassert_eq!(data[1].range, BinRange::range(0, 10)); // The range
    /// `0..10` assert_eq!(data[1].count, 1); // 4 is sampled into this bin
    /// ```\n\nNotes -----\n\nHistograms may be constructed either from their
    /// left bin edges, or from a sequence of ranges. In either case, the
    /// left-most bin may be converted upon construction. In particular, if the
    /// left-most value is not equal to the minimum of the support, a new bin
    /// will be added from the minimum to that provided value. If the left-most
    /// value _is_ the support's minimum, because the provided bin was unbounded
    /// below, such as `(..0)`, then that bin will be converted into one bounded
    /// below, `(MIN..0)` in this case.\n\nThe short of this is that, most of
    /// the time, it shouldn't matter. If one specifies the extremes of the
    /// support as their bins, be aware that the left-most may be converted from
    /// a `BinRange::RangeTo` into a `BinRange::Range`. In other words, the
    /// first bin of a histogram is _always_ a `Bin::Range` or a
    /// `Bin::RangeFrom` after construction. In fact, every bin is one of those
    /// variants, the `BinRange::RangeTo` is only provided as a convenience
    /// during construction.",
    ///  "type": "object",
    ///  "required": [
    ///    "bins",
    ///    "n_samples",
    ///    "start_time"
    ///  ],
    ///  "properties": {
    ///    "bins": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Binint64"
    ///      }
    ///    },
    ///    "n_samples": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
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
    pub struct Histogramint64 {
        pub bins: Vec<Binint64>,
        pub n_samples: u64,
        pub start_time: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Histogramint64> for Histogramint64 {
        fn from(value: &Histogramint64) -> Self {
            value.clone()
        }
    }

    ///Supported set of sort modes for scanning by id only.
    ///
    ///Currently, we only support scanning in ascending order.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Supported set of sort modes for scanning by id
    /// only.\n\nCurrently, we only support scanning in ascending order.",
    ///  "oneOf": [
    ///    {
    ///      "description": "sort in increasing order of \"id\"",
    ///      "type": "string",
    ///      "enum": [
    ///        "id_ascending"
    ///      ]
    ///    }
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
    pub enum IdSortMode {
        ///sort in increasing order of "id"
        #[serde(rename = "id_ascending")]
        IdAscending,
    }

    impl From<&IdSortMode> for IdSortMode {
        fn from(value: &IdSortMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for IdSortMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::IdAscending => write!(f, "id_ascending"),
            }
        }
    }

    impl std::str::FromStr for IdSortMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "id_ascending" => Ok(Self::IdAscending),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for IdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for IdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for IdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Client view of an [`IdentityProvider`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of an [`IdentityProvider`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "name",
    ///    "provider_type",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "provider_type": {
    ///      "description": "Identity provider type",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IdentityProviderType"
    ///        }
    ///      ]
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IdentityProvider {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///Identity provider type
        pub provider_type: IdentityProviderType,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&IdentityProvider> for IdentityProvider {
        fn from(value: &IdentityProvider) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IdentityProvider"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IdentityProviderResultsPage {
        ///list of items on this page of results
        pub items: Vec<IdentityProvider>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&IdentityProviderResultsPage> for IdentityProviderResultsPage {
        fn from(value: &IdentityProviderResultsPage) -> Self {
            value.clone()
        }
    }

    ///IdentityProviderType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "SAML identity provider",
    ///      "type": "string",
    ///      "enum": [
    ///        "saml"
    ///      ]
    ///    }
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
    pub enum IdentityProviderType {
        ///SAML identity provider
        #[serde(rename = "saml")]
        Saml,
    }

    impl From<&IdentityProviderType> for IdentityProviderType {
        fn from(value: &IdentityProviderType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for IdentityProviderType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Saml => write!(f, "saml"),
            }
        }
    }

    impl std::str::FromStr for IdentityProviderType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "saml" => Ok(Self::Saml),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for IdentityProviderType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for IdentityProviderType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for IdentityProviderType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Describes what kind of identity is described by an id
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes what kind of identity is described by an id",
    ///  "type": "string",
    ///  "enum": [
    ///    "silo_user",
    ///    "silo_group"
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
    pub enum IdentityType {
        #[serde(rename = "silo_user")]
        SiloUser,
        #[serde(rename = "silo_group")]
        SiloGroup,
    }

    impl From<&IdentityType> for IdentityType {
        fn from(value: &IdentityType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for IdentityType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SiloUser => write!(f, "silo_user"),
                Self::SiloGroup => write!(f, "silo_group"),
            }
        }
    }

    impl std::str::FromStr for IdentityType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "silo_user" => Ok(Self::SiloUser),
                "silo_group" => Ok(Self::SiloGroup),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for IdentityType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for IdentityType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for IdentityType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///IdpMetadataSource
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "url"
    ///      ],
    ///      "properties": {
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
    ///        "data",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "data": {
    ///          "type": "string"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "base64_encoded_xml"
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
    pub enum IdpMetadataSource {
        #[serde(rename = "url")]
        Url { url: String },
        #[serde(rename = "base64_encoded_xml")]
        Base64EncodedXml { data: String },
    }

    impl From<&IdpMetadataSource> for IdpMetadataSource {
        fn from(value: &IdpMetadataSource) -> Self {
            value.clone()
        }
    }

    ///Client view of project Images
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of project Images",
    ///  "type": "object",
    ///  "required": [
    ///    "block_size",
    ///    "description",
    ///    "id",
    ///    "name",
    ///    "project_id",
    ///    "size",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "block_size": {
    ///      "description": "size of blocks in bytes",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ByteCount"
    ///        }
    ///      ]
    ///    },
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "digest": {
    ///      "description": "Hash of the image contents, if applicable",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Digest"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "project_id": {
    ///      "description": "The project the disk belongs to",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "size": {
    ///      "description": "total size in bytes",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ByteCount"
    ///        }
    ///      ]
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "url": {
    ///      "description": "URL source of this image, if any",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "version": {
    ///      "description": "Version of this, if any",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Image {
        ///size of blocks in bytes
        pub block_size: ByteCount,
        ///human-readable free-form text about a resource
        pub description: String,
        ///Hash of the image contents, if applicable
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub digest: Option<Digest>,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///The project the disk belongs to
        pub project_id: uuid::Uuid,
        ///total size in bytes
        pub size: ByteCount,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        ///URL source of this image, if any
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        ///Version of this, if any
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }

    impl From<&Image> for Image {
        fn from(value: &Image) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for an
    /// [`Image`](crate::external_api::views::Image)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for an
    /// [`Image`](crate::external_api::views::Image)",
    ///  "type": "object",
    ///  "required": [
    ///    "block_size",
    ///    "description",
    ///    "name",
    ///    "source"
    ///  ],
    ///  "properties": {
    ///    "block_size": {
    ///      "description": "block size in bytes",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BlockSize"
    ///        }
    ///      ]
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "source": {
    ///      "description": "The source of the image's contents.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ImageSource"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ImageCreate {
        ///block size in bytes
        pub block_size: BlockSize,
        pub description: String,
        pub name: Name,
        ///The source of the image's contents.
        pub source: ImageSource,
    }

    impl From<&ImageCreate> for ImageCreate {
        fn from(value: &ImageCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Image"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ImageResultsPage {
        ///list of items on this page of results
        pub items: Vec<Image>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&ImageResultsPage> for ImageResultsPage {
        fn from(value: &ImageResultsPage) -> Self {
            value.clone()
        }
    }

    ///The source of the underlying image.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The source of the underlying image.",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "url"
    ///      ],
    ///      "properties": {
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
    ///        "id",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "id": {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "snapshot"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Boot the Alpine ISO that ships with the Propolis
    /// zone. Intended for development purposes only.",
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "you_can_boot_anything_as_long_as_its_alpine"
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
    pub enum ImageSource {
        #[serde(rename = "url")]
        Url { url: String },
        #[serde(rename = "snapshot")]
        Snapshot { id: uuid::Uuid },
        #[serde(rename = "you_can_boot_anything_as_long_as_its_alpine")]
        YouCanBootAnythingAsLongAsItsAlpine,
    }

    impl From<&ImageSource> for ImageSource {
        fn from(value: &ImageSource) -> Self {
            value.clone()
        }
    }

    ///Client view of an [`Instance`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of an [`Instance`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "hostname",
    ///    "id",
    ///    "memory",
    ///    "name",
    ///    "ncpus",
    ///    "project_id",
    ///    "run_state",
    ///    "time_created",
    ///    "time_modified",
    ///    "time_run_state_updated"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "hostname": {
    ///      "description": "RFC1035-compliant hostname for the Instance.",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "memory": {
    ///      "description": "memory allocated for this Instance",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ByteCount"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "ncpus": {
    ///      "description": "number of CPUs allocated for this Instance",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/InstanceCpuCount"
    ///        }
    ///      ]
    ///    },
    ///    "project_id": {
    ///      "description": "id for the project containing this Instance",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "run_state": {
    ///      "$ref": "#/components/schemas/InstanceState"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_run_state_updated": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Instance {
        ///human-readable free-form text about a resource
        pub description: String,
        ///RFC1035-compliant hostname for the Instance.
        pub hostname: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///memory allocated for this Instance
        pub memory: ByteCount,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///number of CPUs allocated for this Instance
        pub ncpus: InstanceCpuCount,
        ///id for the project containing this Instance
        pub project_id: uuid::Uuid,
        pub run_state: InstanceState,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        pub time_run_state_updated: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Instance> for Instance {
        fn from(value: &Instance) -> Self {
            value.clone()
        }
    }

    ///The number of CPUs in an Instance
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The number of CPUs in an Instance",
    ///  "type": "integer",
    ///  "format": "uint16",
    ///  "minimum": 0.0
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceCpuCount(pub u16);
    impl ::std::ops::Deref for InstanceCpuCount {
        type Target = u16;
        fn deref(&self) -> &u16 {
            &self.0
        }
    }

    impl From<InstanceCpuCount> for u16 {
        fn from(value: InstanceCpuCount) -> Self {
            value.0
        }
    }

    impl From<&InstanceCpuCount> for InstanceCpuCount {
        fn from(value: &InstanceCpuCount) -> Self {
            value.clone()
        }
    }

    impl From<u16> for InstanceCpuCount {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for InstanceCpuCount {
        type Err = <u16 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for InstanceCpuCount {
        type Error = <u16 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for InstanceCpuCount {
        type Error = <u16 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for InstanceCpuCount {
        type Error = <u16 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for InstanceCpuCount {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///Create-time parameters for an
    /// [`Instance`](omicron_common::api::external::Instance)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for an
    /// [`Instance`](omicron_common::api::external::Instance)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "hostname",
    ///    "memory",
    ///    "name",
    ///    "ncpus"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "disks": {
    ///      "description": "The disks to be created or attached for this
    /// instance.",
    ///      "default": [],
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InstanceDiskAttachment"
    ///      }
    ///    },
    ///    "external_ips": {
    ///      "description": "The external IP addresses provided to this
    /// instance.\n\nBy default, all instances have outbound connectivity, but
    /// no inbound connectivity. These external addresses can be used to provide
    /// a fixed, known IP address for making inbound connections to the
    /// instance.",
    ///      "default": [],
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ExternalIpCreate"
    ///      }
    ///    },
    ///    "hostname": {
    ///      "type": "string"
    ///    },
    ///    "memory": {
    ///      "$ref": "#/components/schemas/ByteCount"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "ncpus": {
    ///      "$ref": "#/components/schemas/InstanceCpuCount"
    ///    },
    ///    "network_interfaces": {
    ///      "description": "The network interfaces to be created for this
    /// instance.",
    ///      "default": {
    ///        "type": "default"
    ///      },
    ///      "allOf": [
    ///        {
    ///          "$ref":
    /// "#/components/schemas/InstanceNetworkInterfaceAttachment"
    ///        }
    ///      ]
    ///    },
    ///    "start": {
    ///      "description": "Should this instance be started upon creation; true
    /// by default.",
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "user_data": {
    ///      "description": "User data for instance initialization systems (such
    /// as cloud-init). Must be a Base64-encoded string, as specified in RFC
    /// 4648 § 4 (+ and / characters with padding). Maximum 32 KiB unencoded
    /// data.",
    ///      "default": "",
    ///      "type": "string",
    ///      "format": "byte"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceCreate {
        pub description: String,
        ///The disks to be created or attached for this instance.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub disks: Vec<InstanceDiskAttachment>,
        ///The external IP addresses provided to this instance.
        ///
        ///By default, all instances have outbound connectivity, but no inbound
        /// connectivity. These external addresses can be used to provide a
        /// fixed, known IP address for making inbound connections to the
        /// instance.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub external_ips: Vec<ExternalIpCreate>,
        pub hostname: String,
        pub memory: ByteCount,
        pub name: Name,
        pub ncpus: InstanceCpuCount,
        ///The network interfaces to be created for this instance.
        #[serde(default = "defaults::instance_create_network_interfaces")]
        pub network_interfaces: InstanceNetworkInterfaceAttachment,
        ///Should this instance be started upon creation; true by default.
        #[serde(default = "defaults::default_bool::<true>")]
        pub start: bool,
        ///User data for instance initialization systems (such as cloud-init).
        /// Must be a Base64-encoded string, as specified in RFC 4648 § 4 (+ and
        /// / characters with padding). Maximum 32 KiB unencoded data.
        #[serde(default)]
        pub user_data: String,
    }

    impl From<&InstanceCreate> for InstanceCreate {
        fn from(value: &InstanceCreate) -> Self {
            value.clone()
        }
    }

    ///Describe the instance's disks at creation time
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describe the instance's disks at creation time",
    ///  "oneOf": [
    ///    {
    ///      "description": "During instance creation, create and attach disks",
    ///      "type": "object",
    ///      "required": [
    ///        "description",
    ///        "disk_source",
    ///        "name",
    ///        "size",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "description": {
    ///          "type": "string"
    ///        },
    ///        "disk_source": {
    ///          "description": "initial source for this disk",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/DiskSource"
    ///            }
    ///          ]
    ///        },
    ///        "name": {
    ///          "$ref": "#/components/schemas/Name"
    ///        },
    ///        "size": {
    ///          "description": "total size of the Disk in bytes",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/ByteCount"
    ///            }
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "create"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "During instance creation, attach this disk",
    ///      "type": "object",
    ///      "required": [
    ///        "name",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "name": {
    ///          "description": "A disk name to attach",
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
    ///            }
    ///          ]
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "attach"
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
    pub enum InstanceDiskAttachment {
        ///During instance creation, create and attach disks
        #[serde(rename = "create")]
        Create {
            description: String,
            ///initial source for this disk
            disk_source: DiskSource,
            name: Name,
            ///total size of the Disk in bytes
            size: ByteCount,
        },
        ///During instance creation, attach this disk
        #[serde(rename = "attach")]
        Attach {
            ///A disk name to attach
            name: Name,
        },
    }

    impl From<&InstanceDiskAttachment> for InstanceDiskAttachment {
        fn from(value: &InstanceDiskAttachment) -> Self {
            value.clone()
        }
    }

    ///Migration parameters for an
    /// [`Instance`](omicron_common::api::external::Instance)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Migration parameters for an
    /// [`Instance`](omicron_common::api::external::Instance)",
    ///  "type": "object",
    ///  "required": [
    ///    "dst_sled_id"
    ///  ],
    ///  "properties": {
    ///    "dst_sled_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceMigrate {
        pub dst_sled_id: uuid::Uuid,
    }

    impl From<&InstanceMigrate> for InstanceMigrate {
        fn from(value: &InstanceMigrate) -> Self {
            value.clone()
        }
    }

    ///Describes an attachment of a `NetworkInterface` to an `Instance`, at the
    /// time the instance is created.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes an attachment of a `NetworkInterface` to an
    /// `Instance`, at the time the instance is created.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Create one or more `NetworkInterface`s for the
    /// `Instance`.\n\nIf more than one interface is provided, then the first
    /// will be designated the primary interface for the instance.",
    ///      "type": "object",
    ///      "required": [
    ///        "params",
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "params": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/NetworkInterfaceCreate"
    ///          }
    ///        },
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "create"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The default networking configuration for an
    /// instance is to create a single primary interface with an
    /// automatically-assigned IP address. The IP will be pulled from the
    /// Project's default VPC / VPC Subnet.",
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "default"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "No network interfaces at all will be created for
    /// the instance.",
    ///      "type": "object",
    ///      "required": [
    ///        "type"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "none"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "params")]
    pub enum InstanceNetworkInterfaceAttachment {
        ///Create one or more `NetworkInterface`s for the `Instance`.
        ///
        ///If more than one interface is provided, then the first will be
        /// designated the primary interface for the instance.
        #[serde(rename = "create")]
        Create(Vec<NetworkInterfaceCreate>),
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "none")]
        None,
    }

    impl From<&InstanceNetworkInterfaceAttachment> for InstanceNetworkInterfaceAttachment {
        fn from(value: &InstanceNetworkInterfaceAttachment) -> Self {
            value.clone()
        }
    }

    impl From<Vec<NetworkInterfaceCreate>> for InstanceNetworkInterfaceAttachment {
        fn from(value: Vec<NetworkInterfaceCreate>) -> Self {
            Self::Create(value)
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Instance"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceResultsPage {
        ///list of items on this page of results
        pub items: Vec<Instance>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&InstanceResultsPage> for InstanceResultsPage {
        fn from(value: &InstanceResultsPage) -> Self {
            value.clone()
        }
    }

    ///Contents of an Instance's serial console buffer.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Contents of an Instance's serial console buffer.",
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "last_byte_offset"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "description": "The bytes starting from the requested offset up to
    /// either the end of the buffer or the request's `max_bytes`. Provided as a
    /// u8 array rather than a string, as it may not be UTF-8.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint8",
    ///        "minimum": 0.0
    ///      }
    ///    },
    ///    "last_byte_offset": {
    ///      "description": "The absolute offset since boot (suitable for use as
    /// `byte_offset` in a subsequent request) of the last byte returned in
    /// `data`.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstanceSerialConsoleData {
        ///The bytes starting from the requested offset up to either the end of
        /// the buffer or the request's `max_bytes`. Provided as a u8 array
        /// rather than a string, as it may not be UTF-8.
        pub data: Vec<u8>,
        ///The absolute offset since boot (suitable for use as `byte_offset` in
        /// a subsequent request) of the last byte returned in `data`.
        pub last_byte_offset: u64,
    }

    impl From<&InstanceSerialConsoleData> for InstanceSerialConsoleData {
        fn from(value: &InstanceSerialConsoleData) -> Self {
            value.clone()
        }
    }

    ///Running state of an Instance (primarily: booted or stopped)
    ///
    ///This typically reflects whether it's starting, running, stopping, or
    /// stopped, but also includes states related to the Instance's lifecycle
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Running state of an Instance (primarily: booted or
    /// stopped)\n\nThis typically reflects whether it's starting, running,
    /// stopping, or stopped, but also includes states related to the Instance's
    /// lifecycle",
    ///  "oneOf": [
    ///    {
    ///      "description": "The instance is being created.",
    ///      "type": "string",
    ///      "enum": [
    ///        "creating"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The instance is currently starting up.",
    ///      "type": "string",
    ///      "enum": [
    ///        "starting"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The instance is currently running.",
    ///      "type": "string",
    ///      "enum": [
    ///        "running"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The instance has been requested to stop and a
    /// transition to \"Stopped\" is imminent.",
    ///      "type": "string",
    ///      "enum": [
    ///        "stopping"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The instance is currently stopped.",
    ///      "type": "string",
    ///      "enum": [
    ///        "stopped"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The instance is in the process of rebooting - it
    /// will remain in the \"rebooting\" state until the VM is starting once
    /// more.",
    ///      "type": "string",
    ///      "enum": [
    ///        "rebooting"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The instance is in the process of migrating - it will remain in the \"migrating\" state until the migration process is complete and the destination propolis is ready to continue execution.",
    ///      "type": "string",
    ///      "enum": [
    ///        "migrating"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The instance is attempting to recover from a
    /// failure.",
    ///      "type": "string",
    ///      "enum": [
    ///        "repairing"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The instance has encountered a failure.",
    ///      "type": "string",
    ///      "enum": [
    ///        "failed"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The instance has been deleted.",
    ///      "type": "string",
    ///      "enum": [
    ///        "destroyed"
    ///      ]
    ///    }
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
        ///The instance is being created.
        #[serde(rename = "creating")]
        Creating,
        ///The instance is currently starting up.
        #[serde(rename = "starting")]
        Starting,
        ///The instance is currently running.
        #[serde(rename = "running")]
        Running,
        ///The instance has been requested to stop and a transition to
        /// "Stopped" is imminent.
        #[serde(rename = "stopping")]
        Stopping,
        ///The instance is currently stopped.
        #[serde(rename = "stopped")]
        Stopped,
        ///The instance is in the process of rebooting - it will remain in the
        /// "rebooting" state until the VM is starting once more.
        #[serde(rename = "rebooting")]
        Rebooting,
        ///The instance is in the process of migrating - it will remain in the
        /// "migrating" state until the migration process is complete and the
        /// destination propolis is ready to continue execution.
        #[serde(rename = "migrating")]
        Migrating,
        ///The instance is attempting to recover from a failure.
        #[serde(rename = "repairing")]
        Repairing,
        ///The instance has encountered a failure.
        #[serde(rename = "failed")]
        Failed,
        ///The instance has been deleted.
        #[serde(rename = "destroyed")]
        Destroyed,
    }

    impl From<&InstanceState> for InstanceState {
        fn from(value: &InstanceState) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for InstanceState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Creating => write!(f, "creating"),
                Self::Starting => write!(f, "starting"),
                Self::Running => write!(f, "running"),
                Self::Stopping => write!(f, "stopping"),
                Self::Stopped => write!(f, "stopped"),
                Self::Rebooting => write!(f, "rebooting"),
                Self::Migrating => write!(f, "migrating"),
                Self::Repairing => write!(f, "repairing"),
                Self::Failed => write!(f, "failed"),
                Self::Destroyed => write!(f, "destroyed"),
            }
        }
    }

    impl std::str::FromStr for InstanceState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "creating" => Ok(Self::Creating),
                "starting" => Ok(Self::Starting),
                "running" => Ok(Self::Running),
                "stopping" => Ok(Self::Stopping),
                "stopped" => Ok(Self::Stopped),
                "rebooting" => Ok(Self::Rebooting),
                "migrating" => Ok(Self::Migrating),
                "repairing" => Ok(Self::Repairing),
                "failed" => Ok(Self::Failed),
                "destroyed" => Ok(Self::Destroyed),
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

    ///The kind of an external IP address for an instance
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The kind of an external IP address for an instance",
    ///  "type": "string",
    ///  "enum": [
    ///    "ephemeral",
    ///    "floating"
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
    pub enum IpKind {
        #[serde(rename = "ephemeral")]
        Ephemeral,
        #[serde(rename = "floating")]
        Floating,
    }

    impl From<&IpKind> for IpKind {
        fn from(value: &IpKind) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for IpKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ephemeral => write!(f, "ephemeral"),
                Self::Floating => write!(f, "floating"),
            }
        }
    }

    impl std::str::FromStr for IpKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "ephemeral" => Ok(Self::Ephemeral),
                "floating" => Ok(Self::Floating),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for IpKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for IpKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for IpKind {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///IpNet
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "title": "v4",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Ipv4Net"
    ///        }
    ///      ]
    ///    },
    ///    {
    ///      "title": "v6",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Ipv6Net"
    ///        }
    ///      ]
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum IpNet {
        V4(Ipv4Net),
        V6(Ipv6Net),
    }

    impl From<&IpNet> for IpNet {
        fn from(value: &IpNet) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for IpNet {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if let Ok(v) = value.parse() {
                Ok(Self::V4(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::V6(v))
            } else {
                Err("string conversion failed for all variants".into())
            }
        }
    }

    impl std::convert::TryFrom<&str> for IpNet {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for IpNet {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for IpNet {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for IpNet {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::V4(x) => x.fmt(f),
                Self::V6(x) => x.fmt(f),
            }
        }
    }

    impl From<Ipv4Net> for IpNet {
        fn from(value: Ipv4Net) -> Self {
            Self::V4(value)
        }
    }

    impl From<Ipv6Net> for IpNet {
        fn from(value: Ipv6Net) -> Self {
            Self::V6(value)
        }
    }

    ///Identity-related metadata that's included in nearly all public API
    /// objects
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Identity-related metadata that's included in nearly all
    /// public API objects",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "name",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPool {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&IpPool> for IpPool {
        fn from(value: &IpPool) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for an IP Pool.
    ///
    ///See [`IpPool`](crate::external_api::views::IpPool)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for an IP Pool.\n\nSee
    /// [`IpPool`](crate::external_api::views::IpPool)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPoolCreate {
        pub description: String,
        pub name: Name,
    }

    impl From<&IpPoolCreate> for IpPoolCreate {
        fn from(value: &IpPoolCreate) -> Self {
            value.clone()
        }
    }

    ///IpPoolRange
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "range",
    ///    "time_created"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "range": {
    ///      "$ref": "#/components/schemas/IpRange"
    ///    },
    ///    "time_created": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPoolRange {
        pub id: uuid::Uuid,
        pub range: IpRange,
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&IpPoolRange> for IpPoolRange {
        fn from(value: &IpPoolRange) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IpPoolRange"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPoolRangeResultsPage {
        ///list of items on this page of results
        pub items: Vec<IpPoolRange>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&IpPoolRangeResultsPage> for IpPoolRangeResultsPage {
        fn from(value: &IpPoolRangeResultsPage) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IpPool"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpPoolResultsPage {
        ///list of items on this page of results
        pub items: Vec<IpPool>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&IpPoolResultsPage> for IpPoolResultsPage {
        fn from(value: &IpPoolResultsPage) -> Self {
            value.clone()
        }
    }

    ///Parameters for updating an IP Pool
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Parameters for updating an IP Pool",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
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
    pub struct IpPoolUpdate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
    }

    impl From<&IpPoolUpdate> for IpPoolUpdate {
        fn from(value: &IpPoolUpdate) -> Self {
            value.clone()
        }
    }

    ///IpRange
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "title": "v4",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Ipv4Range"
    ///        }
    ///      ]
    ///    },
    ///    {
    ///      "title": "v6",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Ipv6Range"
    ///        }
    ///      ]
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum IpRange {
        V4(Ipv4Range),
        V6(Ipv6Range),
    }

    impl From<&IpRange> for IpRange {
        fn from(value: &IpRange) -> Self {
            value.clone()
        }
    }

    impl From<Ipv4Range> for IpRange {
        fn from(value: Ipv4Range) -> Self {
            Self::V4(value)
        }
    }

    impl From<Ipv6Range> for IpRange {
        fn from(value: Ipv6Range) -> Self {
            Self::V6(value)
        }
    }

    ///An IPv4 subnet, including prefix and subnet mask
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "An IPv4 subnet",
    ///  "description": "An IPv4 subnet, including prefix and subnet mask",
    ///  "examples": [
    ///    "192.168.1.0/24"
    ///  ],
    ///  "type": "string",
    ///  "pattern":
    /// "^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.
    /// ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])/
    /// ([8-9]|1[0-9]|2[0-9]|3[0-2])$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Ipv4Net(String);
    impl ::std::ops::Deref for Ipv4Net {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Ipv4Net> for String {
        fn from(value: Ipv4Net) -> Self {
            value.0
        }
    }

    impl From<&Ipv4Net> for Ipv4Net {
        fn from(value: &Ipv4Net) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for Ipv4Net {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new(
                "^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.\
                 ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])/\
                 ([8-9]|1[0-9]|2[0-9]|3[0-2])$",
            )
            .unwrap()
            .find(value)
            .is_none()
            {
                return Err("doesn't match pattern \
                            \"^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.\
                            ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])/\
                            ([8-9]|1[0-9]|2[0-9]|3[0-2])$\""
                    .into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for Ipv4Net {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Ipv4Net {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Ipv4Net {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Ipv4Net {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///A non-decreasing IPv4 address range, inclusive of both ends.
    ///
    ///The first address must be less than or equal to the last address.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A non-decreasing IPv4 address range, inclusive of both
    /// ends.\n\nThe first address must be less than or equal to the last
    /// address.",
    ///  "type": "object",
    ///  "required": [
    ///    "first",
    ///    "last"
    ///  ],
    ///  "properties": {
    ///    "first": {
    ///      "type": "string",
    ///      "format": "ipv4"
    ///    },
    ///    "last": {
    ///      "type": "string",
    ///      "format": "ipv4"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Ipv4Range {
        pub first: std::net::Ipv4Addr,
        pub last: std::net::Ipv4Addr,
    }

    impl From<&Ipv4Range> for Ipv4Range {
        fn from(value: &Ipv4Range) -> Self {
            value.clone()
        }
    }

    ///An IPv6 subnet, including prefix and subnet mask
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "An IPv6 subnet",
    ///  "description": "An IPv6 subnet, including prefix and subnet mask",
    ///  "examples": [
    ///    "fd12:3456::/64"
    ///  ],
    ///  "type": "string",
    ///  "pattern":
    /// "^([fF][dD])[0-9a-fA-F]{2}:(([0-9a-fA-F]{1,4}:){6}[0-9a-fA-F]{1,
    /// 4}|([0-9a-fA-F]{1,4}:){1,6}:)\\/([1-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8])$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Ipv6Net(String);
    impl ::std::ops::Deref for Ipv6Net {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Ipv6Net> for String {
        fn from(value: Ipv6Net) -> Self {
            value.0
        }
    }

    impl From<&Ipv6Net> for Ipv6Net {
        fn from(value: &Ipv6Net) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for Ipv6Net {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new(
                "^([fF][dD])[0-9a-fA-F]{2}:(([0-9a-fA-F]{1,4}:){6}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,\
                 4}:){1,6}:)\\/([1-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8])$",
            )
            .unwrap()
            .find(value)
            .is_none()
            {
                return Err("doesn't match pattern \
                            \"^([fF][dD])[0-9a-fA-F]{2}:(([0-9a-fA-F]{1,4}:){6}[0-9a-fA-F]{1,\
                            4}|([0-9a-fA-F]{1,4}:){1,6}:)\\/\
                            ([1-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8])$\""
                    .into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for Ipv6Net {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Ipv6Net {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Ipv6Net {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Ipv6Net {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///A non-decreasing IPv6 address range, inclusive of both ends.
    ///
    ///The first address must be less than or equal to the last address.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A non-decreasing IPv6 address range, inclusive of both
    /// ends.\n\nThe first address must be less than or equal to the last
    /// address.",
    ///  "type": "object",
    ///  "required": [
    ///    "first",
    ///    "last"
    ///  ],
    ///  "properties": {
    ///    "first": {
    ///      "type": "string",
    ///      "format": "ipv6"
    ///    },
    ///    "last": {
    ///      "type": "string",
    ///      "format": "ipv6"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Ipv6Range {
        pub first: std::net::Ipv6Addr,
        pub last: std::net::Ipv6Addr,
    }

    impl From<&Ipv6Range> for Ipv6Range {
        fn from(value: &Ipv6Range) -> Self {
            value.clone()
        }
    }

    ///An inclusive-inclusive range of IP ports. The second port may be omitted
    /// to represent a single port
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "A range of IP ports",
    ///  "description": "An inclusive-inclusive range of IP ports. The second
    /// port may be omitted to represent a single port",
    ///  "examples": [
    ///    "22"
    ///  ],
    ///  "type": "string",
    ///  "maxLength": 11,
    ///  "minLength": 1,
    ///  "pattern": "^[0-9]{1,5}(-[0-9]{1,5})?$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct L4PortRange(String);
    impl ::std::ops::Deref for L4PortRange {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<L4PortRange> for String {
        fn from(value: L4PortRange) -> Self {
            value.0
        }
    }

    impl From<&L4PortRange> for L4PortRange {
        fn from(value: &L4PortRange) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for L4PortRange {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if value.len() > 11usize {
                return Err("longer than 11 characters".into());
            }
            if value.len() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            if regress::Regex::new("^[0-9]{1,5}(-[0-9]{1,5})?$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^[0-9]{1,5}(-[0-9]{1,5})?$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for L4PortRange {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for L4PortRange {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for L4PortRange {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for L4PortRange {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///A Media Access Control address, in EUI-48 format
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "A MAC address",
    ///  "description": "A Media Access Control address, in EUI-48 format",
    ///  "examples": [
    ///    "ff:ff:ff:ff:ff:ff"
    ///  ],
    ///  "type": "string",
    ///  "maxLength": 17,
    ///  "minLength": 17,
    ///  "pattern": "^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct MacAddr(String);
    impl ::std::ops::Deref for MacAddr {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<MacAddr> for String {
        fn from(value: MacAddr) -> Self {
            value.0
        }
    }

    impl From<&MacAddr> for MacAddr {
        fn from(value: &MacAddr) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for MacAddr {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if value.len() > 17usize {
                return Err("longer than 17 characters".into());
            }
            if value.len() < 17usize {
                return Err("shorter than 17 characters".into());
            }
            if regress::Regex::new("^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err(
                    "doesn't match pattern \"^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$\"".into(),
                );
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for MacAddr {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for MacAddr {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for MacAddr {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for MacAddr {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///A `Measurement` is a timestamped datum from a single metric
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A `Measurement` is a timestamped datum from a single
    /// metric",
    ///  "type": "object",
    ///  "required": [
    ///    "datum",
    ///    "timestamp"
    ///  ],
    ///  "properties": {
    ///    "datum": {
    ///      "$ref": "#/components/schemas/Datum"
    ///    },
    ///    "timestamp": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Measurement {
        pub datum: Datum,
        pub timestamp: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Measurement> for Measurement {
        fn from(value: &Measurement) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Measurement"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MeasurementResultsPage {
        ///list of items on this page of results
        pub items: Vec<Measurement>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&MeasurementResultsPage> for MeasurementResultsPage {
        fn from(value: &MeasurementResultsPage) -> Self {
            value.clone()
        }
    }

    ///Names must begin with a lower case ASCII letter, be composed exclusively
    /// of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end
    /// with a '-'. Names cannot be a UUID though they may contain a UUID.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "A name unique within the parent collection",
    ///  "description": "Names must begin with a lower case ASCII letter, be
    /// composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and
    /// '-', and may not end with a '-'. Names cannot be a UUID though they may
    /// contain a UUID.",
    ///  "type": "string",
    ///  "maxLength": 63,
    ///  "pattern":
    /// "^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*
    /// [a-zA-Z0-9]$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Name(String);
    impl ::std::ops::Deref for Name {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Name> for String {
        fn from(value: Name) -> Self {
            value.0
        }
    }

    impl From<&Name> for Name {
        fn from(value: &Name) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for Name {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if value.len() > 63usize {
                return Err("longer than 63 characters".into());
            }
            if regress :: Regex :: new ("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$") . unwrap () . find (value) . is_none () { return Err ("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\"" . into ()) ; }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for Name {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Name {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Name {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///NameOrId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "title": "id",
    ///      "allOf": [
    ///        {
    ///          "type": "string",
    ///          "format": "uuid"
    ///        }
    ///      ]
    ///    },
    ///    {
    ///      "title": "name",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum NameOrId {
        Id(uuid::Uuid),
        Name(Name),
    }

    impl From<&NameOrId> for NameOrId {
        fn from(value: &NameOrId) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for NameOrId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if let Ok(v) = value.parse() {
                Ok(Self::Id(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::Name(v))
            } else {
                Err("string conversion failed for all variants".into())
            }
        }
    }

    impl std::convert::TryFrom<&str> for NameOrId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NameOrId {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NameOrId {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for NameOrId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Id(x) => x.fmt(f),
                Self::Name(x) => x.fmt(f),
            }
        }
    }

    impl From<uuid::Uuid> for NameOrId {
        fn from(value: uuid::Uuid) -> Self {
            Self::Id(value)
        }
    }

    impl From<Name> for NameOrId {
        fn from(value: Name) -> Self {
            Self::Name(value)
        }
    }

    ///Supported set of sort modes for scanning by name or id
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Supported set of sort modes for scanning by name or
    /// id",
    ///  "oneOf": [
    ///    {
    ///      "description": "sort in increasing order of \"name\"",
    ///      "type": "string",
    ///      "enum": [
    ///        "name_ascending"
    ///      ]
    ///    },
    ///    {
    ///      "description": "sort in decreasing order of \"name\"",
    ///      "type": "string",
    ///      "enum": [
    ///        "name_descending"
    ///      ]
    ///    },
    ///    {
    ///      "description": "sort in increasing order of \"id\"",
    ///      "type": "string",
    ///      "enum": [
    ///        "id_ascending"
    ///      ]
    ///    }
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
    pub enum NameOrIdSortMode {
        ///sort in increasing order of "name"
        #[serde(rename = "name_ascending")]
        NameAscending,
        ///sort in decreasing order of "name"
        #[serde(rename = "name_descending")]
        NameDescending,
        ///sort in increasing order of "id"
        #[serde(rename = "id_ascending")]
        IdAscending,
    }

    impl From<&NameOrIdSortMode> for NameOrIdSortMode {
        fn from(value: &NameOrIdSortMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for NameOrIdSortMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::NameAscending => write!(f, "name_ascending"),
                Self::NameDescending => write!(f, "name_descending"),
                Self::IdAscending => write!(f, "id_ascending"),
            }
        }
    }

    impl std::str::FromStr for NameOrIdSortMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "name_ascending" => Ok(Self::NameAscending),
                "name_descending" => Ok(Self::NameDescending),
                "id_ascending" => Ok(Self::IdAscending),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for NameOrIdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NameOrIdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NameOrIdSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Supported set of sort modes for scanning by name only
    ///
    ///Currently, we only support scanning in ascending order.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Supported set of sort modes for scanning by name
    /// only\n\nCurrently, we only support scanning in ascending order.",
    ///  "oneOf": [
    ///    {
    ///      "description": "sort in increasing order of \"name\"",
    ///      "type": "string",
    ///      "enum": [
    ///        "name_ascending"
    ///      ]
    ///    }
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
    pub enum NameSortMode {
        ///sort in increasing order of "name"
        #[serde(rename = "name_ascending")]
        NameAscending,
    }

    impl From<&NameSortMode> for NameSortMode {
        fn from(value: &NameSortMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for NameSortMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::NameAscending => write!(f, "name_ascending"),
            }
        }
    }

    impl std::str::FromStr for NameSortMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "name_ascending" => Ok(Self::NameAscending),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for NameSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NameSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NameSortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A `NetworkInterface` represents a virtual network interface device.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A `NetworkInterface` represents a virtual network
    /// interface device.",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "instance_id",
    ///    "ip",
    ///    "mac",
    ///    "name",
    ///    "primary",
    ///    "subnet_id",
    ///    "time_created",
    ///    "time_modified",
    ///    "vpc_id"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "instance_id": {
    ///      "description": "The Instance to which the interface belongs.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "ip": {
    ///      "description": "The IP address assigned to this interface.",
    ///      "type": "string",
    ///      "format": "ip"
    ///    },
    ///    "mac": {
    ///      "description": "The MAC address assigned to this interface.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/MacAddr"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "primary": {
    ///      "description": "True if this interface is the primary for the
    /// instance to which it's attached.",
    ///      "type": "boolean"
    ///    },
    ///    "subnet_id": {
    ///      "description": "The subnet to which the interface belongs.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "vpc_id": {
    ///      "description": "The VPC to which the interface belongs.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterface {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///The Instance to which the interface belongs.
        pub instance_id: uuid::Uuid,
        ///The IP address assigned to this interface.
        pub ip: std::net::IpAddr,
        ///The MAC address assigned to this interface.
        pub mac: MacAddr,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///True if this interface is the primary for the instance to which it's
        /// attached.
        pub primary: bool,
        ///The subnet to which the interface belongs.
        pub subnet_id: uuid::Uuid,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        ///The VPC to which the interface belongs.
        pub vpc_id: uuid::Uuid,
    }

    impl From<&NetworkInterface> for NetworkInterface {
        fn from(value: &NetworkInterface) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a
    /// [`NetworkInterface`](omicron_common::api::external::NetworkInterface)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a [`NetworkInterface`](omicron_common::api::external::NetworkInterface)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "name",
    ///    "subnet_name",
    ///    "vpc_name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "ip": {
    ///      "description": "The IP address for the interface. One will be
    /// auto-assigned if not provided.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "ip"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "subnet_name": {
    ///      "description": "The VPC Subnet in which to create the interface.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "vpc_name": {
    ///      "description": "The VPC in which to create the interface.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterfaceCreate {
        pub description: String,
        ///The IP address for the interface. One will be auto-assigned if not
        /// provided.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ip: Option<std::net::IpAddr>,
        pub name: Name,
        ///The VPC Subnet in which to create the interface.
        pub subnet_name: Name,
        ///The VPC in which to create the interface.
        pub vpc_name: Name,
    }

    impl From<&NetworkInterfaceCreate> for NetworkInterfaceCreate {
        fn from(value: &NetworkInterfaceCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/NetworkInterface"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterfaceResultsPage {
        ///list of items on this page of results
        pub items: Vec<NetworkInterface>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&NetworkInterfaceResultsPage> for NetworkInterfaceResultsPage {
        fn from(value: &NetworkInterfaceResultsPage) -> Self {
            value.clone()
        }
    }

    ///Parameters for updating a
    /// [`NetworkInterface`](omicron_common::api::external::NetworkInterface).
    ///
    ///Note that modifying IP addresses for an interface is not yet supported,
    /// a new interface must be created instead.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Parameters for updating a
    /// [`NetworkInterface`](omicron_common::api::external::NetworkInterface).\
    /// n\nNote that modifying IP addresses for an interface is not yet
    /// supported, a new interface must be created instead.",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "primary": {
    ///      "description": "Make a secondary interface the instance's primary
    /// interface.\n\nIf applied to a secondary interface, that interface will
    /// become the primary on the next reboot of the instance. Note that this
    /// may have implications for routing between instances, as the new primary
    /// interface will be on a distinct subnet from the previous primary
    /// interface.\n\nNote that this can only be used to select a new primary
    /// interface for an instance. Requests to change the primary interface into
    /// a secondary will return an error.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkInterfaceUpdate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
        ///Make a secondary interface the instance's primary interface.
        ///
        ///If applied to a secondary interface, that interface will become the
        /// primary on the next reboot of the instance. Note that this may have
        /// implications for routing between instances, as the new primary
        /// interface will be on a distinct subnet from the previous primary
        /// interface.
        ///
        ///Note that this can only be used to select a new primary interface
        /// for an instance. Requests to change the primary interface into a
        /// secondary will return an error.
        #[serde(default)]
        pub primary: bool,
    }

    impl From<&NetworkInterfaceUpdate> for NetworkInterfaceUpdate {
        fn from(value: &NetworkInterfaceUpdate) -> Self {
            value.clone()
        }
    }

    ///Unique name for a saga [`Node`]
    ///
    ///Each node requires a string name that's unique within its DAG.  The name
    /// is used to identify its output.  Nodes that depend on a given node
    /// (either directly or indirectly) can access the node's output using its
    /// name.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Unique name for a saga [`Node`]\n\nEach node requires a
    /// string name that's unique within its DAG.  The name is used to identify
    /// its output.  Nodes that depend on a given node (either directly or
    /// indirectly) can access the node's output using its name.",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct NodeName(pub String);
    impl ::std::ops::Deref for NodeName {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<NodeName> for String {
        fn from(value: NodeName) -> Self {
            value.0
        }
    }

    impl From<&NodeName> for NodeName {
        fn from(value: &NodeName) -> Self {
            value.clone()
        }
    }

    impl From<String> for NodeName {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for NodeName {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for NodeName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///Client view of an [`Organization`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of an [`Organization`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "name",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Organization {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Organization> for Organization {
        fn from(value: &Organization) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for an
    /// [`Organization`](crate::external_api::views::Organization)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for an
    /// [`Organization`](crate::external_api::views::Organization)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationCreate {
        pub description: String,
        pub name: Name,
    }

    impl From<&OrganizationCreate> for OrganizationCreate {
        fn from(value: &OrganizationCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Organization"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationResultsPage {
        ///list of items on this page of results
        pub items: Vec<Organization>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&OrganizationResultsPage> for OrganizationResultsPage {
        fn from(value: &OrganizationResultsPage) -> Self {
            value.clone()
        }
    }

    ///OrganizationRole
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "admin",
    ///    "collaborator",
    ///    "viewer"
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
    pub enum OrganizationRole {
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "collaborator")]
        Collaborator,
        #[serde(rename = "viewer")]
        Viewer,
    }

    impl From<&OrganizationRole> for OrganizationRole {
        fn from(value: &OrganizationRole) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for OrganizationRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Admin => write!(f, "admin"),
                Self::Collaborator => write!(f, "collaborator"),
                Self::Viewer => write!(f, "viewer"),
            }
        }
    }

    impl std::str::FromStr for OrganizationRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "admin" => Ok(Self::Admin),
                "collaborator" => Ok(Self::Collaborator),
                "viewer" => Ok(Self::Viewer),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for OrganizationRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for OrganizationRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for OrganizationRole {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Client view of a [`Policy`], which describes how this resource may be
    /// accessed
    ///
    ///Note that the Policy only describes access granted explicitly for this
    /// resource.  The policies of parent resources can also cause a user to
    /// have access to this resource.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Policy`], which describes how this
    /// resource may be accessed\n\nNote that the Policy only describes access
    /// granted explicitly for this resource.  The policies of parent resources
    /// can also cause a user to have access to this resource.",
    ///  "type": "object",
    ///  "required": [
    ///    "role_assignments"
    ///  ],
    ///  "properties": {
    ///    "role_assignments": {
    ///      "description": "Roles directly assigned on this resource",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrganizationRoleRoleAssignment"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationRolePolicy {
        ///Roles directly assigned on this resource
        pub role_assignments: Vec<OrganizationRoleRoleAssignment>,
    }

    impl From<&OrganizationRolePolicy> for OrganizationRolePolicy {
        fn from(value: &OrganizationRolePolicy) -> Self {
            value.clone()
        }
    }

    ///Describes the assignment of a particular role on a particular resource
    /// to a particular identity (user, group, etc.)
    ///
    ///The resource is not part of this structure.  Rather, [`RoleAssignment`]s
    /// are put into a [`Policy`] and that Policy is applied to a particular
    /// resource.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes the assignment of a particular role on a
    /// particular resource to a particular identity (user, group, etc.)\n\nThe
    /// resource is not part of this structure.  Rather, [`RoleAssignment`]s are
    /// put into a [`Policy`] and that Policy is applied to a particular
    /// resource.",
    ///  "type": "object",
    ///  "required": [
    ///    "identity_id",
    ///    "identity_type",
    ///    "role_name"
    ///  ],
    ///  "properties": {
    ///    "identity_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "identity_type": {
    ///      "$ref": "#/components/schemas/IdentityType"
    ///    },
    ///    "role_name": {
    ///      "$ref": "#/components/schemas/OrganizationRole"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationRoleRoleAssignment {
        pub identity_id: uuid::Uuid,
        pub identity_type: IdentityType,
        pub role_name: OrganizationRole,
    }

    impl From<&OrganizationRoleRoleAssignment> for OrganizationRoleRoleAssignment {
        fn from(value: &OrganizationRoleRoleAssignment) -> Self {
            value.clone()
        }
    }

    ///Updateable properties of an
    /// [`Organization`](crate::external_api::views::Organization)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Updateable properties of an
    /// [`Organization`](crate::external_api::views::Organization)",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
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
    pub struct OrganizationUpdate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
    }

    impl From<&OrganizationUpdate> for OrganizationUpdate {
        fn from(value: &OrganizationUpdate) -> Self {
            value.clone()
        }
    }

    ///Passwords may be subject to additional constraints.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "A password used to authenticate a user",
    ///  "description": "Passwords may be subject to additional constraints.",
    ///  "type": "string",
    ///  "maxLength": 512
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Password(String);
    impl ::std::ops::Deref for Password {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Password> for String {
        fn from(value: Password) -> Self {
            value.0
        }
    }

    impl From<&Password> for Password {
        fn from(value: &Password) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for Password {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if value.len() > 512usize {
                return Err("longer than 512 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for Password {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for Password {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for Password {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Password {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Client view of a [`PhysicalDisk`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`PhysicalDisk`]",
    ///  "type": "object",
    ///  "required": [
    ///    "disk_type",
    ///    "id",
    ///    "model",
    ///    "serial",
    ///    "time_created",
    ///    "time_modified",
    ///    "vendor"
    ///  ],
    ///  "properties": {
    ///    "disk_type": {
    ///      "$ref": "#/components/schemas/PhysicalDiskType"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "model": {
    ///      "type": "string"
    ///    },
    ///    "serial": {
    ///      "type": "string"
    ///    },
    ///    "sled_id": {
    ///      "description": "The sled to which this disk is attached, if any.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "uuid"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "vendor": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PhysicalDisk {
        pub disk_type: PhysicalDiskType,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        pub model: String,
        pub serial: String,
        ///The sled to which this disk is attached, if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sled_id: Option<uuid::Uuid>,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        pub vendor: String,
    }

    impl From<&PhysicalDisk> for PhysicalDisk {
        fn from(value: &PhysicalDisk) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PhysicalDisk"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PhysicalDiskResultsPage {
        ///list of items on this page of results
        pub items: Vec<PhysicalDisk>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&PhysicalDiskResultsPage> for PhysicalDiskResultsPage {
        fn from(value: &PhysicalDiskResultsPage) -> Self {
            value.clone()
        }
    }

    ///PhysicalDiskType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "internal",
    ///    "external"
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
    pub enum PhysicalDiskType {
        #[serde(rename = "internal")]
        Internal,
        #[serde(rename = "external")]
        External,
    }

    impl From<&PhysicalDiskType> for PhysicalDiskType {
        fn from(value: &PhysicalDiskType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for PhysicalDiskType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Internal => write!(f, "internal"),
                Self::External => write!(f, "external"),
            }
        }
    }

    impl std::str::FromStr for PhysicalDiskType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "internal" => Ok(Self::Internal),
                "external" => Ok(Self::External),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for PhysicalDiskType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for PhysicalDiskType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for PhysicalDiskType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Client view of a [`Project`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Project`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "name",
    ///    "organization_id",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "organization_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Project {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        pub organization_id: uuid::Uuid,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Project> for Project {
        fn from(value: &Project) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a
    /// [`Project`](crate::external_api::views::Project)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a
    /// [`Project`](crate::external_api::views::Project)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectCreate {
        pub description: String,
        pub name: Name,
    }

    impl From<&ProjectCreate> for ProjectCreate {
        fn from(value: &ProjectCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Project"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectResultsPage {
        ///list of items on this page of results
        pub items: Vec<Project>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&ProjectResultsPage> for ProjectResultsPage {
        fn from(value: &ProjectResultsPage) -> Self {
            value.clone()
        }
    }

    ///ProjectRole
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "admin",
    ///    "collaborator",
    ///    "viewer"
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
    pub enum ProjectRole {
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "collaborator")]
        Collaborator,
        #[serde(rename = "viewer")]
        Viewer,
    }

    impl From<&ProjectRole> for ProjectRole {
        fn from(value: &ProjectRole) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ProjectRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Admin => write!(f, "admin"),
                Self::Collaborator => write!(f, "collaborator"),
                Self::Viewer => write!(f, "viewer"),
            }
        }
    }

    impl std::str::FromStr for ProjectRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "admin" => Ok(Self::Admin),
                "collaborator" => Ok(Self::Collaborator),
                "viewer" => Ok(Self::Viewer),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ProjectRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ProjectRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ProjectRole {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Client view of a [`Policy`], which describes how this resource may be
    /// accessed
    ///
    ///Note that the Policy only describes access granted explicitly for this
    /// resource.  The policies of parent resources can also cause a user to
    /// have access to this resource.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Policy`], which describes how this
    /// resource may be accessed\n\nNote that the Policy only describes access
    /// granted explicitly for this resource.  The policies of parent resources
    /// can also cause a user to have access to this resource.",
    ///  "type": "object",
    ///  "required": [
    ///    "role_assignments"
    ///  ],
    ///  "properties": {
    ///    "role_assignments": {
    ///      "description": "Roles directly assigned on this resource",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProjectRoleRoleAssignment"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectRolePolicy {
        ///Roles directly assigned on this resource
        pub role_assignments: Vec<ProjectRoleRoleAssignment>,
    }

    impl From<&ProjectRolePolicy> for ProjectRolePolicy {
        fn from(value: &ProjectRolePolicy) -> Self {
            value.clone()
        }
    }

    ///Describes the assignment of a particular role on a particular resource
    /// to a particular identity (user, group, etc.)
    ///
    ///The resource is not part of this structure.  Rather, [`RoleAssignment`]s
    /// are put into a [`Policy`] and that Policy is applied to a particular
    /// resource.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes the assignment of a particular role on a
    /// particular resource to a particular identity (user, group, etc.)\n\nThe
    /// resource is not part of this structure.  Rather, [`RoleAssignment`]s are
    /// put into a [`Policy`] and that Policy is applied to a particular
    /// resource.",
    ///  "type": "object",
    ///  "required": [
    ///    "identity_id",
    ///    "identity_type",
    ///    "role_name"
    ///  ],
    ///  "properties": {
    ///    "identity_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "identity_type": {
    ///      "$ref": "#/components/schemas/IdentityType"
    ///    },
    ///    "role_name": {
    ///      "$ref": "#/components/schemas/ProjectRole"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectRoleRoleAssignment {
        pub identity_id: uuid::Uuid,
        pub identity_type: IdentityType,
        pub role_name: ProjectRole,
    }

    impl From<&ProjectRoleRoleAssignment> for ProjectRoleRoleAssignment {
        fn from(value: &ProjectRoleRoleAssignment) -> Self {
            value.clone()
        }
    }

    ///Updateable properties of a
    /// [`Project`](crate::external_api::views::Project)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Updateable properties of a
    /// [`Project`](crate::external_api::views::Project)",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
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
    pub struct ProjectUpdate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
    }

    impl From<&ProjectUpdate> for ProjectUpdate {
        fn from(value: &ProjectUpdate) -> Self {
            value.clone()
        }
    }

    ///Client view of an [`Rack`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of an [`Rack`]",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Rack {
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Rack> for Rack {
        fn from(value: &Rack) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Rack"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RackResultsPage {
        ///list of items on this page of results
        pub items: Vec<Rack>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&RackResultsPage> for RackResultsPage {
        fn from(value: &RackResultsPage) -> Self {
            value.clone()
        }
    }

    ///Client view of a [`Role`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Role`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/RoleName"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Role {
        pub description: String,
        pub name: RoleName,
    }

    impl From<&Role> for Role {
        fn from(value: &Role) -> Self {
            value.clone()
        }
    }

    ///Role names consist of two string components separated by dot (".").
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "A name for a built-in role",
    ///  "description": "Role names consist of two string components separated
    /// by dot (\".\").",
    ///  "type": "string",
    ///  "maxLength": 63,
    ///  "pattern": "[a-z-]+\\.[a-z-]+"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct RoleName(String);
    impl ::std::ops::Deref for RoleName {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<RoleName> for String {
        fn from(value: RoleName) -> Self {
            value.0
        }
    }

    impl From<&RoleName> for RoleName {
        fn from(value: &RoleName) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for RoleName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if value.len() > 63usize {
                return Err("longer than 63 characters".into());
            }
            if regress::Regex::new("[a-z-]+\\.[a-z-]+")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"[a-z-]+\\.[a-z-]+\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for RoleName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for RoleName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for RoleName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for RoleName {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Role"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RoleResultsPage {
        ///list of items on this page of results
        pub items: Vec<Role>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&RoleResultsPage> for RoleResultsPage {
        fn from(value: &RoleResultsPage) -> Self {
            value.clone()
        }
    }

    ///A `RouteDestination` is used to match traffic with a routing rule, on
    /// the destination of that traffic.
    ///
    ///When traffic is to be sent to a destination that is within a given
    /// `RouteDestination`, the corresponding [`RouterRoute`] applies, and
    /// traffic will be forward to the [`RouteTarget`] for that rule.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A `RouteDestination` is used to match traffic with a
    /// routing rule, on the destination of that traffic.\n\nWhen traffic is to
    /// be sent to a destination that is within a given `RouteDestination`, the
    /// corresponding [`RouterRoute`] applies, and traffic will be forward to
    /// the [`RouteTarget`] for that rule.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Route applies to traffic destined for a specific IP
    /// address",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ip"
    ///          ]
    ///        },
    ///        "value": {
    ///          "type": "string",
    ///          "format": "ip"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Route applies to traffic destined for a specific IP
    /// subnet",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ip_net"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/IpNet"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Route applies to traffic destined for the given
    /// VPC.",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "vpc"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Route applies to traffic",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "subnet"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum RouteDestination {
        ///Route applies to traffic destined for a specific IP address
        #[serde(rename = "ip")]
        Ip(std::net::IpAddr),
        ///Route applies to traffic destined for a specific IP subnet
        #[serde(rename = "ip_net")]
        IpNet(IpNet),
        ///Route applies to traffic destined for the given VPC.
        #[serde(rename = "vpc")]
        Vpc(Name),
        ///Route applies to traffic
        #[serde(rename = "subnet")]
        Subnet(Name),
    }

    impl From<&RouteDestination> for RouteDestination {
        fn from(value: &RouteDestination) -> Self {
            value.clone()
        }
    }

    impl From<std::net::IpAddr> for RouteDestination {
        fn from(value: std::net::IpAddr) -> Self {
            Self::Ip(value)
        }
    }

    impl From<IpNet> for RouteDestination {
        fn from(value: IpNet) -> Self {
            Self::IpNet(value)
        }
    }

    ///A `RouteTarget` describes the possible locations that traffic matching a
    /// route destination can be sent.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A `RouteTarget` describes the possible locations that
    /// traffic matching a route destination can be sent.",
    ///  "oneOf": [
    ///    {
    ///      "description": "Forward traffic to a particular IP address.",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ip"
    ///          ]
    ///        },
    ///        "value": {
    ///          "type": "string",
    ///          "format": "ip"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Forward traffic to a VPC",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "vpc"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Forward traffic to a VPC Subnet",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "subnet"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Forward traffic to a specific instance",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "instance"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Forward traffic to an internet gateway",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "internet_gateway"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum RouteTarget {
        ///Forward traffic to a particular IP address.
        #[serde(rename = "ip")]
        Ip(std::net::IpAddr),
        ///Forward traffic to a VPC
        #[serde(rename = "vpc")]
        Vpc(Name),
        ///Forward traffic to a VPC Subnet
        #[serde(rename = "subnet")]
        Subnet(Name),
        ///Forward traffic to a specific instance
        #[serde(rename = "instance")]
        Instance(Name),
        ///Forward traffic to an internet gateway
        #[serde(rename = "internet_gateway")]
        InternetGateway(Name),
    }

    impl From<&RouteTarget> for RouteTarget {
        fn from(value: &RouteTarget) -> Self {
            value.clone()
        }
    }

    impl From<std::net::IpAddr> for RouteTarget {
        fn from(value: std::net::IpAddr) -> Self {
            Self::Ip(value)
        }
    }

    ///A route defines a rule that governs where traffic should be sent based
    /// on its destination.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A route defines a rule that governs where traffic
    /// should be sent based on its destination.",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "destination",
    ///    "id",
    ///    "kind",
    ///    "name",
    ///    "target",
    ///    "time_created",
    ///    "time_modified",
    ///    "vpc_router_id"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "destination": {
    ///      "$ref": "#/components/schemas/RouteDestination"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "kind": {
    ///      "description": "Describes the kind of router. Set at creation.
    /// `read-only`",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/RouterRouteKind"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "target": {
    ///      "$ref": "#/components/schemas/RouteTarget"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "vpc_router_id": {
    ///      "description": "The VPC Router to which the route belongs.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RouterRoute {
        ///human-readable free-form text about a resource
        pub description: String,
        pub destination: RouteDestination,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///Describes the kind of router. Set at creation. `read-only`
        pub kind: RouterRouteKind,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        pub target: RouteTarget,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        ///The VPC Router to which the route belongs.
        pub vpc_router_id: uuid::Uuid,
    }

    impl From<&RouterRoute> for RouterRoute {
        fn from(value: &RouterRoute) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a [`RouterRoute`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a [`RouterRoute`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "destination",
    ///    "name",
    ///    "target"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "destination": {
    ///      "$ref": "#/components/schemas/RouteDestination"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "target": {
    ///      "$ref": "#/components/schemas/RouteTarget"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RouterRouteCreateParams {
        pub description: String,
        pub destination: RouteDestination,
        pub name: Name,
        pub target: RouteTarget,
    }

    impl From<&RouterRouteCreateParams> for RouterRouteCreateParams {
        fn from(value: &RouterRouteCreateParams) -> Self {
            value.clone()
        }
    }

    ///The classification of a [`RouterRoute`] as defined by the system. The
    /// kind determines certain attributes such as if the route is modifiable
    /// and describes how or where the route was created.
    ///
    ///See [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The classification of a [`RouterRoute`] as defined by the system. The kind determines certain attributes such as if the route is modifiable and describes how or where the route was created.\n\nSee [RFD-21](https://rfd.shared.oxide.computer/rfd/0021#concept-router) for more context",
    ///  "oneOf": [
    ///    {
    ///      "description": "Determines the default destination of traffic, such
    /// as whether it goes to the internet or not.\n\n`Destination: An Internet
    /// Gateway` `Modifiable: true`",
    ///      "type": "string",
    ///      "enum": [
    ///        "default"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Automatically added for each VPC Subnet in the
    /// VPC\n\n`Destination: A VPC Subnet` `Modifiable: false`",
    ///      "type": "string",
    ///      "enum": [
    ///        "vpc_subnet"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Automatically added when VPC peering is
    /// established\n\n`Destination: A different VPC` `Modifiable: false`",
    ///      "type": "string",
    ///      "enum": [
    ///        "vpc_peering"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Created by a user See
    /// [`RouteTarget`]\n\n`Destination: User defined` `Modifiable: true`",
    ///      "type": "string",
    ///      "enum": [
    ///        "custom"
    ///      ]
    ///    }
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
    pub enum RouterRouteKind {
        ///Determines the default destination of traffic, such as whether it
        /// goes to the internet or not.
        ///
        ///`Destination: An Internet Gateway` `Modifiable: true`
        #[serde(rename = "default")]
        Default,
        ///Automatically added for each VPC Subnet in the VPC
        ///
        ///`Destination: A VPC Subnet` `Modifiable: false`
        #[serde(rename = "vpc_subnet")]
        VpcSubnet,
        ///Automatically added when VPC peering is established
        ///
        ///`Destination: A different VPC` `Modifiable: false`
        #[serde(rename = "vpc_peering")]
        VpcPeering,
        ///Created by a user See [`RouteTarget`]
        ///
        ///`Destination: User defined` `Modifiable: true`
        #[serde(rename = "custom")]
        Custom,
    }

    impl From<&RouterRouteKind> for RouterRouteKind {
        fn from(value: &RouterRouteKind) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for RouterRouteKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Default => write!(f, "default"),
                Self::VpcSubnet => write!(f, "vpc_subnet"),
                Self::VpcPeering => write!(f, "vpc_peering"),
                Self::Custom => write!(f, "custom"),
            }
        }
    }

    impl std::str::FromStr for RouterRouteKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "default" => Ok(Self::Default),
                "vpc_subnet" => Ok(Self::VpcSubnet),
                "vpc_peering" => Ok(Self::VpcPeering),
                "custom" => Ok(Self::Custom),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for RouterRouteKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for RouterRouteKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for RouterRouteKind {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RouterRoute"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RouterRouteResultsPage {
        ///list of items on this page of results
        pub items: Vec<RouterRoute>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&RouterRouteResultsPage> for RouterRouteResultsPage {
        fn from(value: &RouterRouteResultsPage) -> Self {
            value.clone()
        }
    }

    ///Updateable properties of a [`RouterRoute`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Updateable properties of a [`RouterRoute`]",
    ///  "type": "object",
    ///  "required": [
    ///    "destination",
    ///    "target"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "destination": {
    ///      "$ref": "#/components/schemas/RouteDestination"
    ///    },
    ///    "name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "target": {
    ///      "$ref": "#/components/schemas/RouteTarget"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RouterRouteUpdateParams {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        pub destination: RouteDestination,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
        pub target: RouteTarget,
    }

    impl From<&RouterRouteUpdateParams> for RouterRouteUpdateParams {
        fn from(value: &RouterRouteUpdateParams) -> Self {
            value.clone()
        }
    }

    ///Saga
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/SagaState"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Saga {
        pub id: uuid::Uuid,
        pub state: SagaState,
    }

    impl From<&Saga> for Saga {
        fn from(value: &Saga) -> Self {
            value.clone()
        }
    }

    ///SagaErrorInfo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error",
    ///        "source_error"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "type": "string",
    ///          "enum": [
    ///            "action_failed"
    ///          ]
    ///        },
    ///        "source_error": {}
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error",
    ///        "message"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "type": "string",
    ///          "enum": [
    ///            "deserialize_failed"
    ///          ]
    ///        },
    ///        "message": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "type": "string",
    ///          "enum": [
    ///            "injected_error"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error",
    ///        "message"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "type": "string",
    ///          "enum": [
    ///            "serialize_failed"
    ///          ]
    ///        },
    ///        "message": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error",
    ///        "message"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "type": "string",
    ///          "enum": [
    ///            "subsaga_create_failed"
    ///          ]
    ///        },
    ///        "message": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "error")]
    pub enum SagaErrorInfo {
        #[serde(rename = "action_failed")]
        ActionFailed { source_error: ::serde_json::Value },
        #[serde(rename = "deserialize_failed")]
        DeserializeFailed { message: String },
        #[serde(rename = "injected_error")]
        InjectedError,
        #[serde(rename = "serialize_failed")]
        SerializeFailed { message: String },
        #[serde(rename = "subsaga_create_failed")]
        SubsagaCreateFailed { message: String },
    }

    impl From<&SagaErrorInfo> for SagaErrorInfo {
        fn from(value: &SagaErrorInfo) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Saga"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SagaResultsPage {
        ///list of items on this page of results
        pub items: Vec<Saga>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&SagaResultsPage> for SagaResultsPage {
        fn from(value: &SagaResultsPage) -> Self {
            value.clone()
        }
    }

    ///SagaState
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "running"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "succeeded"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error_info",
    ///        "error_node_name",
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "error_info": {
    ///          "$ref": "#/components/schemas/SagaErrorInfo"
    ///        },
    ///        "error_node_name": {
    ///          "$ref": "#/components/schemas/NodeName"
    ///        },
    ///        "state": {
    ///          "type": "string",
    ///          "enum": [
    ///            "failed"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "state")]
    pub enum SagaState {
        #[serde(rename = "running")]
        Running,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed {
            error_info: SagaErrorInfo,
            error_node_name: NodeName,
        },
    }

    impl From<&SagaState> for SagaState {
        fn from(value: &SagaState) -> Self {
            value.clone()
        }
    }

    ///Identity-related metadata that's included in nearly all public API
    /// objects
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Identity-related metadata that's included in nearly all
    /// public API objects",
    ///  "type": "object",
    ///  "required": [
    ///    "acs_url",
    ///    "description",
    ///    "id",
    ///    "idp_entity_id",
    ///    "name",
    ///    "slo_url",
    ///    "sp_client_id",
    ///    "technical_contact_email",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "acs_url": {
    ///      "description": "service provider endpoint where the response will
    /// be sent",
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "idp_entity_id": {
    ///      "description": "idp's entity id",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "public_cert": {
    ///      "description": "optional request signing public certificate (base64
    /// encoded der file)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "slo_url": {
    ///      "description": "service provider endpoint where the idp should send
    /// log out requests",
    ///      "type": "string"
    ///    },
    ///    "sp_client_id": {
    ///      "description": "sp's client id",
    ///      "type": "string"
    ///    },
    ///    "technical_contact_email": {
    ///      "description": "customer's technical contact for saml
    /// configuration",
    ///      "type": "string"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SamlIdentityProvider {
        ///service provider endpoint where the response will be sent
        pub acs_url: String,
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///idp's entity id
        pub idp_entity_id: String,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///optional request signing public certificate (base64 encoded der
        /// file)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub public_cert: Option<String>,
        ///service provider endpoint where the idp should send log out requests
        pub slo_url: String,
        ///sp's client id
        pub sp_client_id: String,
        ///customer's technical contact for saml configuration
        pub technical_contact_email: String,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&SamlIdentityProvider> for SamlIdentityProvider {
        fn from(value: &SamlIdentityProvider) -> Self {
            value.clone()
        }
    }

    ///Create-time identity-related parameters
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time identity-related parameters",
    ///  "type": "object",
    ///  "required": [
    ///    "acs_url",
    ///    "description",
    ///    "idp_entity_id",
    ///    "idp_metadata_source",
    ///    "name",
    ///    "slo_url",
    ///    "sp_client_id",
    ///    "technical_contact_email"
    ///  ],
    ///  "properties": {
    ///    "acs_url": {
    ///      "description": "service provider endpoint where the response will
    /// be sent",
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "group_attribute_name": {
    ///      "description": "If set, SAML attributes with this name will be
    /// considered to denote a user's group membership, where the attribute
    /// value(s) should be a comma-separated list of group names.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "idp_entity_id": {
    ///      "description": "idp's entity id",
    ///      "type": "string"
    ///    },
    ///    "idp_metadata_source": {
    ///      "description": "the source of an identity provider metadata
    /// descriptor",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IdpMetadataSource"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "signing_keypair": {
    ///      "description": "optional request signing key pair",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/DerEncodedKeyPair"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "slo_url": {
    ///      "description": "service provider endpoint where the idp should send
    /// log out requests",
    ///      "type": "string"
    ///    },
    ///    "sp_client_id": {
    ///      "description": "sp's client id",
    ///      "type": "string"
    ///    },
    ///    "technical_contact_email": {
    ///      "description": "customer's technical contact for saml
    /// configuration",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SamlIdentityProviderCreate {
        ///service provider endpoint where the response will be sent
        pub acs_url: String,
        pub description: String,
        ///If set, SAML attributes with this name will be considered to denote
        /// a user's group membership, where the attribute value(s) should be a
        /// comma-separated list of group names.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub group_attribute_name: Option<String>,
        ///idp's entity id
        pub idp_entity_id: String,
        ///the source of an identity provider metadata descriptor
        pub idp_metadata_source: IdpMetadataSource,
        pub name: Name,
        ///optional request signing key pair
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub signing_keypair: Option<DerEncodedKeyPair>,
        ///service provider endpoint where the idp should send log out requests
        pub slo_url: String,
        ///sp's client id
        pub sp_client_id: String,
        ///customer's technical contact for saml configuration
        pub technical_contact_email: String,
    }

    impl From<&SamlIdentityProviderCreate> for SamlIdentityProviderCreate {
        fn from(value: &SamlIdentityProviderCreate) -> Self {
            value.clone()
        }
    }

    ///SemverVersion
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^\\d+\\.\\d+\\.\\d+([\\-\\+].+)?$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct SemverVersion(String);
    impl ::std::ops::Deref for SemverVersion {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<SemverVersion> for String {
        fn from(value: SemverVersion) -> Self {
            value.0
        }
    }

    impl From<&SemverVersion> for SemverVersion {
        fn from(value: &SemverVersion) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for SemverVersion {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^\\d+\\.\\d+\\.\\d+([\\-\\+].+)?$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^\\d+\\.\\d+\\.\\d+([\\-\\+].+)?$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for SemverVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for SemverVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for SemverVersion {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for SemverVersion {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///The service intended to use this certificate.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The service intended to use this certificate.",
    ///  "oneOf": [
    ///    {
    ///      "description": "This certificate is intended for access to the
    /// external API.",
    ///      "type": "string",
    ///      "enum": [
    ///        "external_api"
    ///      ]
    ///    }
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
    pub enum ServiceUsingCertificate {
        ///This certificate is intended for access to the external API.
        #[serde(rename = "external_api")]
        ExternalApi,
    }

    impl From<&ServiceUsingCertificate> for ServiceUsingCertificate {
        fn from(value: &ServiceUsingCertificate) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ServiceUsingCertificate {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ExternalApi => write!(f, "external_api"),
            }
        }
    }

    impl std::str::FromStr for ServiceUsingCertificate {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "external_api" => Ok(Self::ExternalApi),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ServiceUsingCertificate {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ServiceUsingCertificate {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ServiceUsingCertificate {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Client view of a ['Silo']
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a ['Silo']",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "discoverable",
    ///    "id",
    ///    "identity_mode",
    ///    "name",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "discoverable": {
    ///      "description": "A silo where discoverable is false can be retrieved only by its id - it will not be part of the \"list all silos\" output.",
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "identity_mode": {
    ///      "description": "How users and groups are managed in this Silo",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/SiloIdentityMode"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Silo {
        ///human-readable free-form text about a resource
        pub description: String,
        ///A silo where discoverable is false can be retrieved only by its id -
        /// it will not be part of the "list all silos" output.
        pub discoverable: bool,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///How users and groups are managed in this Silo
        pub identity_mode: SiloIdentityMode,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Silo> for Silo {
        fn from(value: &Silo) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a [`Silo`](crate::external_api::views::Silo)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a
    /// [`Silo`](crate::external_api::views::Silo)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "discoverable",
    ///    "identity_mode",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "admin_group_name": {
    ///      "description": "If set, this group will be created during Silo
    /// creation and granted the \"Silo Admin\" role. Identity providers can
    /// assert that users belong to this group and those users can log in and
    /// further initialize the Silo.\n\nNote that if configuring a SAML based
    /// identity provider, group_attribute_name must be set for users to be
    /// considered part of a group. See [`SamlIdentityProviderCreate`] for more
    /// information.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "discoverable": {
    ///      "type": "boolean"
    ///    },
    ///    "identity_mode": {
    ///      "$ref": "#/components/schemas/SiloIdentityMode"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SiloCreate {
        ///If set, this group will be created during Silo creation and granted
        /// the "Silo Admin" role. Identity providers can assert that users
        /// belong to this group and those users can log in and further
        /// initialize the Silo.
        ///
        ///Note that if configuring a SAML based identity provider,
        /// group_attribute_name must be set for users to be considered part of
        /// a group. See [`SamlIdentityProviderCreate`] for more information.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub admin_group_name: Option<String>,
        pub description: String,
        pub discoverable: bool,
        pub identity_mode: SiloIdentityMode,
        pub name: Name,
    }

    impl From<&SiloCreate> for SiloCreate {
        fn from(value: &SiloCreate) -> Self {
            value.clone()
        }
    }

    ///Describes how identities are managed and users are authenticated in this
    /// Silo
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes how identities are managed and users are
    /// authenticated in this Silo",
    ///  "oneOf": [
    ///    {
    ///      "description": "Users are authenticated with SAML using an external
    /// authentication provider.  The system updates information about users and
    /// groups only during successful authentication (i.e,. \"JIT provisioning\"
    /// of users and groups).",
    ///      "type": "string",
    ///      "enum": [
    ///        "saml_jit"
    ///      ]
    ///    },
    ///    {
    ///      "description": "The system is the source of truth about users.
    /// There is no linkage to an external authentication provider or identity
    /// provider.",
    ///      "type": "string",
    ///      "enum": [
    ///        "local_only"
    ///      ]
    ///    }
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
    pub enum SiloIdentityMode {
        ///Users are authenticated with SAML using an external authentication
        /// provider.  The system updates information about users and groups
        /// only during successful authentication (i.e,. "JIT provisioning" of
        /// users and groups).
        #[serde(rename = "saml_jit")]
        SamlJit,
        ///The system is the source of truth about users.  There is no linkage
        /// to an external authentication provider or identity provider.
        #[serde(rename = "local_only")]
        LocalOnly,
    }

    impl From<&SiloIdentityMode> for SiloIdentityMode {
        fn from(value: &SiloIdentityMode) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SiloIdentityMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SamlJit => write!(f, "saml_jit"),
                Self::LocalOnly => write!(f, "local_only"),
            }
        }
    }

    impl std::str::FromStr for SiloIdentityMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "saml_jit" => Ok(Self::SamlJit),
                "local_only" => Ok(Self::LocalOnly),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for SiloIdentityMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SiloIdentityMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SiloIdentityMode {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Silo"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SiloResultsPage {
        ///list of items on this page of results
        pub items: Vec<Silo>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&SiloResultsPage> for SiloResultsPage {
        fn from(value: &SiloResultsPage) -> Self {
            value.clone()
        }
    }

    ///SiloRole
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "admin",
    ///    "collaborator",
    ///    "viewer"
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
    pub enum SiloRole {
        #[serde(rename = "admin")]
        Admin,
        #[serde(rename = "collaborator")]
        Collaborator,
        #[serde(rename = "viewer")]
        Viewer,
    }

    impl From<&SiloRole> for SiloRole {
        fn from(value: &SiloRole) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SiloRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Admin => write!(f, "admin"),
                Self::Collaborator => write!(f, "collaborator"),
                Self::Viewer => write!(f, "viewer"),
            }
        }
    }

    impl std::str::FromStr for SiloRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "admin" => Ok(Self::Admin),
                "collaborator" => Ok(Self::Collaborator),
                "viewer" => Ok(Self::Viewer),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for SiloRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SiloRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SiloRole {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Client view of a [`Policy`], which describes how this resource may be
    /// accessed
    ///
    ///Note that the Policy only describes access granted explicitly for this
    /// resource.  The policies of parent resources can also cause a user to
    /// have access to this resource.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Policy`], which describes how this
    /// resource may be accessed\n\nNote that the Policy only describes access
    /// granted explicitly for this resource.  The policies of parent resources
    /// can also cause a user to have access to this resource.",
    ///  "type": "object",
    ///  "required": [
    ///    "role_assignments"
    ///  ],
    ///  "properties": {
    ///    "role_assignments": {
    ///      "description": "Roles directly assigned on this resource",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SiloRoleRoleAssignment"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SiloRolePolicy {
        ///Roles directly assigned on this resource
        pub role_assignments: Vec<SiloRoleRoleAssignment>,
    }

    impl From<&SiloRolePolicy> for SiloRolePolicy {
        fn from(value: &SiloRolePolicy) -> Self {
            value.clone()
        }
    }

    ///Describes the assignment of a particular role on a particular resource
    /// to a particular identity (user, group, etc.)
    ///
    ///The resource is not part of this structure.  Rather, [`RoleAssignment`]s
    /// are put into a [`Policy`] and that Policy is applied to a particular
    /// resource.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Describes the assignment of a particular role on a
    /// particular resource to a particular identity (user, group, etc.)\n\nThe
    /// resource is not part of this structure.  Rather, [`RoleAssignment`]s are
    /// put into a [`Policy`] and that Policy is applied to a particular
    /// resource.",
    ///  "type": "object",
    ///  "required": [
    ///    "identity_id",
    ///    "identity_type",
    ///    "role_name"
    ///  ],
    ///  "properties": {
    ///    "identity_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "identity_type": {
    ///      "$ref": "#/components/schemas/IdentityType"
    ///    },
    ///    "role_name": {
    ///      "$ref": "#/components/schemas/SiloRole"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SiloRoleRoleAssignment {
        pub identity_id: uuid::Uuid,
        pub identity_type: IdentityType,
        pub role_name: SiloRole,
    }

    impl From<&SiloRoleRoleAssignment> for SiloRoleRoleAssignment {
        fn from(value: &SiloRoleRoleAssignment) -> Self {
            value.clone()
        }
    }

    ///Client view of a [`Sled`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Sled`]",
    ///  "type": "object",
    ///  "required": [
    ///    "baseboard",
    ///    "id",
    ///    "service_address",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "baseboard": {
    ///      "$ref": "#/components/schemas/Baseboard"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "service_address": {
    ///      "type": "string"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Sled {
        pub baseboard: Baseboard,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        pub service_address: String,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Sled> for Sled {
        fn from(value: &Sled) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Sled"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SledResultsPage {
        ///list of items on this page of results
        pub items: Vec<Sled>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&SledResultsPage> for SledResultsPage {
        fn from(value: &SledResultsPage) -> Self {
            value.clone()
        }
    }

    ///Client view of a Snapshot
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a Snapshot",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "disk_id",
    ///    "id",
    ///    "name",
    ///    "project_id",
    ///    "size",
    ///    "state",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "disk_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "project_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "size": {
    ///      "$ref": "#/components/schemas/ByteCount"
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/SnapshotState"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Snapshot {
        ///human-readable free-form text about a resource
        pub description: String,
        pub disk_id: uuid::Uuid,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        pub project_id: uuid::Uuid,
        pub size: ByteCount,
        pub state: SnapshotState,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Snapshot> for Snapshot {
        fn from(value: &Snapshot) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a
    /// [`Snapshot`](crate::external_api::views::Snapshot)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a
    /// [`Snapshot`](crate::external_api::views::Snapshot)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "disk",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "disk": {
    ///      "description": "The name of the disk to be snapshotted",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SnapshotCreate {
        pub description: String,
        ///The name of the disk to be snapshotted
        pub disk: Name,
        pub name: Name,
    }

    impl From<&SnapshotCreate> for SnapshotCreate {
        fn from(value: &SnapshotCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Snapshot"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SnapshotResultsPage {
        ///list of items on this page of results
        pub items: Vec<Snapshot>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&SnapshotResultsPage> for SnapshotResultsPage {
        fn from(value: &SnapshotResultsPage) -> Self {
            value.clone()
        }
    }

    ///SnapshotState
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "creating",
    ///    "ready",
    ///    "faulted",
    ///    "destroyed"
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
    pub enum SnapshotState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "ready")]
        Ready,
        #[serde(rename = "faulted")]
        Faulted,
        #[serde(rename = "destroyed")]
        Destroyed,
    }

    impl From<&SnapshotState> for SnapshotState {
        fn from(value: &SnapshotState) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SnapshotState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Creating => write!(f, "creating"),
                Self::Ready => write!(f, "ready"),
                Self::Faulted => write!(f, "faulted"),
                Self::Destroyed => write!(f, "destroyed"),
            }
        }
    }

    impl std::str::FromStr for SnapshotState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "creating" => Ok(Self::Creating),
                "ready" => Ok(Self::Ready),
                "faulted" => Ok(Self::Faulted),
                "destroyed" => Ok(Self::Destroyed),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for SnapshotState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SnapshotState {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SnapshotState {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///SpoofLoginBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "username": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SpoofLoginBody {
        pub username: String,
    }

    impl From<&SpoofLoginBody> for SpoofLoginBody {
        fn from(value: &SpoofLoginBody) -> Self {
            value.clone()
        }
    }

    ///Client view of a [`SshKey`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`SshKey`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "name",
    ///    "public_key",
    ///    "silo_user_id",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "public_key": {
    ///      "description": "SSH public key, e.g., `\"ssh-ed25519
    /// AAAAC3NzaC...\"`",
    ///      "type": "string"
    ///    },
    ///    "silo_user_id": {
    ///      "description": "The user to whom this key belongs",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SshKey {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///SSH public key, e.g., `"ssh-ed25519 AAAAC3NzaC..."`
        pub public_key: String,
        ///The user to whom this key belongs
        pub silo_user_id: uuid::Uuid,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&SshKey> for SshKey {
        fn from(value: &SshKey) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for an
    /// [`SshKey`](crate::external_api::views::SshKey)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for an
    /// [`SshKey`](crate::external_api::views::SshKey)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "name",
    ///    "public_key"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "public_key": {
    ///      "description": "SSH public key, e.g., `\"ssh-ed25519
    /// AAAAC3NzaC...\"`",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SshKeyCreate {
        pub description: String,
        pub name: Name,
        ///SSH public key, e.g., `"ssh-ed25519 AAAAC3NzaC..."`
        pub public_key: String,
    }

    impl From<&SshKeyCreate> for SshKeyCreate {
        fn from(value: &SshKeyCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SshKey"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SshKeyResultsPage {
        ///list of items on this page of results
        pub items: Vec<SshKey>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&SshKeyResultsPage> for SshKeyResultsPage {
        fn from(value: &SshKeyResultsPage) -> Self {
            value.clone()
        }
    }

    ///SystemMetricName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "virtual_disk_space_provisioned",
    ///    "cpus_provisioned",
    ///    "ram_provisioned"
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
    pub enum SystemMetricName {
        #[serde(rename = "virtual_disk_space_provisioned")]
        VirtualDiskSpaceProvisioned,
        #[serde(rename = "cpus_provisioned")]
        CpusProvisioned,
        #[serde(rename = "ram_provisioned")]
        RamProvisioned,
    }

    impl From<&SystemMetricName> for SystemMetricName {
        fn from(value: &SystemMetricName) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SystemMetricName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::VirtualDiskSpaceProvisioned => write!(f, "virtual_disk_space_provisioned"),
                Self::CpusProvisioned => write!(f, "cpus_provisioned"),
                Self::RamProvisioned => write!(f, "ram_provisioned"),
            }
        }
    }

    impl std::str::FromStr for SystemMetricName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "virtual_disk_space_provisioned" => Ok(Self::VirtualDiskSpaceProvisioned),
                "cpus_provisioned" => Ok(Self::CpusProvisioned),
                "ram_provisioned" => Ok(Self::RamProvisioned),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for SystemMetricName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SystemMetricName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SystemMetricName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Identity-related metadata that's included in "asset" public API objects
    /// (which generally have no name or description)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Identity-related metadata that's included in \"asset\"
    /// public API objects (which generally have no name or description)",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "time_created",
    ///    "time_modified",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "version": {
    ///      "$ref": "#/components/schemas/SemverVersion"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemUpdate {
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        pub version: SemverVersion,
    }

    impl From<&SystemUpdate> for SystemUpdate {
        fn from(value: &SystemUpdate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SystemUpdate"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemUpdateResultsPage {
        ///list of items on this page of results
        pub items: Vec<SystemUpdate>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&SystemUpdateResultsPage> for SystemUpdateResultsPage {
        fn from(value: &SystemUpdateResultsPage) -> Self {
            value.clone()
        }
    }

    ///SystemUpdateStart
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "version": {
    ///      "$ref": "#/components/schemas/SemverVersion"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemUpdateStart {
        pub version: SemverVersion,
    }

    impl From<&SystemUpdateStart> for SystemUpdateStart {
        fn from(value: &SystemUpdateStart) -> Self {
            value.clone()
        }
    }

    ///SystemVersion
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "status",
    ///    "version_range"
    ///  ],
    ///  "properties": {
    ///    "status": {
    ///      "$ref": "#/components/schemas/UpdateStatus"
    ///    },
    ///    "version_range": {
    ///      "$ref": "#/components/schemas/VersionRange"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SystemVersion {
        pub status: UpdateStatus,
        pub version_range: VersionRange,
    }

    impl From<&SystemVersion> for SystemVersion {
        fn from(value: &SystemVersion) -> Self {
            value.clone()
        }
    }

    ///Names are constructed by concatenating the target and metric names with
    /// ':'. Target and metric names must be lowercase alphanumeric characters
    /// with '_' separating words.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "The name of a timeseries",
    ///  "description": "Names are constructed by concatenating the target and
    /// metric names with ':'. Target and metric names must be lowercase
    /// alphanumeric characters with '_' separating words.",
    ///  "type": "string",
    ///  "pattern":
    /// "(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*):(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*)"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct TimeseriesName(String);
    impl ::std::ops::Deref for TimeseriesName {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<TimeseriesName> for String {
        fn from(value: TimeseriesName) -> Self {
            value.0
        }
    }

    impl From<&TimeseriesName> for TimeseriesName {
        fn from(value: &TimeseriesName) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for TimeseriesName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new(
                "(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*):(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*)",
            )
            .unwrap()
            .find(value)
            .is_none()
            {
                return Err("doesn't match pattern \
                            \"(([a-z]+[a-z0-9]*)(_([a-z0-9]+))*):(([a-z]+[a-z0-9]*\
                            )(_([a-z0-9]+))*)\""
                    .into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for TimeseriesName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TimeseriesName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TimeseriesName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for TimeseriesName {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///The schema for a timeseries.
    ///
    ///This includes the name of the timeseries, as well as the datum type of
    /// its metric and the schema for each field.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The schema for a timeseries.\n\nThis includes the name
    /// of the timeseries, as well as the datum type of its metric and the
    /// schema for each field.",
    ///  "type": "object",
    ///  "required": [
    ///    "created",
    ///    "datum_type",
    ///    "field_schema",
    ///    "timeseries_name"
    ///  ],
    ///  "properties": {
    ///    "created": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "datum_type": {
    ///      "$ref": "#/components/schemas/DatumType"
    ///    },
    ///    "field_schema": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FieldSchema"
    ///      }
    ///    },
    ///    "timeseries_name": {
    ///      "$ref": "#/components/schemas/TimeseriesName"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TimeseriesSchema {
        pub created: chrono::DateTime<chrono::offset::Utc>,
        pub datum_type: DatumType,
        pub field_schema: Vec<FieldSchema>,
        pub timeseries_name: TimeseriesName,
    }

    impl From<&TimeseriesSchema> for TimeseriesSchema {
        fn from(value: &TimeseriesSchema) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TimeseriesSchema"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TimeseriesSchemaResultsPage {
        ///list of items on this page of results
        pub items: Vec<TimeseriesSchema>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&TimeseriesSchemaResultsPage> for TimeseriesSchemaResultsPage {
        fn from(value: &TimeseriesSchemaResultsPage) -> Self {
            value.clone()
        }
    }

    ///Identity-related metadata that's included in "asset" public API objects
    /// (which generally have no name or description)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Identity-related metadata that's included in \"asset\"
    /// public API objects (which generally have no name or description)",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "status",
    ///    "time_created",
    ///    "time_modified",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/UpdateStatus"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "version": {
    ///      "$ref": "#/components/schemas/SemverVersion"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateDeployment {
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        pub status: UpdateStatus,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        pub version: SemverVersion,
    }

    impl From<&UpdateDeployment> for UpdateDeployment {
        fn from(value: &UpdateDeployment) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UpdateDeployment"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateDeploymentResultsPage {
        ///list of items on this page of results
        pub items: Vec<UpdateDeployment>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&UpdateDeploymentResultsPage> for UpdateDeploymentResultsPage {
        fn from(value: &UpdateDeploymentResultsPage) -> Self {
            value.clone()
        }
    }

    ///UpdateStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "status"
    ///      ],
    ///      "properties": {
    ///        "status": {
    ///          "type": "string",
    ///          "enum": [
    ///            "updating"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "status"
    ///      ],
    ///      "properties": {
    ///        "status": {
    ///          "type": "string",
    ///          "enum": [
    ///            "steady"
    ///          ]
    ///        }
    ///      }
    ///    }
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
    #[serde(tag = "status")]
    pub enum UpdateStatus {
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "steady")]
        Steady,
    }

    impl From<&UpdateStatus> for UpdateStatus {
        fn from(value: &UpdateStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for UpdateStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Updating => write!(f, "updating"),
                Self::Steady => write!(f, "steady"),
            }
        }
    }

    impl std::str::FromStr for UpdateStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "updating" => Ok(Self::Updating),
                "steady" => Ok(Self::Steady),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for UpdateStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for UpdateStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for UpdateStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Identity-related metadata that's included in "asset" public API objects
    /// (which generally have no name or description)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Identity-related metadata that's included in \"asset\"
    /// public API objects (which generally have no name or description)",
    ///  "type": "object",
    ///  "required": [
    ///    "component_type",
    ///    "device_id",
    ///    "id",
    ///    "status",
    ///    "system_version",
    ///    "time_created",
    ///    "time_modified",
    ///    "version"
    ///  ],
    ///  "properties": {
    ///    "component_type": {
    ///      "$ref": "#/components/schemas/UpdateableComponentType"
    ///    },
    ///    "device_id": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/UpdateStatus"
    ///    },
    ///    "system_version": {
    ///      "$ref": "#/components/schemas/SemverVersion"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "version": {
    ///      "$ref": "#/components/schemas/SemverVersion"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateableComponent {
        pub component_type: UpdateableComponentType,
        pub device_id: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        pub status: UpdateStatus,
        pub system_version: SemverVersion,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        pub version: SemverVersion,
    }

    impl From<&UpdateableComponent> for UpdateableComponent {
        fn from(value: &UpdateableComponent) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UpdateableComponent"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateableComponentResultsPage {
        ///list of items on this page of results
        pub items: Vec<UpdateableComponent>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&UpdateableComponentResultsPage> for UpdateableComponentResultsPage {
        fn from(value: &UpdateableComponentResultsPage) -> Self {
            value.clone()
        }
    }

    ///UpdateableComponentType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "bootloader_for_rot",
    ///    "bootloader_for_sp",
    ///    "bootloader_for_host_proc",
    ///    "hubris_for_psc_rot",
    ///    "hubris_for_psc_sp",
    ///    "hubris_for_sidecar_rot",
    ///    "hubris_for_sidecar_sp",
    ///    "hubris_for_gimlet_rot",
    ///    "hubris_for_gimlet_sp",
    ///    "helios_host_phase1",
    ///    "helios_host_phase2",
    ///    "host_omicron"
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
    pub enum UpdateableComponentType {
        #[serde(rename = "bootloader_for_rot")]
        BootloaderForRot,
        #[serde(rename = "bootloader_for_sp")]
        BootloaderForSp,
        #[serde(rename = "bootloader_for_host_proc")]
        BootloaderForHostProc,
        #[serde(rename = "hubris_for_psc_rot")]
        HubrisForPscRot,
        #[serde(rename = "hubris_for_psc_sp")]
        HubrisForPscSp,
        #[serde(rename = "hubris_for_sidecar_rot")]
        HubrisForSidecarRot,
        #[serde(rename = "hubris_for_sidecar_sp")]
        HubrisForSidecarSp,
        #[serde(rename = "hubris_for_gimlet_rot")]
        HubrisForGimletRot,
        #[serde(rename = "hubris_for_gimlet_sp")]
        HubrisForGimletSp,
        #[serde(rename = "helios_host_phase1")]
        HeliosHostPhase1,
        #[serde(rename = "helios_host_phase2")]
        HeliosHostPhase2,
        #[serde(rename = "host_omicron")]
        HostOmicron,
    }

    impl From<&UpdateableComponentType> for UpdateableComponentType {
        fn from(value: &UpdateableComponentType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for UpdateableComponentType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BootloaderForRot => write!(f, "bootloader_for_rot"),
                Self::BootloaderForSp => write!(f, "bootloader_for_sp"),
                Self::BootloaderForHostProc => write!(f, "bootloader_for_host_proc"),
                Self::HubrisForPscRot => write!(f, "hubris_for_psc_rot"),
                Self::HubrisForPscSp => write!(f, "hubris_for_psc_sp"),
                Self::HubrisForSidecarRot => write!(f, "hubris_for_sidecar_rot"),
                Self::HubrisForSidecarSp => write!(f, "hubris_for_sidecar_sp"),
                Self::HubrisForGimletRot => write!(f, "hubris_for_gimlet_rot"),
                Self::HubrisForGimletSp => write!(f, "hubris_for_gimlet_sp"),
                Self::HeliosHostPhase1 => write!(f, "helios_host_phase1"),
                Self::HeliosHostPhase2 => write!(f, "helios_host_phase2"),
                Self::HostOmicron => write!(f, "host_omicron"),
            }
        }
    }

    impl std::str::FromStr for UpdateableComponentType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "bootloader_for_rot" => Ok(Self::BootloaderForRot),
                "bootloader_for_sp" => Ok(Self::BootloaderForSp),
                "bootloader_for_host_proc" => Ok(Self::BootloaderForHostProc),
                "hubris_for_psc_rot" => Ok(Self::HubrisForPscRot),
                "hubris_for_psc_sp" => Ok(Self::HubrisForPscSp),
                "hubris_for_sidecar_rot" => Ok(Self::HubrisForSidecarRot),
                "hubris_for_sidecar_sp" => Ok(Self::HubrisForSidecarSp),
                "hubris_for_gimlet_rot" => Ok(Self::HubrisForGimletRot),
                "hubris_for_gimlet_sp" => Ok(Self::HubrisForGimletSp),
                "helios_host_phase1" => Ok(Self::HeliosHostPhase1),
                "helios_host_phase2" => Ok(Self::HeliosHostPhase2),
                "host_omicron" => Ok(Self::HostOmicron),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for UpdateableComponentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for UpdateableComponentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for UpdateableComponentType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Client view of a [`User`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`User`]",
    ///  "type": "object",
    ///  "required": [
    ///    "display_name",
    ///    "id",
    ///    "silo_id"
    ///  ],
    ///  "properties": {
    ///    "display_name": {
    ///      "description": "Human-readable name that can identify the user",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "silo_id": {
    ///      "description": "Uuid of the silo to which this user belongs",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct User {
        ///Human-readable name that can identify the user
        pub display_name: String,
        pub id: uuid::Uuid,
        ///Uuid of the silo to which this user belongs
        pub silo_id: uuid::Uuid,
    }

    impl From<&User> for User {
        fn from(value: &User) -> Self {
            value.clone()
        }
    }

    ///Client view of a [`UserBuiltin`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`UserBuiltin`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "name",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserBuiltin {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&UserBuiltin> for UserBuiltin {
        fn from(value: &UserBuiltin) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserBuiltin"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserBuiltinResultsPage {
        ///list of items on this page of results
        pub items: Vec<UserBuiltin>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&UserBuiltinResultsPage> for UserBuiltinResultsPage {
        fn from(value: &UserBuiltinResultsPage) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a [`User`](crate::external_api::views::User)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a
    /// [`User`](crate::external_api::views::User)",
    ///  "type": "object",
    ///  "required": [
    ///    "external_id",
    ///    "password"
    ///  ],
    ///  "properties": {
    ///    "external_id": {
    ///      "description": "username used to log in",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/UserId"
    ///        }
    ///      ]
    ///    },
    ///    "password": {
    ///      "description": "password used to log in",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/UserPassword"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserCreate {
        ///username used to log in
        pub external_id: UserId,
        ///password used to log in
        pub password: UserPassword,
    }

    impl From<&UserCreate> for UserCreate {
        fn from(value: &UserCreate) -> Self {
            value.clone()
        }
    }

    ///Names must begin with a lower case ASCII letter, be composed exclusively
    /// of lowercase ASCII, uppercase ASCII, numbers, and '-', and may not end
    /// with a '-'. Names cannot be a UUID though they may contain a UUID.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "A name unique within the parent collection",
    ///  "description": "Names must begin with a lower case ASCII letter, be
    /// composed exclusively of lowercase ASCII, uppercase ASCII, numbers, and
    /// '-', and may not end with a '-'. Names cannot be a UUID though they may
    /// contain a UUID.",
    ///  "type": "string",
    ///  "maxLength": 63,
    ///  "pattern":
    /// "^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*
    /// [a-zA-Z0-9]$"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct UserId(String);
    impl ::std::ops::Deref for UserId {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<UserId> for String {
        fn from(value: UserId) -> Self {
            value.0
        }
    }

    impl From<&UserId> for UserId {
        fn from(value: &UserId) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for UserId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if value.len() > 63usize {
                return Err("longer than 63 characters".into());
            }
            if regress :: Regex :: new ("^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$") . unwrap () . find (value) . is_none () { return Err ("doesn't match pattern \"^(?![0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$)^[a-z][a-z0-9-]*[a-zA-Z0-9]$\"" . into ()) ; }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for UserId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for UserId {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for UserId {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for UserId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Parameters for setting a user's password
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Parameters for setting a user's password",
    ///  "oneOf": [
    ///    {
    ///      "description": "Sets the user's password to the provided value",
    ///      "type": "object",
    ///      "required": [
    ///        "details",
    ///        "user_password_value"
    ///      ],
    ///      "properties": {
    ///        "details": {
    ///          "$ref": "#/components/schemas/Password"
    ///        },
    ///        "user_password_value": {
    ///          "type": "string",
    ///          "enum": [
    ///            "password"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "Invalidates any current password (disabling
    /// password authentication)",
    ///      "type": "object",
    ///      "required": [
    ///        "user_password_value"
    ///      ],
    ///      "properties": {
    ///        "user_password_value": {
    ///          "type": "string",
    ///          "enum": [
    ///            "invalid_password"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "user_password_value", content = "details")]
    pub enum UserPassword {
        ///Sets the user's password to the provided value
        #[serde(rename = "password")]
        Password(Password),
        #[serde(rename = "invalid_password")]
        InvalidPassword,
    }

    impl From<&UserPassword> for UserPassword {
        fn from(value: &UserPassword) -> Self {
            value.clone()
        }
    }

    impl From<Password> for UserPassword {
        fn from(value: Password) -> Self {
            Self::Password(value)
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/User"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserResultsPage {
        ///list of items on this page of results
        pub items: Vec<User>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&UserResultsPage> for UserResultsPage {
        fn from(value: &UserResultsPage) -> Self {
            value.clone()
        }
    }

    ///Credentials for local user login
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Credentials for local user login",
    ///  "type": "object",
    ///  "required": [
    ///    "password",
    ///    "username"
    ///  ],
    ///  "properties": {
    ///    "password": {
    ///      "$ref": "#/components/schemas/Password"
    ///    },
    ///    "username": {
    ///      "$ref": "#/components/schemas/UserId"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UsernamePasswordCredentials {
        pub password: Password,
        pub username: UserId,
    }

    impl From<&UsernamePasswordCredentials> for UsernamePasswordCredentials {
        fn from(value: &UsernamePasswordCredentials) -> Self {
            value.clone()
        }
    }

    ///VersionRange
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "high",
    ///    "low"
    ///  ],
    ///  "properties": {
    ///    "high": {
    ///      "$ref": "#/components/schemas/SemverVersion"
    ///    },
    ///    "low": {
    ///      "$ref": "#/components/schemas/SemverVersion"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VersionRange {
        pub high: SemverVersion,
        pub low: SemverVersion,
    }

    impl From<&VersionRange> for VersionRange {
        fn from(value: &VersionRange) -> Self {
            value.clone()
        }
    }

    ///Client view of a [`Vpc`]
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Client view of a [`Vpc`]",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "dns_name",
    ///    "id",
    ///    "ipv6_prefix",
    ///    "name",
    ///    "project_id",
    ///    "system_router_id",
    ///    "time_created",
    ///    "time_modified"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "dns_name": {
    ///      "description": "The name used for the VPC in DNS.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "ipv6_prefix": {
    ///      "description": "The unique local IPv6 address range for subnets in
    /// this VPC",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Ipv6Net"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "project_id": {
    ///      "description": "id for the project containing this VPC",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "system_router_id": {
    ///      "description": "id for the system router where subnet default
    /// routes are registered",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Vpc {
        ///human-readable free-form text about a resource
        pub description: String,
        ///The name used for the VPC in DNS.
        pub dns_name: Name,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///The unique local IPv6 address range for subnets in this VPC
        pub ipv6_prefix: Ipv6Net,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///id for the project containing this VPC
        pub project_id: uuid::Uuid,
        ///id for the system router where subnet default routes are registered
        pub system_router_id: uuid::Uuid,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&Vpc> for Vpc {
        fn from(value: &Vpc) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a [`Vpc`](crate::external_api::views::Vpc)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a
    /// [`Vpc`](crate::external_api::views::Vpc)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "dns_name",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "dns_name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    },
    ///    "ipv6_prefix": {
    ///      "description": "The IPv6 prefix for this VPC.\n\nAll IPv6 subnets
    /// created from this VPC must be taken from this range, which sould be a
    /// Unique Local Address in the range `fd00::/48`. The default VPC Subnet
    /// will have the first `/64` range from this prefix.",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Ipv6Net"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcCreate {
        pub description: String,
        pub dns_name: Name,
        ///The IPv6 prefix for this VPC.
        ///
        ///All IPv6 subnets created from this VPC must be taken from this
        /// range, which sould be a Unique Local Address in the range
        /// `fd00::/48`. The default VPC Subnet will have the first `/64` range
        /// from this prefix.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ipv6_prefix: Option<Ipv6Net>,
        pub name: Name,
    }

    impl From<&VpcCreate> for VpcCreate {
        fn from(value: &VpcCreate) -> Self {
            value.clone()
        }
    }

    ///A single rule in a VPC firewall
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single rule in a VPC firewall",
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "description",
    ///    "direction",
    ///    "filters",
    ///    "id",
    ///    "name",
    ///    "priority",
    ///    "status",
    ///    "targets",
    ///    "time_created",
    ///    "time_modified",
    ///    "vpc_id"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "description": "whether traffic matching the rule should be allowed
    /// or dropped",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VpcFirewallRuleAction"
    ///        }
    ///      ]
    ///    },
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "direction": {
    ///      "description": "whether this rule is for incoming or outgoing
    /// traffic",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VpcFirewallRuleDirection"
    ///        }
    ///      ]
    ///    },
    ///    "filters": {
    ///      "description": "reductions on the scope of the rule",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VpcFirewallRuleFilter"
    ///        }
    ///      ]
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "priority": {
    ///      "description": "the relative priority of this rule",
    ///      "type": "integer",
    ///      "format": "uint16",
    ///      "minimum": 0.0
    ///    },
    ///    "status": {
    ///      "description": "whether this rule is in effect",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VpcFirewallRuleStatus"
    ///        }
    ///      ]
    ///    },
    ///    "targets": {
    ///      "description": "list of sets of instances that the rule applies
    /// to",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VpcFirewallRuleTarget"
    ///      }
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "vpc_id": {
    ///      "description": "the VPC to which this rule belongs",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRule {
        ///whether traffic matching the rule should be allowed or dropped
        pub action: VpcFirewallRuleAction,
        ///human-readable free-form text about a resource
        pub description: String,
        ///whether this rule is for incoming or outgoing traffic
        pub direction: VpcFirewallRuleDirection,
        ///reductions on the scope of the rule
        pub filters: VpcFirewallRuleFilter,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///the relative priority of this rule
        pub priority: u16,
        ///whether this rule is in effect
        pub status: VpcFirewallRuleStatus,
        ///list of sets of instances that the rule applies to
        pub targets: Vec<VpcFirewallRuleTarget>,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        ///the VPC to which this rule belongs
        pub vpc_id: uuid::Uuid,
    }

    impl From<&VpcFirewallRule> for VpcFirewallRule {
        fn from(value: &VpcFirewallRule) -> Self {
            value.clone()
        }
    }

    ///VpcFirewallRuleAction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "allow",
    ///    "deny"
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
    pub enum VpcFirewallRuleAction {
        #[serde(rename = "allow")]
        Allow,
        #[serde(rename = "deny")]
        Deny,
    }

    impl From<&VpcFirewallRuleAction> for VpcFirewallRuleAction {
        fn from(value: &VpcFirewallRuleAction) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcFirewallRuleAction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Allow => write!(f, "allow"),
                Self::Deny => write!(f, "deny"),
            }
        }
    }

    impl std::str::FromStr for VpcFirewallRuleAction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "allow" => Ok(Self::Allow),
                "deny" => Ok(Self::Deny),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for VpcFirewallRuleAction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for VpcFirewallRuleAction {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for VpcFirewallRuleAction {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///VpcFirewallRuleDirection
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "inbound",
    ///    "outbound"
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
    pub enum VpcFirewallRuleDirection {
        #[serde(rename = "inbound")]
        Inbound,
        #[serde(rename = "outbound")]
        Outbound,
    }

    impl From<&VpcFirewallRuleDirection> for VpcFirewallRuleDirection {
        fn from(value: &VpcFirewallRuleDirection) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcFirewallRuleDirection {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Inbound => write!(f, "inbound"),
                Self::Outbound => write!(f, "outbound"),
            }
        }
    }

    impl std::str::FromStr for VpcFirewallRuleDirection {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "inbound" => Ok(Self::Inbound),
                "outbound" => Ok(Self::Outbound),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for VpcFirewallRuleDirection {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for VpcFirewallRuleDirection {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for VpcFirewallRuleDirection {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Filter for a firewall rule. A given packet must match every field that
    /// is present for the rule to apply to it. A packet matches a field if any
    /// entry in that field matches the packet.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Filter for a firewall rule. A given packet must match
    /// every field that is present for the rule to apply to it. A packet
    /// matches a field if any entry in that field matches the packet.",
    ///  "type": "object",
    ///  "properties": {
    ///    "hosts": {
    ///      "description": "If present, the sources (if incoming) or
    /// destinations (if outgoing) this rule applies to.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/VpcFirewallRuleHostFilter"
    ///      }
    ///    },
    ///    "ports": {
    ///      "description": "If present, the destination ports this rule applies
    /// to.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/L4PortRange"
    ///      }
    ///    },
    ///    "protocols": {
    ///      "description": "If present, the networking protocols this rule
    /// applies to.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/VpcFirewallRuleProtocol"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRuleFilter {
        ///If present, the sources (if incoming) or destinations (if outgoing)
        /// this rule applies to.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hosts: Option<Vec<VpcFirewallRuleHostFilter>>,
        ///If present, the destination ports this rule applies to.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ports: Option<Vec<L4PortRange>>,
        ///If present, the networking protocols this rule applies to.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocols: Option<Vec<VpcFirewallRuleProtocol>>,
    }

    impl From<&VpcFirewallRuleFilter> for VpcFirewallRuleFilter {
        fn from(value: &VpcFirewallRuleFilter) -> Self {
            value.clone()
        }
    }

    ///The `VpcFirewallRuleHostFilter` is used to filter traffic on the basis
    /// of its source or destination host.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The `VpcFirewallRuleHostFilter` is used to filter
    /// traffic on the basis of its source or destination host.",
    ///  "oneOf": [
    ///    {
    ///      "description": "The rule applies to traffic from/to all instances
    /// in the VPC",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "vpc"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The rule applies to traffic from/to all instances
    /// in the VPC Subnet",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "subnet"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The rule applies to traffic from/to this specific
    /// instance",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "instance"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The rule applies to traffic from/to a specific IP
    /// address",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ip"
    ///          ]
    ///        },
    ///        "value": {
    ///          "type": "string",
    ///          "format": "ip"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The rule applies to traffic from/to a specific IP
    /// subnet",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ip_net"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/IpNet"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum VpcFirewallRuleHostFilter {
        ///The rule applies to traffic from/to all instances in the VPC
        #[serde(rename = "vpc")]
        Vpc(Name),
        ///The rule applies to traffic from/to all instances in the VPC Subnet
        #[serde(rename = "subnet")]
        Subnet(Name),
        ///The rule applies to traffic from/to this specific instance
        #[serde(rename = "instance")]
        Instance(Name),
        ///The rule applies to traffic from/to a specific IP address
        #[serde(rename = "ip")]
        Ip(std::net::IpAddr),
        ///The rule applies to traffic from/to a specific IP subnet
        #[serde(rename = "ip_net")]
        IpNet(IpNet),
    }

    impl From<&VpcFirewallRuleHostFilter> for VpcFirewallRuleHostFilter {
        fn from(value: &VpcFirewallRuleHostFilter) -> Self {
            value.clone()
        }
    }

    impl From<std::net::IpAddr> for VpcFirewallRuleHostFilter {
        fn from(value: std::net::IpAddr) -> Self {
            Self::Ip(value)
        }
    }

    impl From<IpNet> for VpcFirewallRuleHostFilter {
        fn from(value: IpNet) -> Self {
            Self::IpNet(value)
        }
    }

    ///The protocols that may be specified in a firewall rule's filter
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The protocols that may be specified in a firewall
    /// rule's filter",
    ///  "type": "string",
    ///  "enum": [
    ///    "TCP",
    ///    "UDP",
    ///    "ICMP"
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
    pub enum VpcFirewallRuleProtocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
        #[serde(rename = "ICMP")]
        Icmp,
    }

    impl From<&VpcFirewallRuleProtocol> for VpcFirewallRuleProtocol {
        fn from(value: &VpcFirewallRuleProtocol) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcFirewallRuleProtocol {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tcp => write!(f, "TCP"),
                Self::Udp => write!(f, "UDP"),
                Self::Icmp => write!(f, "ICMP"),
            }
        }
    }

    impl std::str::FromStr for VpcFirewallRuleProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "TCP" => Ok(Self::Tcp),
                "UDP" => Ok(Self::Udp),
                "ICMP" => Ok(Self::Icmp),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for VpcFirewallRuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for VpcFirewallRuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for VpcFirewallRuleProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///VpcFirewallRuleStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "disabled",
    ///    "enabled"
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
    pub enum VpcFirewallRuleStatus {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "enabled")]
        Enabled,
    }

    impl From<&VpcFirewallRuleStatus> for VpcFirewallRuleStatus {
        fn from(value: &VpcFirewallRuleStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcFirewallRuleStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Disabled => write!(f, "disabled"),
                Self::Enabled => write!(f, "enabled"),
            }
        }
    }

    impl std::str::FromStr for VpcFirewallRuleStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "disabled" => Ok(Self::Disabled),
                "enabled" => Ok(Self::Enabled),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for VpcFirewallRuleStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for VpcFirewallRuleStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for VpcFirewallRuleStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A `VpcFirewallRuleTarget` is used to specify the set of [`Instance`]s to
    /// which a firewall rule applies.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A `VpcFirewallRuleTarget` is used to specify the set of
    /// [`Instance`]s to which a firewall rule applies.",
    ///  "oneOf": [
    ///    {
    ///      "description": "The rule applies to all instances in the VPC",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "vpc"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The rule applies to all instances in the VPC
    /// Subnet",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "subnet"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The rule applies to this specific instance",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "instance"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The rule applies to a specific IP address",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ip"
    ///          ]
    ///        },
    ///        "value": {
    ///          "type": "string",
    ///          "format": "ip"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "description": "The rule applies to a specific IP subnet",
    ///      "type": "object",
    ///      "required": [
    ///        "type",
    ///        "value"
    ///      ],
    ///      "properties": {
    ///        "type": {
    ///          "type": "string",
    ///          "enum": [
    ///            "ip_net"
    ///          ]
    ///        },
    ///        "value": {
    ///          "$ref": "#/components/schemas/IpNet"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "type", content = "value")]
    pub enum VpcFirewallRuleTarget {
        ///The rule applies to all instances in the VPC
        #[serde(rename = "vpc")]
        Vpc(Name),
        ///The rule applies to all instances in the VPC Subnet
        #[serde(rename = "subnet")]
        Subnet(Name),
        ///The rule applies to this specific instance
        #[serde(rename = "instance")]
        Instance(Name),
        ///The rule applies to a specific IP address
        #[serde(rename = "ip")]
        Ip(std::net::IpAddr),
        ///The rule applies to a specific IP subnet
        #[serde(rename = "ip_net")]
        IpNet(IpNet),
    }

    impl From<&VpcFirewallRuleTarget> for VpcFirewallRuleTarget {
        fn from(value: &VpcFirewallRuleTarget) -> Self {
            value.clone()
        }
    }

    impl From<std::net::IpAddr> for VpcFirewallRuleTarget {
        fn from(value: std::net::IpAddr) -> Self {
            Self::Ip(value)
        }
    }

    impl From<IpNet> for VpcFirewallRuleTarget {
        fn from(value: IpNet) -> Self {
            Self::IpNet(value)
        }
    }

    ///A single rule in a VPC firewall
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single rule in a VPC firewall",
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "description",
    ///    "direction",
    ///    "filters",
    ///    "name",
    ///    "priority",
    ///    "status",
    ///    "targets"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "description": "whether traffic matching the rule should be allowed
    /// or dropped",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VpcFirewallRuleAction"
    ///        }
    ///      ]
    ///    },
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "direction": {
    ///      "description": "whether this rule is for incoming or outgoing
    /// traffic",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VpcFirewallRuleDirection"
    ///        }
    ///      ]
    ///    },
    ///    "filters": {
    ///      "description": "reductions on the scope of the rule",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VpcFirewallRuleFilter"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "name of the rule, unique to this VPC",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "priority": {
    ///      "description": "the relative priority of this rule",
    ///      "type": "integer",
    ///      "format": "uint16",
    ///      "minimum": 0.0
    ///    },
    ///    "status": {
    ///      "description": "whether this rule is in effect",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VpcFirewallRuleStatus"
    ///        }
    ///      ]
    ///    },
    ///    "targets": {
    ///      "description": "list of sets of instances that the rule applies
    /// to",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VpcFirewallRuleTarget"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRuleUpdate {
        ///whether traffic matching the rule should be allowed or dropped
        pub action: VpcFirewallRuleAction,
        ///human-readable free-form text about a resource
        pub description: String,
        ///whether this rule is for incoming or outgoing traffic
        pub direction: VpcFirewallRuleDirection,
        ///reductions on the scope of the rule
        pub filters: VpcFirewallRuleFilter,
        ///name of the rule, unique to this VPC
        pub name: Name,
        ///the relative priority of this rule
        pub priority: u16,
        ///whether this rule is in effect
        pub status: VpcFirewallRuleStatus,
        ///list of sets of instances that the rule applies to
        pub targets: Vec<VpcFirewallRuleTarget>,
    }

    impl From<&VpcFirewallRuleUpdate> for VpcFirewallRuleUpdate {
        fn from(value: &VpcFirewallRuleUpdate) -> Self {
            value.clone()
        }
    }

    ///Updateable properties of a `Vpc`'s firewall Note that VpcFirewallRules
    /// are implicitly created along with a Vpc, so there is no explicit
    /// creation.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Updateable properties of a `Vpc`'s firewall Note that
    /// VpcFirewallRules are implicitly created along with a Vpc, so there is no
    /// explicit creation.",
    ///  "type": "object",
    ///  "required": [
    ///    "rules"
    ///  ],
    ///  "properties": {
    ///    "rules": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VpcFirewallRuleUpdate"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRuleUpdateParams {
        pub rules: Vec<VpcFirewallRuleUpdate>,
    }

    impl From<&VpcFirewallRuleUpdateParams> for VpcFirewallRuleUpdateParams {
        fn from(value: &VpcFirewallRuleUpdateParams) -> Self {
            value.clone()
        }
    }

    ///Collection of a Vpc's firewall rules
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Collection of a Vpc's firewall rules",
    ///  "type": "object",
    ///  "required": [
    ///    "rules"
    ///  ],
    ///  "properties": {
    ///    "rules": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VpcFirewallRule"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcFirewallRules {
        pub rules: Vec<VpcFirewallRule>,
    }

    impl From<&VpcFirewallRules> for VpcFirewallRules {
        fn from(value: &VpcFirewallRules) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Vpc"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcResultsPage {
        ///list of items on this page of results
        pub items: Vec<Vpc>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&VpcResultsPage> for VpcResultsPage {
        fn from(value: &VpcResultsPage) -> Self {
            value.clone()
        }
    }

    ///A VPC router defines a series of rules that indicate where traffic
    /// should be sent depending on its destination.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A VPC router defines a series of rules that indicate
    /// where traffic should be sent depending on its destination.",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "kind",
    ///    "name",
    ///    "time_created",
    ///    "time_modified",
    ///    "vpc_id"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "kind": {
    ///      "$ref": "#/components/schemas/VpcRouterKind"
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "vpc_id": {
    ///      "description": "The VPC to which the router belongs.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcRouter {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        pub kind: VpcRouterKind,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        ///The VPC to which the router belongs.
        pub vpc_id: uuid::Uuid,
    }

    impl From<&VpcRouter> for VpcRouter {
        fn from(value: &VpcRouter) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a
    /// [`VpcRouter`](crate::external_api::views::VpcRouter)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a
    /// [`VpcRouter`](crate::external_api::views::VpcRouter)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcRouterCreate {
        pub description: String,
        pub name: Name,
    }

    impl From<&VpcRouterCreate> for VpcRouterCreate {
        fn from(value: &VpcRouterCreate) -> Self {
            value.clone()
        }
    }

    ///VpcRouterKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "system",
    ///    "custom"
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
    pub enum VpcRouterKind {
        #[serde(rename = "system")]
        System,
        #[serde(rename = "custom")]
        Custom,
    }

    impl From<&VpcRouterKind> for VpcRouterKind {
        fn from(value: &VpcRouterKind) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for VpcRouterKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::System => write!(f, "system"),
                Self::Custom => write!(f, "custom"),
            }
        }
    }

    impl std::str::FromStr for VpcRouterKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "system" => Ok(Self::System),
                "custom" => Ok(Self::Custom),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for VpcRouterKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for VpcRouterKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for VpcRouterKind {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VpcRouter"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcRouterResultsPage {
        ///list of items on this page of results
        pub items: Vec<VpcRouter>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&VpcRouterResultsPage> for VpcRouterResultsPage {
        fn from(value: &VpcRouterResultsPage) -> Self {
            value.clone()
        }
    }

    ///Updateable properties of a
    /// [`VpcRouter`](crate::external_api::views::VpcRouter)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Updateable properties of a
    /// [`VpcRouter`](crate::external_api::views::VpcRouter)",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
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
    pub struct VpcRouterUpdate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
    }

    impl From<&VpcRouterUpdate> for VpcRouterUpdate {
        fn from(value: &VpcRouterUpdate) -> Self {
            value.clone()
        }
    }

    ///A VPC subnet represents a logical grouping for instances that allows
    /// network traffic between them, within a IPv4 subnetwork or optionall an
    /// IPv6 subnetwork.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A VPC subnet represents a logical grouping for
    /// instances that allows network traffic between them, within a IPv4
    /// subnetwork or optionall an IPv6 subnetwork.",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "id",
    ///    "ipv4_block",
    ///    "ipv6_block",
    ///    "name",
    ///    "time_created",
    ///    "time_modified",
    ///    "vpc_id"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "description": "human-readable free-form text about a resource",
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "unique, immutable, system-controlled identifier for
    /// each resource",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "ipv4_block": {
    ///      "description": "The IPv4 subnet CIDR block.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Ipv4Net"
    ///        }
    ///      ]
    ///    },
    ///    "ipv6_block": {
    ///      "description": "The IPv6 subnet CIDR block.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Ipv6Net"
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "unique, mutable, user-controlled identifier for
    /// each resource",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Name"
    ///        }
    ///      ]
    ///    },
    ///    "time_created": {
    ///      "description": "timestamp when this resource was created",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "time_modified": {
    ///      "description": "timestamp when this resource was last modified",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "vpc_id": {
    ///      "description": "The VPC to which the subnet belongs.",
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcSubnet {
        ///human-readable free-form text about a resource
        pub description: String,
        ///unique, immutable, system-controlled identifier for each resource
        pub id: uuid::Uuid,
        ///The IPv4 subnet CIDR block.
        pub ipv4_block: Ipv4Net,
        ///The IPv6 subnet CIDR block.
        pub ipv6_block: Ipv6Net,
        ///unique, mutable, user-controlled identifier for each resource
        pub name: Name,
        ///timestamp when this resource was created
        pub time_created: chrono::DateTime<chrono::offset::Utc>,
        ///timestamp when this resource was last modified
        pub time_modified: chrono::DateTime<chrono::offset::Utc>,
        ///The VPC to which the subnet belongs.
        pub vpc_id: uuid::Uuid,
    }

    impl From<&VpcSubnet> for VpcSubnet {
        fn from(value: &VpcSubnet) -> Self {
            value.clone()
        }
    }

    ///Create-time parameters for a
    /// [`VpcSubnet`](crate::external_api::views::VpcSubnet)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Create-time parameters for a
    /// [`VpcSubnet`](crate::external_api::views::VpcSubnet)",
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "ipv4_block",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "ipv4_block": {
    ///      "description": "The IPv4 address range for this subnet.\n\nIt must
    /// be allocated from an RFC 1918 private address range, and must not
    /// overlap with any other existing subnet in the VPC.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Ipv4Net"
    ///        }
    ///      ]
    ///    },
    ///    "ipv6_block": {
    ///      "description": "The IPv6 address range for this subnet.\n\nIt must
    /// be allocated from the RFC 4193 Unique Local Address range, with the
    /// prefix equal to the parent VPC's prefix. A random `/64` block will be
    /// assigned if one is not provided. It must not overlap with any existing
    /// subnet in the VPC.",
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Ipv6Net"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "$ref": "#/components/schemas/Name"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcSubnetCreate {
        pub description: String,
        ///The IPv4 address range for this subnet.
        ///
        ///It must be allocated from an RFC 1918 private address range, and
        /// must not overlap with any other existing subnet in the VPC.
        pub ipv4_block: Ipv4Net,
        ///The IPv6 address range for this subnet.
        ///
        ///It must be allocated from the RFC 4193 Unique Local Address range,
        /// with the prefix equal to the parent VPC's prefix. A random `/64`
        /// block will be assigned if one is not provided. It must not overlap
        /// with any existing subnet in the VPC.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ipv6_block: Option<Ipv6Net>,
        pub name: Name,
    }

    impl From<&VpcSubnetCreate> for VpcSubnetCreate {
        fn from(value: &VpcSubnetCreate) -> Self {
            value.clone()
        }
    }

    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/VpcSubnet"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VpcSubnetResultsPage {
        ///list of items on this page of results
        pub items: Vec<VpcSubnet>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next_page: Option<String>,
    }

    impl From<&VpcSubnetResultsPage> for VpcSubnetResultsPage {
        fn from(value: &VpcSubnetResultsPage) -> Self {
            value.clone()
        }
    }

    ///Updateable properties of a
    /// [`VpcSubnet`](crate::external_api::views::VpcSubnet)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Updateable properties of a
    /// [`VpcSubnet`](crate::external_api::views::VpcSubnet)",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
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
    pub struct VpcSubnetUpdate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
    }

    impl From<&VpcSubnetUpdate> for VpcSubnetUpdate {
        fn from(value: &VpcSubnetUpdate) -> Self {
            value.clone()
        }
    }

    ///Updateable properties of a [`Vpc`](crate::external_api::views::Vpc)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Updateable properties of a
    /// [`Vpc`](crate::external_api::views::Vpc)",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "dns_name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/Name"
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
    pub struct VpcUpdate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dns_name: Option<Name>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<Name>,
    }

    impl From<&VpcUpdate> for VpcUpdate {
        fn from(value: &VpcUpdate) -> Self {
            value.clone()
        }
    }

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn default_bool<const V: bool>() -> bool {
            V
        }

        pub(super) fn instance_create_network_interfaces(
        ) -> super::InstanceNetworkInterfaceAttachment {
            super::InstanceNetworkInterfaceAttachment::Default
        }
    }
}

#[derive(Clone, Debug)]
///Client for Oxide Region API
///
///API for interacting with the Oxide control plane
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
    ///Fetch a disk by id
    ///
    ///Use `GET /v1/disks/{disk}` instead
    ///
    ///Sends a `GET` request to `/by-id/disks/{id}`
    pub async fn disk_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/disks/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch an image by id
    ///
    ///Sends a `GET` request to `/by-id/images/{id}`
    pub async fn image_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Image>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/images/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch an instance by id
    ///
    ///Sends a `GET` request to `/by-id/instances/{id}`
    pub async fn instance_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/instances/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch a network interface by id
    ///
    ///Sends a `GET` request to `/by-id/network-interfaces/{id}`
    pub async fn instance_network_interface_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::NetworkInterface>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/network-interfaces/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch an organization by id
    ///
    ///Use `GET /v1/organizations/{organization}` instead
    ///
    ///Sends a `GET` request to `/by-id/organizations/{id}`
    pub async fn organization_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/organizations/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch a project by id
    ///
    ///Use `GET /v1/projects/{project}` instead
    ///
    ///Sends a `GET` request to `/by-id/projects/{id}`
    pub async fn project_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/projects/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch a snapshot by id
    ///
    ///Sends a `GET` request to `/by-id/snapshots/{id}`
    pub async fn snapshot_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Snapshot>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/snapshots/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch a route by id
    ///
    ///Sends a `GET` request to `/by-id/vpc-router-routes/{id}`
    pub async fn vpc_router_route_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::RouterRoute>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/vpc-router-routes/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Get a router by id
    ///
    ///Sends a `GET` request to `/by-id/vpc-routers/{id}`
    pub async fn vpc_router_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::VpcRouter>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/vpc-routers/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch a subnet by id
    ///
    ///Sends a `GET` request to `/by-id/vpc-subnets/{id}`
    pub async fn vpc_subnet_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::VpcSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/vpc-subnets/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch a VPC
    ///
    ///Sends a `GET` request to `/by-id/vpcs/{id}`
    pub async fn vpc_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Vpc>, Error<types::Error>> {
        let url = format!(
            "{}/by-id/vpcs/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Start an OAuth 2.0 Device Authorization Grant
    ///
    ///This endpoint is designed to be accessed from an *unauthenticated* API
    /// client. It generates and records a `device_code` and `user_code` which
    /// must be verified and confirmed prior to a token being granted.
    ///
    ///Sends a `POST` request to `/device/auth`
    pub async fn device_auth_request<'a>(
        &'a self,
        body: &'a types::DeviceAuthRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<ByteStream>> {
        let url = format!("{}/device/auth", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self.client.post(url).form_urlencoded(&body)?.build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
        }
    }

    ///Confirm an OAuth 2.0 Device Authorization Grant
    ///
    ///This endpoint is designed to be accessed by the user agent (browser),
    /// not the client requesting the token. So we do not actually return the
    /// token here; it will be returned in response to the poll on
    /// `/device/token`.
    ///
    ///Sends a `POST` request to `/device/confirm`
    pub async fn device_auth_confirm<'a>(
        &'a self,
        body: &'a types::DeviceAuthVerify,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/device/confirm", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Request a device access token
    ///
    ///This endpoint should be polled by the client until the user code is
    /// verified and the grant is confirmed.
    ///
    ///Sends a `POST` request to `/device/token`
    pub async fn device_access_token<'a>(
        &'a self,
        body: &'a types::DeviceAccessTokenRequest,
    ) -> Result<ResponseValue<ByteStream>, Error<ByteStream>> {
        let url = format!("{}/device/token", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self.client.post(url).form_urlencoded(&body)?.build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
        }
    }

    ///List groups
    ///
    ///Sends a `GET` request to `/groups`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn group_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::GroupResultsPage>, Error<types::Error>> {
        let url = format!("{}/groups", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List groups as a Stream
    ///
    ///Sends repeated `GET` requests to `/groups` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn group_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Group, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.group_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.group_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Sends a `POST` request to `/login`
    pub async fn login_spoof<'a>(
        &'a self,
        body: &'a types::SpoofLoginBody,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/login", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Authenticate a user (i.e., log in) via username and password
    ///
    ///Sends a `POST` request to `/login/{silo_name}/local`
    pub async fn login_local<'a>(
        &'a self,
        silo_name: &'a types::Name,
        body: &'a types::UsernamePasswordCredentials,
    ) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
        let url = format!(
            "{}/login/{}/local",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Prompt user login
    ///
    ///Either display a page asking a user for their credentials, or redirect
    /// them to their identity provider.
    ///
    ///Sends a `GET` request to `/login/{silo_name}/saml/{provider_name}`
    pub async fn login_saml_begin<'a>(
        &'a self,
        silo_name: &'a types::Name,
        provider_name: &'a types::Name,
    ) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
        let url = format!(
            "{}/login/{}/saml/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&provider_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Authenticate a user (i.e., log in) via SAML
    ///
    ///Sends a `POST` request to `/login/{silo_name}/saml/{provider_name}`
    pub async fn login_saml<'a, B: Into<reqwest::Body>>(
        &'a self,
        silo_name: &'a types::Name,
        provider_name: &'a types::Name,
        body: B,
    ) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
        let url = format!(
            "{}/login/{}/saml/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&provider_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/octet-stream"),
            )
            .body(body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200..=299 => Ok(ResponseValue::stream(response)),
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/logout`
    pub async fn logout<'a>(&'a self) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/logout", self.baseurl,);
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

    ///List organizations
    ///
    ///Use `GET /v1/organizations` instead
    ///
    ///Sends a `GET` request to `/organizations`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn organization_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::OrganizationResultsPage>, Error<types::Error>> {
        let url = format!("{}/organizations", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List organizations as a Stream
    ///
    ///Use `GET /v1/organizations` instead
    ///
    ///Sends repeated `GET` requests to `/organizations` until there are no
    /// more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn organization_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Organization, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.organization_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.organization_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create an organization
    ///
    ///Use `POST /v1/organizations` instead
    ///
    ///Sends a `POST` request to `/organizations`
    pub async fn organization_create<'a>(
        &'a self,
        body: &'a types::OrganizationCreate,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!("{}/organizations", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch an organization
    ///
    ///Use `GET /v1/organizations/{organization}` instead
    ///
    ///Sends a `GET` request to `/organizations/{organization_name}`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    pub async fn organization_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
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

    ///Update an organization
    ///
    ///Use `PUT /v1/organizations/{organization}` instead
    ///
    ///Sends a `PUT` request to `/organizations/{organization_name}`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `body`
    pub async fn organization_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        body: &'a types::OrganizationUpdate,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
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

    ///Delete an organization
    ///
    ///Use `DELETE /v1/organizations/{organization}` instead
    ///
    ///Sends a `DELETE` request to `/organizations/{organization_name}`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    pub async fn organization_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///Fetch an organization's IAM policy
    ///
    ///Use `GET /v1/organizations/{organization}/policy` instead
    ///
    ///Sends a `GET` request to `/organizations/{organization_name}/policy`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    pub async fn organization_policy_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
    ) -> Result<ResponseValue<types::OrganizationRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/policy",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
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

    ///Update an organization's IAM policy
    ///
    ///Use `PUT /v1/organizations/{organization}/policy` instead
    ///
    ///Sends a `PUT` request to `/organizations/{organization_name}/policy`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `body`
    pub async fn organization_policy_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        body: &'a types::OrganizationRolePolicy,
    ) -> Result<ResponseValue<types::OrganizationRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/policy",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
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

    ///List projects
    ///
    ///Use `GET /v1/projects` instead
    ///
    ///Sends a `GET` request to `/organizations/{organization_name}/projects`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn project_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::ProjectResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List projects as a Stream
    ///
    ///Use `GET /v1/projects` instead
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn project_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Project, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.project_list(organization_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.project_list(organization_name, limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create a project
    ///
    ///Use `POST /v1/projects` instead
    ///
    ///Sends a `POST` request to `/organizations/{organization_name}/projects`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `body`
    pub async fn project_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        body: &'a types::ProjectCreate,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects",
            self.baseurl,
            encode_path(&organization_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a project
    ///
    ///Use `GET /v1/projects/{project}` instead
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    pub async fn project_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
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

    ///Update a project
    ///
    ///Use `PUT /v1/projects/{project}` instead
    ///
    ///Sends a `PUT` request to
    /// `/organizations/{organization_name}/projects/{project_name}`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `body`
    pub async fn project_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::ProjectUpdate,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
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

    ///Delete a project
    ///
    ///Use `DELETE /v1/projects/{project}` instead
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    pub async fn project_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List disks
    ///
    ///Use `GET /v1/disks` instead
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/disks`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn disk_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::DiskResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List disks as a Stream
    ///
    ///Use `GET /v1/disks` instead
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/disks` until
    /// there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn disk_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Disk, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.disk_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.disk_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Use `POST /v1/disks` instead
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/disks`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `body`
    pub async fn disk_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::DiskCreate,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a disk
    ///
    ///Use `GET /v1/disks/{disk}` instead
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/disks/
    /// {disk_name}`
    pub async fn disk_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        disk_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&disk_name.to_string()),
        );
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

    ///Use `DELETE /v1/disks/{disk}` instead
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}/disks/
    /// {disk_name}`
    pub async fn disk_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        disk_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&disk_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///Fetch disk metrics
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/disks/
    /// {disk_name}/metrics/{metric_name}`
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `disk_name`
    /// - `metric_name`
    /// - `end_time`: An exclusive end time of metrics.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `start_time`: An inclusive start time of metrics.
    pub async fn disk_metrics_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        disk_name: &'a types::Name,
        metric_name: types::DiskMetricName,
        end_time: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        start_time: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
    ) -> Result<ResponseValue<types::MeasurementResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/disks/{}/metrics/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&disk_name.to_string()),
            encode_path(&metric_name.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(end_time) = &end_time {
            query.push(("end_time", end_time.to_string()));
        }

        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(start_time) = &start_time {
            query.push(("start_time", start_time.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Fetch disk metrics as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/disks/
    /// {disk_name}/metrics/{metric_name}` until there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `disk_name`
    /// - `metric_name`
    /// - `end_time`: An exclusive end time of metrics.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `start_time`: An inclusive start time of metrics.
    pub fn disk_metrics_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        disk_name: &'a types::Name,
        metric_name: types::DiskMetricName,
        end_time: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        limit: Option<std::num::NonZeroU32>,
        start_time: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
    ) -> impl futures::Stream<Item = Result<types::Measurement, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.disk_metrics_list(
            organization_name,
            project_name,
            disk_name,
            metric_name,
            end_time,
            limit,
            None,
            start_time,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.disk_metrics_list(
                        organization_name,
                        project_name,
                        disk_name,
                        metric_name,
                        None,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    ///List images
    ///
    ///List images in a project. The images are returned sorted by creation
    /// date, with the most recent images appearing first.
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/images`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn image_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::ImageResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/images",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List images as a Stream
    ///
    ///List images in a project. The images are returned sorted by creation
    /// date, with the most recent images appearing first.
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/images`
    /// until there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn image_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Image, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.image_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.image_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create an image
    ///
    ///Create a new image in a project.
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/images`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `body`
    pub async fn image_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::ImageCreate,
    ) -> Result<ResponseValue<types::Image>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/images",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch an image
    ///
    ///Fetch the details for a specific image in a project.
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/images/
    /// {image_name}`
    pub async fn image_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        image_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Image>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/images/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&image_name.to_string()),
        );
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

    ///Delete an image
    ///
    ///Permanently delete an image from a project. This operation cannot be
    /// undone. Any instances in the project using the image will continue to
    /// run, however new instances can not be created with this image.
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}/images/
    /// {image_name}`
    pub async fn image_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        image_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/images/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&image_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List instances
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn instance_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::InstanceResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List instances as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/instances`
    /// until there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn instance_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Instance, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.instance_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create an instance
    ///
    ///Use `POST /v1/instances` instead
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `body`
    pub async fn instance_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::InstanceCreate,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch an instance
    ///
    ///Use `GET /v1/instances/{instance}` instead
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}`
    pub async fn instance_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
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

    ///Delete an instance
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}`
    pub async fn instance_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List an instance's disks
    ///
    ///Use `GET /v1/instances/{instance}/disks` instead
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/disks`
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `instance_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn instance_disk_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::DiskResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/disks",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List an instance's disks as a Stream
    ///
    ///Use `GET /v1/instances/{instance}/disks` instead
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/disks` until there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `instance_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn instance_disk_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Disk, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_disk_list(
            organization_name,
            project_name,
            instance_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.instance_disk_list(
                        organization_name,
                        project_name,
                        instance_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    ///Attach a disk to an instance
    ///
    ///Use `POST /v1/instances/{instance}/disks/attach` instead
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/disks/attach`
    pub async fn instance_disk_attach<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        body: &'a types::DiskIdentifier,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/disks/attach",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Detach a disk from an instance
    ///
    ///Use `POST /v1/disks/{disk}/detach` instead
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/disks/detach`
    pub async fn instance_disk_detach<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        body: &'a types::DiskIdentifier,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/disks/detach",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List external IP addresses
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/external-ips`
    pub async fn instance_external_ip_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::ExternalIpResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/external-ips",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
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

    ///Migrate an instance
    ///
    ///Use `POST /v1/instances/{instance}/migrate` instead
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/migrate`
    pub async fn instance_migrate<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        body: &'a types::InstanceMigrate,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/migrate",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///List network interfaces
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/network-interfaces`
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `instance_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn instance_network_interface_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::NetworkInterfaceResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List network interfaces as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/network-interfaces` until there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `instance_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn instance_network_interface_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::NetworkInterface, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_network_interface_list(
            organization_name,
            project_name,
            instance_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.instance_network_interface_list(
                        organization_name,
                        project_name,
                        instance_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    ///Create a network interface
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/network-interfaces`
    pub async fn instance_network_interface_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        body: &'a types::NetworkInterfaceCreate,
    ) -> Result<ResponseValue<types::NetworkInterface>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a network interface
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/network-interfaces/{interface_name}`
    pub async fn instance_network_interface_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        interface_name: &'a types::Name,
    ) -> Result<ResponseValue<types::NetworkInterface>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
            encode_path(&interface_name.to_string()),
        );
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

    ///Update a network interface
    ///
    ///Sends a `PUT` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/network-interfaces/{interface_name}`
    pub async fn instance_network_interface_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        interface_name: &'a types::Name,
        body: &'a types::NetworkInterfaceUpdate,
    ) -> Result<ResponseValue<types::NetworkInterface>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
            encode_path(&interface_name.to_string()),
        );
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

    ///Delete a network interface
    ///
    ///Note that the primary interface for an instance cannot be deleted if
    /// there are any secondary interfaces. A new primary interface must be
    /// designated first. The primary interface can be deleted if there are no
    /// secondary interfaces.
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/network-interfaces/{interface_name}`
    pub async fn instance_network_interface_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        interface_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/network-interfaces/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
            encode_path(&interface_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///Reboot an instance
    ///
    ///Use `POST /v1/instances/{instance}/reboot` instead
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/reboot`
    pub async fn instance_reboot<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/reboot",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Fetch an instance's serial console
    ///
    ///Use `GET /v1/instances/{instance}/serial-console` instead
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/serial-console`
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `instance_name`
    /// - `from_start`: Character index in the serial buffer from which to read,
    ///   counting the bytes output since instance start. If this is not
    ///   provided, `most_recent` must be provided, and if this *is* provided,
    ///   `most_recent` must *not* be provided.
    /// - `max_bytes`: Maximum number of bytes of buffered serial console
    ///   contents to return. If the requested range runs to the end of the
    ///   available buffer, the data returned will be shorter than `max_bytes`.
    /// - `most_recent`: Character index in the serial buffer from which to
    ///   read, counting *backward* from the most recently buffered data
    ///   retrieved from the instance. (See note on `from_start` about mutual
    ///   exclusivity)
    pub async fn instance_serial_console<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
        from_start: Option<u64>,
        max_bytes: Option<u64>,
        most_recent: Option<u64>,
    ) -> Result<ResponseValue<types::InstanceSerialConsoleData>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/serial-console",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(from_start) = &from_start {
            query.push(("from_start", from_start.to_string()));
        }

        if let Some(max_bytes) = &max_bytes {
            query.push(("max_bytes", max_bytes.to_string()));
        }

        if let Some(most_recent) = &most_recent {
            query.push(("most_recent", most_recent.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Connect to an instance's serial console
    ///
    ///Use `GET /v1/instances/{instance}/serial-console/stream` instead
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/serial-console/stream`
    pub async fn instance_serial_console_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<reqwest::Upgraded>, Error<reqwest::Upgraded>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/serial-console/stream",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
        );
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

    ///Boot an instance
    ///
    ///Use `POST /v1/instances/{instance}/start` instead
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/start`
    pub async fn instance_start<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/start",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Halt an instance
    ///
    ///Use `POST /v1/instances/{instance}/stop` instead
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/instances/
    /// {instance_name}/stop`
    pub async fn instance_stop<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        instance_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/instances/{}/stop",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&instance_name.to_string()),
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
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Fetch a project's IAM policy
    ///
    ///Use `GET /v1/projects/{project}/policy` instead
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/policy`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    pub async fn project_policy_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
    ) -> Result<ResponseValue<types::ProjectRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/policy",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
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

    ///Update a project's IAM policy
    ///
    ///Sends a `PUT` request to
    /// `/organizations/{organization_name}/projects/{project_name}/policy`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `body`
    pub async fn project_policy_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::ProjectRolePolicy,
    ) -> Result<ResponseValue<types::ProjectRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/policy",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
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

    ///List snapshots
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/snapshots`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn snapshot_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::SnapshotResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/snapshots",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List snapshots as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/snapshots`
    /// until there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn snapshot_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Snapshot, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.snapshot_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.snapshot_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create a snapshot
    ///
    ///Creates a point-in-time snapshot from a disk.
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/snapshots`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `body`
    pub async fn snapshot_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::SnapshotCreate,
    ) -> Result<ResponseValue<types::Snapshot>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/snapshots",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a snapshot
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/snapshots/
    /// {snapshot_name}`
    pub async fn snapshot_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        snapshot_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Snapshot>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/snapshots/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&snapshot_name.to_string()),
        );
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

    ///Delete a snapshot
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}/snapshots/
    /// {snapshot_name}`
    pub async fn snapshot_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        snapshot_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/snapshots/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&snapshot_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List VPCs
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn vpc_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::VpcResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List VPCs as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs` until
    /// there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn vpc_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Vpc, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_list(organization_name, project_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.vpc_list(
                            organization_name,
                            project_name,
                            limit,
                            state.as_deref(),
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create a VPC
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs`
    ///
    ///Arguments:
    /// - `organization_name`: The organization's unique name.
    /// - `project_name`: The project's unique name within the organization.
    /// - `body`
    pub async fn vpc_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::VpcCreate,
    ) -> Result<ResponseValue<types::Vpc>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a VPC
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}`
    pub async fn vpc_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Vpc>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
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

    ///Update a VPC
    ///
    ///Sends a `PUT` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}`
    pub async fn vpc_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        body: &'a types::VpcUpdate,
    ) -> Result<ResponseValue<types::Vpc>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
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

    ///Delete a VPC
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}`
    pub async fn vpc_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List firewall rules
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/firewall/rules`
    pub async fn vpc_firewall_rules_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
    ) -> Result<ResponseValue<types::VpcFirewallRules>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/firewall/rules",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
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

    ///Replace firewall rules
    ///
    ///Sends a `PUT` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/firewall/rules`
    pub async fn vpc_firewall_rules_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        body: &'a types::VpcFirewallRuleUpdateParams,
    ) -> Result<ResponseValue<types::VpcFirewallRules>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/firewall/rules",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
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

    ///List routers
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers`
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `vpc_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn vpc_router_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::VpcRouterResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List routers as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers` until there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `vpc_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn vpc_router_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::VpcRouter, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_router_list(
            organization_name,
            project_name,
            vpc_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.vpc_router_list(
                        organization_name,
                        project_name,
                        vpc_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    ///Create a router
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers`
    pub async fn vpc_router_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        body: &'a types::VpcRouterCreate,
    ) -> Result<ResponseValue<types::VpcRouter>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Get a router
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers/{router_name}`
    pub async fn vpc_router_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
    ) -> Result<ResponseValue<types::VpcRouter>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
        );
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

    ///Update a router
    ///
    ///Sends a `PUT` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers/{router_name}`
    pub async fn vpc_router_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        body: &'a types::VpcRouterUpdate,
    ) -> Result<ResponseValue<types::VpcRouter>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
        );
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

    ///Delete a router
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers/{router_name}`
    pub async fn vpc_router_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List routes
    ///
    ///List the routes associated with a router in a particular VPC.
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers/{router_name}/routes`
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `vpc_name`
    /// - `router_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn vpc_router_route_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::RouterRouteResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List routes as a Stream
    ///
    ///List the routes associated with a router in a particular VPC.
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers/{router_name}/routes` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `vpc_name`
    /// - `router_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn vpc_router_route_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::RouterRoute, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_router_route_list(
            organization_name,
            project_name,
            vpc_name,
            router_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.vpc_router_route_list(
                        organization_name,
                        project_name,
                        vpc_name,
                        router_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    ///Create a router
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers/{router_name}/routes`
    pub async fn vpc_router_route_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        body: &'a types::RouterRouteCreateParams,
    ) -> Result<ResponseValue<types::RouterRoute>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a route
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers/{router_name}/routes/{route_name}`
    pub async fn vpc_router_route_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        route_name: &'a types::Name,
    ) -> Result<ResponseValue<types::RouterRoute>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
            encode_path(&route_name.to_string()),
        );
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

    ///Update a route
    ///
    ///Sends a `PUT` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers/{router_name}/routes/{route_name}`
    pub async fn vpc_router_route_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        route_name: &'a types::Name,
        body: &'a types::RouterRouteUpdateParams,
    ) -> Result<ResponseValue<types::RouterRoute>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
            encode_path(&route_name.to_string()),
        );
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

    ///Delete a route
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/routers/{router_name}/routes/{route_name}`
    pub async fn vpc_router_route_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        router_name: &'a types::Name,
        route_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/routers/{}/routes/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&router_name.to_string()),
            encode_path(&route_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List subnets
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/subnets`
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `vpc_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn vpc_subnet_list<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::VpcSubnetResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List subnets as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/subnets` until there are no more results.
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `vpc_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn vpc_subnet_list_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::VpcSubnet, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_subnet_list(
            organization_name,
            project_name,
            vpc_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.vpc_subnet_list(
                        organization_name,
                        project_name,
                        vpc_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    ///Create a subnet
    ///
    ///Sends a `POST` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/subnets`
    pub async fn vpc_subnet_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        body: &'a types::VpcSubnetCreate,
    ) -> Result<ResponseValue<types::VpcSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a subnet
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/subnets/{subnet_name}`
    pub async fn vpc_subnet_view<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
    ) -> Result<ResponseValue<types::VpcSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&subnet_name.to_string()),
        );
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

    ///Update a subnet
    ///
    ///Sends a `PUT` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/subnets/{subnet_name}`
    pub async fn vpc_subnet_update<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
        body: &'a types::VpcSubnetUpdate,
    ) -> Result<ResponseValue<types::VpcSubnet>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&subnet_name.to_string()),
        );
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

    ///Delete a subnet
    ///
    ///Sends a `DELETE` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/subnets/{subnet_name}`
    pub async fn vpc_subnet_delete<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets/{}",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&subnet_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List network interfaces
    ///
    ///Sends a `GET` request to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/subnets/{subnet_name}/network-interfaces`
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `vpc_name`
    /// - `subnet_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn vpc_subnet_list_network_interfaces<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::NetworkInterfaceResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/organizations/{}/projects/{}/vpcs/{}/subnets/{}/network-interfaces",
            self.baseurl,
            encode_path(&organization_name.to_string()),
            encode_path(&project_name.to_string()),
            encode_path(&vpc_name.to_string()),
            encode_path(&subnet_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List network interfaces as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/organizations/{organization_name}/projects/{project_name}/vpcs/
    /// {vpc_name}/subnets/{subnet_name}/network-interfaces` until there are no
    /// more results.
    ///
    ///Arguments:
    /// - `organization_name`
    /// - `project_name`
    /// - `vpc_name`
    /// - `subnet_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn vpc_subnet_list_network_interfaces_stream<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        vpc_name: &'a types::Name,
        subnet_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::NetworkInterface, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.vpc_subnet_list_network_interfaces(
            organization_name,
            project_name,
            vpc_name,
            subnet_name,
            limit,
            None,
            sort_by,
        )
        .map_ok(move |page| {
            let page = page.into_inner();
            let first = futures::stream::iter(page.items).map(Ok);
            let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                if state.is_none() {
                    Ok(None)
                } else {
                    self.vpc_subnet_list_network_interfaces(
                        organization_name,
                        project_name,
                        vpc_name,
                        subnet_name,
                        limit,
                        state.as_deref(),
                        None,
                    )
                    .map_ok(|page| {
                        let page = page.into_inner();
                        Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                    })
                    .await
                }
            })
            .try_flatten();
            first.chain(rest)
        })
        .try_flatten_stream()
        .boxed()
    }

    ///Fetch the current silo's IAM policy
    ///
    ///Sends a `GET` request to `/policy`
    pub async fn policy_view<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::SiloRolePolicy>, Error<types::Error>> {
        let url = format!("{}/policy", self.baseurl,);
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

    ///Update the current silo's IAM policy
    ///
    ///Sends a `PUT` request to `/policy`
    pub async fn policy_update<'a>(
        &'a self,
        body: &'a types::SiloRolePolicy,
    ) -> Result<ResponseValue<types::SiloRolePolicy>, Error<types::Error>> {
        let url = format!("{}/policy", self.baseurl,);
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

    ///List built-in roles
    ///
    ///Sends a `GET` request to `/roles`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    pub async fn role_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::RoleResultsPage>, Error<types::Error>> {
        let url = format!("{}/roles", self.baseurl,);
        let mut query = Vec::with_capacity(2usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List built-in roles as a Stream
    ///
    ///Sends repeated `GET` requests to `/roles` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    pub fn role_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
    ) -> impl futures::Stream<Item = Result<types::Role, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.role_list(limit, None)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.role_list(limit, state.as_deref())
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Fetch a built-in role
    ///
    ///Sends a `GET` request to `/roles/{role_name}`
    ///
    ///Arguments:
    /// - `role_name`: The built-in role's unique name.
    pub async fn role_view<'a>(
        &'a self,
        role_name: &'a str,
    ) -> Result<ResponseValue<types::Role>, Error<types::Error>> {
        let url = format!(
            "{}/roles/{}",
            self.baseurl,
            encode_path(&role_name.to_string()),
        );
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

    ///Fetch the user associated with the current session
    ///
    ///Sends a `GET` request to `/session/me`
    pub async fn session_me<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::User>, Error<types::Error>> {
        let url = format!("{}/session/me", self.baseurl,);
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

    ///Fetch the silo groups the current user belongs to
    ///
    ///Sends a `GET` request to `/session/me/groups`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn session_me_groups<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::GroupResultsPage>, Error<types::Error>> {
        let url = format!("{}/session/me/groups", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Fetch the silo groups the current user belongs to as a Stream
    ///
    ///Sends repeated `GET` requests to `/session/me/groups` until there are no
    /// more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn session_me_groups_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Group, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.session_me_groups(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.session_me_groups(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///List SSH public keys
    ///
    ///Lists SSH public keys for the currently authenticated user.
    ///
    ///Sends a `GET` request to `/session/me/sshkeys`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn session_sshkey_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::SshKeyResultsPage>, Error<types::Error>> {
        let url = format!("{}/session/me/sshkeys", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List SSH public keys as a Stream
    ///
    ///Lists SSH public keys for the currently authenticated user.
    ///
    ///Sends repeated `GET` requests to `/session/me/sshkeys` until there are
    /// no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn session_sshkey_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::SshKey, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.session_sshkey_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.session_sshkey_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create an SSH public key
    ///
    ///Create an SSH public key for the currently authenticated user.
    ///
    ///Sends a `POST` request to `/session/me/sshkeys`
    pub async fn session_sshkey_create<'a>(
        &'a self,
        body: &'a types::SshKeyCreate,
    ) -> Result<ResponseValue<types::SshKey>, Error<types::Error>> {
        let url = format!("{}/session/me/sshkeys", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch an SSH public key
    ///
    ///Fetch an SSH public key associated with the currently authenticated
    /// user.
    ///
    ///Sends a `GET` request to `/session/me/sshkeys/{ssh_key_name}`
    pub async fn session_sshkey_view<'a>(
        &'a self,
        ssh_key_name: &'a types::Name,
    ) -> Result<ResponseValue<types::SshKey>, Error<types::Error>> {
        let url = format!(
            "{}/session/me/sshkeys/{}",
            self.baseurl,
            encode_path(&ssh_key_name.to_string()),
        );
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

    ///Delete an SSH public key
    ///
    ///Delete an SSH public key associated with the currently authenticated
    /// user.
    ///
    ///Sends a `DELETE` request to `/session/me/sshkeys/{ssh_key_name}`
    pub async fn session_sshkey_delete<'a>(
        &'a self,
        ssh_key_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/session/me/sshkeys/{}",
            self.baseurl,
            encode_path(&ssh_key_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///Fetch a system-wide image by id
    ///
    ///Sends a `GET` request to `/system/by-id/images/{id}`
    pub async fn system_image_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::GlobalImage>, Error<types::Error>> {
        let url = format!(
            "{}/system/by-id/images/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch an IP pool by id
    ///
    ///Sends a `GET` request to `/system/by-id/ip-pools/{id}`
    pub async fn ip_pool_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!(
            "{}/system/by-id/ip-pools/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Fetch a silo by id
    ///
    ///Sends a `GET` request to `/system/by-id/silos/{id}`
    pub async fn silo_view_by_id<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Silo>, Error<types::Error>> {
        let url = format!(
            "{}/system/by-id/silos/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///List system-wide certificates
    ///
    ///Returns a list of all the system-wide certificates. System-wide
    /// certificates are returned sorted by creation date, with the most recent
    /// certificates appearing first.
    ///
    ///Sends a `GET` request to `/system/certificates`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn certificate_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::CertificateResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/certificates", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List system-wide certificates as a Stream
    ///
    ///Returns a list of all the system-wide certificates. System-wide
    /// certificates are returned sorted by creation date, with the most recent
    /// certificates appearing first.
    ///
    ///Sends repeated `GET` requests to `/system/certificates` until there are
    /// no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn certificate_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Certificate, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.certificate_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.certificate_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create a new system-wide x.509 certificate
    ///
    ///This certificate is automatically used by the Oxide Control plane to
    /// serve external connections.
    ///
    ///Sends a `POST` request to `/system/certificates`
    pub async fn certificate_create<'a>(
        &'a self,
        body: &'a types::CertificateCreate,
    ) -> Result<ResponseValue<types::Certificate>, Error<types::Error>> {
        let url = format!("{}/system/certificates", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a certificate
    ///
    ///Returns the details of a specific certificate
    ///
    ///Sends a `GET` request to `/system/certificates/{certificate}`
    pub async fn certificate_view<'a>(
        &'a self,
        certificate: &'a types::NameOrId,
    ) -> Result<ResponseValue<types::Certificate>, Error<types::Error>> {
        let url = format!(
            "{}/system/certificates/{}",
            self.baseurl,
            encode_path(&certificate.to_string()),
        );
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

    ///Delete a certificate
    ///
    ///Permanently delete a certificate. This operation cannot be undone.
    ///
    ///Sends a `DELETE` request to `/system/certificates/{certificate}`
    pub async fn certificate_delete<'a>(
        &'a self,
        certificate: &'a types::NameOrId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/certificates/{}",
            self.baseurl,
            encode_path(&certificate.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List physical disks
    ///
    ///Sends a `GET` request to `/system/hardware/disks`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn physical_disk_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::PhysicalDiskResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/hardware/disks", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List physical disks as a Stream
    ///
    ///Sends repeated `GET` requests to `/system/hardware/disks` until there
    /// are no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn physical_disk_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::PhysicalDisk, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.physical_disk_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.physical_disk_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///List racks
    ///
    ///Sends a `GET` request to `/system/hardware/racks`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn rack_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::RackResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/hardware/racks", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List racks as a Stream
    ///
    ///Sends repeated `GET` requests to `/system/hardware/racks` until there
    /// are no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn rack_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Rack, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.rack_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.rack_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Fetch a rack
    ///
    ///Sends a `GET` request to `/system/hardware/racks/{rack_id}`
    ///
    ///Arguments:
    /// - `rack_id`: The rack's unique ID.
    pub async fn rack_view<'a>(
        &'a self,
        rack_id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Rack>, Error<types::Error>> {
        let url = format!(
            "{}/system/hardware/racks/{}",
            self.baseurl,
            encode_path(&rack_id.to_string()),
        );
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

    ///List sleds
    ///
    ///Sends a `GET` request to `/system/hardware/sleds`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn sled_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::SledResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/hardware/sleds", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List sleds as a Stream
    ///
    ///Sends repeated `GET` requests to `/system/hardware/sleds` until there
    /// are no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn sled_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Sled, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.sled_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.sled_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Fetch a sled
    ///
    ///Sends a `GET` request to `/system/hardware/sleds/{sled_id}`
    ///
    ///Arguments:
    /// - `sled_id`: The sled's unique ID.
    pub async fn sled_view<'a>(
        &'a self,
        sled_id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Sled>, Error<types::Error>> {
        let url = format!(
            "{}/system/hardware/sleds/{}",
            self.baseurl,
            encode_path(&sled_id.to_string()),
        );
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

    ///List physical disks attached to sleds
    ///
    ///Sends a `GET` request to `/system/hardware/sleds/{sled_id}/disks`
    ///
    ///Arguments:
    /// - `sled_id`: The sled's unique ID.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn sled_physical_disk_list<'a>(
        &'a self,
        sled_id: &'a uuid::Uuid,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::PhysicalDiskResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/hardware/sleds/{}/disks",
            self.baseurl,
            encode_path(&sled_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List physical disks attached to sleds as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/system/hardware/sleds/{sled_id}/disks` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `sled_id`: The sled's unique ID.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn sled_physical_disk_list_stream<'a>(
        &'a self,
        sled_id: &'a uuid::Uuid,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::PhysicalDisk, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.sled_physical_disk_list(sled_id, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.sled_physical_disk_list(sled_id, limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///List system-wide images
    ///
    ///Returns a list of all the system-wide images. System-wide images are
    /// returned sorted by creation date, with the most recent images appearing
    /// first.
    ///
    ///Sends a `GET` request to `/system/images`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn system_image_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::GlobalImageResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/images", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List system-wide images as a Stream
    ///
    ///Returns a list of all the system-wide images. System-wide images are
    /// returned sorted by creation date, with the most recent images appearing
    /// first.
    ///
    ///Sends repeated `GET` requests to `/system/images` until there are no
    /// more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn system_image_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::GlobalImage, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.system_image_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.system_image_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create a system-wide image
    ///
    ///Create a new system-wide image. This image can then be used by any user
    /// in any silo as a base for instances.
    ///
    ///Sends a `POST` request to `/system/images`
    pub async fn system_image_create<'a>(
        &'a self,
        body: &'a types::GlobalImageCreate,
    ) -> Result<ResponseValue<types::GlobalImage>, Error<types::Error>> {
        let url = format!("{}/system/images", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a system-wide image
    ///
    ///Returns the details of a specific system-wide image.
    ///
    ///Sends a `GET` request to `/system/images/{image_name}`
    pub async fn system_image_view<'a>(
        &'a self,
        image_name: &'a types::Name,
    ) -> Result<ResponseValue<types::GlobalImage>, Error<types::Error>> {
        let url = format!(
            "{}/system/images/{}",
            self.baseurl,
            encode_path(&image_name.to_string()),
        );
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

    ///Delete a system-wide image
    ///
    ///Permanently delete a system-wide image. This operation cannot be undone.
    /// Any instances using the system-wide image will continue to run, however
    /// new instances can not be created with this image.
    ///
    ///Sends a `DELETE` request to `/system/images/{image_name}`
    pub async fn system_image_delete<'a>(
        &'a self,
        image_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/images/{}",
            self.baseurl,
            encode_path(&image_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List IP pools
    ///
    ///Sends a `GET` request to `/system/ip-pools`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn ip_pool_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::IpPoolResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List IP pools as a Stream
    ///
    ///Sends repeated `GET` requests to `/system/ip-pools` until there are no
    /// more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn ip_pool_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::IpPool, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.ip_pool_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.ip_pool_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create an IP pool
    ///
    ///Sends a `POST` request to `/system/ip-pools`
    pub async fn ip_pool_create<'a>(
        &'a self,
        body: &'a types::IpPoolCreate,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch an IP pool
    ///
    ///Sends a `GET` request to `/system/ip-pools/{pool_name}`
    pub async fn ip_pool_view<'a>(
        &'a self,
        pool_name: &'a types::Name,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}",
            self.baseurl,
            encode_path(&pool_name.to_string()),
        );
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

    ///Update an IP Pool
    ///
    ///Sends a `PUT` request to `/system/ip-pools/{pool_name}`
    pub async fn ip_pool_update<'a>(
        &'a self,
        pool_name: &'a types::Name,
        body: &'a types::IpPoolUpdate,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}",
            self.baseurl,
            encode_path(&pool_name.to_string()),
        );
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

    ///Delete an IP Pool
    ///
    ///Sends a `DELETE` request to `/system/ip-pools/{pool_name}`
    pub async fn ip_pool_delete<'a>(
        &'a self,
        pool_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}",
            self.baseurl,
            encode_path(&pool_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List ranges for an IP pool
    ///
    ///Ranges are ordered by their first address.
    ///
    ///Sends a `GET` request to `/system/ip-pools/{pool_name}/ranges`
    ///
    ///Arguments:
    /// - `pool_name`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    pub async fn ip_pool_range_list<'a>(
        &'a self,
        pool_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::IpPoolRangeResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}/ranges",
            self.baseurl,
            encode_path(&pool_name.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List ranges for an IP pool as a Stream
    ///
    ///Ranges are ordered by their first address.
    ///
    ///Sends repeated `GET` requests to `/system/ip-pools/{pool_name}/ranges`
    /// until there are no more results.
    ///
    ///Arguments:
    /// - `pool_name`
    /// - `limit`: Maximum number of items returned by a single call
    pub fn ip_pool_range_list_stream<'a>(
        &'a self,
        pool_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
    ) -> impl futures::Stream<Item = Result<types::IpPoolRange, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.ip_pool_range_list(pool_name, limit, None)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.ip_pool_range_list(pool_name, limit, state.as_deref())
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Add a range to an IP pool
    ///
    ///Sends a `POST` request to `/system/ip-pools/{pool_name}/ranges/add`
    pub async fn ip_pool_range_add<'a>(
        &'a self,
        pool_name: &'a types::Name,
        body: &'a types::IpRange,
    ) -> Result<ResponseValue<types::IpPoolRange>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}/ranges/add",
            self.baseurl,
            encode_path(&pool_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Remove a range from an IP pool
    ///
    ///Sends a `POST` request to `/system/ip-pools/{pool_name}/ranges/remove`
    pub async fn ip_pool_range_remove<'a>(
        &'a self,
        pool_name: &'a types::Name,
        body: &'a types::IpRange,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/ip-pools/{}/ranges/remove",
            self.baseurl,
            encode_path(&pool_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch the IP pool used for Oxide services
    ///
    ///Sends a `GET` request to `/system/ip-pools-service`
    pub async fn ip_pool_service_view<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::IpPool>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools-service", self.baseurl,);
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

    ///List ranges for the IP pool used for Oxide services
    ///
    ///Ranges are ordered by their first address.
    ///
    ///Sends a `GET` request to `/system/ip-pools-service/ranges`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    pub async fn ip_pool_service_range_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::IpPoolRangeResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools-service/ranges", self.baseurl,);
        let mut query = Vec::with_capacity(2usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List ranges for the IP pool used for Oxide services as a Stream
    ///
    ///Ranges are ordered by their first address.
    ///
    ///Sends repeated `GET` requests to `/system/ip-pools-service/ranges` until
    /// there are no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    pub fn ip_pool_service_range_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
    ) -> impl futures::Stream<Item = Result<types::IpPoolRange, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.ip_pool_service_range_list(limit, None)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.ip_pool_service_range_list(limit, state.as_deref())
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Add a range to an IP pool used for Oxide services
    ///
    ///Sends a `POST` request to `/system/ip-pools-service/ranges/add`
    pub async fn ip_pool_service_range_add<'a>(
        &'a self,
        body: &'a types::IpRange,
    ) -> Result<ResponseValue<types::IpPoolRange>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools-service/ranges/add", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Remove a range from an IP pool used for Oxide services
    ///
    ///Sends a `POST` request to `/system/ip-pools-service/ranges/remove`
    pub async fn ip_pool_service_range_remove<'a>(
        &'a self,
        body: &'a types::IpRange,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/system/ip-pools-service/ranges/remove", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Access metrics data
    ///
    ///Sends a `GET` request to `/system/metrics/{metric_name}`
    ///
    ///Arguments:
    /// - `metric_name`
    /// - `end_time`: An exclusive end time of metrics.
    /// - `id`: The UUID of the container being queried
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `start_time`: An inclusive start time of metrics.
    pub async fn system_metric<'a>(
        &'a self,
        metric_name: types::SystemMetricName,
        end_time: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        id: &'a uuid::Uuid,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        start_time: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
    ) -> Result<ResponseValue<types::MeasurementResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/metrics/{}",
            self.baseurl,
            encode_path(&metric_name.to_string()),
        );
        let mut query = Vec::with_capacity(5usize);
        if let Some(end_time) = &end_time {
            query.push(("end_time", end_time.to_string()));
        }

        query.push(("id", id.to_string()));
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(start_time) = &start_time {
            query.push(("start_time", start_time.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Fetch the top-level IAM policy
    ///
    ///Sends a `GET` request to `/system/policy`
    pub async fn system_policy_view<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::FleetRolePolicy>, Error<types::Error>> {
        let url = format!("{}/system/policy", self.baseurl,);
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

    ///Update the top-level IAM policy
    ///
    ///Sends a `PUT` request to `/system/policy`
    pub async fn system_policy_update<'a>(
        &'a self,
        body: &'a types::FleetRolePolicy,
    ) -> Result<ResponseValue<types::FleetRolePolicy>, Error<types::Error>> {
        let url = format!("{}/system/policy", self.baseurl,);
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

    ///List sagas
    ///
    ///Sends a `GET` request to `/system/sagas`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn saga_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::SagaResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/sagas", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List sagas as a Stream
    ///
    ///Sends repeated `GET` requests to `/system/sagas` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn saga_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Saga, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.saga_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.saga_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Fetch a saga
    ///
    ///Sends a `GET` request to `/system/sagas/{saga_id}`
    pub async fn saga_view<'a>(
        &'a self,
        saga_id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::Saga>, Error<types::Error>> {
        let url = format!(
            "{}/system/sagas/{}",
            self.baseurl,
            encode_path(&saga_id.to_string()),
        );
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

    ///List silos
    ///
    ///Lists silos that are discoverable based on the current permissions.
    ///
    ///Sends a `GET` request to `/system/silos`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn silo_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::SiloResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/silos", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List silos as a Stream
    ///
    ///Lists silos that are discoverable based on the current permissions.
    ///
    ///Sends repeated `GET` requests to `/system/silos` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn silo_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Silo, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.silo_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.silo_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create a silo
    ///
    ///Sends a `POST` request to `/system/silos`
    pub async fn silo_create<'a>(
        &'a self,
        body: &'a types::SiloCreate,
    ) -> Result<ResponseValue<types::Silo>, Error<types::Error>> {
        let url = format!("{}/system/silos", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a silo
    ///
    ///Fetch a silo by name.
    ///
    ///Sends a `GET` request to `/system/silos/{silo_name}`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    pub async fn silo_view<'a>(
        &'a self,
        silo_name: &'a types::Name,
    ) -> Result<ResponseValue<types::Silo>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
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

    ///Delete a silo
    ///
    ///Delete a silo by name.
    ///
    ///Sends a `DELETE` request to `/system/silos/{silo_name}`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    pub async fn silo_delete<'a>(
        &'a self,
        silo_name: &'a types::Name,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///List a silo's IDPs
    ///
    ///Sends a `GET` request to `/system/silos/{silo_name}/identity-providers`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn silo_identity_provider_list<'a>(
        &'a self,
        silo_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::IdentityProviderResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List a silo's IDPs as a Stream
    ///
    ///Sends repeated `GET` requests to
    /// `/system/silos/{silo_name}/identity-providers` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn silo_identity_provider_list_stream<'a>(
        &'a self,
        silo_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::IdentityProvider, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.silo_identity_provider_list(silo_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.silo_identity_provider_list(silo_name, limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create a user
    ///
    ///Users can only be created in Silos with `provision_type` == `Fixed`.
    /// Otherwise, Silo users are just-in-time (JIT) provisioned when a user
    /// first logs in using an external Identity Provider.
    ///
    ///Sends a `POST` request to
    /// `/system/silos/{silo_name}/identity-providers/local/users`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `body`
    pub async fn local_idp_user_create<'a>(
        &'a self,
        silo_name: &'a types::Name,
        body: &'a types::UserCreate,
    ) -> Result<ResponseValue<types::User>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/local/users",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Delete a user
    ///
    ///Sends a `DELETE` request to
    /// `/system/silos/{silo_name}/identity-providers/local/users/{user_id}`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `user_id`: The user's internal id
    pub async fn local_idp_user_delete<'a>(
        &'a self,
        silo_name: &'a types::Name,
        user_id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/local/users/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///Set or invalidate a user's password
    ///
    ///Passwords can only be updated for users in Silos with identity mode
    /// `LocalOnly`.
    ///
    ///Sends a `POST` request to
    /// `/system/silos/{silo_name}/identity-providers/local/users/{user_id}/
    /// set-password`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `user_id`: The user's internal id
    /// - `body`
    pub async fn local_idp_user_set_password<'a>(
        &'a self,
        silo_name: &'a types::Name,
        user_id: &'a uuid::Uuid,
        body: &'a types::UserPassword,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/local/users/{}/set-password",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Create a SAML IDP
    ///
    ///Sends a `POST` request to
    /// `/system/silos/{silo_name}/identity-providers/saml`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `body`
    pub async fn saml_identity_provider_create<'a>(
        &'a self,
        silo_name: &'a types::Name,
        body: &'a types::SamlIdentityProviderCreate,
    ) -> Result<ResponseValue<types::SamlIdentityProvider>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/saml",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch a SAML IDP
    ///
    ///Sends a `GET` request to
    /// `/system/silos/{silo_name}/identity-providers/saml/{provider_name}`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `provider_name`: The SAML identity provider's name
    pub async fn saml_identity_provider_view<'a>(
        &'a self,
        silo_name: &'a types::Name,
        provider_name: &'a types::Name,
    ) -> Result<ResponseValue<types::SamlIdentityProvider>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/identity-providers/saml/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&provider_name.to_string()),
        );
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

    ///Fetch a silo's IAM policy
    ///
    ///Sends a `GET` request to `/system/silos/{silo_name}/policy`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    pub async fn silo_policy_view<'a>(
        &'a self,
        silo_name: &'a types::Name,
    ) -> Result<ResponseValue<types::SiloRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/policy",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
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

    ///Update a silo's IAM policy
    ///
    ///Sends a `PUT` request to `/system/silos/{silo_name}/policy`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `body`
    pub async fn silo_policy_update<'a>(
        &'a self,
        silo_name: &'a types::Name,
        body: &'a types::SiloRolePolicy,
    ) -> Result<ResponseValue<types::SiloRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/policy",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
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

    ///List users in a silo
    ///
    ///Sends a `GET` request to `/system/silos/{silo_name}/users/all`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn silo_users_list<'a>(
        &'a self,
        silo_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::UserResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/users/all",
            self.baseurl,
            encode_path(&silo_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List users in a silo as a Stream
    ///
    ///Sends repeated `GET` requests to `/system/silos/{silo_name}/users/all`
    /// until there are no more results.
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn silo_users_list_stream<'a>(
        &'a self,
        silo_name: &'a types::Name,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::User, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.silo_users_list(silo_name, limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.silo_users_list(silo_name, limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Fetch a user
    ///
    ///Sends a `GET` request to `/system/silos/{silo_name}/users/id/{user_id}`
    ///
    ///Arguments:
    /// - `silo_name`: The silo's unique name.
    /// - `user_id`: The user's internal id
    pub async fn silo_user_view<'a>(
        &'a self,
        silo_name: &'a types::Name,
        user_id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::User>, Error<types::Error>> {
        let url = format!(
            "{}/system/silos/{}/users/id/{}",
            self.baseurl,
            encode_path(&silo_name.to_string()),
            encode_path(&user_id.to_string()),
        );
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

    ///List built-in users
    ///
    ///Sends a `GET` request to `/system/user`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn system_user_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameSortMode>,
    ) -> Result<ResponseValue<types::UserBuiltinResultsPage>, Error<types::Error>> {
        let url = format!("{}/system/user", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List built-in users as a Stream
    ///
    ///Sends repeated `GET` requests to `/system/user` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn system_user_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameSortMode>,
    ) -> impl futures::Stream<Item = Result<types::UserBuiltin, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.system_user_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.system_user_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Fetch a built-in user
    ///
    ///Sends a `GET` request to `/system/user/{user_name}`
    ///
    ///Arguments:
    /// - `user_name`: The built-in user's unique name.
    pub async fn system_user_view<'a>(
        &'a self,
        user_name: &'a types::Name,
    ) -> Result<ResponseValue<types::UserBuiltin>, Error<types::Error>> {
        let url = format!(
            "{}/system/user/{}",
            self.baseurl,
            encode_path(&user_name.to_string()),
        );
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

    ///List timeseries schema
    ///
    ///Sends a `GET` request to `/timeseries/schema`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    pub async fn timeseries_schema_get<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
    ) -> Result<ResponseValue<types::TimeseriesSchemaResultsPage>, Error<types::Error>> {
        let url = format!("{}/timeseries/schema", self.baseurl,);
        let mut query = Vec::with_capacity(2usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List timeseries schema as a Stream
    ///
    ///Sends repeated `GET` requests to `/timeseries/schema` until there are no
    /// more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    pub fn timeseries_schema_get_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
    ) -> impl futures::Stream<Item = Result<types::TimeseriesSchema, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.timeseries_schema_get(limit, None)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.timeseries_schema_get(limit, state.as_deref())
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///List users
    ///
    ///Sends a `GET` request to `/users`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn user_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::UserResultsPage>, Error<types::Error>> {
        let url = format!("{}/users", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List users as a Stream
    ///
    ///Sends repeated `GET` requests to `/users` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn user_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::User, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.user_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.user_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///List disks
    ///
    ///Sends a `GET` request to `/v1/disks`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `organization`
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`
    /// - `sort_by`
    pub async fn disk_list_v1<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        page_token: Option<&'a str>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::DiskResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/disks", self.baseurl,);
        let mut query = Vec::with_capacity(5usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List disks as a Stream
    ///
    ///Sends repeated `GET` requests to `/v1/disks` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `organization`
    /// - `project`
    /// - `sort_by`
    pub fn disk_list_v1_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Disk, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.disk_list_v1(limit, organization, None, project, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.disk_list_v1(limit, None, state.as_deref(), None, None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create a disk
    ///
    ///Sends a `POST` request to `/v1/disks`
    pub async fn disk_create_v1<'a>(
        &'a self,
        organization: Option<&'a types::NameOrId>,
        project: &'a types::NameOrId,
        body: &'a types::DiskCreate,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!("{}/v1/disks", self.baseurl,);
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        query.push(("project", project.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
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

    ///Fetch a disk
    ///
    ///Sends a `GET` request to `/v1/disks/{disk}`
    pub async fn disk_view_v1<'a>(
        &'a self,
        disk: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/v1/disks/{}",
            self.baseurl,
            encode_path(&disk.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Delete a disk
    ///
    ///Sends a `DELETE` request to `/v1/disks/{disk}`
    pub async fn disk_delete_v1<'a>(
        &'a self,
        disk: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/v1/disks/{}",
            self.baseurl,
            encode_path(&disk.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List instances
    ///
    ///Sends a `GET` request to `/v1/instances`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `organization`
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`
    /// - `sort_by`
    pub async fn instance_list_v1<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        page_token: Option<&'a str>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::InstanceResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/instances", self.baseurl,);
        let mut query = Vec::with_capacity(5usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List instances as a Stream
    ///
    ///Sends repeated `GET` requests to `/v1/instances` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `organization`
    /// - `project`
    /// - `sort_by`
    pub fn instance_list_v1_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Instance, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_list_v1(limit, organization, None, project, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.instance_list_v1(limit, None, state.as_deref(), None, None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create an instance
    ///
    ///Sends a `POST` request to `/v1/instances`
    pub async fn instance_create_v1<'a>(
        &'a self,
        organization: Option<&'a types::NameOrId>,
        project: &'a types::NameOrId,
        body: &'a types::InstanceCreate,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!("{}/v1/instances", self.baseurl,);
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        query.push(("project", project.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
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

    ///Fetch an instance
    ///
    ///Sends a `GET` request to `/v1/instances/{instance}`
    pub async fn instance_view_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Delete an instance
    ///
    ///Sends a `DELETE` request to `/v1/instances/{instance}`
    pub async fn instance_delete_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List an instance's disks
    ///
    ///Sends a `GET` request to `/v1/instances/{instance}/disks`
    ///
    ///Arguments:
    /// - `instance`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `organization`
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `project`
    /// - `sort_by`
    pub async fn instance_disk_list_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        limit: Option<std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        page_token: Option<&'a str>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::DiskResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/disks",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(5usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List an instance's disks as a Stream
    ///
    ///Sends repeated `GET` requests to `/v1/instances/{instance}/disks` until
    /// there are no more results.
    ///
    ///Arguments:
    /// - `instance`
    /// - `limit`: Maximum number of items returned by a single call
    /// - `organization`
    /// - `project`
    /// - `sort_by`
    pub fn instance_disk_list_v1_stream<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        limit: Option<std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Disk, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.instance_disk_list_v1(instance, limit, organization, None, project, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.instance_disk_list_v1(
                            instance,
                            limit,
                            None,
                            state.as_deref(),
                            None,
                            None,
                        )
                        .map_ok(|page| {
                            let page = page.into_inner();
                            Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                        })
                        .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Attach a disk to an instance
    ///
    ///Sends a `POST` request to `/v1/instances/{instance}/disks/attach`
    pub async fn instance_disk_attach_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        body: &'a types::DiskPath,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/disks/attach",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Detach a disk from an instance
    ///
    ///Sends a `POST` request to `/v1/instances/{instance}/disks/detach`
    pub async fn instance_disk_detach_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        body: &'a types::DiskPath,
    ) -> Result<ResponseValue<types::Disk>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/disks/detach",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Migrate an instance
    ///
    ///Sends a `POST` request to `/v1/instances/{instance}/migrate`
    pub async fn instance_migrate_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
        body: &'a types::InstanceMigrate,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/migrate",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
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

    ///Reboot an instance
    ///
    ///Sends a `POST` request to `/v1/instances/{instance}/reboot`
    pub async fn instance_reboot_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/reboot",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Fetch an instance's serial console
    ///
    ///Sends a `GET` request to `/v1/instances/{instance}/serial-console`
    ///
    ///Arguments:
    /// - `instance`
    /// - `from_start`: Character index in the serial buffer from which to read,
    ///   counting the bytes output since instance start. If this is not
    ///   provided, `most_recent` must be provided, and if this *is* provided,
    ///   `most_recent` must *not* be provided.
    /// - `max_bytes`: Maximum number of bytes of buffered serial console
    ///   contents to return. If the requested range runs to the end of the
    ///   available buffer, the data returned will be shorter than `max_bytes`.
    /// - `most_recent`: Character index in the serial buffer from which to
    ///   read, counting *backward* from the most recently buffered data
    ///   retrieved from the instance. (See note on `from_start` about mutual
    ///   exclusivity)
    /// - `organization`
    /// - `project`
    pub async fn instance_serial_console_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        from_start: Option<u64>,
        max_bytes: Option<u64>,
        most_recent: Option<u64>,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::InstanceSerialConsoleData>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/serial-console",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(5usize);
        if let Some(from_start) = &from_start {
            query.push(("from_start", from_start.to_string()));
        }

        if let Some(max_bytes) = &max_bytes {
            query.push(("max_bytes", max_bytes.to_string()));
        }

        if let Some(most_recent) = &most_recent {
            query.push(("most_recent", most_recent.to_string()));
        }

        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Stream an instance's serial console
    ///
    ///Sends a `GET` request to
    /// `/v1/instances/{instance}/serial-console/stream`
    pub async fn instance_serial_console_stream_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<reqwest::Upgraded>, Error<reqwest::Upgraded>> {
        let url = format!(
            "{}/v1/instances/{}/serial-console/stream",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .query(&query)
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

    ///Boot an instance
    ///
    ///Sends a `POST` request to `/v1/instances/{instance}/start`
    pub async fn instance_start_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/start",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stop an instance
    ///
    ///Sends a `POST` request to `/v1/instances/{instance}/stop`
    pub async fn instance_stop_v1<'a>(
        &'a self,
        instance: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        project: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        let url = format!(
            "{}/v1/instances/{}/stop",
            self.baseurl,
            encode_path(&instance.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(project) = &project {
            query.push(("project", project.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List organizations
    ///
    ///Sends a `GET` request to `/v1/organizations`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn organization_list_v1<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::OrganizationResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/organizations", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List organizations as a Stream
    ///
    ///Sends repeated `GET` requests to `/v1/organizations` until there are no
    /// more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn organization_list_v1_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Organization, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.organization_list_v1(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.organization_list_v1(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create an organization
    ///
    ///Sends a `POST` request to `/v1/organizations`
    pub async fn organization_create_v1<'a>(
        &'a self,
        body: &'a types::OrganizationCreate,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!("{}/v1/organizations", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
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

    ///Fetch an organization
    ///
    ///Sends a `GET` request to `/v1/organizations/{organization}`
    pub async fn organization_view_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}",
            self.baseurl,
            encode_path(&organization.to_string()),
        );
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

    ///Update an organization
    ///
    ///Sends a `PUT` request to `/v1/organizations/{organization}`
    pub async fn organization_update_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
        body: &'a types::OrganizationUpdate,
    ) -> Result<ResponseValue<types::Organization>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}",
            self.baseurl,
            encode_path(&organization.to_string()),
        );
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

    ///Delete an organization
    ///
    ///Sends a `DELETE` request to `/v1/organizations/{organization}`
    pub async fn organization_delete_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}",
            self.baseurl,
            encode_path(&organization.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
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

    ///Fetch an organization's IAM policy
    ///
    ///Sends a `GET` request to `/v1/organizations/{organization}/policy`
    pub async fn organization_policy_view_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
    ) -> Result<ResponseValue<types::OrganizationRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}/policy",
            self.baseurl,
            encode_path(&organization.to_string()),
        );
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

    ///Update an organization's IAM policy
    ///
    ///Sends a `PUT` request to `/v1/organizations/{organization}/policy`
    pub async fn organization_policy_update_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
        body: &'a types::OrganizationRolePolicy,
    ) -> Result<ResponseValue<types::OrganizationRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/v1/organizations/{}/policy",
            self.baseurl,
            encode_path(&organization.to_string()),
        );
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

    ///List projects
    ///
    ///Sends a `GET` request to `/v1/projects`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `organization`
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn project_list_v1<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        page_token: Option<&'a str>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> Result<ResponseValue<types::ProjectResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/projects", self.baseurl,);
        let mut query = Vec::with_capacity(4usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List projects as a Stream
    ///
    ///Sends repeated `GET` requests to `/v1/projects` until there are no more
    /// results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `organization`
    /// - `sort_by`
    pub fn project_list_v1_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        organization: Option<&'a types::NameOrId>,
        sort_by: Option<types::NameOrIdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::Project, Error<types::Error>>> + Unpin + '_ {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.project_list_v1(limit, organization, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.project_list_v1(limit, None, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Create a project
    ///
    ///Sends a `POST` request to `/v1/projects`
    pub async fn project_create_v1<'a>(
        &'a self,
        organization: &'a types::NameOrId,
        body: &'a types::ProjectCreate,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!("{}/v1/projects", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        query.push(("organization", organization.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
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

    ///Fetch a project
    ///
    ///Sends a `GET` request to `/v1/projects/{project}`
    pub async fn project_view_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}",
            self.baseurl,
            encode_path(&project.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Update a project
    ///
    ///Sends a `PUT` request to `/v1/projects/{project}`
    pub async fn project_update_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        body: &'a types::ProjectUpdate,
    ) -> Result<ResponseValue<types::Project>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}",
            self.baseurl,
            encode_path(&project.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
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

    ///Delete a project
    ///
    ///Sends a `DELETE` request to `/v1/projects/{project}`
    pub async fn project_delete_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}",
            self.baseurl,
            encode_path(&project.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Fetch a project's IAM policy
    ///
    ///Sends a `GET` request to `/v1/projects/{project}/policy`
    pub async fn project_policy_view_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
    ) -> Result<ResponseValue<types::ProjectRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}/policy",
            self.baseurl,
            encode_path(&project.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///Update a project's IAM policy
    ///
    ///Sends a `PUT` request to `/v1/projects/{project}/policy`
    pub async fn project_policy_update_v1<'a>(
        &'a self,
        project: &'a types::NameOrId,
        organization: Option<&'a types::NameOrId>,
        body: &'a types::ProjectRolePolicy,
    ) -> Result<ResponseValue<types::ProjectRolePolicy>, Error<types::Error>> {
        let url = format!(
            "{}/v1/projects/{}/policy",
            self.baseurl,
            encode_path(&project.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(organization) = &organization {
            query.push(("organization", organization.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
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

    ///View version and update status of component tree
    ///
    ///Sends a `GET` request to `/v1/system/update/components`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn system_component_version_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::UpdateableComponentResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/components", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///View version and update status of component tree as a Stream
    ///
    ///Sends repeated `GET` requests to `/v1/system/update/components` until
    /// there are no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn system_component_version_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::UpdateableComponent, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.system_component_version_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.system_component_version_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///List all update deployments
    ///
    ///Sends a `GET` request to `/v1/system/update/deployments`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn update_deployments_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::UpdateDeploymentResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/deployments", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List all update deployments as a Stream
    ///
    ///Sends repeated `GET` requests to `/v1/system/update/deployments` until
    /// there are no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn update_deployments_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::UpdateDeployment, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.update_deployments_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.update_deployments_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///Fetch a system update deployment
    ///
    ///Sends a `GET` request to `/v1/system/update/deployments/{id}`
    pub async fn update_deployment_view<'a>(
        &'a self,
        id: &'a uuid::Uuid,
    ) -> Result<ResponseValue<types::UpdateDeployment>, Error<types::Error>> {
        let url = format!(
            "{}/v1/system/update/deployments/{}",
            self.baseurl,
            encode_path(&id.to_string()),
        );
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

    ///Refresh update data
    ///
    ///Sends a `POST` request to `/v1/system/update/refresh`
    pub async fn system_update_refresh<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/refresh", self.baseurl,);
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

    ///Start system update
    ///
    ///Sends a `POST` request to `/v1/system/update/start`
    pub async fn system_update_start<'a>(
        &'a self,
        body: &'a types::SystemUpdateStart,
    ) -> Result<ResponseValue<types::UpdateDeployment>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/start", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => ResponseValue::from_response(response).await,
            400u16..=499u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            500u16..=599u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Stop system update
    ///
    ///If there is no update in progress, do nothing.
    ///
    ///Sends a `POST` request to `/v1/system/update/stop`
    pub async fn system_update_stop<'a>(
        &'a self,
    ) -> Result<ResponseValue<()>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/stop", self.baseurl,);
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

    ///List all updates
    ///
    ///Sends a `GET` request to `/v1/system/update/updates`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort_by`
    pub async fn system_update_list<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        page_token: Option<&'a str>,
        sort_by: Option<types::IdSortMode>,
    ) -> Result<ResponseValue<types::SystemUpdateResultsPage>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/updates", self.baseurl,);
        let mut query = Vec::with_capacity(3usize);
        if let Some(limit) = &limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(page_token) = &page_token {
            query.push(("page_token", page_token.to_string()));
        }

        if let Some(sort_by) = &sort_by {
            query.push(("sort_by", sort_by.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
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

    ///List all updates as a Stream
    ///
    ///Sends repeated `GET` requests to `/v1/system/update/updates` until there
    /// are no more results.
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `sort_by`
    pub fn system_update_list_stream<'a>(
        &'a self,
        limit: Option<std::num::NonZeroU32>,
        sort_by: Option<types::IdSortMode>,
    ) -> impl futures::Stream<Item = Result<types::SystemUpdate, Error<types::Error>>> + Unpin + '_
    {
        use futures::StreamExt;
        use futures::TryFutureExt;
        use futures::TryStreamExt;
        self.system_update_list(limit, None, sort_by)
            .map_ok(move |page| {
                let page = page.into_inner();
                let first = futures::stream::iter(page.items).map(Ok);
                let rest = futures::stream::try_unfold(page.next_page, move |state| async move {
                    if state.is_none() {
                        Ok(None)
                    } else {
                        self.system_update_list(limit, state.as_deref(), None)
                            .map_ok(|page| {
                                let page = page.into_inner();
                                Some((futures::stream::iter(page.items).map(Ok), page.next_page))
                            })
                            .await
                    }
                })
                .try_flatten();
                first.chain(rest)
            })
            .try_flatten_stream()
            .boxed()
    }

    ///View system update
    ///
    ///Sends a `GET` request to `/v1/system/update/updates/{version}`
    pub async fn system_update_view<'a>(
        &'a self,
        version: &'a types::SemverVersion,
    ) -> Result<ResponseValue<types::SystemUpdate>, Error<types::Error>> {
        let url = format!(
            "{}/v1/system/update/updates/{}",
            self.baseurl,
            encode_path(&version.to_string()),
        );
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

    ///View system update component tree
    ///
    ///Sends a `GET` request to
    /// `/v1/system/update/updates/{version}/components`
    pub async fn system_update_components_list<'a>(
        &'a self,
        version: &'a types::SemverVersion,
    ) -> Result<ResponseValue<types::ComponentUpdateResultsPage>, Error<types::Error>> {
        let url = format!(
            "{}/v1/system/update/updates/{}/components",
            self.baseurl,
            encode_path(&version.to_string()),
        );
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

    ///View system version and update status
    ///
    ///Sends a `GET` request to `/v1/system/update/version`
    pub async fn system_version<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::SystemVersion>, Error<types::Error>> {
        let url = format!("{}/v1/system/update/version", self.baseurl,);
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
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
