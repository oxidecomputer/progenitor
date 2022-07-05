// Copyright 2022 Oxide Computer Company

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use openapiv3::{Components, Response, StatusCode};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use typify::TypeId;

use crate::{
    template::PathTemplate,
    util::{sanitize, Case},
    Error, Generator, Result, TagStyle,
};
use crate::{to_schema::ToSchema, util::ReferenceOrExt};

/// The intermediate representation of an operation that will become a method.
pub(crate) struct OperationMethod {
    operation_id: String,
    pub tags: Vec<String>,
    method: HttpMethod,
    path: PathTemplate,
    summary: Option<String>,
    description: Option<String>,
    params: Vec<OperationParameter>,
    raw_body_param: bool,
    responses: Vec<OperationResponse>,
    dropshot_paginated: Option<DropshotPagination>,
}

enum HttpMethod {
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

struct DropshotPagination {
    item: TypeId,
}

#[derive(Debug, PartialEq, Eq)]
enum OperationParameterKind {
    Path,
    Query(bool),
    Body,
}
struct OperationParameter {
    /// Sanitized parameter name.
    name: String,
    /// Original parameter name provided by the API.
    api_name: String,
    description: Option<String>,
    typ: OperationParameterType,
    kind: OperationParameterKind,
}

#[derive(Eq, PartialEq)]
enum OperationParameterType {
    Type(TypeId),
    RawBody,
}
#[derive(Debug)]
struct OperationResponse {
    status_code: OperationResponseStatus,
    typ: OperationResponseType,
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
enum OperationResponseStatus {
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

    fn is_success_or_default(&self) -> bool {
        matches!(
            self,
            OperationResponseStatus::Default
                | OperationResponseStatus::Code(200..=299)
                | OperationResponseStatus::Range(2)
        )
    }

    fn is_error_or_default(&self) -> bool {
        matches!(
            self,
            OperationResponseStatus::Default
                | OperationResponseStatus::Code(400..=599)
                | OperationResponseStatus::Range(4..=5)
        )
    }

    fn is_default(&self) -> bool {
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
enum OperationResponseType {
    Type(TypeId),
    None,
    Raw,
}

impl Generator {
    pub(crate) fn process_operation(
        &mut self,
        operation: &openapiv3::Operation,
        components: &Option<Components>,
        path: &str,
        method: &str,
    ) -> Result<OperationMethod> {
        let operation_id = operation.operation_id.as_ref().unwrap();

        let mut query: Vec<(String, bool)> = Vec::new();
        let mut params = operation
            .parameters
            .iter()
            .map(|parameter| {
                match parameter.item(components)? {
                    openapiv3::Parameter::Path {
                        parameter_data,
                        style: openapiv3::PathStyle::Simple,
                    } => {
                        // Path parameters MUST be required.
                        assert!(parameter_data.required);

                        let schema = parameter_data.schema()?.to_schema();

                        let name = sanitize(
                            &format!(
                                "{}-{}",
                                operation_id, &parameter_data.name
                            ),
                            Case::Pascal,
                        );
                        let typ = self
                            .type_space
                            .add_type_with_name(&schema, Some(name))?;

                        Ok(OperationParameter {
                            name: sanitize(&parameter_data.name, Case::Snake),
                            api_name: parameter_data.name.clone(),
                            description: parameter_data.description.clone(),
                            typ: OperationParameterType::Type(typ),
                            kind: OperationParameterKind::Path,
                        })
                    }
                    openapiv3::Parameter::Query {
                        parameter_data,
                        allow_reserved: _, // We always encode reserved chars
                        style: openapiv3::QueryStyle::Form,
                        allow_empty_value: _, // Irrelevant for this client
                    } => {
                        let mut schema = parameter_data.schema()?.to_schema();
                        let name = sanitize(
                            &format!(
                                "{}-{}",
                                operation.operation_id.as_ref().unwrap(),
                                &parameter_data.name,
                            ),
                            Case::Pascal,
                        );

                        if !parameter_data.required {
                            schema = make_optional(schema);
                        }

                        let typ = self
                            .type_space
                            .add_type_with_name(&schema, Some(name))?;

                        query.push((
                            parameter_data.name.clone(),
                            !parameter_data.required,
                        ));
                        Ok(OperationParameter {
                            name: sanitize(&parameter_data.name, Case::Snake),
                            api_name: parameter_data.name.clone(),
                            description: parameter_data.description.clone(),
                            typ: OperationParameterType::Type(typ),
                            kind: OperationParameterKind::Query(
                                parameter_data.required,
                            ),
                        })
                    }

                    openapiv3::Parameter::Path { style, .. } => {
                        Err(Error::UnexpectedFormat(format!(
                            "unsupported style of path parameter {:#?}",
                            style,
                        )))
                    }
                    openapiv3::Parameter::Query { style, .. } => {
                        Err(Error::UnexpectedFormat(format!(
                            "unsupported style of query parameter {:#?}",
                            style,
                        )))
                    }
                    header @ openapiv3::Parameter::Header { .. } => {
                        Err(Error::UnexpectedFormat(format!(
                            "header parameters are not supported {:#?}",
                            header,
                        )))
                    }
                    cookie @ openapiv3::Parameter::Cookie { .. } => {
                        Err(Error::UnexpectedFormat(format!(
                            "cookie parameters are not supported {:#?}",
                            cookie,
                        )))
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?;
        let mut raw_body_param = false;
        if let Some(b) = &operation.request_body {
            let b = b.item(components)?;
            let typ = if b.is_binary(components)? {
                raw_body_param = true;
                OperationParameterType::RawBody
            } else {
                let mt = b.content_json()?;
                if !mt.encoding.is_empty() {
                    todo!("media type encoding not empty: {:#?}", mt);
                }

                if let Some(s) = &mt.schema {
                    let schema = s.to_schema();
                    let name = sanitize(
                        &format!(
                            "{}-body",
                            operation.operation_id.as_ref().unwrap(),
                        ),
                        Case::Pascal,
                    );
                    let typ = self
                        .type_space
                        .add_type_with_name(&schema, Some(name))?;
                    OperationParameterType::Type(typ)
                } else {
                    todo!("media type encoding, no schema: {:#?}", mt);
                }
            };

            params.push(OperationParameter {
                name: "body".to_string(),
                api_name: "body".to_string(),
                description: b.description.clone(),
                typ,
                kind: OperationParameterKind::Body,
            });
        }
        let tmp = crate::template::parse(path)?;
        let names = tmp.names();

        sort_params(&mut params, &names);

        let mut success = false;

        let mut responses = operation
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
                            StatusCode::Code(code) => {
                                OperationResponseStatus::Code(*code)
                            }
                            StatusCode::Range(range) => {
                                OperationResponseStatus::Range(*range)
                            }
                        },
                        response_or_ref.item(components)?,
                    ))
                },
            ))
            .map(|v: Result<(OperationResponseStatus, &Response)>| {
                let (status_code, response) = v?;

                // We categorize responses as "typed" based on the
                // "application/json" content type, "raw" if there's any other
                // response content type (we don't investigate further), or
                // "none" if there is no content.
                // TODO if there are multiple response content types we could
                // treat those like different response types and create an
                // enum; the generated client method would check for the
                // content type of the response just as it currently examines
                // the status code.
                let typ = if let Some(mt) =
                    response.content.get("application/json")
                {
                    assert!(mt.encoding.is_empty());

                    let typ = if let Some(schema) = &mt.schema {
                        let schema = schema.to_schema();
                        let name = sanitize(
                            &format!(
                                "{}-response",
                                operation.operation_id.as_ref().unwrap(),
                            ),
                            Case::Pascal,
                        );
                        self.type_space
                            .add_type_with_name(&schema, Some(name))?
                    } else {
                        todo!("media type encoding, no schema: {:#?}", mt);
                    };

                    OperationResponseType::Type(typ)
                } else if response.content.first().is_some() {
                    OperationResponseType::Raw
                } else {
                    OperationResponseType::None
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
                typ: OperationResponseType::Raw,
                description: None,
            });
        }

        let dropshot_paginated =
            self.dropshot_pagination_data(operation, &params, &responses);

        Ok(OperationMethod {
            operation_id: sanitize(operation_id, Case::Snake),
            tags: operation.tags.clone(),
            method: HttpMethod::from_str(method)?,
            path: tmp,
            summary: operation.summary.clone().filter(|s| !s.is_empty()),
            description: operation
                .description
                .clone()
                .filter(|s| !s.is_empty()),
            params,
            raw_body_param,
            responses,
            dropshot_paginated,
        })
    }

    pub(crate) fn positional_method(
        &mut self,
        method: &OperationMethod,
    ) -> Result<TokenStream> {
        let operation_id = format_ident!("{}", method.operation_id);

        // Render each parameter as it will appear in the method signature.
        let params = method
            .params
            .iter()
            .map(|param| {
                let name = format_ident!("{}", param.name);
                let typ = match &param.typ {
                    OperationParameterType::Type(type_id) => self
                        .type_space
                        .get_type(type_id)
                        .unwrap()
                        .parameter_ident_with_lifetime("a"),
                    OperationParameterType::RawBody => {
                        quote! { B }
                    }
                };
                quote! {
                    #name: #typ
                }
            })
            .collect::<Vec<_>>();

        let bounds = if method.raw_body_param {
            quote! { <'a, B: Into<reqwest::Body> > }
        } else {
            quote! { <'a> }
        };

        let doc_comment = make_doc_comment(method);

        let MethodSigBody {
            success: success_type,
            error: error_type,
            body,
        } = self.method_sig_body(method, quote! { self })?;

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
            let stream_params = method.params.iter().zip(params).filter_map(
                |(param, stream)| {
                    if param.name.as_str() == "page_token" {
                        None
                    } else {
                        Some(stream)
                    }
                },
            );

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
                } else if let OperationParameterKind::Query(_) = param.kind {
                    // Query parameters are None; having page_token as Some(_)
                    // is mutually exclusive with other query parameters.
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
                >> + Unpin + '_ {
                    use futures::StreamExt;
                    use futures::TryFutureExt;
                    use futures::TryStreamExt;

                    // Execute the operation with the basic parameters
                    // (omitting page_token) to get the first page.
                    self.#operation_id( #(#first_params,)* )
                        .map_ok(move |page| {
                            let page = page.into_inner();

                            // Create a stream from the items of the first page.
                            let first = futures::stream::iter(
                                page.items.into_iter().map(Ok)
                            );

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
                                                    page
                                                        .items
                                                        .into_iter()
                                                        .map(Ok),
                                                ),
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

    fn method_sig_body(
        &self,
        method: &OperationMethod,
        client: TokenStream,
    ) -> Result<MethodSigBody> {
        // Generate code for query parameters.
        let query_items = method
            .params
            .iter()
            .filter_map(|param| match &param.kind {
                OperationParameterKind::Query(required) => {
                    let qn = &param.api_name;
                    let qn_ident = format_ident!("{}", &param.name);
                    Some(if *required {
                        quote! {
                            query.push((#qn, #qn_ident .to_string()));
                        }
                    } else {
                        quote! {
                            if let Some(v) = & #qn_ident {
                                query.push((#qn, v.to_string()));
                            }
                        }
                    })
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let (query_build, query_use) = if query_items.is_empty() {
            (quote! {}, quote! {})
        } else {
            let query_build = quote! {
                let mut query = Vec::new();
                #(#query_items)*
            };
            let query_use = quote! {
                .query(&query)
            };

            (query_build, query_use)
        };

        // Generate the path rename map; then use it to generate code for
        // assigning the path parameters to the `url` variable.
        let url_renames = method
            .params
            .iter()
            .filter_map(|param| match &param.kind {
                OperationParameterKind::Path => {
                    Some((&param.api_name, &param.name))
                }
                _ => None,
            })
            .collect();
        let url_path = method.path.compile(url_renames, client.clone());

        // Generate code to handle the body...
        let body_func =
            method.params.iter().filter_map(|param| match &param.kind {
                OperationParameterKind::Body => match &param.typ {
                    OperationParameterType::Type(_) => {
                        Some(quote! { .json(&body) })
                    }
                    OperationParameterType::RawBody => {
                        Some(quote! { .body(body) })
                    }
                },
                _ => None,
            });
        // ... and there can be at most one body.
        assert!(body_func.clone().count() <= 1);

        let (success_response_items, response_type) = self.extract_responses(
            method,
            OperationResponseStatus::is_success_or_default,
        );

        let success_response_matches =
            success_response_items.iter().map(|response| {
                let pat = match &response.status_code {
                    OperationResponseStatus::Code(code) => quote! { #code },
                    OperationResponseStatus::Range(_)
                    | OperationResponseStatus::Default => {
                        quote! { 200 ..= 299 }
                    }
                };

                let decode = match &response.typ {
                    OperationResponseType::Type(_) => {
                        quote! {
                            ResponseValue::from_response(response).await
                        }
                    }
                    OperationResponseType::None => {
                        quote! {
                            Ok(ResponseValue::empty(response))
                        }
                    }
                    OperationResponseType::Raw => {
                        quote! {
                            Ok(ResponseValue::stream(response))
                        }
                    }
                };

                quote! { #pat => { #decode } }
            });

        // Errors...
        let (error_response_items, error_type) = self.extract_responses(
            method,
            OperationResponseStatus::is_error_or_default,
        );

        let error_response_matches =
            error_response_items.iter().map(|response| {
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
                    OperationResponseType::Type(_) => {
                        quote! {
                            Err(Error::ErrorResponse(
                                ResponseValue::from_response(response)
                                    .await?
                            ))
                        }
                    }
                    OperationResponseType::None => {
                        quote! {
                            Err(Error::ErrorResponse(
                                ResponseValue::empty(response)
                            ))
                        }
                    }
                    OperationResponseType::Raw => {
                        quote! {
                            Err(Error::ErrorResponse(
                                ResponseValue::stream(response)
                            ))
                        }
                    }
                };

                quote! { #pat => { #decode } }
            });

        // Generate the catch-all case for other statuses. If the operation
        // specifies a default response, we've already generated a default
        // match as part of error response code handling. (And we've handled
        // the default as a success response as well.) Otherwise the catch-all
        // produces an error corresponding to a response not specified in the
        // API description.
        let default_response = match method.responses.iter().last() {
            Some(response) if response.status_code.is_default() => quote! {},
            _ => quote! { _ => Err(Error::UnexpectedResponse(response)), },
        };

        let pre_hook = self.settings.pre_hook.as_ref().map(|hook| {
            quote! {
                (#hook)(&#client.inner, &request);
            }
        });
        let post_hook = self.settings.post_hook.as_ref().map(|hook| {
            quote! {
                (#hook)(&#client.inner, &result);
            }
        });

        let method_func = format_ident!("{}", method.method.as_str());

        let body_impl = quote! {
            #url_path
            #query_build

            let request = #client.client
                . #method_func (url)
                #(#body_func)*
                #query_use
                .build()?;
            #pre_hook
            let result = #client.client
                .execute(request)
                .await;
            #post_hook

            let response = result?;

            match response.status().as_u16() {
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
            success: response_type,
            error: error_type,
            body: body_impl,
        })
    }

    fn extract_responses<'a>(
        &self,
        method: &'a OperationMethod,
        filter: fn(&OperationResponseStatus) -> bool,
    ) -> (Vec<&'a OperationResponse>, TokenStream) {
        let mut response_items = method
            .responses
            .iter()
            .filter(|response| filter(&response.status_code))
            .collect::<Vec<_>>();
        response_items.sort();

        // If we have a success range and a default, we can pop off the default
        // since it will never be hit. Note that this is a no-op for error
        // responses.
        {
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
        }

        let response_types = response_items
            .iter()
            .map(|response| response.typ.clone())
            .collect::<BTreeSet<_>>();

        // TODO to deal with multiple response types, we'll need to create an
        // enum type with variants for each of the response types.
        assert!(response_types.len() <= 1);
        let response_type = response_types
            .iter()
            .next()
            .map(|typ| match typ {
                OperationResponseType::Type(type_id) => {
                    let type_name =
                        self.type_space.get_type(type_id).unwrap().ident();
                    quote! { #type_name }
                }
                OperationResponseType::None => {
                    quote! { () }
                }
                OperationResponseType::Raw => {
                    quote! { ByteStream }
                }
            })
            // TODO should this be a bytestream?
            .unwrap_or_else(|| quote! { () });
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
        if operation
            .extensions
            .get("x-dropshot-pagination")
            .and_then(|v| v.as_bool())
            != Some(true)
        {
            return None;
        }

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
            responses.iter().filter_map(|response| {
                match (&response.status_code, &response.typ) {
                    (
                        OperationResponseStatus::Code(200..=299)
                        | OperationResponseStatus::Range(2),
                        OperationResponseType::Type(type_id),
                    ) => Some(type_id),
                    _ => None,
                }
            });

        let success_response = match (
            success_response_items.next(),
            success_response_items.next(),
        ) {
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
            typify::TypeDetails::Array(item) => {
                Some(DropshotPagination { item })
            }
            _ => None,
        }
    }

    /// Create the builder structs along with their impls
    ///
    /// Builder structs are generally of this form:
    /// ```ignore
    /// struct OperationId<'a> {
    ///     client: &'a super::Client,
    ///     param_1: Option<SomeType>,
    ///     param_2: Option<String>,
    /// }
    /// ```
    ///
    /// All parameters are present and all their types are Option<T>. Each
    /// parameter also has a corresponding method:
    /// ```ignore
    /// impl<'a> OperationId<'a> {
    ///     pub fn param_1(self, value: SomeType) {
    ///         self.param_1 = Some(value);
    ///         self
    ///     }
    ///     pub fn param_2<S: ToString>(self, value: S) {
    ///         self.param_2 = Some(value.into());
    ///         self
    ///     }
    /// }
    /// ```
    ///
    /// The Client's operation_id method simply invokes the builder's new
    /// method:
    /// ```ignore
    /// impl<'a> OperationId<'a> {
    ///     pub fn new(client: &'a super::Client) -> Self {
    ///         Self {
    ///             client,
    ///             param_1: None,
    ///             param_2: None,
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// Finally, builders have methods to execute the method, which takes care
    /// to check that required parameters are specified:
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
    ///         let (param_1, param_2) = match (param_1, param_2) {
    ///             (Some(param_1), Some(param_2)) => (param_1, param_2),
    ///             (param_1, param_2) => {
    ///                 let mut missing = Vec::new();
    ///                 if param_1.is_none() {
    ///                     missing.push(stringify!(param_1));
    ///                 }
    ///                 if param_2.is_none() {
    ///                     missing.push(stringify!(param_2));
    ///                 }
    ///                 return Err(super::Error::InvalidRequest(format!(
    ///                     "the following parameters are required: {}",
    ///                     missing.join(", "),
    ///                 )));
    ///             }
    ///         };
    ///     }
    /// }
    /// ```
    ///
    /// Finally, paginated interfaces have a `stream()` method which uses the
    /// `send()` method above to fetch each page of results to assemble the
    /// items into a single impl Stream.
    pub(crate) fn builder_struct(
        &mut self,
        method: &OperationMethod,
        tag_style: TagStyle,
    ) -> Result<TokenStream> {
        let mut cloneable = true;
        // Generate the builder structure properties, turning each type T into
        // an Option<T> (if it isn't already).
        let properties = method
            .params
            .iter()
            .map(|param| {
                let name = format_ident!("{}", param.name);
                let typ = match &param.typ {
                    OperationParameterType::Type(type_id) => {
                        // TODO currently we explicitly turn optional paramters
                        // into Option types; we could probably defer this to
                        // the code generation step to avoid the special
                        // handling here.
                        let ty = self.type_space.get_type(type_id)?;
                        let t = ty.ident();
                        let details = ty.details();
                        if let typify::TypeDetails::Option(_) = details {
                            t
                        } else {
                            quote! { Option<#t> }
                        }
                    }

                    OperationParameterType::RawBody => {
                        cloneable = false;
                        quote! { Option<reqwest::Body> }
                    }
                };

                Ok(quote! {
                    #name: #typ
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let struct_name = sanitize(&method.operation_id, Case::Pascal);
        let struct_ident = format_ident!("{}", struct_name);

        // For each parameter, we need an impl for the builder to let consumers
        // provide a value for the parameter.
        let property_impls = method
            .params
            .iter()
            .map(|param| {
                let param_name = format_ident!("{}", param.name);
                match &param.typ {
                    OperationParameterType::Type(type_id) => {
                        let ty = self.type_space.get_type(type_id)?;
                        let x = ty.details();
                        match &x {
                            typify::TypeDetails::String => {
                                Ok(quote! {
                                    pub fn #param_name<S: ToString>(mut self, value: S) -> Self {
                                        self.#param_name = Some(value.to_string());
                                        self
                                    }
                                })
                            }
                            typify::TypeDetails::Option(ref opt_id) => {
                                let typ = self.type_space.get_type(opt_id)?.ident();
                                Ok(quote!{
                                    pub fn #param_name(mut self, value: #typ) -> Self {
                                        self.#param_name = Some(value);
                                        self
                                    }
                                })
                            }
                            _ =>  {
                                let typ = ty.ident();
                                Ok(quote! {
                                    pub fn #param_name(mut self, value: #typ) -> Self {
                                        self.#param_name = Some(value);
                                        self
                                    }
                                })
                            }
                        }
                    }

                    OperationParameterType::RawBody => {
                        Ok(quote! {
                            pub fn #param_name<B: Into<reqwest::Body>>(mut self, value: B) -> Self {
                                self.#param_name = Some(value.into());
                                self
                            }
                        })
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let destructure = method
            .params
            .iter()
            .map(|param| format_ident!("{}", param.name))
            .collect::<Vec<_>>();

        let req_names = method
            .params
            .iter()
            .filter_map(|param| match param.kind {
                OperationParameterKind::Path
                | OperationParameterKind::Query(true)
                | OperationParameterKind::Body => {
                    Some(format_ident!("{}", param.name))
                }
                OperationParameterKind::Query(false) => None,
            })
            .collect::<Vec<_>>();

        let required_extract = (!req_names.is_empty()).then(|| {
            quote! {
                let ( #( #req_names, )* ) = match ( #( #req_names, )* ) {
                    ( #( Some(#req_names), )* ) => ( #( #req_names, )* ),
                    ( #( #req_names, )* ) => {
                        let mut missing = Vec::new();

                        #(
                        if #req_names.is_none() {
                            missing.push(stringify!(#req_names));
                        }
                        )*

                        return Err(super::Error::InvalidRequest(format!(
                            "the following parameters are required: {}",
                            missing.join(", "),
                        )));
                    }
                };
            }
        });

        let param_props = method
            .params
            .iter()
            .map(|param| {
                let name = format_ident!("{}", param.name);
                quote! {
                    #name: None
                }
            })
            .collect::<Vec<_>>();

        let new_impl = quote! {
            pub fn new(client: &'a super::Client) -> Self {
                Self {
                    client,
                    #(#param_props,)*
                }
            }
        };

        let MethodSigBody {
            success,
            error,
            body,
        } = self.method_sig_body(method, quote! { client})?;

        let send_doc = format!(
            "Sends a `{}` request to `{}`",
            method.method.as_str().to_ascii_uppercase(),
            method.path.to_string(),
        );

        let send_impl = quote! {
            #[doc = #send_doc]
            pub async fn send(self) -> Result<
                ResponseValue<#success>,
                Error<#error>,
            > {
                // Destructure the builder for convenience.
                let Self {
                    client,
                    #( #destructure ),*
                } = self;

                // Extract parameters into variables, returning an error if
                // a value has not been provided for all required parameters.
                #required_extract

                // Do the work.
                #body
            }
        };

        let stream_impl = method.dropshot_paginated.as_ref().map(|page_data| {
            // We're now using futures.
            self.uses_futures = true;

            let step_params = method.params.iter().filter_map(|param| {
                if let OperationParameterKind::Query(_) = param.kind {
                    let name = format_ident!("{}", param.name);
                    Some(quote! {
                        #name: None
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
                method.path.to_string(),
            );

            quote! {
                #[doc = #stream_doc]
                pub fn stream(self) -> impl futures::Stream<Item = Result<
                    #item_type,
                    Error<#error>,
                >> + Unpin + 'a {
                    use futures::StreamExt;
                    use futures::TryFutureExt;
                    use futures::TryStreamExt;

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

                            // Create a stream from the items of the first page.
                            let first = futures::stream::iter(
                                page.items.into_iter().map(Ok)
                            );

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
                                            page_token: next_page,
                                            ..next.clone()
                                        }
                                        .send()
                                        .map_ok(|page| {
                                            let page = page.into_inner();
                                            Some((
                                                futures::stream::iter(
                                                    page
                                                        .items
                                                        .into_iter()
                                                        .map(Ok),
                                                ),
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

        let maybe_clone = cloneable.then(|| quote! { #[derive(Clone)] });

        // Build a reasonable doc comment depending on whether this struct is
        // the output from
        // 1. A Client method
        // 2. An extension trait method
        // 3. Several extension trait methods
        let struct_doc =
            match (tag_style, method.tags.len(), method.tags.first()) {
                (TagStyle::Merged, _, _) | (TagStyle::Separate, 0, _) => {
                    let ty = format!("Client::{}", method.operation_id);
                    format!(
                        "Builder for [`{}`]\n\n[`{}`]: super::{}",
                        ty, ty, ty,
                    )
                }
                (TagStyle::Separate, 1, Some(tag)) => {
                    let ty = format!(
                        "Client{}Ext::{}",
                        sanitize(tag, Case::Pascal),
                        method.operation_id
                    );
                    format!(
                        "Builder for [`{}`]\n\n[`{}`]: super::{}",
                        ty, ty, ty,
                    )
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

        let ret = quote! {
            #[doc = #struct_doc]
            #maybe_clone
            pub struct #struct_ident<'a> {
                client: &'a super::Client,
                #( #properties ),*
            }

            impl<'a> #struct_ident<'a> {
                #new_impl
                #( #property_impls )*
                #send_impl
                #stream_impl
            }
        };
        Ok(ret)
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
        let doc =
            format!("{}\n```ignore\n{}\n```", make_doc_comment(method), eg);

        let sig = quote! {
            fn #operation_id(&self) -> builder:: #struct_ident
        };

        let body = quote! {
            builder:: #struct_ident ::new(self)
        };
        BuilderImpl { doc, sig, body }
    }

    pub(crate) fn builder_tags(
        &self,
        methods: &[OperationMethod],
    ) -> TokenStream {
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

        let ext_impl = ext.into_iter().map(|(tag, trait_methods)| {
            let tr = format_ident!("Client{}Ext", sanitize(&tag, Case::Pascal));
            let (trait_methods, trait_impls): (
                Vec<TokenStream>,
                Vec<TokenStream>,
            ) = trait_methods.into_iter().unzip();
            quote! {
                pub trait #tr {
                    #(#trait_methods)*
                }

                impl #tr for Client {
                    #(#trait_impls)*
                }
            }
        });

        quote! {
            #base_impl

            #(#ext_impl)*
        }
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
        "Sends a `{}` request to `{}`",
        method.method.as_str().to_ascii_uppercase(),
        method.path.to_string(),
    ));

    if method
        .params
        .iter()
        .filter(|param| param.description.is_some())
        .count()
        > 0
    {
        buf.push_str("\n\nArguments:\n");
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
        "Sends repeated `{}` requests to `{}` until there are no more results.",
        method.method.as_str().to_ascii_uppercase(),
        method.path.to_string(),
    ));

    if method
        .params
        .iter()
        .filter(|param| param.api_name.as_str() != "page_token")
        .filter(|param| param.description.is_some())
        .count()
        > 0
    {
        buf.push_str("\n\nArguments:\n");
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

/// Make the schema optional if it isn't already.
fn make_optional(schema: schemars::schema::Schema) -> schemars::schema::Schema {
    match &schema {
        // If the instance_type already includes Null then this is already
        // optional.
        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::SingleOrVec::Vec(types)),
            ..
        }) if types.contains(&schemars::schema::InstanceType::Null) => schema,

        // Otherwise, create a oneOf where one of the branches is the null
        // type. We could potentially check to see if the schema already
        // conforms to this pattern as well, but it doesn't hurt as typify will
        // already reduce nested Options to a single Option.
        _ => {
            let null_schema = schemars::schema::Schema::Object(
                schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::SingleOrVec::Single(
                        Box::new(schemars::schema::InstanceType::Null),
                    )),
                    ..Default::default()
                },
            );
            schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                subschemas: Some(Box::new(
                    schemars::schema::SubschemaValidation {
                        one_of: Some(vec![schema, null_schema]),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            })
        }
    }
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
                (
                    OperationParameterKind::Path,
                    OperationParameterKind::Path,
                ) => {
                    let a_index =
                        names.iter().position(|x| x == a_name).unwrap_or_else(
                            || panic!("{} missing from path", a_name),
                        );
                    let b_index =
                        names.iter().position(|x| x == b_name).unwrap_or_else(
                            || panic!("{} missing from path", b_name),
                        );
                    a_index.cmp(&b_index)
                }
                (
                    OperationParameterKind::Path,
                    OperationParameterKind::Query(_),
                ) => Ordering::Less,
                (
                    OperationParameterKind::Path,
                    OperationParameterKind::Body,
                ) => Ordering::Less,

                // Query params are in lexicographic order.
                (
                    OperationParameterKind::Query(_),
                    OperationParameterKind::Body,
                ) => Ordering::Less,
                (
                    OperationParameterKind::Query(_),
                    OperationParameterKind::Query(_),
                ) => a_name.cmp(b_name),
                (
                    OperationParameterKind::Query(_),
                    OperationParameterKind::Path,
                ) => Ordering::Greater,

                // Body params are last and should be singular.
                (
                    OperationParameterKind::Body,
                    OperationParameterKind::Path,
                ) => Ordering::Greater,
                (
                    OperationParameterKind::Body,
                    OperationParameterKind::Query(_),
                ) => Ordering::Greater,
                (
                    OperationParameterKind::Body,
                    OperationParameterKind::Body,
                ) => {
                    panic!("should only be one body")
                }
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
            openapiv3::ParameterSchemaOrContent::Content(c) => Err(
                Error::UnexpectedFormat(format!("unexpected content {:#?}", c)),
            ),
        }
    }
}

// TODO do I want/need this?
trait ExtractJsonMediaType {
    fn is_binary(&self, components: &Option<Components>) -> Result<bool>;
    fn content_json(&self) -> Result<openapiv3::MediaType>;
}

impl ExtractJsonMediaType for openapiv3::RequestBody {
    fn content_json(&self) -> Result<openapiv3::MediaType> {
        if self.content.len() != 1 {
            todo!("expected one content entry, found {}", self.content.len());
        }

        if let Some(mt) = self.content.get("application/json") {
            Ok(mt.clone())
        } else if let Some(mt) =
            self.content.get("application/x-www-form-urlencoded")
        {
            Ok(mt.clone())
        } else {
            todo!(
                "could not find application/json or application/x-www-form-urlencoded, only found {}",
                self.content.keys().next().unwrap()
            );
        }
    }

    fn is_binary(&self, components: &Option<Components>) -> Result<bool> {
        if self.content.is_empty() {
            /*
             * XXX If there are no content types, I guess it is not binary?
             */
            return Ok(false);
        }

        if self.content.len() != 1 {
            todo!("expected one content entry, found {}", self.content.len());
        }

        if let Some(mt) = self.content.get("application/octet-stream") {
            if !mt.encoding.is_empty() {
                todo!("XXX encoding");
            }

            if let Some(s) = &mt.schema {
                use openapiv3::{
                    SchemaKind, StringFormat, Type,
                    VariantOrUnknownOrEmpty::Item,
                };

                let s = s.item(components)?;
                if s.schema_data.nullable {
                    todo!("XXX nullable binary?");
                }
                if s.schema_data.default.is_some() {
                    todo!("XXX default binary?");
                }
                if s.schema_data.discriminator.is_some() {
                    todo!("XXX binary discriminator?");
                }
                match &s.schema_kind {
                    SchemaKind::Type(Type::String(st)) => {
                        if st.min_length.is_some() || st.max_length.is_some() {
                            todo!("binary min/max length");
                        }
                        if !matches!(st.format, Item(StringFormat::Binary)) {
                            todo!(
                                "expected binary format string, got {:?}",
                                st.format
                            );
                        }
                        if st.pattern.is_some() {
                            todo!("XXX pattern");
                        }
                        if !st.enumeration.is_empty() {
                            todo!("XXX enumeration");
                        }
                        return Ok(true);
                    }
                    x => {
                        todo!("XXX schemakind type {:?}", x);
                    }
                }
            } else {
                todo!("binary thing had no schema?");
            }
        }

        Ok(false)
    }
}
