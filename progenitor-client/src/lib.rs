// Copyright 2021 Oxide Computer Company

//! Support code for generated clients.

use std::ops::Deref;

/// Error produced by generated client methods.
pub enum Error<E> {
    /// Indicates an error from the server, with the data, or with the
    /// connection.
    CommunicationError(reqwest::Error),

    /// A documented error response.
    ErrorResponse(ResponseValue<E>),

    /// A response not listed in the API description. This may represent a
    /// success or failure response; check `status()::is_success()`.
    UnexpectedResponse(reqwest::Response),
}

pub struct ResponseValue<T> {
    inner: T,
    response: reqwest::Response,
}

impl<T> ResponseValue<T> {
    #[doc(hidden)]
    pub fn new(inner: T, response: reqwest::Response) -> Self {
        Self { inner, response }
    }

    pub fn request(&self) -> &reqwest::Response {
        &self.response
    }
}

impl<T> Deref for ResponseValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
