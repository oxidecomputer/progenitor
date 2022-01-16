// Copyright 2022 Oxide Computer Company

use std::{cmp::Ordering, collections::HashMap};

use convert_case::{Case, Casing};
use indexmap::IndexMap;
use openapiv3::{
    Components, OpenAPI, Parameter, ReferenceOr, RequestBody, Response, Schema,
    StatusCode,
};
use proc_macro2::TokenStream;

use quote::{format_ident, quote, ToTokens};
use template::PathTemplate;
use thiserror::Error;
use typify::{TypeId, TypeSpace};
use unicode_xid::UnicodeXID;

use crate::to_schema::ToSchema;

mod template;
mod to_schema;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected value type")]
    BadValue(String, serde_json::Value),
    #[error("type error")]
    TypeError(#[from] typify::Error),
    #[error("XXX")]
    BadConversion(String),
    #[error("invalid operation path")]
    InvalidPath(String),
    //#[error("unknown")]
    //Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Generator {
    type_space: TypeSpace,
    inner_type: Option<TokenStream>,
    pre_hook: Option<TokenStream>,
    post_hook: Option<TokenStream>,
    uses_futures: bool,
}

struct OperationMethod {
    operation_id: String,
    method: String,
    path: PathTemplate,
    doc_comment: Option<String>,
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
    name: String,
    typ: OperationParameterType,
    kind: OperationParameterKind,
}

enum OperationParameterType {
    Type(TypeId),
    RawBody,
}
#[derive(Debug)]
struct OperationResponse {
    status_code: StatusCode,
    typ: OperationResponseType,
}

#[derive(Debug)]
enum OperationResponseType {
    Type(TypeId),
    None,
    Raw,
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_inner_type(&mut self, inner_type: TokenStream) -> &mut Self {
        self.inner_type = Some(inner_type);
        self
    }

    pub fn with_pre_hook(&mut self, pre_hook: TokenStream) -> &mut Self {
        self.pre_hook = Some(pre_hook);
        self
    }

    pub fn with_post_hook(&mut self, post_hook: TokenStream) -> &mut Self {
        self.post_hook = Some(post_hook);
        self
    }

    pub fn with_derive(&mut self, derive: TokenStream) -> &mut Self {
        self.type_space.add_derive(derive);
        self
    }

    pub fn generate_tokens(&mut self, spec: &OpenAPI) -> Result<TokenStream> {
        // Convert our components dictionary to schemars
        let schemas = spec
            .components
            .iter()
            .flat_map(|components| {
                components.schemas.iter().map(|(name, ref_or_schema)| {
                    (name.clone(), ref_or_schema.to_schema())
                })
            })
            .collect::<Vec<(String, _)>>();

        self.type_space.set_type_mod("types");
        self.type_space.add_ref_types(schemas)?;

        let raw_methods = spec
            .paths
            .iter()
            .flat_map(|(path, ref_or_item)| {
                // Exclude externally defined path items.
                let item = ref_or_item.as_item().unwrap();
                // TODO punt on paramters that apply to all path items for now.
                assert!(item.parameters.is_empty());
                item.iter().map(move |(method, operation)| {
                    (path.as_str(), method, operation)
                })
            })
            .map(|(path, method, operation)| {
                self.process_operation(
                    operation,
                    &spec.components,
                    path,
                    method,
                )
            })
            .collect::<Result<Vec<_>>>()?;

        let methods = raw_methods
            .iter()
            .map(|method| self.process_method(method))
            .collect::<Result<Vec<_>>>()?;

        let mut types = self
            .type_space
            .iter_types()
            .map(|t| (t.name(), t.definition()))
            .collect::<Vec<_>>();
        types.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));
        let types = types.into_iter().map(|(_, def)| def);

        let inner_property = self.inner_type.as_ref().map(|inner| {
            quote! {
                inner: #inner,
            }
        });
        let inner_value = self.inner_type.as_ref().map(|_| {
            quote! {
                inner
            }
        });

        let file = quote! {
            use anyhow::Result;

            mod progenitor_support {
                use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

                #[allow(dead_code)]
                const PATH_SET: &AsciiSet = &CONTROLS
                    .add(b' ')
                    .add(b'"')
                    .add(b'#')
                    .add(b'<')
                    .add(b'>')
                    .add(b'?')
                    .add(b'`')
                    .add(b'{')
                    .add(b'}');

                #[allow(dead_code)]
                pub(crate) fn encode_path(pc: &str) -> String {
                    utf8_percent_encode(pc, PATH_SET).to_string()
                }
            }

            pub mod types {
                use serde::{Deserialize, Serialize};
                #(#types)*
            }

            #[derive(Clone)]
            pub struct Client {
                baseurl: String,
                client: reqwest::Client,
                #inner_property
            }

            impl Client {
                pub fn new(
                    baseurl: &str,
                    #inner_property
                ) -> Self {
                    let dur = std::time::Duration::from_secs(15);
                    let client = reqwest::ClientBuilder::new()
                        .connect_timeout(dur)
                        .timeout(dur)
                        .build()
                        .unwrap();
                    Self::new_with_client(baseurl, client, #inner_value)
                }

                pub fn new_with_client(
                    baseurl: &str,
                    client: reqwest::Client,
                    #inner_property
                ) -> Self {
                    Self {
                        baseurl: baseurl.to_string(),
                        client,
                        #inner_value
                    }
                }

                pub fn baseurl(&self) -> &String {
                    &self.baseurl
                }

                pub fn client(&self) -> &reqwest::Client {
                    &self.client
                }

                #(#methods)*
            }
        };

        Ok(file)
    }

    fn process_operation(
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

                        let nam = parameter_data.name.clone();
                        let schema = parameter_data.schema()?.to_schema();

                        let name = sanitize(
                            &format!("{}-{}", operation_id, nam),
                            Case::Pascal,
                        );
                        let typ = self
                            .type_space
                            .add_type_with_name(&schema, Some(name))?;

                        Ok(OperationParameter {
                            name: sanitize(&parameter_data.name, Case::Snake),
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

                        let nam = parameter_data.name.clone();
                        let mut schema = parameter_data.schema()?.to_schema();
                        let name = sanitize(
                            &format!(
                                "{}-{}",
                                operation.operation_id.as_ref().unwrap(),
                                nam
                            ),
                            Case::Pascal,
                        );

                        if !parameter_data.required {
                            schema = make_optional(schema);
                        }

                        let typ = self
                            .type_space
                            .add_type_with_name(&schema, Some(name))?;

                        query.push((nam, !parameter_data.required));
                        Ok(OperationParameter {
                            name: sanitize(&parameter_data.name, Case::Snake),
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
                typ,
                kind: OperationParameterKind::Body,
            });
        }
        let tmp = template::parse(path)?;
        let names = tmp.names();
        raw_params.sort_by(
            |OperationParameter {
                 kind: a_kind,
                 name: a_name,
                 ..
             },
             OperationParameter {
                 kind: b_kind,
                 name: b_name,
                 ..
             }| {
                match (a_kind, b_kind) {
                    // Path params are first and are in positional order.
                    (
                        OperationParameterKind::Path,
                        OperationParameterKind::Path,
                    ) => {
                        let a_index =
                            names.iter().position(|x| x == a_name).unwrap();
                        let b_index =
                            names.iter().position(|x| x == b_name).unwrap();
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

                    // Body params are last and should be unique
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

        let mut success = false;

        let mut responses = operation
            .responses
            .responses
            .iter()
            .map(|(status_code, response_or_ref)| {
                let response = response_or_ref.item(components)?;

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

                if matches!(
                    status_code,
                    StatusCode::Code(200..=299) | StatusCode::Range(2)
                ) {
                    success = true;
                }

                Ok(OperationResponse {
                    status_code: status_code.clone(),
                    typ,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        // If the API has declined to specify the characteristics of a
        // successful response, we cons up a generic one. Note that this is
        // technically permissible within OpenAPI, but advised against in the
        // spec.
        if !success {
            responses.push(OperationResponse {
                status_code: StatusCode::Range(2),
                typ: OperationResponseType::Raw,
            });
        }

        let dropshot_paginated =
            self.dropshot_pagination_data(operation, &raw_params, &responses);

        Ok(OperationMethod {
            operation_id: sanitize(operation_id, Case::Snake),
            method: method.to_string(),
            path: tmp,
            doc_comment: operation.description.clone(),
            params: raw_params,
            responses,
            dropshot_paginated,
        })
    }

    fn process_method(
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
                    let qn = &param.name;
                    Some(if *required {
                        quote! {
                            query.push((#qn, #qn.to_string()));
                        }
                    } else {
                        let qn_ident = format_ident!("{}", qn);
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

        let url_path = method.path.compile();

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

        let mut success_response_items =
            method.responses.iter().filter(|response| {
                matches!(
                    response.status_code,
                    StatusCode::Code(200..=299) | StatusCode::Range(2)
                )
            });

        assert_eq!(success_response_items.clone().count(), 1);

        let (response_type, decode_response) = success_response_items
            .next()
            .map(|response| match &response.typ {
                OperationResponseType::Type(type_id) => (
                    self.type_space.get_type(type_id).unwrap().ident(),
                    quote! { res.json().await? },
                ),
                OperationResponseType::None => {
                    (quote! { reqwest::Response }, quote! { res })
                }
                OperationResponseType::Raw => {
                    (quote! { reqwest::Response }, quote! { res })
                }
            })
            .unwrap();

        // TODO document parameters
        let doc_comment = format!(
            "{}{}: {} {}",
            method
                .doc_comment
                .as_ref()
                .map(|s| format!("{}\n\n", s))
                .unwrap_or_else(String::new),
            method.operation_id,
            method.method.to_ascii_uppercase(),
            method.path.to_string(),
        );

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
            ) -> Result<#response_type> {
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

                // TODO we should do a match here for result?.status().as_u16()
                let res = result?.error_for_status()?;

                Ok(#decode_response)
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
                    if param.name.as_str() == "page_token" {
                        None
                    } else {
                        Some(stream)
                    }
                });

            // The values passed to get the first page are the inputs to the
            // stream method with "None" for the page_token.
            let first_params = typed_params.iter().map(|(param, _)| {
                if param.name.as_str() == "page_token" {
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
                if param.name.as_str() == "page_token" {
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

            // TODO document parameters
            let doc_comment = format!(
                "{}returns a Stream<Item = {}> by making successive calls to {}",
                method
                    .doc_comment
                    .as_ref()
                    .map(|s| format!("{}\n\n", s))
                    .unwrap_or_else(String::new),
                item.name(),
                method.operation_id,
            );


            quote! {
                #[doc = #doc_comment]
                pub fn #stream_id #bounds (
                    &'a self,
                    #(#stream_params),*
                ) -> impl futures::Stream<Item = Result<#item_type>> + Unpin + '_ {
                    use futures::StreamExt;
                    use futures::TryFutureExt;
                    use futures::TryStreamExt;

                    // Execute the operation with the basic parameters
                    // (omitting page_token) to get the first page.
                    self.#operation_id(
                        #(#first_params,)*
                    )
                    .map_ok(move |page| {
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
                    (param.name.as_str(), &param.kind),
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
                        StatusCode::Code(200..=299) | StatusCode::Range(2),
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

    pub fn generate_text(&mut self, spec: &OpenAPI) -> Result<String> {
        let output = self.generate_tokens(spec)?;

        // Format the file with rustfmt.
        let content = rustfmt_wrapper::rustfmt(output).unwrap();

        // Add newlines after end-braces at <= two levels of indentation.
        Ok(if cfg!(not(windows)) {
            let regex = regex::Regex::new(r#"(})(\n\s{0,8}[^} ])"#).unwrap();
            regex.replace_all(&content, "$1\n$2").to_string()
        } else {
            let regex = regex::Regex::new(r#"(})(\r\n\s{0,8}[^} ])"#).unwrap();
            regex.replace_all(&content, "$1\r\n$2").to_string()
        })
    }

    pub fn dependencies(&self) -> Vec<String> {
        let mut deps = vec![
            "anyhow = \"1.0\"",
            "percent-encoding = \"2.1\"",
            "serde = { version = \"1.0\", features = [\"derive\"] }",
            "reqwest = { version = \"0.11\", features = [\"json\", \"stream\"] }",
        ];
        if self.type_space.uses_uuid() {
            deps.push(
                "uuid = { version = \"0.8\", features = [\"serde\", \"v4\"] }",
            )
        }
        if self.type_space.uses_chrono() {
            deps.push("chrono = { version = \"0.4\", features = [\"serde\"] }")
        }
        if self.uses_futures {
            deps.push("futures = \"0.3\"")
        }
        if self.type_space.uses_serde_json() {
            deps.push("serde_json = \"1.0\"")
        }
        deps.sort_unstable();
        deps.iter().map(ToString::to_string).collect()
    }

    pub fn get_type_space(&self) -> &TypeSpace {
        &self.type_space
    }
}

/// Make the schema optional if it isn't already.
pub fn make_optional(
    schema: schemars::schema::Schema,
) -> schemars::schema::Schema {
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

trait ExtractJsonMediaType {
    fn is_binary(&self, components: &Option<Components>) -> Result<bool>;
    fn content_json(&self) -> Result<openapiv3::MediaType>;
}

impl ExtractJsonMediaType for openapiv3::Response {
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

    fn is_binary(&self, _components: &Option<Components>) -> Result<bool> {
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

                let s = s.item(&None)?;
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

trait ReferenceOrExt<T: ComponentLookup> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T>;
}
trait ComponentLookup: Sized {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>>;
}

impl<T: ComponentLookup> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T> {
        match self {
            ReferenceOr::Item(item) => Ok(item),
            ReferenceOr::Reference { reference } => {
                let idx = reference.rfind('/').unwrap();
                let key = &reference[idx + 1..];
                let parameters =
                    T::get_components(components.as_ref().unwrap());
                parameters.get(key).unwrap().item(components)
            }
        }
    }
}

impl ComponentLookup for Parameter {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.parameters
    }
}

impl ComponentLookup for RequestBody {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.request_bodies
    }
}

impl ComponentLookup for Response {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.responses
    }
}

impl ComponentLookup for Schema {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.schemas
    }
}

fn sanitize(input: &str, case: Case) -> String {
    let out = input
        .replace("'", "")
        .replace(|c: char| !c.is_xid_continue(), "-")
        .to_case(case);

    let out = match out.chars().next() {
        None => "x".to_case(case),
        Some(c) if c.is_xid_start() => out,
        Some(_) => format!("_{}", out),
    };

    // Make sure the string is a valid Rust identifier.
    if syn::parse_str::<syn::Ident>(&out).is_ok() {
        out
    } else {
        format!("{}_", out)
    }
}
