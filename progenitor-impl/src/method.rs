// Copyright 2022 Oxide Computer Company

use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

use convert_case::Case;
use openapiv3::{Components, Response, StatusCode};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use typify::TypeId;

use crate::{template::PathTemplate, util::sanitize, Error, Generator, Result};
use crate::{to_schema::ToSchema, util::ReferenceOrExt};

pub(crate) struct OperationMethod {
    operation_id: String,
    method: String,
    path: PathTemplate,
    summary: Option<String>,
    description: Option<String>,
    params: Vec<OperationParameter>,
    responses: Vec<OperationResponse>,
    dropshot_paginated: Option<DropshotPagination>,
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
        let mut raw_params = operation
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
                        allow_reserved: _,
                        style: openapiv3::QueryStyle::Form,
                        allow_empty_value,
                    } => {
                        if let Some(true) = allow_empty_value {
                            todo!("allow empty value is a no go");
                        }

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
                    x => todo!("unhandled parameter type: {:#?}", x),
                }
            })
            .collect::<Result<Vec<_>>>()?;
        if let Some(b) = &operation.request_body {
            let b = b.item(components)?;
            let typ = if b.is_binary(components)? {
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

            raw_params.push(OperationParameter {
                name: "body".to_string(),
                api_name: "body".to_string(),
                description: b.description.clone(),
                typ,
                kind: OperationParameterKind::Body,
            });
        }
        let tmp = crate::template::parse(path)?;
        let names = tmp.names();

        sort_params(&mut raw_params, &names);

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
            self.dropshot_pagination_data(operation, &raw_params, &responses);

        Ok(OperationMethod {
            operation_id: sanitize(operation_id, Case::Snake),
            method: method.to_string(),
            path: tmp,
            summary: operation.summary.clone().filter(|s| !s.is_empty()),
            description: operation
                .description
                .clone()
                .filter(|s| !s.is_empty()),
            params: raw_params,
            responses,
            dropshot_paginated,
        })
    }

    pub(crate) fn process_method(
        &mut self,
        method: &OperationMethod,
    ) -> Result<TokenStream> {
        let operation_id = format_ident!("{}", method.operation_id);
        let mut bounds_items: Vec<TokenStream> = Vec::new();
        let typed_params = method
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
                        bounds_items.push(quote! { B: Into<reqwest::Body> });
                        quote! { B }
                    }
                };
                (
                    param,
                    quote! {
                        #name: #typ
                    },
                )
            })
            .collect::<Vec<_>>();

        let params = typed_params.iter().map(|(_, stream)| stream);

        let bounds = quote! { < 'a, #(#bounds_items),* > };

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

        let url_path = method.path.compile(
            method
                .params
                .iter()
                .filter_map(|param| match &param.kind {
                    OperationParameterKind::Path => {
                        Some((&param.api_name, &param.name))
                    }
                    _ => None,
                })
                .collect(),
        );

        let body_func =
            method.params.iter().filter_map(|param| match &param.kind {
                OperationParameterKind::Body => match &param.typ {
                    OperationParameterType::Type(_) => {
                        Some(quote! { .json(body) })
                    }
                    OperationParameterType::RawBody => {
                        Some(quote! { .body(body) })
                    }
                },
                _ => None,
            });

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

        let doc_comment = make_doc_comment(method);

        let pre_hook = self.pre_hook.as_ref().map(|hook| {
            quote! {
                (#hook)(&self.inner, &request);
            }
        });
        let post_hook = self.post_hook.as_ref().map(|hook| {
            quote! {
                (#hook)(&self.inner, &result);
            }
        });

        // TODO validate that method is one of the expected methods.
        let method_func = format_ident!("{}", method.method.to_lowercase());

        let method_impl = quote! {
            #[doc = #doc_comment]
            pub async fn #operation_id #bounds (
                &'a self,
                #(#params),*
            ) -> Result<
                ResponseValue<#response_type>,
                Error<#error_type>,
            > {
                #url_path
                #query_build

                let request = self.client
                    . #method_func (url)
                    #(#body_func)*
                    #query_use
                    .build()?;
                #pre_hook
                let result = self.client
                    .execute(request)
                    .await;
                #post_hook

                let response = result?;

                match response.status().as_u16() {
                    // These will be of the form...
                    // 201 => ResponseValue::from_response(response).await,
                    // 200..299 => ResponseValue::empty(response),
                    // TODO this isn't implemented
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
            }
        };

        let stream_impl = method.dropshot_paginated.as_ref().map(|page_data| {
            // We're now using futures.
            self.uses_futures = true;

            let stream_id = format_ident!("{}_stream", method.operation_id);

            // The parameters are the same as those to the paged method, but
            // without "page_token"
            let stream_params =
                typed_params.iter().filter_map(|(param, stream)| {
                    if param.api_name.as_str() == "page_token" {
                        None
                    } else {
                        Some(stream)
                    }
                });

            // The values passed to get the first page are the inputs to the
            // stream method with "None" for the page_token.
            let first_params = typed_params.iter().map(|(param, _)| {
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
            // - The method inputs for non-query parameters
            let step_params = typed_params.iter().map(|(param, _)| {
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
                    self.#operation_id(
                        #(#first_params,)*
                    )
                    .map_ok(move |page| {
                        let page = page.into_inner();
                        // The first page is just an iter
                        let first = futures::stream::iter(
                            page.items.into_iter().map(Ok)
                        );

                        // We unfold subsequent pages using page.next_page as
                        // the seed value. Each iteration returns its items and
                        // the next page token.
                        let rest = futures::stream::try_unfold(
                            page.next_page,
                            move |state| async move {
                                if state.is_none() {
                                    // The page_token was None so we've reached
                                    // the end.
                                    Ok(None)
                                } else {
                                    // Get the next page; here we set all query
                                    // parameters to None (except for the
                                    // page_token), and all other parameters as
                                    // specified at the start of this method.
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
                    ("page_token", OperationParameterKind::Query(_))
                        | ("limit", OperationParameterKind::Query(_))
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

        let properties = details.properties().collect::<HashMap<_, _>>();

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
                typify::TypeDetails::Builtin("String")
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
        method.method.to_ascii_uppercase(),
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
        method.method.to_ascii_uppercase(),
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
            x => {
                Err(Error::BadConversion(format!("XXX param format {:#?}", x)))
            }
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
        } else {
            todo!(
                "could not find application/json, only found {}",
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
