// Copyright 2025 Oxide Computer Company

#![allow(dead_code)]

//! Support code for generated clients using gloo-net.

use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

use crate::OperationInfo;

/// Interface for which an implementation is generated for all clients.
pub trait ClientInfo<Inner> {
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI document and may
    /// be in any format the API selects.
    fn api_version() -> &'static str;

    /// Get the base URL to which requests are made.
    fn baseurl(&self) -> &str;

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

    fn inner(&self) -> &Inner {
        (*self).inner()
    }
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
        request: &mut gloo_net::http::Request,
        info: &OperationInfo,
    ) -> std::result::Result<(), Error<E>> {
        Ok(())
    }

    /// Runs after completion of the request.
    async fn post<E>(
        &self,
        result: &Result<gloo_net::http::Response, gloo_net::Error>,
        info: &OperationInfo,
    ) -> std::result::Result<(), Error<E>> {
        Ok(())
    }

    /// Execute the request. Note that for gloo-net, this is a simple wrapper
    /// around the request's send method since there's no client object to manage.
    async fn exec(
        &self,
        request: gloo_net::http::Request,
        info: &OperationInfo,
    ) -> Result<gloo_net::http::Response, gloo_net::Error> {
        request.send().await
    }
}

/// Wrapper around gloo_net::http::Headers to provide a nicer API.
#[derive(Debug)]
pub struct HeadersWrapper(gloo_net::http::Headers);

impl HeadersWrapper {
    /// Create a new HeadersWrapper from gloo_net::http::Headers.
    pub fn from_gloo(headers: gloo_net::http::Headers) -> Self {
        Self(headers)
    }

    /// Get a header value by name.
    pub fn get(&self, name: &str) -> Option<String> {
        self.0.get(name)
    }

    /// Get the underlying gloo_net::http::Headers.
    pub fn as_gloo(&self) -> &gloo_net::http::Headers {
        &self.0
    }
}

type InnerByteStream = Pin<Box<dyn Stream<Item = Result<Vec<u8>, JsValue>>>>;

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
    status: u16,
    headers: HeadersWrapper,
}

impl<T: DeserializeOwned> ResponseValue<T> {
    #[doc(hidden)]
    pub async fn from_response<E>(response: gloo_net::http::Response) -> Result<Self, Error<E>> {
        let status = response.status();
        let headers = HeadersWrapper::from_gloo(response.headers());

        let text = response
            .text()
            .await
            .map_err(|e| Error::ResponseBodyError(format!("{:?}", e)))?;

        let inner = serde_json::from_str(&text)
            .map_err(|e| Error::InvalidResponsePayload(text.into_bytes(), e))?;

        Ok(Self {
            inner,
            status,
            headers,
        })
    }
}

impl ResponseValue<()> {
    #[doc(hidden)]
    pub fn empty(response: gloo_net::http::Response) -> Self {
        let status = response.status();
        let headers = HeadersWrapper::from_gloo(response.headers());
        Self {
            inner: (),
            status,
            headers,
        }
    }
}

// Helper struct for empty stream
struct EmptyStream;

impl Stream for EmptyStream {
    type Item = Result<Vec<u8>, JsValue>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}

// Helper to convert JsValue chunks to Vec<u8>
struct JsValueStream {
    inner: wasm_streams::readable::IntoStream<'static>,
}

impl Stream for JsValueStream {
    type Item = Result<Vec<u8>, JsValue>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Ready(Some(Ok(js_value))) => {
                // Convert JsValue to Uint8Array to Vec<u8>
                match js_sys::Uint8Array::new(&js_value).to_vec().into() {
                    vec => Poll::Ready(Some(Ok(vec))),
                }
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl ResponseValue<ByteStream> {
    #[doc(hidden)]
    pub fn stream(response: gloo_net::http::Response) -> Self {
        let status = response.status();
        let headers = HeadersWrapper::from_gloo(response.headers());

        // Convert web_sys::ReadableStream to futures::Stream
        let stream: InnerByteStream = match response.body() {
            Some(readable_stream) => {
                let stream = wasm_streams::ReadableStream::from_raw(readable_stream);
                Box::pin(JsValueStream {
                    inner: stream.into_stream(),
                })
            }
            None => {
                // Empty stream
                Box::pin(EmptyStream)
            }
        };

        Self {
            inner: ByteStream(stream),
            status,
            headers,
        }
    }
}

impl ResponseValue<web_sys::WebSocket> {
    #[doc(hidden)]
    pub fn websocket(ws: web_sys::WebSocket) -> Result<Self, Error<web_sys::WebSocket>> {
        // For WebSocket connections, we don't have traditional HTTP response status/headers
        // since the WebSocket is created directly. We use a synthetic status of 101
        // (Switching Protocols) to indicate a successful upgrade.
        Ok(Self {
            inner: ws,
            status: 101,
            headers: HeadersWrapper::from_gloo(gloo_net::http::Headers::new()),
        })
    }
}

impl<T> ResponseValue<T> {
    /// Creates a [`ResponseValue`] from the inner type, status, and headers.
    ///
    /// Useful for generating test fixtures.
    pub fn new(inner: T, status: u16, headers: HeadersWrapper) -> Self {
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
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Gets the headers from this response.
    pub fn headers(&self) -> &HeadersWrapper {
        &self.headers
    }

    /// Gets the parsed value of the Content-Length header, if present and
    /// valid.
    pub fn content_length(&self) -> Option<u64> {
        self.headers
            .get("content-length")?
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
    CommunicationError(String),

    /// A documented, expected error response.
    ErrorResponse(ResponseValue<E>),

    /// Encountered an error reading the body for an expected response.
    ResponseBodyError(String),

    /// An expected response code whose deserialization failed.
    InvalidResponsePayload(Vec<u8>, serde_json::Error),

    /// A response not listed in the API description. This may represent a
    /// success or failure response; check the status code.
    UnexpectedResponse(gloo_net::http::Response),

    /// A custom error from a consumer-defined hook.
    Custom(String),
}

impl<E> Error<E> {
    /// Returns the status code, if the error was generated from a response.
    pub fn status(&self) -> Option<u16> {
        match self {
            Error::InvalidRequest(_) => None,
            Error::Custom(_) => None,
            Error::CommunicationError(_) => None,
            Error::ErrorResponse(rv) => Some(rv.status()),
            Error::ResponseBodyError(_) => None,
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
            Error::CommunicationError(s) => Error::CommunicationError(s),
            Error::ErrorResponse(ResponseValue {
                inner: _,
                status,
                headers,
            }) => Error::ErrorResponse(ResponseValue {
                inner: (),
                status,
                headers,
            }),
            Error::ResponseBodyError(s) => Error::ResponseBodyError(s),
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

impl<E> From<gloo_net::Error> for Error<E> {
    fn from(e: gloo_net::Error) -> Self {
        Self::CommunicationError(format!("{:?}", e))
    }
}

impl<E> From<JsValue> for Error<E> {
    fn from(e: JsValue) -> Self {
        Self::InvalidRequest(format!("{:?}", e))
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
            Error::ResponseBodyError(e) => {
                write!(f, "Invalid Response Body: {}", e)?;
            }
            Error::InvalidResponsePayload(b, e) => {
                write!(
                    f,
                    "Invalid Response Payload ({}): {}",
                    String::from_utf8_lossy(b),
                    e
                )?;
            }
            Error::UnexpectedResponse(r) => {
                write!(f, "Unexpected Response: status {}", r.status())?;
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
            Error::InvalidResponsePayload(_b, e) => Some(e),
            _ => None,
        }
    }
}

#[doc(hidden)]
pub trait RequestBuilderExt<E> {
    fn form_urlencoded<T: Serialize + ?Sized>(
        self,
        body: &T,
    ) -> Result<gloo_net::http::Request, Error<E>>;
}

impl<E> RequestBuilderExt<E> for gloo_net::http::Request {
    fn form_urlencoded<T: Serialize + ?Sized>(
        self,
        _body: &T,
    ) -> Result<Self, Error<E>> {
        // form_urlencoded not supported in gloo-net
        Err(Error::InvalidRequest(
            "form_urlencoded is not supported as a post-construction operation in gloo-net".to_string()
        ))
    }
}

