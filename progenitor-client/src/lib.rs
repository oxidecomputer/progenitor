// Copyright 2021 Oxide Computer Company

//! Support code for generated clients.

use std::ops::Deref;

use serde::de::DeserializeOwned;

/// Error produced by generated client methods. The type parameter may be a
/// struct if there's a single expected error type or an enum if there are
/// multiple valid error types.
pub enum Error<E> {
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

impl<E> From<reqwest::Error> for Error<E> {
    fn from(e: reqwest::Error) -> Self {
        Self::CommunicationError(e)
    }
}

pub struct ResponseValue<T> {
    inner: T,
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    // TODO cookies?
}

impl<T: DeserializeOwned> ResponseValue<T> {
    pub async fn from_response<E>(
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
    pub fn empty<E>(response: reqwest::Response) -> Result<Self, E> {
        let status = response.status();
        let headers = response.headers().clone();
        // TODO is there anything we want to do to confirm that there is no
        // content?
        Ok(Self {
            inner: (),
            status,
            headers,
        })
    }
}

impl<T> ResponseValue<T> {
    pub fn status(&self) -> &reqwest::StatusCode {
        &self.status
    }
    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    pub fn map<U, F, E>(self, f: F) -> Result<ResponseValue<U>, E>
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

    pub fn to_error<U>(self) -> Result<U, Error<T>> {
        Err(Error::ErrorResponse(self))
    }
}

impl<T> Deref for ResponseValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
