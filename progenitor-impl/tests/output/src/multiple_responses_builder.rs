#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    ///`BadRequest`
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct BadRequest {
        pub message: ::std::string::String,
    }

    impl BadRequest {
        pub fn builder() -> builder::BadRequest {
            Default::default()
        }
    }

    ///`InternalError`
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InternalError {
        pub request_id: ::std::string::String,
    }

    impl InternalError {
        pub fn builder() -> builder::InternalError {
            Default::default()
        }
    }

    ///`Progress`
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Progress {
        pub percent: u8,
    }

    impl Progress {
        pub fn builder() -> builder::Progress {
            Default::default()
        }
    }

    ///`Thing`
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Thing {
        pub name: ::std::string::String,
    }

    impl Thing {
        pub fn builder() -> builder::Thing {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct BadRequest {
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for BadRequest {
            fn default() -> Self {
                Self {
                    message: Err("no value supplied for message".to_string()),
                }
            }
        }

        impl BadRequest {
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<BadRequest> for super::BadRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: BadRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::BadRequest> for BadRequest {
            fn from(value: super::BadRequest) -> Self {
                Self {
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InternalError {
            request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for InternalError {
            fn default() -> Self {
                Self {
                    request_id: Err("no value supplied for request_id".to_string()),
                }
            }
        }

        impl InternalError {
            pub fn request_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.request_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for request_id: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<InternalError> for super::InternalError {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InternalError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    request_id: value.request_id?,
                })
            }
        }

        impl ::std::convert::From<super::InternalError> for InternalError {
            fn from(value: super::InternalError) -> Self {
                Self {
                    request_id: Ok(value.request_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Progress {
            percent: ::std::result::Result<u8, ::std::string::String>,
        }

        impl ::std::default::Default for Progress {
            fn default() -> Self {
                Self {
                    percent: Err("no value supplied for percent".to_string()),
                }
            }
        }

        impl Progress {
            pub fn percent<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u8>,
                T::Error: ::std::fmt::Display,
            {
                self.percent = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for percent: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Progress> for super::Progress {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Progress,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    percent: value.percent?,
                })
            }
        }

        impl ::std::convert::From<super::Progress> for Progress {
            fn from(value: super::Progress) -> Self {
                Self {
                    percent: Ok(value.percent),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Thing {
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for Thing {
            fn default() -> Self {
                Self {
                    name: Err("no value supplied for name".to_string()),
                }
            }
        }

        impl Thing {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Thing> for super::Thing {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Thing,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self { name: value.name? })
            }
        }

        impl ::std::convert::From<super::Thing> for Thing {
            fn from(value: super::Thing) -> Self {
                Self {
                    name: Ok(value.name),
                }
            }
        }
    }

    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
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

    #[allow(unused_imports)]
    use super::{types, ByteStream};
    ///Success responses for `get_thing`
    #[derive(Debug)]
    pub enum GetThingResponses {
        ///Status 200
        Thing(types::Thing),
        ///Status 202
        Response202(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    }

    ///Error responses for `get_thing`
    #[derive(Debug)]
    pub enum GetThingErrorResponses {
        ///Status 400
        BadRequest(types::BadRequest),
        ///Status 404
        Empty,
        ///Status 500, 503
        InternalError(types::InternalError),
    }

    ///Success responses for `put_thing`
    #[derive(Debug)]
    pub enum PutThingResponses {
        ///Status 201
        Thing(types::Thing),
        ///Status 204
        Empty,
        ///Status default
        InternalError(types::InternalError),
    }

    ///Success responses for `get_thing_contents`
    pub enum GetThingContentsResponses {
        ///Status 200
        Stream(ByteStream),
        ///Status 202
        Progress(types::Progress),
    }

    impl ::std::fmt::Debug for GetThingContentsResponses {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Stream(_) => f.debug_tuple("Stream").finish_non_exhaustive(),
                Self::Progress(value) => f.debug_tuple("Progress").field(value).finish(),
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for Multiple response types
///
///Operations whose success or error responses have more than one type
///
///Version: 1.0.0
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
            let dur = ::std::time::Duration::from_secs(15u64);
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
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.0.0"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
impl Client {
    ///Sends a `GET` request to `/things/{id}`
    ///
    ///```ignore
    /// let response = client.get_thing()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_thing(&self) -> builder::GetThing<'_> {
        builder::GetThing::new(self)
    }

    ///Sends a `PUT` request to `/things/{id}`
    ///
    ///```ignore
    /// let response = client.put_thing()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn put_thing(&self) -> builder::PutThing<'_> {
        builder::PutThing::new(self)
    }

    ///Sends a `GET` request to `/things/{id}/contents`
    ///
    ///```ignore
    /// let response = client.get_thing_contents()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_thing_contents(&self) -> builder::GetThingContents<'_> {
        builder::GetThingContents::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, ClientHooks, ClientInfo, Error, OperationInfo, RequestBuilderExt,
        ResponseValue,
    };
    ///Builder for [`Client::get_thing`]
    ///
    ///[`Client::get_thing`]: super::Client::get_thing
    #[derive(Debug, Clone)]
    pub struct GetThing<'a> {
        client: &'a super::Client,
        id: Result<::std::string::String, String>,
    }

    impl<'a> GetThing<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for id failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/things/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetThingResponses>, Error<types::GetThingErrorResponses>>
        {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!("{}/things/{}", client.baseurl, encode_path(&id.to_string()),);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_thing",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => {
                    let value: ResponseValue<types::Thing> =
                        ResponseValue::from_response(response).await?;
                    value.map(types::GetThingResponses::Thing)
                }
                202u16 => {
                    let value: ResponseValue<
                        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                    > = ResponseValue::from_response(response).await?;
                    value.map(types::GetThingResponses::Response202)
                }
                400u16 => {
                    let value: ResponseValue<types::BadRequest> =
                        ResponseValue::from_response(response).await?;
                    value
                        .map(types::GetThingErrorResponses::BadRequest)
                        .and_then(|value| Err(Error::ErrorResponse(value)))
                }
                404u16 => {
                    let value = ResponseValue::empty(response);
                    value
                        .map(|()| types::GetThingErrorResponses::Empty)
                        .and_then(|value| Err(Error::ErrorResponse(value)))
                }
                500u16 => {
                    let value: ResponseValue<types::InternalError> =
                        ResponseValue::from_response(response).await?;
                    value
                        .map(types::GetThingErrorResponses::InternalError)
                        .and_then(|value| Err(Error::ErrorResponse(value)))
                }
                503u16 => {
                    let value: ResponseValue<types::InternalError> =
                        ResponseValue::from_response(response).await?;
                    value
                        .map(types::GetThingErrorResponses::InternalError)
                        .and_then(|value| Err(Error::ErrorResponse(value)))
                }
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::put_thing`]
    ///
    ///[`Client::put_thing`]: super::Client::put_thing
    #[derive(Debug, Clone)]
    pub struct PutThing<'a> {
        client: &'a super::Client,
        id: Result<::std::string::String, String>,
        body: Result<types::builder::Thing, String>,
    }

    impl<'a> PutThing<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::Thing>,
            <V as std::convert::TryInto<types::Thing>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `Thing` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::Thing) -> types::builder::Thing,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/things/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::PutThingResponses>, Error<types::InternalError>> {
            let Self { client, id, body } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::Thing::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/things/{}", client.baseurl, encode_path(&id.to_string()),);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "put_thing",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => {
                    let value: ResponseValue<types::Thing> =
                        ResponseValue::from_response(response).await?;
                    value.map(types::PutThingResponses::Thing)
                }
                204u16 => {
                    let value = ResponseValue::empty(response);
                    value.map(|()| types::PutThingResponses::Empty)
                }
                200..=299 => {
                    let value: ResponseValue<types::InternalError> =
                        ResponseValue::from_response(response).await?;
                    value.map(types::PutThingResponses::InternalError)
                }
                _ => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_thing_contents`]
    ///
    ///[`Client::get_thing_contents`]: super::Client::get_thing_contents
    #[derive(Debug, Clone)]
    pub struct GetThingContents<'a> {
        client: &'a super::Client,
        id: Result<::std::string::String, String>,
    }

    impl<'a> GetThingContents<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for id failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/things/{id}/contents`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetThingContentsResponses>, Error<()>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/things/{}/contents",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_thing_contents",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => {
                    let value = ResponseValue::stream(response);
                    value.map(types::GetThingContentsResponses::Stream)
                }
                202u16 => {
                    let value: ResponseValue<types::Progress> =
                        ResponseValue::from_response(response).await?;
                    value.map(types::GetThingContentsResponses::Progress)
                }
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
