// Copyright 2025 Oxide Computer Company

#![allow(dead_code)]

//! Support code for generated clients.

use std::ops::{Deref, DerefMut};

use bytes::Bytes;
use futures_core::Stream;
use reqwest::RequestBuilder;
use serde::{de::DeserializeOwned, ser::SerializeStruct, Serialize};

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
