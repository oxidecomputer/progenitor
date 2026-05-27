// Copyright 2025 Oxide Computer Company

#![allow(dead_code)]

//! Support code for generated clients.

use std::ops::{Deref, DerefMut};

use bytes::Bytes;
use futures_core::Stream;
use reqwest::RequestBuilder;
use serde::{Serialize, de::DeserializeOwned, ser::SerializeStruct};

#[cfg(feature = "cli")]
use schemars::schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec};

#[cfg(not(target_arch = "wasm32"))]
type InnerByteStream = std::pin::Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>> + Send + Sync>>;

#[cfg(target_arch = "wasm32")]
type InnerByteStream = std::pin::Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>>>>;

/// Untyped byte stream used for both success and error responses.
pub struct ByteStream(InnerByteStream);

impl ByteStream {
    /// Creates a new ByteStream
    ///
    /// Useful for generating test fixtures.
    pub fn new(inner: InnerByteStream) -> Self {
        Self(inner)
    }

    /// Consumes the [`ByteStream`] and return its inner [`Stream`].
    pub fn into_inner(self) -> InnerByteStream {
        self.0
    }
}

impl Deref for ByteStream {
    type Target = InnerByteStream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Interface for which an implementation is generated for all clients.
pub trait ClientInfo<Inner> {
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI document and may
    /// be in any format the API selects.
    fn api_version() -> &'static str;

    /// Get the base URL to which requests are made.
    fn baseurl(&self) -> &str;

    /// Get the internal `reqwest::Client` used to make requests.
    fn client(&self) -> &reqwest::Client;

    /// Get the inner value of type `T` if one is specified.
    fn inner(&self) -> &Inner;
}

impl<T, Inner> ClientInfo<Inner> for &T
where
    T: ClientInfo<Inner>,
{
    fn api_version() -> &'static str {
        T::api_version()
    }

    fn baseurl(&self) -> &str {
        (*self).baseurl()
    }

    fn client(&self) -> &reqwest::Client {
        (*self).client()
    }

    fn inner(&self) -> &Inner {
        (*self).inner()
    }
}

/// Information about an operation, consumed by hook implementations.
pub struct OperationInfo {
    /// The corresponding operationId from the source OpenAPI document.
    pub operation_id: &'static str,
}

/// Interface for changing the behavior of generated clients. All clients
/// implement this for `&Client`; to override the default behavior, implement
/// some or all of the interfaces for the `Client` type (without the
/// reference). This mechanism relies on so-called "auto-ref specialization".
#[allow(async_fn_in_trait, unused)]
pub trait ClientHooks<Inner = ()>
where
    Self: ClientInfo<Inner>,
{
    /// Runs prior to the execution of the request. This may be used to modify
    /// the request before it is transmitted.
    async fn pre<E>(
        &self,
        request: &mut reqwest::Request,
        info: &OperationInfo,
    ) -> std::result::Result<(), Error<E>> {
        Ok(())
    }

    /// Runs after completion of the request.
    async fn post<E>(
        &self,
        result: &reqwest::Result<reqwest::Response>,
        info: &OperationInfo,
    ) -> std::result::Result<(), Error<E>> {
        Ok(())
    }

    /// Execute the request. Note that for almost any reasonable implementation
    /// this will include code equivalent to this:
    /// ```
    /// # use progenitor_client::{ClientHooks, ClientInfo, OperationInfo};
    /// # struct X;
    /// # impl ClientInfo<()> for X {
    /// #   fn api_version() -> &'static str { panic!() }
    /// #   fn baseurl(&self) -> &str { panic!() }
    /// #   fn client(&self) -> &reqwest::Client { panic!() }
    /// #   fn inner(&self) -> &() { panic!() }
    /// # }
    /// # impl ClientHooks for X {
    /// #   async fn exec(
    /// #       &self,
    /// #       request: reqwest::Request,
    /// #       info: &OperationInfo,
    /// #   ) -> reqwest::Result<reqwest::Response> {
    ///         self.client().execute(request).await
    /// #   }
    /// # }
    /// ```
    async fn exec(
        &self,
        request: reqwest::Request,
        info: &OperationInfo,
    ) -> reqwest::Result<reqwest::Response> {
        self.client().execute(request).await
    }
}

/// Typed value returned by generated client methods.
///
/// This is used for successful responses and may appear in error responses
/// generated from the server (see [`Error::ErrorResponse`])
pub struct ResponseValue<T> {
    inner: T,
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    // TODO cookies?
}

impl<T: DeserializeOwned> ResponseValue<T> {
    #[doc(hidden)]
    pub async fn from_response<E>(response: reqwest::Response) -> Result<Self, Error<E>> {
        let status = response.status();
        let headers = response.headers().clone();
        let full = response.bytes().await.map_err(Error::ResponseBodyError)?;
        let inner =
            serde_json::from_slice(&full).map_err(|e| Error::InvalidResponsePayload(full, e))?;

        Ok(Self {
            inner,
            status,
            headers,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ResponseValue<reqwest::Upgraded> {
    #[doc(hidden)]
    pub async fn upgrade<E: std::fmt::Debug>(
        response: reqwest::Response,
    ) -> Result<Self, Error<E>> {
        let status = response.status();
        let headers = response.headers().clone();
        if status == reqwest::StatusCode::SWITCHING_PROTOCOLS {
            let inner = response.upgrade().await.map_err(Error::InvalidUpgrade)?;

            Ok(Self {
                inner,
                status,
                headers,
            })
        } else {
            Err(Error::UnexpectedResponse(response))
        }
    }
}

impl ResponseValue<ByteStream> {
    #[doc(hidden)]
    pub fn stream(response: reqwest::Response) -> Self {
        let status = response.status();
        let headers = response.headers().clone();
        Self {
            inner: ByteStream(Box::pin(response.bytes_stream())),
            status,
            headers,
        }
    }
}

impl ResponseValue<()> {
    #[doc(hidden)]
    pub fn empty(response: reqwest::Response) -> Self {
        let status = response.status();
        let headers = response.headers().clone();
        // TODO is there anything we want to do to confirm that there is no
        // content?
        Self {
            inner: (),
            status,
            headers,
        }
    }
}

impl<T> ResponseValue<T> {
    /// Creates a [`ResponseValue`] from the inner type, status, and headers.
    ///
    /// Useful for generating test fixtures.
    pub fn new(inner: T, status: reqwest::StatusCode, headers: reqwest::header::HeaderMap) -> Self {
        Self {
            inner,
            status,
            headers,
        }
    }

    /// Consumes the ResponseValue, returning the wrapped value.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets the status from this response.
    pub fn status(&self) -> reqwest::StatusCode {
        self.status
    }

    /// Gets the headers from this response.
    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    /// Gets the parsed value of the Content-Length header, if present and
    /// valid.
    pub fn content_length(&self) -> Option<u64> {
        self.headers
            .get(reqwest::header::CONTENT_LENGTH)?
            .to_str()
            .ok()?
            .parse::<u64>()
            .ok()
    }

    #[doc(hidden)]
    pub fn map<U: std::fmt::Debug, F, E>(self, f: F) -> Result<ResponseValue<U>, E>
    where
        F: FnOnce(T) -> U,
    {
        let Self {
            inner,
            status,
            headers,
        } = self;

        Ok(ResponseValue {
            inner: f(inner),
            status,
            headers,
        })
    }
}

impl ResponseValue<ByteStream> {
    /// Consumes the `ResponseValue`, returning the wrapped [`Stream`].
    pub fn into_inner_stream(self) -> InnerByteStream {
        self.into_inner().into_inner()
    }
}

impl<T> Deref for ResponseValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for ResponseValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<T> for ResponseValue<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ResponseValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

/// Error produced by generated client methods.
///
/// The type parameter may be a struct if there's a single expected error type
/// or an enum if there are multiple valid error types. It can be the unit type
/// if there are no structured returns expected.
pub enum Error<E = ()> {
    /// The request did not conform to API requirements.
    InvalidRequest(String),

    /// A server error either due to the data, or with the connection.
    CommunicationError(reqwest::Error),

    /// An expected response when upgrading connection.
    InvalidUpgrade(reqwest::Error),

    /// A documented, expected error response.
    ErrorResponse(ResponseValue<E>),

    /// Encountered an error reading the body for an expected response.
    ResponseBodyError(reqwest::Error),

    /// An expected response code whose deserialization failed.
    InvalidResponsePayload(Bytes, serde_json::Error),

    /// A response not listed in the API description. This may represent a
    /// success or failure response; check `status().is_success()`.
    UnexpectedResponse(reqwest::Response),

    /// A custom error from a consumer-defined hook.
    Custom(String),
}

impl<E> Error<E> {
    /// Returns the status code, if the error was generated from a response.
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            Error::InvalidRequest(_) => None,
            Error::Custom(_) => None,
            Error::CommunicationError(e) => e.status(),
            Error::ErrorResponse(rv) => Some(rv.status()),
            Error::InvalidUpgrade(e) => e.status(),
            Error::ResponseBodyError(e) => e.status(),
            Error::InvalidResponsePayload(_, _) => None,
            Error::UnexpectedResponse(r) => Some(r.status()),
        }
    }

    /// Returns `true` if this error indicates that it is transient, and that
    /// the operation could succeed if retried.
    ///
    /// The following are considered retryable:
    ///
    /// * Communication errors (connection reset, timeout, etc.)
    /// * 429 Too Many Requests
    /// * 502 Bad Gateway
    /// * 503 Service Unavailable
    /// * 504 Gateway Timeout
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::CommunicationError(_) => true,
            Error::ErrorResponse(rv) => is_retryable_status(rv.status()),
            Error::UnexpectedResponse(r) => is_retryable_status(r.status()),
            Error::InvalidUpgrade(e) | Error::ResponseBodyError(e) => {
                // We expect InvalidUpgrade and ResponseBodyError to be 2xx
                // statuses.
                //
                // * If they are 2xx statuses, is_retryable_status returns
                //   false so there's no harm in checking.
                // * In the unlikely case that they are not 2xx statuses
                //   (e.g., the error type was created outside of Progenitor),
                //   it is appropriate to check for retryability.
                e.status().is_some_and(is_retryable_status)
            }
            Error::InvalidRequest(_) => false,
            Error::InvalidResponsePayload(_, _) => false,
            Error::Custom(_) => false,
        }
    }

    /// Converts this error into one without a typed body.
    ///
    /// This is useful for unified error handling with APIs that distinguish
    /// various error response bodies.
    pub fn into_untyped(self) -> Error {
        match self {
            Error::InvalidRequest(s) => Error::InvalidRequest(s),
            Error::Custom(s) => Error::Custom(s),
            Error::CommunicationError(e) => Error::CommunicationError(e),
            Error::ErrorResponse(ResponseValue {
                inner: _,
                status,
                headers,
            }) => Error::ErrorResponse(ResponseValue {
                inner: (),
                status,
                headers,
            }),
            Error::InvalidUpgrade(e) => Error::InvalidUpgrade(e),
            Error::ResponseBodyError(e) => Error::ResponseBodyError(e),
            Error::InvalidResponsePayload(b, e) => Error::InvalidResponsePayload(b, e),
            Error::UnexpectedResponse(r) => Error::UnexpectedResponse(r),
        }
    }
}

impl<E> From<std::convert::Infallible> for Error<E> {
    fn from(x: std::convert::Infallible) -> Self {
        match x {}
    }
}

impl<E> From<reqwest::Error> for Error<E> {
    fn from(e: reqwest::Error) -> Self {
        Self::CommunicationError(e)
    }
}

impl<E> From<reqwest::header::InvalidHeaderValue> for Error<E> {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        Self::InvalidRequest(e.to_string())
    }
}

impl<E> std::fmt::Display for Error<E>
where
    ResponseValue<E>: ErrorFormat,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidRequest(s) => {
                write!(f, "Invalid Request: {}", s)?;
            }
            Error::CommunicationError(e) => {
                write!(f, "Communication Error: {}", e)?;
            }
            Error::ErrorResponse(rve) => {
                write!(f, "Error Response: ")?;
                rve.fmt_info(f)?;
            }
            Error::InvalidUpgrade(e) => {
                write!(f, "Invalid Response Upgrade: {}", e)?;
            }
            Error::ResponseBodyError(e) => {
                write!(f, "Invalid Response Body Bytes: {}", e)?;
            }
            Error::InvalidResponsePayload(b, e) => {
                write!(f, "Invalid Response Payload ({:?}): {}", b, e)?;
            }
            Error::UnexpectedResponse(r) => {
                write!(f, "Unexpected Response: {:?}", r)?;
            }
            Error::Custom(s) => {
                write!(f, "Error: {}", s)?;
            }
        }

        if f.alternate() {
            use std::error::Error as _;

            let mut src = self.source().and_then(|e| e.source());
            while let Some(s) = src {
                write!(f, ": {s}")?;
                src = s.source();
            }
        }
        Ok(())
    }
}

trait ErrorFormat {
    fn fmt_info(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<E> ErrorFormat for ResponseValue<E>
where
    E: std::fmt::Debug,
{
    fn fmt_info(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "status: {}; headers: {:?}; value: {:?}",
            self.status, self.headers, self.inner,
        )
    }
}

impl ErrorFormat for ResponseValue<ByteStream> {
    fn fmt_info(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "status: {}; headers: {:?}; value: <stream>",
            self.status, self.headers,
        )
    }
}

impl<E> std::fmt::Debug for Error<E>
where
    ResponseValue<E>: ErrorFormat,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl<E> std::error::Error for Error<E>
where
    ResponseValue<E>: ErrorFormat,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::CommunicationError(e) => Some(e),
            Error::InvalidUpgrade(e) => Some(e),
            Error::ResponseBodyError(e) => Some(e),
            Error::InvalidResponsePayload(_b, e) => Some(e),
            _ => None,
        }
    }
}

/// Returns `true` if the given HTTP status code is considered transient and
/// worth retrying.
fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(
        status,
        reqwest::StatusCode::TOO_MANY_REQUESTS
            | reqwest::StatusCode::BAD_GATEWAY
            | reqwest::StatusCode::SERVICE_UNAVAILABLE
            | reqwest::StatusCode::GATEWAY_TIMEOUT
    )
}

// See https://url.spec.whatwg.org/#url-path-segment-string
const PATH_SET: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b'%');

#[doc(hidden)]
/// Percent encode input string.
pub fn encode_path(pc: &str) -> String {
    percent_encoding::utf8_percent_encode(pc, PATH_SET).to_string()
}

#[doc(hidden)]
pub trait RequestBuilderExt<E> {
    fn form_urlencoded<T: Serialize + ?Sized>(self, body: &T) -> Result<RequestBuilder, Error<E>>;
}

impl<E> RequestBuilderExt<E> for RequestBuilder {
    fn form_urlencoded<T: Serialize + ?Sized>(self, body: &T) -> Result<Self, Error<E>> {
        Ok(self
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .body(
                serde_urlencoded::to_string(body)
                    .map_err(|_| Error::InvalidRequest("failed to serialize body".to_string()))?,
            ))
    }
}

#[doc(hidden)]
pub struct QueryParam<'a, T> {
    name: &'a str,
    value: &'a T,
}

impl<'a, T> QueryParam<'a, T> {
    #[doc(hidden)]
    pub fn new(name: &'a str, value: &'a T) -> Self {
        Self { name, value }
    }
}
impl<T> Serialize for QueryParam<'_, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, inner: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serializer = QuerySerializer {
            inner,
            name: self.name,
        };
        self.value.serialize(serializer)
    }
}

pub(crate) struct QuerySerializer<'a, S> {
    inner: S,
    name: &'a str,
}

macro_rules! serialize_scalar {
    ($f:ident, $t:ty) => {
        fn $f(self, v: $t) -> Result<Self::Ok, Self::Error> {
            [(self.name, v)].serialize(self.inner)
        }
    };
}

impl<'a, S> serde::Serializer for QuerySerializer<'a, S>
where
    S: serde::Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = QuerySeq<'a, S::SerializeSeq>;
    type SerializeTuple = S::SerializeTuple;
    type SerializeTupleStruct = S::SerializeTupleStruct;
    type SerializeTupleVariant = S::SerializeTupleVariant;
    type SerializeMap = S::SerializeMap;
    type SerializeStruct = S::SerializeStruct;
    type SerializeStructVariant = S::SerializeStructVariant;

    serialize_scalar!(serialize_bool, bool);
    serialize_scalar!(serialize_i8, i8);
    serialize_scalar!(serialize_i16, i16);
    serialize_scalar!(serialize_i32, i32);
    serialize_scalar!(serialize_i64, i64);
    serialize_scalar!(serialize_u8, u8);
    serialize_scalar!(serialize_u16, u16);
    serialize_scalar!(serialize_u32, u32);
    serialize_scalar!(serialize_u64, u64);
    serialize_scalar!(serialize_f32, f32);
    serialize_scalar!(serialize_f64, f64);
    serialize_scalar!(serialize_char, char);
    serialize_scalar!(serialize_str, &str);

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // Serialize the value through self which will proxy into the inner
        // Serializer as appropriate.
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        // A query parameter with a list of enumerated values will produce an
        // enum with unit variants. We treat these as scalar values, ignoring
        // the unit variant wrapper.
        variant.serialize(self)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // As with serde_json, we treat a newtype variant like a struct with a
        // single field. This may seem a little weird, but if an OpenAPI
        // document were to specify a query parameter whose schema was a oneOf
        // whose elements were objects with a single field, the user would end
        // up with an enum like this as a parameter.
        let mut map = self.inner.serialize_struct(name, 1)?;
        map.serialize_field(variant, value)?;
        map.end()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let Self { inner, name, .. } = self;
        Ok(QuerySeq {
            inner: inner.serialize_seq(len)?,
            name,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.inner.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.inner.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.inner
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.inner.serialize_map(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.inner.serialize_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.inner
            .serialize_struct_variant(name, variant_index, variant, len)
    }
}

#[doc(hidden)]
pub struct QuerySeq<'a, S> {
    inner: S,
    name: &'a str,
}

impl<S> serde::ser::SerializeSeq for QuerySeq<'_, S>
where
    S: serde::ser::SerializeSeq,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = (self.name, value);
        self.inner.serialize_element(&v)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

/// Returns true if the body schema contains any non-trivial `oneOf` / `anyOf`
/// (i.e., the template silently committed to a variant choice). Callers can
/// surface a hint pointing at `--json-body-schema` for the full grammar.
#[cfg(feature = "cli")]
pub fn body_has_variants(root_schema: &RootSchema) -> bool {
    let mut visited: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    schema_contains_variants(
        &Schema::Object(root_schema.schema.clone()),
        &root_schema.definitions,
        &mut visited,
    )
}

#[cfg(feature = "cli")]
fn schema_contains_variants(
    schema: &Schema,
    defs: &std::collections::BTreeMap<String, Schema>,
    visited: &mut std::collections::BTreeSet<String>,
) -> bool {
    let Schema::Object(o) = schema else { return false };

    if let Some(r) = &o.reference {
        let name = ref_name(r);
        if !visited.insert(name.clone()) {
            return false;
        }
        let result = defs
            .get(&name)
            .map(|def| schema_contains_variants(def, defs, visited))
            .unwrap_or(false);
        visited.remove(&name);
        return result;
    }

    if let Some(sub) = &o.subschemas {
        if let Some(variants) = sub.one_of.as_ref().or(sub.any_of.as_ref()) {
            let non_null = variants.iter().filter(|v| !is_null_schema(v)).count();
            if non_null > 1 {
                return true;
            }
        }
        if let Some(all) = &sub.all_of {
            for s in all {
                if schema_contains_variants(s, defs, visited) {
                    return true;
                }
            }
        }
    }

    if let Some(ov) = &o.object {
        for v in ov.properties.values() {
            if schema_contains_variants(v, defs, visited) {
                return true;
            }
        }
    }

    if let Some(av) = &o.array {
        match &av.items {
            Some(SingleOrVec::Single(s)) => {
                if schema_contains_variants(s, defs, visited) {
                    return true;
                }
            }
            Some(SingleOrVec::Vec(v)) => {
                for s in v {
                    if schema_contains_variants(s, defs, visited) {
                        return true;
                    }
                }
            }
            None => {}
        }
    }

    false
}

/// Generate a fill-in JSON template for a request body from its schema.
///
/// The template includes only required fields with placeholder values
/// (empty strings, zero numbers, first enum value, etc.) and picks the
/// first variant for any `oneOf`. It is intended as a starting point for
/// `--json-body` submission, not as a reference.
#[cfg(feature = "cli")]
pub fn generate_body_template(root_schema: &RootSchema) -> serde_json::Value {
    generate_from_schema(&Schema::Object(root_schema.schema.clone()), root_schema)
}

#[cfg(feature = "cli")]
fn generate_from_schema(schema: &Schema, root: &RootSchema) -> serde_json::Value {
    match schema {
        Schema::Bool(_) => serde_json::Value::Null,
        Schema::Object(obj) => generate_from_schema_object(obj, root),
    }
}

#[cfg(feature = "cli")]
fn generate_from_schema_object(schema: &SchemaObject, root: &RootSchema) -> serde_json::Value {
    if let Some(reference) = &schema.reference {
        if let Some(def_name) = reference.strip_prefix("#/definitions/") {
            if let Some(def_schema) = root.definitions.get(def_name) {
                return generate_from_schema(def_schema, root);
            }
        }
        return serde_json::Value::Null;
    }

    if let Some(sub) = &schema.subschemas {
        if let Some(any_of) = &sub.any_of {
            for sub_schema in any_of {
                let value = generate_from_schema(sub_schema, root);
                if !value.is_null() {
                    return value;
                }
            }
        }
        if let Some(one_of) = &sub.one_of {
            if let Some(first) = one_of.first() {
                return generate_from_schema(first, root);
            }
        }
        if let Some(all_of) = &sub.all_of {
            if let Some(first) = all_of.first() {
                return generate_from_schema(first, root);
            }
        }
    }

    if let Some(enum_values) = &schema.enum_values {
        if let Some(first) = enum_values.first() {
            return first.clone();
        }
    }

    let Some(instance_type) = &schema.instance_type else {
        return serde_json::Value::Null;
    };

    match instance_type {
        SingleOrVec::Single(t) => match **t {
            InstanceType::Null => serde_json::Value::Null,
            InstanceType::Boolean => serde_json::Value::Bool(false),
            InstanceType::Number | InstanceType::Integer => {
                serde_json::Value::Number(serde_json::Number::from(0))
            }
            InstanceType::String => serde_json::Value::String(String::new()),
            InstanceType::Array => match &schema.array {
                Some(items) => match &items.items {
                    Some(SingleOrVec::Single(item_schema)) => {
                        serde_json::Value::Array(vec![generate_from_schema(item_schema, root)])
                    }
                    Some(SingleOrVec::Vec(item_schemas)) => serde_json::Value::Array(
                        item_schemas.iter().map(|s| generate_from_schema(s, root)).collect(),
                    ),
                    None => serde_json::Value::Array(vec![]),
                },
                None => serde_json::Value::Array(vec![]),
            },
            InstanceType::Object => {
                let mut map = serde_json::Map::new();
                if let Some(object) = &schema.object {
                    for (prop_name, prop_schema) in &object.properties {
                        if !object.required.contains(prop_name) {
                            continue;
                        }
                        map.insert(prop_name.clone(), generate_from_schema(prop_schema, root));
                    }
                }
                serde_json::Value::Object(map)
            }
        },
        SingleOrVec::Vec(_) => serde_json::Value::Null,
    }
}

// ----------------------------------------------------------------------------
// Body schema renderer (Oracle-style grammar reference).
// ----------------------------------------------------------------------------

/// Render a request body schema as a structured grammar reference.
///
/// Output style is modeled after Oracle's SQL reference docs: a top-level
/// production for the request body, with each `$ref`'d type emitted as a
/// separate named production below. Required fields appear bare; optional
/// fields are wrapped in `[brackets]`. `oneOf` variants are joined with `|`.
///
/// Each production header carries metadata: `(root)` or `(used by: ...)` to
/// orient the reader, plus `(tagged on \`field\`)` for discriminated unions.
///
/// ANSI color is applied when stdout is a terminal and `NO_COLOR` is unset.
#[cfg(feature = "cli")]
pub fn render_body_schema(root_schema: &RootSchema) -> String {
    use std::collections::BTreeSet;

    let colorize = should_colorize();
    let style = Style::new(colorize);

    let top_name = root_schema
        .schema
        .metadata
        .as_ref()
        .and_then(|m| m.title.clone())
        .map(|t| snake_case(&t))
        .unwrap_or_else(|| "request".to_string());

    let usage_map = build_usage_map(root_schema, &top_name);

    let mut referenced: BTreeSet<String> = BTreeSet::new();
    let mut out = String::new();

    // Brief legend so first-time readers know what the conventions mean.
    out.push_str(&style.dim(
        "# [foo] = optional   <Type> = named ref   | = alternative",
    ));
    out.push('\n');
    out.push_str(&style.dim(
        "# (default: X) = default value   (tagged on `f`) = discriminator field",
    ));
    out.push_str("\n\n");

    out.push_str(&render_production(
        &top_name,
        &Schema::Object(root_schema.schema.clone()),
        true,
        &usage_map,
        &root_schema.definitions,
        &mut referenced,
        &style,
    ));

    let mut emitted: BTreeSet<String> = BTreeSet::new();
    while let Some(name) = referenced.iter().find(|n| !emitted.contains(*n)).cloned() {
        emitted.insert(name.clone());
        if let Some(def) = root_schema.definitions.get(&name) {
            out.push('\n');
            out.push_str(&render_production(
                &name,
                def,
                false,
                &usage_map,
                &root_schema.definitions,
                &mut referenced,
                &style,
            ));
        }
    }

    out
}

#[cfg(feature = "cli")]
fn render_production(
    name: &str,
    schema: &Schema,
    is_root: bool,
    usage_map: &std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    defs: &std::collections::BTreeMap<String, Schema>,
    refs: &mut std::collections::BTreeSet<String>,
    style: &Style,
) -> String {
    // Effective schema (unwrap single-allOf metadata wrappers).
    let effective: &SchemaObject = match schema {
        Schema::Object(o) => {
            if let Some(sub) = &o.subschemas {
                if let Some(all) = &sub.all_of {
                    if all.len() == 1 {
                        if let Schema::Object(inner) = &all[0] {
                            // We need a stable reference; fall back to outer if not Object.
                            inner
                        } else {
                            o
                        }
                    } else {
                        o
                    }
                } else {
                    o
                }
            } else {
                o
            }
        }
        _ => {
            // Non-object production (rare): emit on header line as before.
            return format!(
                "{} {} {}\n",
                style.bold(name),
                style.dim("::="),
                render_schema(schema, defs, refs, 0, style),
            );
        }
    };

    let oneof_variants: Option<Vec<&Schema>> = effective
        .subschemas
        .as_ref()
        .and_then(|s| s.one_of.as_ref().or(s.any_of.as_ref()))
        .map(|v| v.iter().filter(|s| !is_null_schema(s)).collect());

    // Header annotations.
    let mut annotations: Vec<String> = Vec::new();
    if is_root {
        annotations.push(style.dim("(root)"));
    } else if let Some(users) = usage_map.get(name) {
        let users_vec: Vec<&String> = users.iter().filter(|u| u.as_str() != name).collect();
        if !users_vec.is_empty() {
            let take = 3;
            let mut text = String::from("(used by: ");
            text.push_str(
                &users_vec
                    .iter()
                    .take(take)
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            if users_vec.len() > take {
                text.push_str(&format!(", +{} more", users_vec.len() - take));
            }
            text.push(')');
            annotations.push(style.dim(&text));
        }
    }

    let tag = oneof_variants.as_ref().and_then(|v| detect_tag(v));
    if let Some(t) = &tag {
        annotations.push(style.dim(&format!("(tagged on `{}`)", t)));
    }

    let mut header = format!("{} {}", style.bold(name), style.dim("::="));
    if !annotations.is_empty() {
        header.push_str("  ");
        header.push_str(&annotations.join("  "));
    }

    // Body.
    let body = match &oneof_variants {
        Some(variants) if variants.len() > 1 => {
            render_variants_production(variants, tag.as_deref(), defs, refs, style)
        }
        Some(variants) if variants.len() == 1 => render_schema(variants[0], defs, refs, 0, style),
        _ => render_schema(schema, defs, refs, 0, style),
    };

    format!("{}\n{}\n", header, body)
}

#[cfg(feature = "cli")]
fn render_variants_production(
    variants: &[&Schema],
    tag: Option<&str>,
    defs: &std::collections::BTreeMap<String, Schema>,
    refs: &mut std::collections::BTreeSet<String>,
    style: &Style,
) -> String {
    // Try inline rendering first. If all variants fit, emit each on its own
    // line with the BNF-style `|` separator (first variant indented; rest at
    // column 0 with leading `| `).
    let inline_budget = 76;
    let inline_attempts: Vec<Option<String>> = variants
        .iter()
        .map(|v| render_variant_inline(v, tag, defs, refs, style, inline_budget))
        .collect();

    let all_inline = inline_attempts.iter().all(|o| o.is_some());
    if all_inline {
        let rendered: Vec<String> = inline_attempts.into_iter().flatten().collect();
        let any_object = rendered.iter().any(|r| r.contains('{'));
        let single_line = rendered.join(&format!(" {} ", style.bold_yellow("|")));
        // Collapse purely-scalar unions to one line (e.g., `Uuid | String`).
        // Keep object unions in BNF style so each alternative reads as its own row.
        if !any_object && plain_len(&single_line) <= 80 {
            return single_line;
        }
        let mut out = String::new();
        for (i, v) in rendered.iter().enumerate() {
            if i == 0 {
                out.push_str("  ");
            } else {
                out.push('\n');
                out.push_str(&style.bold_yellow("|"));
                out.push(' ');
            }
            out.push_str(v);
        }
        return out;
    }

    // Block style fallback (each variant as a full object).
    let priority = tag.map(|t| vec![t.to_string()]).unwrap_or_default();
    let rendered: Vec<String> = variants
        .iter()
        .map(|v| render_schema_with_priority(v, &priority, defs, refs, 0, style))
        .collect();
    let sep = format!("\n{} ", style.bold_yellow("|"));
    rendered.join(&sep)
}

#[cfg(feature = "cli")]
fn render_variant_inline(
    schema: &Schema,
    tag: Option<&str>,
    defs: &std::collections::BTreeMap<String, Schema>,
    refs: &mut std::collections::BTreeSet<String>,
    style: &Style,
    budget: usize,
) -> Option<String> {
    // Object variants get tag-first ordering and inline key/value rendering.
    if let Schema::Object(o) = schema {
        if let Some(ov) = &o.object {
            let priority: Vec<String> = tag.map(|t| vec![t.to_string()]).unwrap_or_default();
            let ordered = order_properties(ov, &priority);
            let mut parts: Vec<String> = Vec::new();
            for (k, v, is_required) in ordered {
                let key_plain = if is_required {
                    format!("{}:", k)
                } else {
                    format!("[{}]:", k)
                };
                let key_styled = if is_required {
                    key_plain.clone()
                } else {
                    style.dim(&key_plain)
                };
                let rendered = render_schema(v, defs, refs, 0, style);
                let default_suffix = match v {
                    Schema::Object(o) => default_annotation(o)
                        .map(|d| style.dim(&format!(" (default: {})", d)))
                        .unwrap_or_default(),
                    _ => String::new(),
                };
                parts.push(format!("{} {}{}", key_styled, rendered, default_suffix));
            }
            let inline = if parts.is_empty() {
                "{}".to_string()
            } else {
                format!("{{ {} }}", parts.join(", "))
            };
            if plain_len(&inline) <= budget && !inline.contains('\n') {
                return Some(inline);
            }
            return None;
        }
    }

    // Scalar / non-object variants: render normally and check the budget.
    let rendered = render_schema(schema, defs, refs, 0, style);
    if plain_len(&rendered) <= budget && !rendered.contains('\n') {
        Some(rendered)
    } else {
        None
    }
}

#[cfg(feature = "cli")]
fn render_schema_with_priority(
    schema: &Schema,
    priority: &[String],
    defs: &std::collections::BTreeMap<String, Schema>,
    refs: &mut std::collections::BTreeSet<String>,
    depth: usize,
    style: &Style,
) -> String {
    if priority.is_empty() {
        return render_schema(schema, defs, refs, depth, style);
    }
    match schema {
        Schema::Object(o) => render_object_with_priority(o, priority, defs, refs, depth, style),
        _ => render_schema(schema, defs, refs, depth, style),
    }
}

#[cfg(feature = "cli")]
fn order_properties<'a>(
    ov: &'a schemars::schema::ObjectValidation,
    priority: &[String],
) -> Vec<(&'a String, &'a Schema, bool)> {
    let mut out: Vec<(&String, &Schema, bool)> = Vec::new();
    let is_required = |k: &String| ov.required.contains(k);

    // Priority fields first, in order.
    for p in priority {
        if let Some((k, v)) = ov.properties.iter().find(|(k, _)| *k == p) {
            out.push((k, v, is_required(k)));
        }
    }
    let used: std::collections::BTreeSet<&String> = out.iter().map(|t| t.0).collect();

    // Then required (alphabetical via BTreeMap iteration).
    for (k, v) in &ov.properties {
        if used.contains(k) {
            continue;
        }
        if is_required(k) {
            out.push((k, v, true));
        }
    }
    // Then optional.
    for (k, v) in &ov.properties {
        if used.contains(k) {
            continue;
        }
        if !is_required(k) {
            out.push((k, v, false));
        }
    }
    out
}

#[cfg(feature = "cli")]
fn render_object_with_priority(
    o: &SchemaObject,
    priority: &[String],
    defs: &std::collections::BTreeMap<String, Schema>,
    refs: &mut std::collections::BTreeSet<String>,
    depth: usize,
    style: &Style,
) -> String {
    let ov = match &o.object {
        Some(ov) => ov,
        None => return "{}".to_string(),
    };
    if ov.properties.is_empty() {
        return "{}".to_string();
    }

    let indent_outer = "  ".repeat(depth);
    let indent_inner = "  ".repeat(depth + 1);

    let key_width = ov
        .properties
        .keys()
        .map(|k| {
            if ov.required.contains(k) {
                k.len()
            } else {
                k.len() + 2
            }
        })
        .max()
        .unwrap_or(0);

    let ordered = order_properties(ov, priority);
    let mut lines = Vec::new();
    for (k, v, is_required) in ordered {
        let default = match v {
            Schema::Object(o) => default_annotation(o),
            _ => None,
        };
        let rendered = render_schema(v, defs, refs, depth + 1, style);
        let key_plain = if is_required {
            format!("{}:", k)
        } else {
            format!("[{}]:", k)
        };
        let key_styled = if is_required {
            key_plain.clone()
        } else {
            style.dim(&key_plain)
        };
        let pad = " ".repeat((key_width + 1).saturating_sub(key_plain.len()));
        let suffix = default
            .map(|d| style.dim(&format!("   (default: {})", d)))
            .unwrap_or_default();
        lines.push(format!(
            "{}{}{}  {}{}",
            indent_inner, key_styled, pad, rendered, suffix
        ));
    }

    format!("{{\n{}\n{}}}", lines.join("\n"), indent_outer)
}

#[cfg(feature = "cli")]
fn build_usage_map(
    root: &RootSchema,
    top_name: &str,
) -> std::collections::BTreeMap<String, std::collections::BTreeSet<String>> {
    let mut map: std::collections::BTreeMap<String, std::collections::BTreeSet<String>> =
        std::collections::BTreeMap::new();
    walk_for_refs(&Schema::Object(root.schema.clone()), top_name, &mut map);
    for (def_name, def_schema) in &root.definitions {
        walk_for_refs(def_schema, def_name, &mut map);
    }
    map
}

#[cfg(feature = "cli")]
fn walk_for_refs(
    schema: &Schema,
    parent: &str,
    map: &mut std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
) {
    let Schema::Object(o) = schema else { return };
    if let Some(r) = &o.reference {
        let name = ref_name(r);
        map.entry(name).or_default().insert(parent.to_string());
        return;
    }
    if let Some(sub) = &o.subschemas {
        for list in [&sub.one_of, &sub.any_of, &sub.all_of] {
            if let Some(items) = list {
                for s in items {
                    walk_for_refs(s, parent, map);
                }
            }
        }
    }
    if let Some(ov) = &o.object {
        for v in ov.properties.values() {
            walk_for_refs(v, parent, map);
        }
    }
    if let Some(av) = &o.array {
        match &av.items {
            Some(SingleOrVec::Single(s)) => walk_for_refs(s, parent, map),
            Some(SingleOrVec::Vec(v)) => {
                for s in v {
                    walk_for_refs(s, parent, map);
                }
            }
            None => {}
        }
    }
}

#[cfg(feature = "cli")]
fn detect_tag(variants: &[&Schema]) -> Option<String> {
    // For each variant, collect fields that are single-value string enums.
    // The tag is a field present (with that shape) in every variant.
    let candidate_sets: Vec<std::collections::BTreeSet<String>> = variants
        .iter()
        .map(|v| {
            let Schema::Object(o) = v else {
                return std::collections::BTreeSet::new();
            };
            let Some(ov) = &o.object else {
                return std::collections::BTreeSet::new();
            };
            ov.properties
                .iter()
                .filter_map(|(name, schema)| {
                    let Schema::Object(s) = schema else { return None };
                    let values = s.enum_values.as_ref()?;
                    if values.len() == 1 && values[0].is_string() {
                        Some(name.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    if candidate_sets.is_empty() || candidate_sets.iter().any(|s| s.is_empty()) {
        return None;
    }

    let mut intersection = candidate_sets[0].clone();
    for s in &candidate_sets[1..] {
        intersection = intersection.intersection(s).cloned().collect();
    }

    if intersection.len() == 1 {
        intersection.into_iter().next()
    } else {
        None
    }
}

#[cfg(feature = "cli")]
fn should_colorize() -> bool {
    use std::io::IsTerminal as _;
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if std::env::var_os("CLICOLOR_FORCE").is_some() {
        return true;
    }
    std::io::stdout().is_terminal()
}

#[cfg(feature = "cli")]
struct Style {
    enabled: bool,
}

#[cfg(feature = "cli")]
impl Style {
    fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    fn wrap(&self, code: &str, s: &str) -> String {
        if self.enabled {
            format!("\x1b[{}m{}\x1b[0m", code, s)
        } else {
            s.to_string()
        }
    }
    fn dim(&self, s: &str) -> String {
        self.wrap("2", s)
    }
    fn bold(&self, s: &str) -> String {
        self.wrap("1", s)
    }
    fn cyan(&self, s: &str) -> String {
        self.wrap("36", s)
    }
    fn green(&self, s: &str) -> String {
        self.wrap("32", s)
    }
    fn bold_yellow(&self, s: &str) -> String {
        self.wrap("1;33", s)
    }
}

#[cfg(feature = "cli")]
fn render_schema(
    s: &Schema,
    defs: &std::collections::BTreeMap<String, Schema>,
    refs: &mut std::collections::BTreeSet<String>,
    depth: usize,
    style: &Style,
) -> String {
    match s {
        Schema::Bool(true) => "any".to_string(),
        Schema::Bool(false) => "never".to_string(),
        Schema::Object(o) => render_schema_object(o, defs, refs, depth, style),
    }
}

#[cfg(feature = "cli")]
fn render_schema_object(
    o: &SchemaObject,
    defs: &std::collections::BTreeMap<String, Schema>,
    refs: &mut std::collections::BTreeSet<String>,
    depth: usize,
    style: &Style,
) -> String {
    if let Some(r) = &o.reference {
        let name = ref_name(r);
        refs.insert(name.clone());
        return style.cyan(&format!("<{}>", name));
    }

    // allOf wrapping a single ref is just a description carrier; unwrap.
    if let Some(sub) = &o.subschemas {
        if let Some(all) = &sub.all_of {
            if all.len() == 1 {
                return render_schema(&all[0], defs, refs, depth, style);
            }
        }
    }

    // oneOf / anyOf: drop null alternatives (optionality is shown by brackets).
    if let Some(sub) = &o.subschemas {
        if let Some(variants) = sub.one_of.as_ref().or(sub.any_of.as_ref()) {
            let non_null: Vec<&Schema> = variants.iter().filter(|v| !is_null_schema(v)).collect();
            if non_null.len() == 1 {
                return render_schema(non_null[0], defs, refs, depth, style);
            }
            let rendered: Vec<String> = non_null
                .iter()
                .map(|v| render_schema(v, defs, refs, depth, style))
                .collect();
            return join_variants(&rendered, depth, style);
        }
    }

    if let Some(values) = &o.enum_values {
        if !values.is_empty() {
            let rendered: Vec<String> = values
                .iter()
                .map(|v| match v {
                    serde_json::Value::String(s) => style.green(&format!("\"{}\"", s)),
                    other => other.to_string(),
                })
                .collect();
            let sep = format!(" {} ", style.bold_yellow("|"));
            return rendered.join(&sep);
        }
    }

    match instance_type(&o.instance_type) {
        Some(InstanceType::String) => render_string(o),
        Some(InstanceType::Integer) => render_integer(o),
        Some(InstanceType::Number) => "Number".to_string(),
        Some(InstanceType::Boolean) => "bool".to_string(),
        Some(InstanceType::Null) => "null".to_string(),
        Some(InstanceType::Array) => render_array(o, defs, refs, depth, style),
        Some(InstanceType::Object) | None => render_object(o, defs, refs, depth, style),
    }
}

#[cfg(feature = "cli")]
fn is_null_schema(s: &Schema) -> bool {
    match s {
        Schema::Object(o) => matches!(instance_type(&o.instance_type), Some(InstanceType::Null)),
        _ => false,
    }
}

#[cfg(feature = "cli")]
fn render_string(o: &SchemaObject) -> String {
    match o.format.as_deref() {
        Some("uuid") => "Uuid".to_string(),
        Some("date-time") => "DateTime".to_string(),
        Some("ip") => "IpAddr".to_string(),
        Some("ipv4") => "Ipv4Addr".to_string(),
        Some("ipv6") => "Ipv6Addr".to_string(),
        Some("byte") => "Base64".to_string(),
        Some(other) => format!("String ({})", other),
        None => "String".to_string(),
    }
}

#[cfg(feature = "cli")]
fn render_integer(o: &SchemaObject) -> String {
    o.format.clone().unwrap_or_else(|| "Integer".to_string())
}

#[cfg(feature = "cli")]
fn render_array(
    o: &SchemaObject,
    defs: &std::collections::BTreeMap<String, Schema>,
    refs: &mut std::collections::BTreeSet<String>,
    depth: usize,
    style: &Style,
) -> String {
    let item = match &o.array {
        Some(av) => match &av.items {
            Some(SingleOrVec::Single(s)) => render_schema(s, defs, refs, depth, style),
            Some(SingleOrVec::Vec(v)) if !v.is_empty() => {
                render_schema(&v[0], defs, refs, depth, style)
            }
            _ => "any".to_string(),
        },
        None => "any".to_string(),
    };
    format!("[{}, ...]", item)
}

#[cfg(feature = "cli")]
fn render_object(
    o: &SchemaObject,
    defs: &std::collections::BTreeMap<String, Schema>,
    refs: &mut std::collections::BTreeSet<String>,
    depth: usize,
    style: &Style,
) -> String {
    let ov = match &o.object {
        Some(ov) => ov,
        None => return "{}".to_string(),
    };

    if ov.properties.is_empty() {
        return "{}".to_string();
    }

    let indent_outer = "  ".repeat(depth);
    let indent_inner = "  ".repeat(depth + 1);

    let mut required: Vec<(&String, &Schema)> = Vec::new();
    let mut optional: Vec<(&String, &Schema)> = Vec::new();
    for (k, v) in &ov.properties {
        if ov.required.contains(k) {
            required.push((k, v));
        } else {
            optional.push((k, v));
        }
    }

    // Align all colons. Optional keys are 2 chars wider for the brackets.
    let key_width = ov
        .properties
        .keys()
        .map(|k| {
            if ov.required.contains(k) {
                k.len()
            } else {
                k.len() + 2
            }
        })
        .max()
        .unwrap_or(0);

    let mut lines = Vec::new();
    for (k, v) in required.iter().chain(optional.iter()) {
        let is_required = ov.required.contains(*k);
        let default = match v {
            Schema::Object(o) => default_annotation(o),
            _ => None,
        };
        let rendered = render_schema(v, defs, refs, depth + 1, style);
        let key_plain = if is_required {
            format!("{}:", k)
        } else {
            format!("[{}]:", k)
        };
        let key_styled = if is_required {
            key_plain.clone()
        } else {
            style.dim(&key_plain)
        };
        let pad = " ".repeat((key_width + 1).saturating_sub(key_plain.len()));
        let suffix = default
            .map(|d| style.dim(&format!("   (default: {})", d)))
            .unwrap_or_default();
        lines.push(format!(
            "{}{}{}  {}{}",
            indent_inner, key_styled, pad, rendered, suffix
        ));
    }

    format!("{{\n{}\n{}}}", lines.join("\n"), indent_outer)
}

#[cfg(feature = "cli")]
fn join_variants(variants: &[String], depth: usize, style: &Style) -> String {
    let sep_inline = format!(" {} ", style.bold_yellow("|"));
    let inline = variants.join(&sep_inline);
    let budget = 80usize.saturating_sub(2 * depth);
    if !inline.contains('\n') && plain_len(&inline) <= budget {
        return inline;
    }
    let indent = "  ".repeat(depth);
    let sep = format!("\n{}{} ", indent, style.bold_yellow("|"));
    let mut out = String::new();
    out.push_str(&variants[0]);
    for v in &variants[1..] {
        out.push_str(&sep);
        out.push_str(v);
    }
    out
}

#[cfg(feature = "cli")]
fn plain_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_esc = false;
    for c in s.chars() {
        if in_esc {
            if c.is_ascii_alphabetic() {
                in_esc = false;
            }
        } else if c == '\x1b' {
            in_esc = true;
        } else {
            len += 1;
        }
    }
    len
}

#[cfg(feature = "cli")]
fn default_annotation(o: &SchemaObject) -> Option<String> {
    let d = o.metadata.as_ref()?.default.as_ref()?;
    Some(match d {
        serde_json::Value::String(s) => format!("\"{}\"", s),
        other => other.to_string(),
    })
}

#[cfg(feature = "cli")]
fn instance_type(t: &Option<SingleOrVec<InstanceType>>) -> Option<InstanceType> {
    match t {
        Some(SingleOrVec::Single(t)) => Some(**t),
        Some(SingleOrVec::Vec(v)) => v.iter().find(|t| !matches!(t, InstanceType::Null)).copied(),
        None => None,
    }
}

#[cfg(feature = "cli")]
fn ref_name(r: &str) -> String {
    r.rsplit('/').next().unwrap_or(r).to_string()
}

#[cfg(feature = "cli")]
fn snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}
