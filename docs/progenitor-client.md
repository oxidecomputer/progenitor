# Progenitor Client

The `progenitor-client` crate contains types that are exported by generated
clients as well as functions that are used internally by generated clients.
Depending on how `progenitor` is being used, the crate will be included in
different ways (see ["Using Progenitor"](../README.md#using_progenitor)).

- For macro consumers, it comes from the `progenitor` dependency.

- For builder consumers, it must be specified under `[dependencies]` (while `progenitor` is under `[build-dependencies]`).

- For statically generated consumers, the code is emitted into
  `src/progenitor_client.rs`.

The two types that `progenitor-client` exports are `Error<E>` and
`ResponseValue<T>`. A typical generated method will use these types in its
signature:

```rust
impl Client {
    pub async fn operation_name<'a>(
        &'a self,
        // parameters ...
    ) -> Result<
        ResponseValue<types::SuccessResponseType>,
        Error<types::ErrorResponseType>>
    {
        // ...
    }
```

## `ResponseValue<T>`

OpenAPI documents defines the types that an operation returns. Generated
methods wrap these types in `ResponseValue<T>` for two reasons: there is
additional information that may be included in a response such as the specific
status code and headers, and that information cannot simply be included in the
response type because that type may be used in other context (e.g. as a body
parameter).

These are the relevant implementations for `ResponseValue<T>`:

```rust
/// Success value returned by generated client methods.
pub struct ResponseValue<T> { .. }

impl<T> ResponseValue<T> {
    pub fn status(&self) -> &reqwest::StatusCode { .. }
    pub fn headers(&self) -> &reqwest::header::HeaderMap { .. }
    pub fn into_inner(self) -> T { .. }
}
impl<T> std::ops::Deref for ResponseValue<T> {
    type Target = T;
    ..
}
impl<T> std::ops::DerefMut for ResponseValue<T> { .. }

impl<T: std::fmt::Debug> std::fmt::Debug for ResponseValue<T> { .. }
```

It can be used as the type `T` in most instances and extracted as a `T` using
`into_inner()`.

## `Error<E>`

There are seven sub-categories of error covered by the error type variants:

- A request that did not conform to API requirements.
  This can occur when required builder or body parameters were not specified,
  and the error message will denote the specific failure.

- A communication error

- An expected error response when upgrading connection.

- An expected error response, defined by the OpenAPI document
  with a 4xx or 5xx status code

- An expected status code that encountered an error reading the body
  or the payload deserialization failed
  (this could be viewed as a sub-type of communication error),
  but it is separately identified as there's more information;
  note that this covers both success and error status codes

- An unexpected status code in the response

These errors are covered by the variants of the `Error<E>` type:

```rust
pub enum Error<E = ()> {
    InvalidRequest(String),
    CommunicationError(reqwest::Error),
    InvalidUpgrade(reqwest::Error),
    ErrorResponse(ResponseValue<E>),
    ResponseBodyError(reqwest::Error),
    InvalidResponsePayload(bytes::Bytes, reqwest::Error),
    UnexpectedResponse(reqwest::Response),
}
```
