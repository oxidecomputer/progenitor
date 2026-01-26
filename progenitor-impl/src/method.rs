// Copyright 2025 Oxide Computer Company

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use openapiv3::{Components, Parameter, ReferenceOr, Response, StatusCode};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use typify::{TypeId, TypeSpace};

use crate::{
    template::PathTemplate,
    util::{items, parameter_map, sanitize, unique_ident_from, Case},
    Error, Generator, Result, TagStyle,
};
use crate::{to_schema::ToSchema, util::ReferenceOrExt};

/// The intermediate representation of an operation that will become a method.
pub(crate) struct OperationMethod {
    pub operation_id: String,
    pub tags: Vec<String>,
    pub method: HttpMethod,
    pub path: PathTemplate,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub params: Vec<OperationParameter>,
    pub responses: Vec<OperationResponse>,
    pub dropshot_paginated: Option<DropshotPagination>,
    dropshot_websocket: bool,
}

pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
}

impl std::str::FromStr for HttpMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "get" => Ok(Self::Get),
            "put" => Ok(Self::Put),
            "post" => Ok(Self::Post),
            "delete" => Ok(Self::Delete),
            "options" => Ok(Self::Options),
            "head" => Ok(Self::Head),
            "patch" => Ok(Self::Patch),
            "trace" => Ok(Self::Trace),
            _ => Err(Error::InternalError(format!("bad method: {}", s))),
        }
    }
}
impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "get",
            HttpMethod::Put => "put",
            HttpMethod::Post => "post",
            HttpMethod::Delete => "delete",
            HttpMethod::Options => "options",
            HttpMethod::Head => "head",
            HttpMethod::Patch => "patch",
            HttpMethod::Trace => "trace",
        }
    }
}

struct MethodSigBody {
    success: TokenStream,
    error: TokenStream,
    body: TokenStream,
}

struct BuilderImpl {
    doc: String,
    sig: TokenStream,
    body: TokenStream,
}

pub struct DropshotPagination {
    pub item: TypeId,
    pub first_page_params: Vec<String>,
}

pub struct OperationParameter {
    /// Sanitized parameter name.
    pub name: String,
    /// Original parameter name provided by the API.
    pub api_name: String,
    pub description: Option<String>,
    pub typ: OperationParameterType,
    pub kind: OperationParameterKind,
    /// Default value from the schema, if present.
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OperationParameterType {
    Type(TypeId),
    MultipartRelated(TypeId),
    RawBody,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperationParameterKind {
    Path,
    Query(bool),
    Header(bool),
    // TODO bodies may be optional
    Body(BodyContentType),
}

impl OperationParameterKind {
    fn is_required(&self) -> bool {
        match self {
            OperationParameterKind::Path => true,
            OperationParameterKind::Query(required) => *required,
            OperationParameterKind::Header(required) => *required,
            // TODO may be optional
            OperationParameterKind::Body(_) => true,
        }
    }
    fn is_optional(&self) -> bool {
        !self.is_required()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BodyContentType {
    OctetStream,
    Json,
    FormUrlencoded,
    MultipartRelated,
    Text(String),
}

impl FromStr for BodyContentType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let offset = s.find(';').unwrap_or(s.len());
        match &s[..offset] {
            "application/octet-stream" => Ok(Self::OctetStream),
            "application/json" => Ok(Self::Json),
            "application/x-www-form-urlencoded" => Ok(Self::FormUrlencoded),
            "multipart/related" => Ok(Self::MultipartRelated),
            "text/plain" | "text/x-markdown" => Ok(Self::Text(String::from(&s[..offset]))),
            _ => Err(Error::UnexpectedFormat(format!(
                "unexpected content type: {}",
                s
            ))),
        }
    }
}

impl std::fmt::Display for BodyContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::OctetStream => "application/octet-stream",
            Self::Json => "application/json",
            Self::FormUrlencoded => "application/x-www-form-urlencoded",
            Self::MultipartRelated => "multipart/related",
            Self::Text(typ) => typ,
        })
    }
}

#[derive(Debug)]
pub(crate) struct OperationResponse {
    pub status_code: OperationResponseStatus,
    pub typ: OperationResponseKind,
    // TODO this isn't currently used because dropshot doesn't give us a
    // particularly useful message here.
    #[allow(dead_code)]
    description: Option<String>,
}

impl Eq for OperationResponse {}
impl PartialEq for OperationResponse {
    fn eq(&self, other: &Self) -> bool {
        self.status_code == other.status_code
    }
}
impl Ord for OperationResponse {
    fn cmp(&self, other: &Self) -> Ordering {
        self.status_code.cmp(&other.status_code)
    }
}
impl PartialOrd for OperationResponse {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum OperationResponseStatus {
    Code(u16),
    Range(u16),
    Default,
}

impl OperationResponseStatus {
    fn to_value(&self) -> u16 {
        match self {
            OperationResponseStatus::Code(code) => {
                assert!(*code < 1000);
                *code
            }
            OperationResponseStatus::Range(range) => {
                assert!(*range < 10);
                *range * 100
            }
            OperationResponseStatus::Default => 1000,
        }
    }

    pub fn is_success_or_default(&self) -> bool {
        matches!(
            self,
            OperationResponseStatus::Default
                | OperationResponseStatus::Code(101)
                | OperationResponseStatus::Code(200..=299)
                | OperationResponseStatus::Range(2)
        )
    }

    pub fn is_error_or_default(&self) -> bool {
        matches!(
            self,
            OperationResponseStatus::Default
                | OperationResponseStatus::Code(400..=599)
                | OperationResponseStatus::Range(4..=5)
        )
    }

    pub fn is_default(&self) -> bool {
        matches!(self, OperationResponseStatus::Default)
    }
}

impl Ord for OperationResponseStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_value().cmp(&other.to_value())
    }
}

impl PartialOrd for OperationResponseStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum OperationResponseKind {
    Type(TypeId),
    None,
    Raw,
    Upgrade,
}

impl OperationResponseKind {
    pub fn into_tokens(self, type_space: &TypeSpace) -> TokenStream {
        match self {
            OperationResponseKind::Type(ref type_id) => {
                let type_name = type_space.get_type(type_id).unwrap().ident();
                quote! { #type_name }
            }
            OperationResponseKind::None => {
                quote! { () }
            }
            OperationResponseKind::Raw => {
                quote! { ByteStream }
            }
            OperationResponseKind::Upgrade => {
                quote! { reqwest::Upgraded }
            }
        }
    }
}

impl Generator {
    pub(crate) fn process_operation(
        &mut self,
        operation: &openapiv3::Operation,
        components: &Option<Components>,
        path: &str,
        method: &str,
        path_parameters: &[ReferenceOr<Parameter>],
    ) -> Result<OperationMethod> {
        let operation_id = operation.operation_id.as_ref().unwrap();

        let mut combined_path_parameters = parameter_map(path_parameters, components)?;
        for operation_param in items(&operation.parameters, components) {
            let parameter = operation_param?;
            combined_path_parameters.insert(&parameter.parameter_data_ref().name, parameter);
        }

        // Filter out any path parameters that have been overridden by an
        // operation parameter
        let mut params = combined_path_parameters
            .values()
            .map(|parameter| {
                match parameter {
                    openapiv3::Parameter::Path {
                        parameter_data,
                        style: openapiv3::PathStyle::Simple,
                    } => {
                        // Path parameters MUST be required.
                        assert!(parameter_data.required);

                        let schema = parameter_data.schema()?.to_schema();

                        let name = sanitize(
                            &format!("{}-{}", operation_id, &parameter_data.name),
                            Case::Pascal,
                        );
                        let typ = self.type_space.add_type_with_name(&schema, Some(name))?;

                        Ok(OperationParameter {
                            name: sanitize(&parameter_data.name, Case::Snake),
                            api_name: parameter_data.name.clone(),
                            description: parameter_data.description.clone(),
                            typ: OperationParameterType::Type(typ),
                            kind: OperationParameterKind::Path,
                            default: None,
                        })
                    }
                    openapiv3::Parameter::Query {
                        parameter_data,
                        allow_reserved: _, // We always encode reserved chars
                        style: openapiv3::QueryStyle::Form,
                        allow_empty_value: _, // Irrelevant for this client
                    } => {
                        let schema_ref = parameter_data.schema()?;
                        let schema = schema_ref.to_schema();

                        // Extract default value from the schema if present
                        let default_value = match schema_ref {
                            openapiv3::ReferenceOr::Item(schema_kind) => {
                                schema_kind.schema_data.default.clone()
                            }
                            _ => None,
                        };

                        let name = sanitize(
                            &format!(
                                "{}-{}",
                                operation.operation_id.as_ref().unwrap(),
                                &parameter_data.name,
                            ),
                            Case::Pascal,
                        );

                        let type_id = self.type_space.add_type_with_name(&schema, Some(name))?;

                        let ty = self.type_space.get_type(&type_id).unwrap();

                        // If the type is itself optional, then we'll treat it
                        // as optional (irrespective of the `required` field on
                        // the parameter) and use the "inner" type.
                        let details = ty.details();
                        let (type_id, required) =
                            if let typify::TypeDetails::Option(inner_type_id) = details {
                                (inner_type_id, false)
                            } else {
                                (type_id, parameter_data.required)
                            };

                        Ok(OperationParameter {
                            name: sanitize(&parameter_data.name, Case::Snake),
                            api_name: parameter_data.name.clone(),
                            description: parameter_data.description.clone(),
                            typ: OperationParameterType::Type(type_id),
                            kind: OperationParameterKind::Query(required),
                            default: default_value,
                        })
                    }
                    openapiv3::Parameter::Header {
                        parameter_data,
                        style: openapiv3::HeaderStyle::Simple,
                    } => {
                        let schema_ref = parameter_data.schema()?;
                        let schema = schema_ref.to_schema();

                        // Extract default value from the schema if present
                        let default_value = match schema_ref {
                            openapiv3::ReferenceOr::Item(schema_kind) => {
                                schema_kind.schema_data.default.clone()
                            }
                            _ => None,
                        };

                        let name = sanitize(
                            &format!(
                                "{}-{}",
                                operation.operation_id.as_ref().unwrap(),
                                &parameter_data.name,
                            ),
                            Case::Pascal,
                        );

                        let typ = self.type_space.add_type_with_name(&schema, Some(name))?;

                        Ok(OperationParameter {
                            name: sanitize(&parameter_data.name, Case::Snake),
                            api_name: parameter_data.name.clone(),
                            description: parameter_data.description.clone(),
                            typ: OperationParameterType::Type(typ),
                            kind: OperationParameterKind::Header(parameter_data.required),
                            default: default_value,
                        })
                    }
                    openapiv3::Parameter::Path { style, .. } => Err(Error::UnexpectedFormat(
                        format!("unsupported style of path parameter {:#?}", style,),
                    )),
                    openapiv3::Parameter::Query { style, .. } => Err(Error::UnexpectedFormat(
                        format!("unsupported style of query parameter {:#?}", style,),
                    )),
                    cookie @ openapiv3::Parameter::Cookie { .. } => Err(Error::UnexpectedFormat(
                        format!("cookie parameters are not supported {:#?}", cookie,),
                    )),
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let dropshot_websocket = operation.extensions.get("x-dropshot-websocket").is_some();
        if dropshot_websocket {
            self.uses_websockets = true;
        }

        if let Some(body_param) = self.get_body_param(operation, components)? {
            params.push(body_param);
        }

        let tmp = crate::template::parse(path)?;
        let names = tmp.names();

        sort_params(&mut params, &names);

        let mut success = false;

        let mut responses =
            operation
                .responses
                .default
                .iter()
                .map(|response_or_ref| {
                    Ok((
                        OperationResponseStatus::Default,
                        response_or_ref.item(components)?,
                    ))
                })
                .chain(operation.responses.responses.iter().map(
                    |(status_code, response_or_ref)| {
                        Ok((
                            match status_code {
                                StatusCode::Code(code) => OperationResponseStatus::Code(*code),
                                StatusCode::Range(range) => OperationResponseStatus::Range(*range),
                            },
                            response_or_ref.item(components)?,
                        ))
                    },
                ))
                .map(|v: Result<(OperationResponseStatus, &Response)>| {
                    let (status_code, response) = v?;

                    // We categorize responses as "typed" based on the
                    // "application/json" content type, "upgrade" if it's a
                    // websocket channel without a meaningful content-type,
                    // "raw" if there's any other response content type (we don't
                    // investigate further), or "none" if there is no content.
                    // TODO if there are multiple response content types we could
                    // treat those like different response types and create an
                    // enum; the generated client method would check for the
                    // content type of the response just as it currently examines
                    // the status code.
                    let typ = if let Some(mt) = response.content.iter().find_map(|(x, v)| {
                        (x == "application/json" || x.starts_with("application/json;")).then_some(v)
                    }) {
                        assert!(mt.encoding.is_empty());

                        let typ = if let Some(schema) = &mt.schema {
                            let schema = schema.to_schema();
                            let name = sanitize(
                                &format!("{}-response", operation.operation_id.as_ref().unwrap(),),
                                Case::Pascal,
                            );
                            self.type_space.add_type_with_name(&schema, Some(name))?
                        } else {
                            todo!("media type encoding, no schema: {:#?}", mt);
                        };

                        OperationResponseKind::Type(typ)
                    } else if dropshot_websocket {
                        OperationResponseKind::Upgrade
                    } else if response.content.first().is_some() {
                        OperationResponseKind::Raw
                    } else {
                        OperationResponseKind::None
                    };

                    // See if there's a status code that covers success cases.
                    if matches!(
                        status_code,
                        OperationResponseStatus::Default
                            | OperationResponseStatus::Code(200..=299)
                            | OperationResponseStatus::Range(2)
                    ) {
                        success = true;
                    }

                    let description = if response.description.is_empty() {
                        None
                    } else {
                        Some(response.description.clone())
                    };

                    Ok(OperationResponse {
                        status_code,
                        typ,
                        description,
                    })
                })
                .collect::<Result<Vec<_>>>()?;

        // If the API has declined to specify the characteristics of a
        // successful response, we cons up a generic one. Note that this is
        // technically permissible within OpenAPI, but advised against by the
        // spec.
        if !success {
            responses.push(OperationResponse {
                status_code: OperationResponseStatus::Range(2),
                typ: OperationResponseKind::Raw,
                description: None,
            });
        }

        // Must accept HTTP 101 Switching Protocols
        if dropshot_websocket {
            responses.push(OperationResponse {
                status_code: OperationResponseStatus::Code(101),
                typ: OperationResponseKind::Upgrade,
                description: None,
            })
        }

        let dropshot_paginated = self.dropshot_pagination_data(operation, &params, &responses);

        if dropshot_websocket && dropshot_paginated.is_some() {
            return Err(Error::InvalidExtension(format!(
                "conflicting extensions in {:?}",
                operation_id
            )));
        }

        Ok(OperationMethod {
            operation_id: sanitize(operation_id, Case::Snake),
            tags: operation.tags.clone(),
            method: HttpMethod::from_str(method)?,
            path: tmp,
            summary: operation.summary.clone().filter(|s| !s.is_empty()),
            description: operation.description.clone().filter(|s| !s.is_empty()),
            params,
            responses,
            dropshot_paginated,
            dropshot_websocket,
        })
    }

    pub(crate) fn positional_method(
        &mut self,
        method: &OperationMethod,
        has_inner: bool,
    ) -> Result<TokenStream> {
        let operation_id = format_ident!("{}", method.operation_id);

        // Render each parameter as it will appear in the method signature.
        let params = method
            .params
            .iter()
            .map(|param| {
                let name = format_ident!("{}", param.name);
                let typ = match (&param.typ, param.kind.is_optional()) {
                    (OperationParameterType::Type(type_id), false) => self
                        .type_space
                        .get_type(type_id)
                        .unwrap()
                        .parameter_ident_with_lifetime("a"),
                    (OperationParameterType::Type(type_id), true) => {
                        let t = self
                            .type_space
                            .get_type(type_id)
                            .unwrap()
                            .parameter_ident_with_lifetime("a");
                        quote! { Option<#t> }
                    }
                    (OperationParameterType::MultipartRelated(type_id), false) => {
                        let t = self
                            .type_space
                            .get_type(type_id)
                            .unwrap()
                            .parameter_ident_with_lifetime("a");
                        quote! { #t }
                    }
                    (OperationParameterType::MultipartRelated(_), true) => unreachable!(),
                    (OperationParameterType::RawBody, false) => match &param.kind {
                        OperationParameterKind::Body(BodyContentType::OctetStream)
                        | OperationParameterKind::Body(BodyContentType::MultipartRelated) => {
                            quote! { B }
                        }
                        OperationParameterKind::Body(BodyContentType::Text(_)) => {
                            quote! { String }
                        }
                        _ => unreachable!(),
                    },
                    (OperationParameterType::RawBody, true) => unreachable!(),
                };
                quote! {
                    #name: #typ
                }
            })
            .collect::<Vec<_>>();

        let raw_body_param = method.params.iter().any(|param| {
            param.typ == OperationParameterType::RawBody
                && matches!(
                    param.kind,
                    OperationParameterKind::Body(BodyContentType::OctetStream)
                        | OperationParameterKind::Body(BodyContentType::MultipartRelated)
                )
        });

        let bounds = if raw_body_param {
            quote! { <'a, B: Into<reqwest::Body> > }
        } else {
            quote! { <'a> }
        };

        let doc_comment = make_doc_comment(method);

        let MethodSigBody {
            success: success_type,
            error: error_type,
            body,
        } = self.method_sig_body(method, quote! { Self }, quote! { self }, has_inner)?;

        let method_impl = quote! {
            #[doc = #doc_comment]
            pub async fn #operation_id #bounds (
                &'a self,
                #(#params),*
            ) -> Result<
                ResponseValue<#success_type>,
                Error<#error_type>,
            > {
                #body
            }
        };

        let stream_impl = method.dropshot_paginated.as_ref().map(|page_data| {
            // We're now using futures.
            self.uses_futures = true;

            let stream_id = format_ident!("{}_stream", method.operation_id);

            // The parameters are the same as those to the paged method, but
            // without "page_token"
            let stream_params = method
                .params
                .iter()
                .zip(params)
                .filter_map(|(param, stream)| {
                    if param.name.as_str() == "page_token" {
                        None
                    } else {
                        Some(stream)
                    }
                });

            // The values passed to get the first page are the inputs to the
            // stream method with "None" for the page_token.
            let first_params = method.params.iter().map(|param| {
                if param.api_name.as_str() == "page_token" {
                    // The page_token is None when getting the first page.
                    quote! { None }
                } else {
                    // All other parameters are passed through directly.
                    format_ident!("{}", param.name).to_token_stream()
                }
            });

            // The values passed to get subsequent pages are...
            // - the state variable for the page_token
            // - None for all other query parameters
            // - The initial inputs for non-query parameters
            let step_params = method.params.iter().map(|param| {
                if param.api_name.as_str() == "page_token" {
                    quote! { state.as_deref() }
                } else if param.api_name.as_str() != "limit"
                    && matches!(param.kind, OperationParameterKind::Query(_))
                {
                    // Query parameters (other than "page_token" and "limit")
                    // are None; having page_token as Some(_) is mutually
                    // exclusive with other query parameters.
                    quote! { None }
                } else {
                    // Non-query parameters are passed in; this is necessary
                    // e.g. to specify the right path. (We don't really expect
                    // to see a body parameter here, but we pass it through
                    // regardless.)
                    format_ident!("{}", param.name).to_token_stream()
                }
            });

            // The item type that we've saved (by picking apart the original
            // function's return type) will be the Item type parameter for the
            // Stream type we return.
            let item = self.type_space.get_type(&page_data.item).unwrap();
            let item_type = item.ident();

            let doc_comment = make_stream_doc_comment(method);

            quote! {
                #[doc = #doc_comment]
                pub fn #stream_id #bounds (
                    &'a self,
                    #(#stream_params),*
                ) -> impl futures::Stream<Item = Result<
                    #item_type,
                    Error<#error_type>,
                >> + Unpin + 'a {
                    use futures::StreamExt;
                    use futures::TryFutureExt;
                    use futures::TryStreamExt;

                    // Execute the operation with the basic parameters
                    // (omitting page_token) to get the first page.
                    self.#operation_id( #(#first_params,)* )
                        .map_ok(move |page| {
                            let page = page.into_inner();

                            // Create a stream from the items of the first page.
                            let first =
                                futures::stream::iter(page.items).map(Ok);

                            // We unfold subsequent pages using page.next_page
                            // as the seed value. Each iteration returns its
                            // items and the next page token.
                            let rest = futures::stream::try_unfold(
                                page.next_page,
                                move |state| async move {
                                    if state.is_none() {
                                        // The page_token was None so we've
                                        // reached the end.
                                        Ok(None)
                                    } else {
                                        // Get the next page; here we set all
                                        // query parameters to None (except for
                                        // the page_token), and all other
                                        // parameters as specified at the start
                                        // of this method.
                                        self.#operation_id(
                                            #(#step_params,)*
                                        )
                                        .map_ok(|page| {
                                            let page = page.into_inner();
                                            Some((
                                                futures::stream::iter(
                                                    page.items
                                                ).map(Ok),
                                                page.next_page,
                                            ))
                                        })
                                        .await
                                    }
                                },
                            )
                            .try_flatten();

                            first.chain(rest)
                        })
                        .try_flatten_stream()
                        .boxed()
                }
            }
        });

        let all = quote! {
            #method_impl
            #stream_impl
        };

        Ok(all)
    }

    /// Common code generation between positional and builder interface-styles.
    /// Returns a struct with the success and error types and the core body
    /// implementation that marshals arguments and executes the request.
    fn method_sig_body(
        &self,
        method: &OperationMethod,
        client_type: TokenStream,
        client_value: TokenStream,
        has_inner: bool,
    ) -> Result<MethodSigBody> {
        let param_names = method
            .params
            .iter()
            .map(|param| format_ident!("{}", param.name))
            .collect::<Vec<_>>();

        // Generate a unique Ident for internal variables
        let url_ident = unique_ident_from("url", &param_names);
        let request_ident = unique_ident_from("request", &param_names);
        let response_ident = unique_ident_from("response", &param_names);
        let result_ident = unique_ident_from("result", &param_names);

        // Generate code for query parameters.
        let query_params = method
            .params
            .iter()
            .filter_map(|param| match &param.kind {
                OperationParameterKind::Query(_) => {
                    let qn = &param.api_name;
                    let qn_ident = format_ident!("{}", &param.name);
                    Some(quote! {
                        &progenitor_client::QueryParam::new(#qn, &#qn_ident)
                    })
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        let headers = method
            .params
            .iter()
            .filter_map(|param| match &param.kind {
                OperationParameterKind::Header(required) => {
                    let hn = &param.api_name;
                    let hn_ident = format_ident!("{}", &param.name);
                    let res = if *required {
                        quote! {
                            header_map.append(
                                #hn,
                                #hn_ident.to_string().try_into()?
                            );
                        }
                    } else {
                        quote! {
                            if let Some(value) = #hn_ident {
                                header_map.append(
                                    #hn,
                                    value.to_string().try_into()?
                                );
                            }
                        }
                    };
                    Some(res)
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        let headers_size = headers.len() + 1;
        let headers_build = quote! {
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(#headers_size);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(#client_type::api_version()),
            );

            #(#headers)*
        };

        let headers_use = quote! {
            .headers(header_map)
        };

        let websock_hdrs = if method.dropshot_websocket {
            quote! {
                .header(::reqwest::header::CONNECTION, "Upgrade")
                .header(::reqwest::header::UPGRADE, "websocket")
                .header(::reqwest::header::SEC_WEBSOCKET_VERSION, "13")
                .header(
                    ::reqwest::header::SEC_WEBSOCKET_KEY,
                    ::base64::Engine::encode(
                        &::base64::engine::general_purpose::STANDARD,
                        ::rand::random::<[u8; 16]>(),
                    )
                )
            }
        } else {
            quote! {}
        };

        // Generate the path rename map; then use it to generate code for
        // assigning the path parameters to the `url` variable.
        let url_renames = method
            .params
            .iter()
            .filter_map(|param| match &param.kind {
                OperationParameterKind::Path => Some((&param.api_name, &param.name)),
                _ => None,
            })
            .collect();

        let url_path = method.path.compile(url_renames, client_value.clone());
        let url_path = quote! {
            let #url_ident = #url_path;
        };

        // Generate code to handle the body param.
        let body_func = method.params.iter().filter_map(|param| {
            match (&param.kind, &param.typ) {
                (
                    OperationParameterKind::Body(BodyContentType::OctetStream),
                    OperationParameterType::RawBody,
                ) => Some(quote! {
                    // Set the content type (this is handled by helper
                    // functions for other MIME types).
                    .header(
                        ::reqwest::header::CONTENT_TYPE,
                        ::reqwest::header::HeaderValue::from_static("application/octet-stream"),
                    )
                    .body(body)
                }),
                (
                    OperationParameterKind::Body(BodyContentType::MultipartRelated),
                    OperationParameterType::RawBody,
                ) => Some(quote! {
                    // Set the content type for manual multipart/related construction
                    .header(
                        ::reqwest::header::CONTENT_TYPE,
                        ::reqwest::header::HeaderValue::from_static("multipart/related"),
                    )
                    .body(body)
                }),
                (
                    OperationParameterKind::Body(BodyContentType::Text(mime_type)),
                    OperationParameterType::RawBody,
                ) => Some(quote! {
                    // Set the content type (this is handled by helper
                    // functions for other MIME types).
                    .header(
                        ::reqwest::header::CONTENT_TYPE,
                        ::reqwest::header::HeaderValue::from_static(#mime_type),
                    )
                    .body(body)
                }),
                (
                    OperationParameterKind::Body(BodyContentType::Json),
                    OperationParameterType::Type(_),
                ) => Some(quote! {
                    // Serialization errors are deferred.
                    .json(&body)
                }),
                (
                    OperationParameterKind::Body(BodyContentType::FormUrlencoded),
                    OperationParameterType::Type(_),
                ) => Some(quote! {
                    // This uses progenitor_client::RequestBuilderExt which
                    // returns an error in the case of a serialization failure.
                    .form_urlencoded(&body)?
                }),
                (
                    OperationParameterKind::Body(BodyContentType::MultipartRelated),
                    OperationParameterType::MultipartRelated(_),
                ) => Some(quote! {
                    // This uses progenitor_client::RequestBuilderExt which
                    // constructs a multipart/related request body.
                    .multipart_related(&body)?
                }),
                (OperationParameterKind::Body(_), _) => {
                    unreachable!("invalid body kind/type combination")
                }
                _ => None,
            }
        });
        // ... and there can be at most one body.
        assert!(body_func.clone().count() <= 1);

        let (success_response_items, response_type) =
            self.extract_responses(method, OperationResponseStatus::is_success_or_default);

        let success_response_matches = success_response_items.iter().map(|response| {
            let pat = match &response.status_code {
                OperationResponseStatus::Code(code) => quote! { #code },
                OperationResponseStatus::Range(_) | OperationResponseStatus::Default => {
                    quote! { 200 ..= 299 }
                }
            };

            let decode = match &response.typ {
                OperationResponseKind::Type(_) => {
                    quote! {
                        ResponseValue::from_response(#response_ident).await
                    }
                }
                OperationResponseKind::None => {
                    quote! {
                        Ok(ResponseValue::empty(#response_ident))
                    }
                }
                OperationResponseKind::Raw => {
                    quote! {
                        Ok(ResponseValue::stream(#response_ident))
                    }
                }
                OperationResponseKind::Upgrade => {
                    quote! {
                        ResponseValue::upgrade(#response_ident).await
                    }
                }
            };

            quote! { #pat => { #decode } }
        });

        // Errors...
        let (error_response_items, error_type) =
            self.extract_responses(method, OperationResponseStatus::is_error_or_default);

        let error_response_matches = error_response_items.iter().map(|response| {
            let pat = match &response.status_code {
                OperationResponseStatus::Code(code) => {
                    quote! { #code }
                }
                OperationResponseStatus::Range(r) => {
                    let min = r * 100;
                    let max = min + 99;
                    quote! { #min ..= #max }
                }

                OperationResponseStatus::Default => {
                    quote! { _ }
                }
            };

            let decode = match &response.typ {
                OperationResponseKind::Type(_) => {
                    quote! {
                        Err(Error::ErrorResponse(
                            ResponseValue::from_response(#response_ident)
                                .await?
                        ))
                    }
                }
                OperationResponseKind::None => {
                    quote! {
                        Err(Error::ErrorResponse(
                            ResponseValue::empty(#response_ident)
                        ))
                    }
                }
                OperationResponseKind::Raw => {
                    quote! {
                        Err(Error::ErrorResponse(
                            ResponseValue::stream(#response_ident)
                        ))
                    }
                }
                OperationResponseKind::Upgrade => {
                    if response.status_code == OperationResponseStatus::Default {
                        return quote! {}; // catch-all handled below
                    } else {
                        todo!(
                            "non-default error response handling for \
                                upgrade requests is not yet implemented"
                        );
                    }
                }
            };

            quote! { #pat => { #decode } }
        });

        let accept_header = matches!(
            (&response_type, &error_type),
            (OperationResponseKind::Type(_), _)
                | (OperationResponseKind::None, OperationResponseKind::Type(_))
        )
        .then(|| {
            quote! {
                    .header(
                        ::reqwest::header::ACCEPT,
                        ::reqwest::header::HeaderValue::from_static(
                            "application/json",
                        ),
                    )
            }
        });

        // Generate the catch-all case for other statuses. If the operation
        // specifies a default response, we've already generated a default
        // match as part of error response code handling. (And we've handled
        // the default as a success response as well.) Otherwise the catch-all
        // produces an error corresponding to a response not specified in the
        // API description.
        let default_response = match method.responses.iter().last() {
            Some(response) if response.status_code.is_default() => quote! {},
            _ => {
                quote! { _ => Err(Error::UnexpectedResponse(#response_ident)), }
            }
        };

        let inner = match has_inner {
            true => quote! { &#client_value.inner, },
            false => quote! {},
        };
        let pre_hook = self.settings.pre_hook.as_ref().map(|hook| {
            quote! {
                (#hook)(#inner &#request_ident);
            }
        });
        let pre_hook_async = self.settings.pre_hook_async.as_ref().map(|hook| {
            quote! {
                match (#hook)(#inner &mut #request_ident).await {
                    Ok(_) => (),
                    Err(e) => return Err(Error::Custom(e.to_string())),
                }
            }
        });
        let post_hook = self.settings.post_hook.as_ref().map(|hook| {
            quote! {
                (#hook)(#inner &#result_ident);
            }
        });
        let post_hook_async = self.settings.post_hook_async.as_ref().map(|hook| {
            quote! {
                match (#hook)(#inner &#result_ident).await {
                    Ok(_) => (),
                    Err(e) => return Err(Error::Custom(e.to_string())),
                }
            }
        });

        let operation_id = &method.operation_id;
        let method_func = format_ident!("{}", method.method.as_str());

        let body_impl = quote! {
            #url_path

            #headers_build

            #[allow(unused_mut)]
            let mut #request_ident = #client_value.client
                . #method_func (#url_ident)
                #accept_header
                #(#body_func)*
                #( .query(#query_params) )*
                #headers_use
                #websock_hdrs
                .build()?;

            let info = OperationInfo {
                operation_id: #operation_id,
            };

            #pre_hook
            #pre_hook_async
            #client_value
                .pre(&mut #request_ident, &info)
                .await?;

            let #result_ident = #client_value
                .exec(#request_ident, &info)
                .await;

            #client_value
                .post(&#result_ident, &info)
                .await?;
            #post_hook_async
            #post_hook

            let #response_ident = #result_ident?;

            match #response_ident.status().as_u16() {
                // These will be of the form...
                // 201 => ResponseValue::from_response(response).await,
                // 200..299 => ResponseValue::empty(response),
                // TODO this kind of enumerated response isn't implemented
                // ... or in the case of an operation with multiple
                // successful response types...
                // 200 => {
                //     ResponseValue::from_response()
                //         .await?
                //         .map(OperationXResponse::ResponseTypeA)
                // }
                // 201 => {
                //     ResponseValue::from_response()
                //         .await?
                //         .map(OperationXResponse::ResponseTypeB)
                // }
                #(#success_response_matches)*

                // This is almost identical to the success types except
                // they are wrapped in Error::ErrorResponse...
                // 400 => {
                //     Err(Error::ErrorResponse(
                //         ResponseValue::from_response(response.await?)
                //     ))
                // }
                #(#error_response_matches)*

                // The default response is either an Error with a known
                // type if the operation defines a default (as above) or
                // an Error::UnexpectedResponse...
                // _ => Err(Error::UnexpectedResponse(response)),
                #default_response
            }
        };

        Ok(MethodSigBody {
            success: response_type.into_tokens(&self.type_space),
            error: error_type.into_tokens(&self.type_space),
            body: body_impl,
        })
    }

    /// Extract responses that match criteria specified by the `filter`. The
    /// result is a `Vec<OperationResponse>` that enumerates the cases matching
    /// the filter, and a `TokenStream` that represents the generated type for
    /// those cases.
    pub(crate) fn extract_responses<'a>(
        &self,
        method: &'a OperationMethod,
        filter: fn(&OperationResponseStatus) -> bool,
    ) -> (Vec<&'a OperationResponse>, OperationResponseKind) {
        let mut response_items = method
            .responses
            .iter()
            .filter(|response| filter(&response.status_code))
            .collect::<Vec<_>>();
        response_items.sort();

        // If we have a success range and a default, we can pop off the default
        // since it will never be hit. Note that this is a no-op for error
        // responses.
        let len = response_items.len();
        if len >= 2 {
            if let (
                OperationResponse {
                    status_code: OperationResponseStatus::Range(2),
                    ..
                },
                OperationResponse {
                    status_code: OperationResponseStatus::Default,
                    ..
                },
            ) = (&response_items[len - 2], &response_items[len - 1])
            {
                response_items.pop();
            }
        }

        let response_types = response_items
            .iter()
            .map(|response| response.typ.clone())
            .collect::<BTreeSet<_>>();

        // TODO to deal with multiple response types, we'll need to create an
        // enum type with variants for each of the response types.
        assert!(response_types.len() <= 1);
        let response_type = response_types
            .into_iter()
            .next()
            // TODO should this be OperationResponseType::Raw?
            .unwrap_or(OperationResponseKind::None);
        (response_items, response_type)
    }

    // Validates all the necessary conditions for Dropshot pagination. Returns
    // the paginated item type data if all conditions are met.
    fn dropshot_pagination_data(
        &self,
        operation: &openapiv3::Operation,
        parameters: &[OperationParameter],
        responses: &[OperationResponse],
    ) -> Option<DropshotPagination> {
        let value = operation.extensions.get("x-dropshot-pagination")?;

        // We expect to see at least "page_token" and "limit" parameters.
        if parameters
            .iter()
            .filter(|param| {
                matches!(
                    (param.api_name.as_str(), &param.kind),
                    ("page_token", OperationParameterKind::Query(false))
                        | ("limit", OperationParameterKind::Query(false))
                )
            })
            .count()
            != 2
        {
            return None;
        }

        // All query parameters must be optional since page_token may not be
        // specified in conjunction with other query parameters.
        if !parameters.iter().all(|param| match &param.kind {
            OperationParameterKind::Query(required) => !required,
            _ => true,
        }) {
            return None;
        }

        // A raw body parameter can only be passed to a single call as it may
        // be a streaming type. We can't use a streaming type for a paginated
        // interface because we can only stream it once rather than for the
        // multiple calls required to collect all pages.
        if parameters
            .iter()
            .any(|param| param.typ == OperationParameterType::RawBody)
        {
            return None;
        }

        // There must be exactly one successful response type.
        let mut success_response_items =
            responses
                .iter()
                .filter_map(|response| match (&response.status_code, &response.typ) {
                    (
                        OperationResponseStatus::Code(200..=299)
                        | OperationResponseStatus::Range(2),
                        OperationResponseKind::Type(type_id),
                    ) => Some(type_id),
                    _ => None,
                });

        let success_response = match (success_response_items.next(), success_response_items.next())
        {
            (None, _) | (_, Some(_)) => return None,
            (Some(success), None) => success,
        };

        let typ = self.type_space.get_type(success_response).ok()?;
        let details = match typ.details() {
            typify::TypeDetails::Struct(details) => details,
            _ => return None,
        };

        let properties = details.properties().collect::<BTreeMap<_, _>>();

        // There should be exactly two properties: items and next_page
        if properties.len() != 2 {
            return None;
        }

        // We need a next_page property that's an Option<String>.
        if let typify::TypeDetails::Option(ref opt_id) = self
            .type_space
            .get_type(properties.get("next_page")?)
            .ok()?
            .details()
        {
            if !matches!(
                self.type_space.get_type(opt_id).ok()?.details(),
                typify::TypeDetails::String
            ) {
                return None;
            }
        } else {
            return None;
        }

        match self
            .type_space
            .get_type(properties.get("items")?)
            .ok()?
            .details()
        {
            typify::TypeDetails::Vec(item) => {
                #[derive(serde::Deserialize, Default)]
                struct DropshotPaginationFormat {
                    required: Vec<String>,
                }
                let first_page_params =
                    serde_json::from_value::<DropshotPaginationFormat>(value.clone())
                        .unwrap_or_default()
                        .required;
                Some(DropshotPagination {
                    item,
                    first_page_params,
                })
            }
            _ => None,
        }
    }

    /// Create the builder structs along with their impl bodies.
    ///
    /// Builder structs are generally of this form for a mandatory `param_1`
    /// and an optional `param_2`:
    /// ```ignore
    /// struct OperationId<'a> {
    ///     client: &'a super::Client,
    ///     param_1: Result<SomeType, String>,
    ///     param_2: Result<Option<String>, String>,
    /// }
    /// ```
    ///
    /// All parameters are present and all their types are `Result<T, String>`
    /// or `Result<Option<T>, String>` for optional parameters. Each parameter
    /// also has a corresponding method:
    /// ```ignore
    /// impl<'a> OperationId<'a> {
    ///     pub fn param_1<V>(self, value: V)
    ///         where V: std::convert::TryInto<SomeType>
    ///     {
    ///         self.param_1 = value.try_into()
    ///             .map_err(|_| #err_msg.to_string());
    ///         self
    ///     }
    ///     pub fn param_2<V>(self, value: V)
    ///         where V: std::convert::TryInto<SomeType>
    ///     {
    ///         self.param_2 = value.try_into()
    ///             .map(Some)
    ///             .map_err(|_| #err_msg.to_string());
    ///         self
    ///     }
    /// }
    /// ```
    ///
    /// The Client's operation_id method simply invokes the builder's new
    /// method, which assigns an error value to mandatory field and a
    /// `Ok(None)` value to optional ones:
    /// ```ignore
    /// impl<'a> OperationId<'a> {
    ///     pub fn new(client: &'a super::Client) -> Self {
    ///         Self {
    ///             client,
    ///             param_1: Err("param_1 was not initialized".to_string()),
    ///             param_2: Ok(None),
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// Finally, builders have methods to execute the operation. This simply
    /// resolves each parameter with the ? (`Try` operator).
    /// ```ignore
    /// impl<'a> OperationId<'a> {
    ///     pub fn send(self) -> Result<
    ///         ResponseValue<SuccessType>,
    ///         Error<ErrorType>,
    ///     > {
    ///         let Self {
    ///             client,
    ///             param_1,
    ///             param_2,
    ///         } = self;
    ///     
    ///         let param_1 = param_1.map_err(Error::InvalidRequest)?;
    ///         let param_2 = param_1.map_err(Error::InvalidRequest)?;
    ///
    ///         // ... execute the body (see `method_sig_body`) ...
    ///     }
    /// }
    /// ```
    ///
    /// Finally, paginated interfaces have a `stream()` method which uses the
    /// `send()` method above to fetch each page of results to assemble the
    /// items into a single `impl Stream`.
    pub(crate) fn builder_struct(
        &mut self,
        method: &OperationMethod,
        tag_style: TagStyle,
        has_inner: bool,
    ) -> Result<TokenStream> {
        let struct_name = sanitize(&method.operation_id, Case::Pascal);
        let struct_ident = format_ident!("{}", struct_name);

        // Expanded parameters: MultipartRelated params are expanded into their
        // constituent properties
        #[derive(Clone)]
        struct ExpandedParam {
            name: String,
            type_id: TypeId,
            required: bool,
            is_binary: bool,
            default: Option<serde_json::Value>,
        }

        let expanded_params: Vec<ExpandedParam> = method
            .params
            .iter()
            .flat_map(|param| {
                match &param.typ {
                    OperationParameterType::MultipartRelated(type_id) => {
                        // Expand into individual properties
                        let ty = self.type_space.get_type(type_id).unwrap();
                        let typify::TypeDetails::Struct(struct_details) = ty.details() else {
                            panic!("multipart type must be a struct");
                        };

                        struct_details
                            .properties()
                            .filter_map(|(prop_name, prop_type_id)| {
                                // Skip content_type fields - they're metadata, not builder params
                                if prop_name.ends_with("_content_type") {
                                    return None;
                                }

                                // Check if this property is binary by looking at the type
                                let prop_ty = self.type_space.get_type(&prop_type_id).ok()?;
                                let is_binary = if let typify::TypeDetails::Vec(inner_id) = prop_ty.details() {
                                    // Vec<u8> is binary
                                    if let Ok(inner_ty) = self.type_space.get_type(&inner_id) {
                                        inner_ty.ident().to_string() == "u8"
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                };

                                Some(ExpandedParam {
                                    name: prop_name.to_string(),
                                    type_id: prop_type_id,
                                    required: true, // TODO: Check if property is required
                                    is_binary,
                                    default: None, // Multipart properties don't have defaults
                                })
                            })
                            .collect::<Vec<_>>()
                    }
                    OperationParameterType::Type(type_id) => {
                        let ty = self.type_space.get_type(type_id).unwrap();
                        // Skip body parameters with builders - they use the old approach
                        if matches!(&param.kind, OperationParameterKind::Body(_)) && ty.builder().is_some() {
                            vec![]
                        } else {
                            vec![ExpandedParam {
                                name: param.name.clone(),
                                type_id: type_id.clone(),
                                required: param.kind.is_required(),
                                is_binary: false,
                                default: param.default.clone(),
                            }]
                        }
                    }
                    OperationParameterType::RawBody => {
                        // RawBody doesn't have a TypeId, handle separately
                        vec![]
                    }
                }
            })
            .collect();

        // Generate an ident for each expanded parameter
        let param_names = expanded_params
            .iter()
            .map(|ep| format_ident!("{}", ep.name))
            .chain(
                // Add content_type fields for binary expanded params
                expanded_params.iter().filter_map(|ep| {
                    if ep.is_binary {
                        Some(format_ident!("{}_content_type", ep.name))
                    } else {
                        None
                    }
                })
            )
            .chain(
                // Add Type params with builders
                method.params.iter().filter_map(|param| {
                    match &param.typ {
                        OperationParameterType::Type(type_id) => {
                            let ty = self.type_space.get_type(type_id).ok()?;
                            if let (OperationParameterKind::Body(_), Some(_)) =
                                (&param.kind, ty.builder())
                            {
                                Some(format_ident!("{}", param.name))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
            )
            .chain(
                // Add RawBody params separately
                method.params.iter().filter_map(|param| {
                    if param.typ == OperationParameterType::RawBody {
                        Some(format_ident!("{}", param.name))
                    } else {
                        None
                    }
                })
            )
            .collect::<Vec<_>>();

        let client_ident = unique_ident_from("client", &param_names);

        let mut cloneable = true;

        // Generate the type for each expanded parameter
        let param_types = expanded_params
            .iter()
            .map(|ep| {
                let ty = self.type_space.get_type(&ep.type_id)?;

                // For binary fields in multipart, use Vec<u8>
                let type_token = if ep.is_binary {
                    quote! { Vec<u8> }
                } else {
                    let t = ty.ident();
                    quote! { #t }
                };

                if ep.required {
                    Ok(quote! { Result<#type_token, String> })
                } else {
                    Ok(quote! { Result<Option<#type_token>, String> })
                }
            })
            .chain(
                // Add content_type fields for binary expanded params
                expanded_params.iter().filter_map(|ep| {
                    if ep.is_binary {
                        // content_type is always required (String, not Option<String>)
                        Some(Ok(quote! { Result<String, String> }))
                    } else {
                        None
                    }
                })
            )
            .chain(
                // Add Type params that weren't expanded (non-multipart bodies with builders)
                method.params.iter().filter_map(|param| {
                    match &param.typ {
                        OperationParameterType::Type(type_id) => {
                            let ty = self.type_space.get_type(type_id).ok()?;
                            // Only handle builder-capable bodies here (others were expanded)
                            if let (OperationParameterKind::Body(_), Some(builder_name)) =
                                (&param.kind, ty.builder())
                            {
                                Some(Ok(quote! { Result<#builder_name, String> }))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
            )
            .chain(
                // Add RawBody params
                method.params.iter().filter_map(|param| {
                    if param.typ == OperationParameterType::RawBody {
                        cloneable = false;
                        Some(Ok(quote! { Result<reqwest::Body, String> }))
                    } else {
                        None
                    }
                })
            )
            .collect::<Result<Vec<_>>>()?;

        // Generate the default value for each expanded parameter
        let param_values = expanded_params
            .iter()
            .map(|ep| {
                if ep.required {
                    let err_msg = format!("{} was not initialized", ep.name);
                    Ok(quote! { Err(#err_msg.to_string()) })
                } else if ep.default.is_some() {
                    // Optional parameter with a default value - use Default::default()
                    Ok(quote! { Ok(Some(::std::default::Default::default())) })
                } else {
                    // Optional parameter without a default value
                    Ok(quote! { Ok(None) })
                }
            })
            .chain(
                // Add default values for content_type fields (all required, start uninitialized)
                expanded_params.iter().filter_map(|ep| {
                    if ep.is_binary {
                        let err_msg = format!("{}_content_type was not initialized", ep.name);
                        Some(Ok(quote! { Err(#err_msg.to_string()) }))
                    } else {
                        None
                    }
                })
            )
            .chain(
                // Add default values for Type params with builders
                method.params.iter().filter_map(|param| {
                    match &param.typ {
                        OperationParameterType::Type(type_id) => {
                            let ty = self.type_space.get_type(type_id).ok()?;
                            // Only handle builder-capable bodies here
                            if let (OperationParameterKind::Body(_), Some(_)) =
                                (&param.kind, ty.builder())
                            {
                                Some(Ok(quote! { Ok(::std::default::Default::default()) }))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
            )
            .chain(
                // Add RawBody params
                method.params.iter().filter_map(|param| {
                    if param.typ == OperationParameterType::RawBody {
                        let err_msg = format!("{} was not initialized", param.name);
                        Some(Ok(quote! { Err(#err_msg.to_string()) }))
                    } else {
                        None
                    }
                })
            )
            .collect::<Result<Vec<_>>>()?;

        // Build param_finalize: for builder-capable bodies, convert Builder → Type
        let param_finalize = method.params.iter().flat_map(|param| {
            match &param.typ {
                OperationParameterType::MultipartRelated(_) => {
                    // Multipart params were expanded into individual fields
                    let ty = self.type_space.get_type(
                        if let OperationParameterType::MultipartRelated(type_id) = &param.typ {
                            type_id
                        } else {
                            unreachable!()
                        }
                    ).unwrap();
                    let typify::TypeDetails::Struct(struct_details) = ty.details() else {
                        panic!("multipart type must be a struct");
                    };
                    // Each property gets no finalization (already right type)
                    struct_details.properties().map(|_| quote! {}).collect::<Vec<_>>()
                }
                OperationParameterType::Type(type_id) => {
                    let ty = self.type_space.get_type(type_id).unwrap();
                    if matches!(&param.kind, OperationParameterKind::Body(_)) && ty.builder().is_some() {
                        // Builder-capable body: convert Builder → Type
                        let type_name = ty.ident();
                        vec![quote! {
                            .and_then(|v| #type_name::try_from(v).map_err(|e| e.to_string()))
                        }]
                    } else {
                        // Non-body Type param: no finalization needed
                        vec![quote! {}]
                    }
                }
                OperationParameterType::RawBody => {
                    // RawBody: no finalization needed
                    vec![quote! {}]
                }
            }
        }).collect::<Vec<_>>();

        // For each expanded parameter, generate a setter method
        let param_impls = expanded_params
            .iter()
            .map(|ep| {
                let param_name = format_ident!("{}", ep.name);
                let ty = self.type_space.get_type(&ep.type_id)?;

                // For binary fields, generate a method that accepts both value and content_type
                if ep.is_binary {
                    let content_type_field = format_ident!("{}_content_type", ep.name);
                    let err_msg = format!("conversion to Vec<u8> for {} failed", ep.name);
                    let content_type_err = format!("conversion to String for {}_content_type failed", ep.name);

                    if ep.required {
                        Ok(quote! {
                            pub fn #param_name<V, C>(mut self, value: V, content_type: C) -> Self
                            where
                                V: std::convert::TryInto<Vec<u8>>,
                                C: std::convert::TryInto<String>,
                            {
                                self.#param_name = value.try_into()
                                    .map_err(|_| #err_msg.to_string());
                                self.#content_type_field = content_type.try_into()
                                    .map_err(|_| #content_type_err.to_string());
                                self
                            }
                        })
                    } else {
                        Ok(quote! {
                            pub fn #param_name<V, C>(mut self, value: V, content_type: C) -> Self
                            where
                                V: std::convert::TryInto<Vec<u8>>,
                                C: std::convert::TryInto<String>,
                            {
                                self.#param_name = value.try_into()
                                    .map(Some)
                                    .map_err(|_| #err_msg.to_string());
                                self.#content_type_field = content_type.try_into()
                                    .map(Some)
                                    .map_err(|_| #content_type_err.to_string());
                                self
                            }
                        })
                    }
                } else {
                    // Non-binary fields: use the generated type
                    let typ = ty.ident().to_token_stream();
                    let err_msg = format!("conversion to `{}` for {} failed", ty.name(), ep.name);

                    if ep.required {
                        Ok(quote! {
                            pub fn #param_name<V>(mut self, value: V) -> Self
                            where
                                V: std::convert::TryInto<#typ>,
                            {
                                self.#param_name = value.try_into()
                                    .map_err(|_| #err_msg.to_string());
                                self
                            }
                        })
                    } else {
                        Ok(quote! {
                            pub fn #param_name<V>(mut self, value: V) -> Self
                            where
                                V: std::convert::TryInto<#typ>,
                            {
                                self.#param_name = value.try_into()
                                    .map(Some)
                                    .map_err(|_| #err_msg.to_string());
                                self
                            }
                        })
                    }
                }
            })
            .chain(
                // Add RawBody param methods
                method.params.iter().filter_map(|param| {
                    if param.typ == OperationParameterType::RawBody {
                        let param_name = format_ident!("{}", param.name);
                        match &param.kind {
                            OperationParameterKind::Body(BodyContentType::OctetStream)
                            | OperationParameterKind::Body(BodyContentType::MultipartRelated) => {
                                let err_msg = format!("conversion to `reqwest::Body` for {} failed", param.name);
                                Some(Ok(quote! {
                                    pub fn #param_name<B>(mut self, value: B) -> Self
                                        where B: std::convert::TryInto<reqwest::Body>
                                    {
                                        self.#param_name = value.try_into()
                                            .map_err(|_| #err_msg.to_string());
                                        self
                                    }
                                }))
                            }
                            OperationParameterKind::Body(BodyContentType::Text(_)) => {
                                let err_msg = format!("conversion to `String` for {} failed", param.name);
                                Some(Ok(quote! {
                                    pub fn #param_name<V>(mut self, value: V) -> Self
                                        where V: std::convert::TryInto<String>
                                    {
                                        self.#param_name = value
                                            .try_into()
                                            .map_err(|_| #err_msg.to_string())
                                            .map(|v| v.into());
                                        self
                                    }
                                }))
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
            )
            .chain(
                // Add methods for Type parameters with builders (body() and body_map())
                method.params.iter().filter_map(|param| {
                    match &param.typ {
                        OperationParameterType::Type(type_id) => {
                            let ty = self.type_space.get_type(type_id).ok()?;

                            // For builder-capable bodies, generate both body() and body_map()
                            if let (OperationParameterKind::Body(_), Some(builder_name)) =
                                (&param.kind, ty.builder())
                            {
                                assert_eq!(param.name, "body");
                                let typ = ty.ident();
                                let err_msg = format!(
                                    "conversion to `{}` for {} failed: {{}}",
                                    ty.name(),
                                    param.name,
                                );
                                Some(Ok(quote! {
                                    pub fn body<V>(mut self, value: V) -> Self
                                    where
                                        V: std::convert::TryInto<#typ>,
                                        <V as std::convert::TryInto<#typ>>::Error:
                                            std::fmt::Display,
                                    {
                                        self.body = value.try_into()
                                            .map(From::from)
                                            .map_err(|s| format!(#err_msg, s));
                                        self
                                    }

                                    pub fn body_map<F>(mut self, f: F) -> Self
                                    where
                                        F: std::ops::FnOnce(#builder_name)
                                            -> #builder_name,
                                    {
                                        self.body = self.body.map(f);
                                        self
                                    }
                                }))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
            )
            .collect::<Result<Vec<_>>>()?;

        let MethodSigBody {
            success,
            error,
            body,
        } = self.method_sig_body(
            method,
            quote! { super::Client },
            quote! { #client_ident },
            has_inner,
        )?;

        let send_doc = format!(
            "Sends a `{}` request to `{}`",
            method.method.as_str().to_ascii_uppercase(),
            method.path,
        );
        // Identify which multipart structs need to be reconstructed
        let multipart_reconstructions = method.params.iter().filter_map(|param| {
            if let OperationParameterType::MultipartRelated(type_id) = &param.typ {
                let ty = self.type_space.get_type(type_id).unwrap();
                let type_name = ty.ident();
                let parent_param_name = format_ident!("{}", param.name);

                let typify::TypeDetails::Struct(struct_details) = ty.details() else {
                    panic!("multipart type must be a struct");
                };

                // Generate field assignments
                let field_assignments = struct_details.properties().map(|(prop_name, _)| {
                    let prop_ident = format_ident!("{}", prop_name);
                    quote! { #prop_ident }
                }).collect::<Vec<_>>();

                Some(quote! {
                    let #parent_param_name = #type_name {
                        #(#field_assignments),*
                    };
                })
            } else {
                None
            }
        }).collect::<Vec<_>>();

        let send_impl = quote! {
            #[doc = #send_doc]
            pub async fn send(self) -> Result<
                ResponseValue<#success>,
                Error<#error>,
            > {
                // Destructure the builder for convenience.
                let Self {
                    #client_ident,
                    #( #param_names, )*
                } = self;

                // Extract parameters into variables, returning an error if
                // a value has not been provided or there was a conversion
                // error.
                //
                // TODO we could do something a bit nicer by collecting all
                // errors rather than just reporting the first one.
                #(
                let #param_names =
                    #param_names
                        #param_finalize
                        .map_err(Error::InvalidRequest)?;
                )*

                // Reconstruct multipart structs from individual properties
                #(#multipart_reconstructions)*

                // Do the work.
                #body
            }
        };

        let stream_impl = method.dropshot_paginated.as_ref().map(|page_data| {
            // We're now using futures.
            self.uses_futures = true;

            let step_params = method.params.iter().filter_map(|param| {
                if param.api_name.as_str() != "limit"
                    && matches!(param.kind, OperationParameterKind::Query(_))
                {
                    // Query parameters (other than "limit") are None; having
                    // page_token as Some(_), as we will during the loop below,
                    // is mutually exclusive with other query parameters.
                    let name = format_ident!("{}", param.name);
                    Some(quote! {
                        #name: Ok(None)
                    })
                } else {
                    None
                }
            });

            // The item type that we've saved (by picking apart the original
            // function's return type) will be the Item type parameter for the
            // Stream impl we return.
            let item = self.type_space.get_type(&page_data.item).unwrap();
            let item_type = item.ident();

            let stream_doc = format!(
                "Streams `{}` requests to `{}`",
                method.method.as_str().to_ascii_uppercase(),
                method.path,
            );

            quote! {
                #[doc = #stream_doc]
                pub fn stream(self) -> impl futures::Stream<Item = Result<
                    #item_type,
                    Error<#error>,
                >> + Unpin + 'a {
                    use ::futures::StreamExt;
                    use ::futures::TryFutureExt;
                    use ::futures::TryStreamExt;

                    // This is the builder template we'll use for iterative
                    // steps past the first; it has all query params set to
                    // None (the step will fill in page_token).
                    let next = Self {
                        #( #step_params, )*
                        ..self.clone()
                    };

                    self.send()
                        .map_ok(move |page| {
                            let page = page.into_inner();

                            // Create a stream from the first page of items.
                            let first =
                                futures::stream::iter(page.items).map(Ok);

                            // We unfold subsequent pages using page.next_page
                            // as the seed value. Each iteration returns its
                            // items and the new state which is a tuple of the
                            // next page token and the Self template.
                            let rest = futures::stream::try_unfold(
                                (page.next_page, next),
                                |(next_page, next)| async {
                                    if next_page.is_none() {
                                        // The page_token was None so we've
                                        // reached the end.
                                        Ok(None)
                                    } else {
                                        // Get the next page using the next
                                        // template (with query parameters set
                                        // to None), overriding page_token.
                                        Self {
                                            page_token: Ok(next_page),
                                            ..next.clone()
                                        }
                                        .send()
                                        .map_ok(|page| {
                                            let page = page.into_inner();
                                            Some((
                                                futures::stream::iter(
                                                    page.items
                                                ).map(Ok),
                                                (page.next_page, next),
                                            ))
                                        })
                                        .await
                                    }
                                },
                            )
                            .try_flatten();

                            first.chain(rest)
                        })
                        .try_flatten_stream()
                        .boxed()
                }
            }
        });

        let mut derives = vec![quote! { Debug }];
        if cloneable {
            derives.push(quote! { Clone });
        }

        let derive = quote! {
            #[derive( #( #derives ),* )]
        };

        // Build a reasonable doc comment depending on whether this struct is
        // the output from
        // 1. A Client method
        // 2. An extension trait method
        // 3. Several extension trait methods
        let struct_doc = match (tag_style, method.tags.len(), method.tags.first()) {
            (TagStyle::Merged, _, _) | (TagStyle::Separate, 0, _) => {
                let ty = format!("Client::{}", method.operation_id);
                format!("Builder for [`{}`]\n\n[`{}`]: super::{}", ty, ty, ty,)
            }
            (TagStyle::Separate, 1, Some(tag)) => {
                let ty = format!(
                    "Client{}Ext::{}",
                    sanitize(tag, Case::Pascal),
                    method.operation_id
                );
                format!("Builder for [`{}`]\n\n[`{}`]: super::{}", ty, ty, ty,)
            }
            (TagStyle::Separate, _, _) => {
                format!(
                    "Builder for `{}` operation\n\nSee {}\n\n{}",
                    method.operation_id,
                    method
                        .tags
                        .iter()
                        .map(|tag| {
                            format!(
                                "[`Client{}Ext::{}`]",
                                sanitize(tag, Case::Pascal),
                                method.operation_id,
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(", "),
                    method
                        .tags
                        .iter()
                        .map(|tag| {
                            let ty = format!(
                                "Client{}Ext::{}",
                                sanitize(tag, Case::Pascal),
                                method.operation_id,
                            );
                            format!("[`{}`]: super::{}", ty, ty)
                        })
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            }
        };

        Ok(quote! {
            #[doc = #struct_doc]
            #derive
            pub struct #struct_ident<'a> {
                #client_ident: &'a super::Client,
                #( #param_names: #param_types, )*
            }

            impl<'a> #struct_ident<'a> {
                pub fn new(client: &'a super::Client) -> Self {
                    Self {
                        #client_ident: client,
                        #( #param_names: #param_values, )*
                    }
                }

                #( #param_impls )*
                #send_impl
                #stream_impl
            }
        })
    }

    fn builder_helper(&self, method: &OperationMethod) -> BuilderImpl {
        let operation_id = format_ident!("{}", method.operation_id);
        let struct_name = sanitize(&method.operation_id, Case::Pascal);
        let struct_ident = format_ident!("{}", struct_name);

        let params = method
            .params
            .iter()
            .map(|param| format!("\n    .{}({})", param.name, param.name))
            .collect::<Vec<_>>()
            .join("");

        let eg = format!(
            "\
            let response = client.{}(){}
    .send()
    .await;",
            method.operation_id, params,
        );

        // Note that it would be nice to have a non-ignored example that could
        // be validated by doc tests, but in order to use the Client we need
        // to import it, and in order to import it we need to know the name of
        // the containing crate... which we can't from this context.
        let doc = format!("{}```ignore\n{}\n```", make_doc_comment(method), eg);

        let sig = quote! {
            fn #operation_id(&self) -> builder:: #struct_ident <'_>
        };

        let body = quote! {
            builder:: #struct_ident ::new(self)
        };
        BuilderImpl { doc, sig, body }
    }

    /// Generates a pair of TokenStreams.
    ///
    /// The first includes all the operation code; impl Client for operations
    /// with no tags and code of this form for each tag:
    ///
    /// ```ignore
    /// pub trait ClientTagExt {
    ///     ...
    /// }
    ///
    /// impl ClientTagExt for Client {
    ///     ...
    /// }
    /// ```
    ///
    /// The second is the code for the prelude for each tag extension trait:
    ///
    /// ```ignore
    /// pub use super::ClientTagExt;
    /// ```
    pub(crate) fn builder_tags(
        &self,
        methods: &[OperationMethod],
        tag_info: &BTreeMap<&String, &openapiv3::Tag>,
    ) -> (TokenStream, TokenStream) {
        let mut base = Vec::new();
        let mut ext = BTreeMap::new();

        methods.iter().for_each(|method| {
            let BuilderImpl { doc, sig, body } = self.builder_helper(method);

            if method.tags.is_empty() {
                let impl_body = quote! {
                    #[doc = #doc]
                    pub #sig {
                        #body
                    }
                };
                base.push(impl_body);
            } else {
                let trait_sig = quote! {
                    #[doc = #doc]
                    #sig;
                };

                let impl_body = quote! {
                    #sig {
                        #body
                    }
                };
                method.tags.iter().for_each(|tag| {
                    ext.entry(tag.clone())
                        .or_insert_with(Vec::new)
                        .push((trait_sig.clone(), impl_body.clone()));
                });
            }
        });

        let base_impl = (!base.is_empty()).then(|| {
            quote! {
                impl Client {
                    #(#base)*
                }
            }
        });

        let (ext_impl, ext_use): (Vec<_>, Vec<_>) = ext
            .into_iter()
            .map(|(tag, trait_methods)| {
                let desc = tag_info
                    .get(&tag)
                    .and_then(|tag| tag.description.as_ref())
                    .map(|d| quote! { #[doc = #d] });
                let tr = format_ident!("Client{}Ext", sanitize(&tag, Case::Pascal));
                let (trait_methods, trait_impls): (Vec<TokenStream>, Vec<TokenStream>) =
                    trait_methods.into_iter().unzip();
                (
                    quote! {
                        #desc
                        pub trait #tr {
                            #(#trait_methods)*
                        }

                        impl #tr for Client {
                            #(#trait_impls)*
                        }
                    },
                    tr,
                )
            })
            .unzip();

        (
            quote! {
                #base_impl

                #(#ext_impl)*
            },
            quote! {
                #(pub use super::#ext_use;)*
            },
        )
    }

    pub(crate) fn builder_impl(&self, method: &OperationMethod) -> TokenStream {
        let BuilderImpl { doc, sig, body } = self.builder_helper(method);

        let impl_body = quote! {
            #[doc = #doc]
            pub #sig {
                #body
            }
        };

        impl_body
    }

    fn parse_multipart_related_schema(
        &mut self,
        operation: &openapiv3::Operation,
        _components: &Option<Components>,
        object_type: &openapiv3::ObjectType,
    ) -> Result<OperationParameterType> {
        // Validate that the object has properties
        if object_type.properties.is_empty() {
            return Err(Error::UnexpectedFormat(
                "multipart/related object schema must have properties".to_string(),
            ));
        }

        // For now, we expect two properties:
        // 1. metadata: a JSON object (the first part)
        // 2. file/content: binary data (the second part)
        // We'll validate that there's at least one binary property

        let has_binary = object_type.properties.values().any(|prop_ref| {
            match prop_ref {
                openapiv3::ReferenceOr::Item(prop_schema) => {
                    matches!(
                        prop_schema.as_ref(),
                        openapiv3::Schema {
                            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::String(
                                openapiv3::StringType {
                                    format: openapiv3::VariantOrUnknownOrEmpty::Item(
                                        openapiv3::StringFormat::Binary
                                    ),
                                    ..
                                }
                            )),
                            ..
                        }
                    )
                }
                openapiv3::ReferenceOr::Reference { .. } => false,
            }
        });

        if !has_binary {
            return Err(Error::UnexpectedFormat(
                "multipart/related object schema must have at least one binary property".to_string(),
            ));
        }

        // Generate a type for the multipart parts struct
        let parts_name = sanitize(
            &format!("{}-multipart-parts", operation.operation_id.as_ref().unwrap()),
            Case::Pascal,
        );

        // Create a schema for this struct
        let mut properties_schemas: BTreeMap<String, schemars::schema::Schema> = BTreeMap::new();
        let mut content_type_fields: Vec<String> = Vec::new();
        // Preserve the order of properties from the OpenAPI schema
        let mut property_order: Vec<String> = Vec::new();

        for (name, prop_ref) in &object_type.properties {
            property_order.push(name.clone());
            let prop_schema = match prop_ref {
                openapiv3::ReferenceOr::Reference { reference } => {
                    // For references, create a reference schema
                    schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                        reference: Some(reference.clone()),
                        ..Default::default()
                    })
                }
                openapiv3::ReferenceOr::Item(item) => {
                    // Check if this is a binary field (string with format: binary)
                    let is_binary = matches!(
                        item.as_ref(),
                        openapiv3::Schema {
                            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::String(
                                openapiv3::StringType {
                                    format: openapiv3::VariantOrUnknownOrEmpty::Item(
                                        openapiv3::StringFormat::Binary
                                    ),
                                    ..
                                }
                            )),
                            ..
                        }
                    );

                    if is_binary {
                        // Track that we need a content_type field for this binary field
                        content_type_fields.push(name.clone());

                        // For binary fields, create a schema for Vec<u8>
                        // This is an array of bytes (integers 0-255)
                        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                            instance_type: Some(schemars::schema::InstanceType::Array.into()),
                            array: Some(Box::new(schemars::schema::ArrayValidation {
                                items: Some(schemars::schema::SingleOrVec::Single(Box::new(
                                    schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                                        instance_type: Some(schemars::schema::InstanceType::Integer.into()),
                                        format: Some("uint8".to_string()),
                                        number: Some(Box::new(schemars::schema::NumberValidation {
                                            minimum: Some(0.0),
                                            maximum: Some(255.0),
                                            ..Default::default()
                                        })),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        })
                    } else {
                        item.to_schema()
                    }
                }
            };
            properties_schemas.insert(name.clone(), prop_schema);
        }

        // Add content_type fields for each binary field
        for field_name in &content_type_fields {
            let content_type_field_name = format!("{}_content_type", field_name);
            properties_schemas.insert(
                content_type_field_name,
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::String.into()),
                    metadata: Some(Box::new(schemars::schema::Metadata {
                        description: Some(format!("MIME type for the {} field", field_name)),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
            );
        }

        // Build the required fields list - includes original required fields plus content_type fields
        // for required binary fields only
        let mut required_fields: BTreeSet<String> = object_type.required.iter().cloned().collect();
        for field_name in &content_type_fields {
            // Only make content_type required if the binary field itself is required
            if object_type.required.contains(field_name) {
                required_fields.insert(format!("{}_content_type", field_name));
            }
        }

        let schema = schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::Object.into()),
            object: Some(Box::new(schemars::schema::ObjectValidation {
                properties: properties_schemas,
                required: required_fields,
                ..Default::default()
            })),
            ..Default::default()
        };

        let type_id = self.type_space.add_type_with_name(
            &schemars::schema::Schema::Object(schema),
            Some(parts_name),
        )?;

        // Track this type for later code generation, preserving property order and required fields
        self.multipart_related.insert(
            type_id.clone(),
            crate::MultipartRelatedInfo {
                property_order,
                required_fields: object_type.required.iter().cloned().collect(),
            },
        );

        // Multipart/related uses ::serde_json::to_vec() for non-binary fields
        self.uses_serde_json = true;

        Ok(OperationParameterType::MultipartRelated(type_id))
    }

    fn get_body_param(
        &mut self,
        operation: &openapiv3::Operation,
        components: &Option<Components>,
    ) -> Result<Option<OperationParameter>> {
        let body = match &operation.request_body {
            Some(body) => body.item(components)?,
            None => return Ok(None),
        };

        let (content_str, media_type) = match (body.content.first(), body.content.len()) {
            (None, _) => return Ok(None),
            (Some(first), 1) => first,
            (_, n) => todo!(
                "more media types than expected for {}: {}",
                operation.operation_id.as_ref().unwrap(),
                n,
            ),
        };

        let schema = media_type.schema.as_ref().ok_or_else(|| {
            Error::UnexpectedFormat("No schema specified for request body".to_string())
        })?;

        let content_type = BodyContentType::from_str(content_str)?;

        let typ = match content_type {
            BodyContentType::OctetStream => {
                // For an octet stream, we expect a simple, specific schema:
                // "schema": {
                //     "type": "string",
                //     "format": "binary"
                // }
                match schema.item(components)? {
                    openapiv3::Schema {
                        schema_data:
                            openapiv3::SchemaData {
                                nullable: false,
                                discriminator: None,
                                default: None,
                                // Other fields that describe or document the
                                // schema are fine.
                                ..
                            },
                        schema_kind:
                            openapiv3::SchemaKind::Type(openapiv3::Type::String(
                                openapiv3::StringType {
                                    format:
                                        openapiv3::VariantOrUnknownOrEmpty::Item(
                                            openapiv3::StringFormat::Binary,
                                        ),
                                    pattern: None,
                                    enumeration,
                                    min_length: None,
                                    max_length: None,
                                },
                            )),
                    } if enumeration.is_empty() => Ok(()),
                    _ => Err(Error::UnexpectedFormat(format!(
                        "invalid schema for {}: {:?}",
                        content_type, schema
                    ))),
                }?;
                OperationParameterType::RawBody
            }
            BodyContentType::MultipartRelated => {
                // For multipart/related, we support two schemas:
                // 1. Simple binary string (for manual construction)
                // 2. Object with properties (for structured parts)
                match schema.item(components)? {
                    // Simple binary string schema
                    openapiv3::Schema {
                        schema_data:
                            openapiv3::SchemaData {
                                nullable: false,
                                discriminator: None,
                                default: None,
                                ..
                            },
                        schema_kind:
                            openapiv3::SchemaKind::Type(openapiv3::Type::String(
                                openapiv3::StringType {
                                    format:
                                        openapiv3::VariantOrUnknownOrEmpty::Item(
                                            openapiv3::StringFormat::Binary,
                                        ),
                                    pattern: None,
                                    enumeration,
                                    min_length: None,
                                    max_length: None,
                                },
                            )),
                    } if enumeration.is_empty() => OperationParameterType::RawBody,
                    // Object schema with properties for structured multipart
                    openapiv3::Schema {
                        schema_data: _,
                        schema_kind:
                            openapiv3::SchemaKind::Type(openapiv3::Type::Object(object_type)),
                    } => {
                        // Parse the object schema to identify parts
                        self.parse_multipart_related_schema(
                            operation,
                            components,
                            object_type,
                        )?
                    }
                    _ => {
                        return Err(Error::UnexpectedFormat(format!(
                            "invalid schema for multipart/related: {:?}",
                            schema
                        )))
                    }
                }
            }
            BodyContentType::Text(_) => {
                // For a plain text body, we expect a simple, specific schema:
                // "schema": {
                //     "type": "string",
                // }
                match schema.item(components)? {
                    openapiv3::Schema {
                        schema_data:
                            openapiv3::SchemaData {
                                nullable: false,
                                discriminator: None,
                                default: None,
                                // Other fields that describe or document the
                                // schema are fine.
                                ..
                            },
                        schema_kind:
                            openapiv3::SchemaKind::Type(openapiv3::Type::String(
                                openapiv3::StringType {
                                    format: openapiv3::VariantOrUnknownOrEmpty::Empty,
                                    pattern: None,
                                    enumeration,
                                    min_length: None,
                                    max_length: None,
                                },
                            )),
                    } if enumeration.is_empty() => Ok(()),
                    _ => Err(Error::UnexpectedFormat(format!(
                        "invalid schema for {}: {:?}",
                        content_type, schema
                    ))),
                }?;
                OperationParameterType::RawBody
            }
            BodyContentType::Json | BodyContentType::FormUrlencoded => {
                // TODO it would be legal to have the encoding field set for
                // application/x-www-form-urlencoded content, but I'm not sure
                // how to interpret the values.
                if !media_type.encoding.is_empty() {
                    todo!("media type encoding not empty: {:#?}", media_type);
                }
                let name = sanitize(
                    &format!("{}-body", operation.operation_id.as_ref().unwrap(),),
                    Case::Pascal,
                );
                let typ = self
                    .type_space
                    .add_type_with_name(&schema.to_schema(), Some(name))?;
                OperationParameterType::Type(typ)
            }
        };

        Ok(Some(OperationParameter {
            name: "body".to_string(),
            api_name: "body".to_string(),
            description: body.description.clone(),
            typ,
            kind: OperationParameterKind::Body(content_type),
            default: None,
        }))
    }
}

fn make_doc_comment(method: &OperationMethod) -> String {
    let mut buf = String::new();

    if let Some(summary) = &method.summary {
        buf.push_str(summary.trim_end_matches(['.', ',']));
        buf.push_str("\n\n");
    }
    if let Some(description) = &method.description {
        buf.push_str(description);
        buf.push_str("\n\n");
    }

    buf.push_str(&format!(
        "Sends a `{}` request to `{}`\n\n",
        method.method.as_str().to_ascii_uppercase(),
        method.path,
    ));

    if method
        .params
        .iter()
        .filter(|param| param.description.is_some())
        .count()
        > 0
    {
        buf.push_str("Arguments:\n");
        for param in &method.params {
            buf.push_str(&format!("- `{}`", param.name));
            if let Some(description) = &param.description {
                buf.push_str(": ");
                buf.push_str(description);
            }
            buf.push('\n');
        }
    }

    buf
}

fn make_stream_doc_comment(method: &OperationMethod) -> String {
    let mut buf = String::new();

    if let Some(summary) = &method.summary {
        buf.push_str(summary.trim_end_matches(['.', ',']));
        buf.push_str(" as a Stream\n\n");
    }
    if let Some(description) = &method.description {
        buf.push_str(description);
        buf.push_str("\n\n");
    }

    buf.push_str(&format!(
        "Sends repeated `{}` requests to `{}` until there are no more results.\n\n",
        method.method.as_str().to_ascii_uppercase(),
        method.path,
    ));

    if method
        .params
        .iter()
        .filter(|param| param.api_name.as_str() != "page_token")
        .filter(|param| param.description.is_some())
        .count()
        > 0
    {
        buf.push_str("Arguments:\n");
        for param in &method.params {
            if param.api_name.as_str() == "page_token" {
                continue;
            }

            buf.push_str(&format!("- `{}`", param.name));
            if let Some(description) = &param.description {
                buf.push_str(": ");
                buf.push_str(description);
            }
            buf.push('\n');
        }
    }

    buf
}

fn sort_params(raw_params: &mut [OperationParameter], names: &[String]) {
    raw_params.sort_by(
        |OperationParameter {
             kind: a_kind,
             api_name: a_name,
             ..
         },
         OperationParameter {
             kind: b_kind,
             api_name: b_name,
             ..
         }| {
            match (a_kind, b_kind) {
                // Path params are first and are in positional order.
                (OperationParameterKind::Path, OperationParameterKind::Path) => {
                    let a_index = names
                        .iter()
                        .position(|x| x == a_name)
                        .unwrap_or_else(|| panic!("{} missing from path", a_name));
                    let b_index = names
                        .iter()
                        .position(|x| x == b_name)
                        .unwrap_or_else(|| panic!("{} missing from path", b_name));
                    a_index.cmp(&b_index)
                }
                (OperationParameterKind::Path, OperationParameterKind::Query(_)) => Ordering::Less,
                (OperationParameterKind::Path, OperationParameterKind::Body(_)) => Ordering::Less,
                (OperationParameterKind::Path, OperationParameterKind::Header(_)) => Ordering::Less,

                // Query params are in lexicographic order.
                (OperationParameterKind::Query(_), OperationParameterKind::Body(_)) => {
                    Ordering::Less
                }
                (OperationParameterKind::Query(_), OperationParameterKind::Query(_)) => {
                    a_name.cmp(b_name)
                }
                (OperationParameterKind::Query(_), OperationParameterKind::Path) => {
                    Ordering::Greater
                }
                (OperationParameterKind::Query(_), OperationParameterKind::Header(_)) => {
                    Ordering::Less
                }

                // Body params are last and should be singular.
                (OperationParameterKind::Body(_), OperationParameterKind::Path) => {
                    Ordering::Greater
                }
                (OperationParameterKind::Body(_), OperationParameterKind::Query(_)) => {
                    Ordering::Greater
                }
                (OperationParameterKind::Body(_), OperationParameterKind::Header(_)) => {
                    Ordering::Greater
                }
                (OperationParameterKind::Body(_), OperationParameterKind::Body(_)) => {
                    panic!("should only be one body")
                }

                // Header params are in lexicographic order.
                (OperationParameterKind::Header(_), OperationParameterKind::Header(_)) => {
                    a_name.cmp(b_name)
                }
                (OperationParameterKind::Header(_), _) => Ordering::Greater,
            }
        },
    );
}

trait ParameterDataExt {
    fn schema(&self) -> Result<&openapiv3::ReferenceOr<openapiv3::Schema>>;
}

impl ParameterDataExt for openapiv3::ParameterData {
    fn schema(&self) -> Result<&openapiv3::ReferenceOr<openapiv3::Schema>> {
        match &self.format {
            openapiv3::ParameterSchemaOrContent::Schema(s) => Ok(s),
            openapiv3::ParameterSchemaOrContent::Content(c) => Err(Error::UnexpectedFormat(
                format!("unexpected content {:#?}", c),
            )),
        }
    }
}
