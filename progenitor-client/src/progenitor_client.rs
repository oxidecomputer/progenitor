// Copyright 2023 Oxide Computer Company

#![allow(dead_code)]

//! Support code for generated clients.

use std::ops::{Deref, DerefMut};

use bytes::Bytes;
use futures_core::Stream;
use reqwest::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};

#[cfg(not(target_arch = "wasm32"))]
type InnerByteStream =
    std::pin::Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>> + Send + Sync>>;

#[cfg(target_arch = "wasm32")]
type InnerByteStream =
    std::pin::Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>>>>;

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
    pub async fn from_response<E: std::fmt::Debug>(
        response: reqwest::Response,
    ) -> Result<Self, Error<E>> {
        let status = response.status();
        let headers = response.headers().clone();
        let full = response.bytes().await.map_err(Error::ResponseBodyError)?;
        let inner = serde_json::from_slice(&full)
            .map_err(|e| Error::InvalidResponsePayload(full, e))?;

        Ok(Self {
            inner,
            status,
            headers,
        })
    }

    /// Transforms the inner data of this `ResponseValue` using a provided function, returning a new `ResponseValue` with the transformed data.
    pub fn map_inner<U, F>(self, op: F) -> ResponseValue<U>
    where
        F: FnOnce(T) -> U,
    {
        ResponseValue {
            inner: op(self.inner), // Apply the operation to the inner data
            status: self.status,  // Preserve the status
            headers: self.headers, // Preserve the headers
        }
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
            let inner =
                response.upgrade().await.map_err(Error::InvalidUpgrade)?;

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
    pub fn new(
        inner: T,
        status: reqwest::StatusCode,
        headers: reqwest::header::HeaderMap,
    ) -> Self {
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
    pub fn map<U: std::fmt::Debug, F, E>(
        self,
        f: F,
    ) -> Result<ResponseValue<U>, E>
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

    /// An error occurred in the processing of a request pre-hook.
    PreHookError(String),
}

impl<E> Error<E> {
    /// Returns the status code, if the error was generated from a response.
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            Error::InvalidRequest(_) => None,
            Error::PreHookError(_) => None,
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
            Error::PreHookError(s) => Error::PreHookError(s),
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
            Error::InvalidResponsePayload(b, e) => {
                Error::InvalidResponsePayload(b, e)
            }
            Error::UnexpectedResponse(r) => Error::UnexpectedResponse(r),
        }
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
                write!(f, "Invalid Request: {}", s)
            }
            Error::CommunicationError(e) => {
                write!(f, "Communication Error: {}", e)
            }
            Error::ErrorResponse(rve) => {
                write!(f, "Error Response: ")?;
                rve.fmt_info(f)
            }
            Error::InvalidUpgrade(e) => {
                write!(f, "Invalid Response Upgrade: {}", e)
            }
            Error::ResponseBodyError(e) => {
                write!(f, "Invalid Response Body Bytes: {}", e)
            }
            Error::InvalidResponsePayload(b, e) => {
                write!(f, "Invalid Response Payload ({:?}): {}", b, e)
            }
            Error::UnexpectedResponse(r) => {
                write!(f, "Unexpected Response: {:?}", r)
            }
            Error::PreHookError(s) => {
                write!(f, "Pre-hook Error: {}", s)
            }
        }
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
    fn form_urlencoded<T: Serialize + ?Sized>(
        self,
        body: &T,
    ) -> Result<RequestBuilder, Error<E>>;
}

impl<E> RequestBuilderExt<E> for RequestBuilder {
    fn form_urlencoded<T: Serialize + ?Sized>(
        self,
        body: &T,
    ) -> Result<Self, Error<E>> {
        Ok(self
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static(
                    "application/x-www-form-urlencoded",
                ),
            )
            .body(serde_urlencoded::to_string(body).map_err(|_| {
                Error::InvalidRequest("failed to serialize body".to_string())
            })?))
    }
}
