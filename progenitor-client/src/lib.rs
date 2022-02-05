// Copyright 2021 Oxide Computer Company

//! Support code for generated clients.

use std::ops::{Deref, DerefMut};

use serde::de::DeserializeOwned;

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
    pub fn status(&self) -> &reqwest::StatusCode {
        &self.status
    }
    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

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

    pub fn into_inner(self) -> T {
        self.inner
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

/// Error produced by generated client methods. The type parameter may be a
/// struct if there's a single expected error type or an enum if there are
/// multiple valid error types. It can be the unit type if there are no
/// structured returns expected.
#[derive(Debug)]
pub enum Error<E: std::fmt::Debug> {
    /// A server error either with the data, or with the connection.
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

impl<E: std::fmt::Debug> From<reqwest::Error> for Error<E> {
    fn from(e: reqwest::Error) -> Self {
        Self::CommunicationError(e)
    }
}

impl<E: std::fmt::Debug> std::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CommunicationError(e) => {
                write!(f, "Communication Error {}", e)
            }
            Error::ErrorResponse(rv) => {
                write!(f, "Error Response {:?}", rv)
            }
            Error::InvalidResponsePayload(e) => {
                write!(f, "Invalid Response Payload {}", e)
            }
            Error::UnexpectedResponse(r) => {
                write!(f, "Unexpected Response {:?}", r)
            }
        }
    }
}
impl<E: std::fmt::Debug> std::error::Error for Error<E> {
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

pub fn encode_path(pc: &str) -> String {
    percent_encoding::utf8_percent_encode(pc, PATH_SET).to_string()
}
