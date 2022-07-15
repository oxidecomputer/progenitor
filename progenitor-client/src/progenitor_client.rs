// Copyright 2022 Oxide Computer Company

#![allow(dead_code)]

//! Support code for generated clients.

use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

use bytes::Bytes;
use futures_core::Stream;
use reqwest::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};

/// Represents an untyped byte stream for both success and error responses.
pub struct ByteStream(
    Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>> + Send>>,
);

impl Deref for ByteStream {
    type Target = Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>> + Send>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Success value returned by generated client methods.
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
        let inner = response
            .json()
            .await
            .map_err(Error::InvalidResponsePayload)?;

        Ok(Self {
            inner,
            status,
            headers,
        })
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
    /// Create an instance for testing
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

    /// Get the status from this response.
    pub fn status(&self) -> reqwest::StatusCode {
        self.status
    }

    /// Get the headers from this response.
    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
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

    /// A documented, expected error response.
    ErrorResponse(ResponseValue<E>),

    /// An expected response code whose deserialization failed.
    // TODO we have stuff from the response; should we include it?
    InvalidResponsePayload(reqwest::Error),

    /// A response not listed in the API description. This may represent a
    /// success or failure response; check `status().is_success()`.
    UnexpectedResponse(reqwest::Response),
}

impl<E> Error<E> {
    /// Returns the status code, if the error was generated from a response.
    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            Error::InvalidRequest(_) => None,
            Error::CommunicationError(e) => e.status(),
            Error::ErrorResponse(rv) => Some(rv.status()),
            Error::InvalidResponsePayload(e) => e.status(),
            Error::UnexpectedResponse(r) => Some(r.status()),
        }
    }

    /// Convert this error into one without a typed body for unified error
    /// handling with APIs that distinguish various error response bodies.
    pub fn into_untyped(self) -> Error {
        match self {
            Error::InvalidRequest(s) => Error::InvalidRequest(s),
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
            Error::InvalidResponsePayload(e) => {
                Error::InvalidResponsePayload(e)
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
            Error::InvalidResponsePayload(e) => {
                write!(f, "Invalid Response Payload: {}", e)
            }
            Error::UnexpectedResponse(r) => {
                write!(f, "Unexpected Response: {:?}", r)
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
            Error::InvalidResponsePayload(e) => Some(e),
            _ => None,
        }
    }
}

const PATH_SET: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}');

#[doc(hidden)]
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
            .body(serde_urlencoded::to_string(&body).map_err(|_| {
                Error::InvalidRequest("failed to serialize body".to_string())
            })?))
    }
}
