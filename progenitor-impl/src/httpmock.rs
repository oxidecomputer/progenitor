// Copyright 2025 Oxide Computer Company

//! Generation of mocking extensions for `httpmock`

use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::{
    method::{
        BodyContentType, HttpMethod, OperationParameter, OperationParameterKind,
        OperationParameterType, OperationResponse, OperationResponseStatus,
    },
    to_schema::ToSchema,
    util::{sanitize, Case},
    validate_openapi, Generator, Result,
};

struct MockOp {
    when: TokenStream,
    when_impl: TokenStream,
    then: TokenStream,
    then_impl: TokenStream,
}

impl Generator {
    /// Generate a strongly-typed mocking extension to the `httpmock` crate.
    ///
    /// The `crate_path` parameter should be a valid Rust path corresponding to
    /// the SDK. This can include `::` and instances of `-` in the crate name
    /// should be converted to `_`.
    pub fn httpmock(&mut self, spec: &OpenAPI, crate_path: &str) -> Result<TokenStream> {
        validate_openapi(spec)?;

        // Convert our components dictionary to schemars
        let schemas = spec.components.iter().flat_map(|components| {
            components
                .schemas
                .iter()
                .map(|(name, ref_or_schema)| (name.clone(), ref_or_schema.to_schema()))
        });

        self.type_space.add_ref_types(schemas)?;

        let raw_methods = spec
            .paths
            .iter()
            .flat_map(|(path, ref_or_item)| {
                // Exclude externally defined path items.
                let item = ref_or_item.as_item().unwrap();
                item.iter().map(move |(method, operation)| {
                    (path.as_str(), method, operation, &item.parameters)
                })
            })
            .map(|(path, method, operation, path_parameters)| {
                self.process_operation(operation, &spec.components, path, method, path_parameters)
            })
            .collect::<Result<Vec<_>>>()?;

        let methods = raw_methods
            .iter()
            .map(|method| self.httpmock_method(method))
            .collect::<Vec<_>>();

        let op = raw_methods
            .iter()
            .map(|method| format_ident!("{}", &method.operation_id))
            .collect::<Vec<_>>();
        let when = methods.iter().map(|op| &op.when).collect::<Vec<_>>();
        let when_impl = methods.iter().map(|op| &op.when_impl).collect::<Vec<_>>();
        let then = methods.iter().map(|op| &op.then).collect::<Vec<_>>();
        let then_impl = methods.iter().map(|op| &op.then_impl).collect::<Vec<_>>();

        let crate_path = syn::TypePath {
            qself: None,
            path: syn::parse_str(crate_path)
                .unwrap_or_else(|_| panic!("{} is not a valid identifier", crate_path)),
        };

        let code = quote! {
            pub mod operations {

                //! [`When`](::httpmock::When) and [`Then`](::httpmock::Then)
                //! wrappers for each operation. Each can be converted to
                //! its inner type with a call to `into_inner()`. This can
                //! be used to explicitly deviate from permitted values.

                use #crate_path::*;

                #(
                    pub struct #when(::httpmock::When);
                    #when_impl

                    pub struct #then(::httpmock::Then);
                    #then_impl
                )*
            }

            /// An extension trait for [`MockServer`](::httpmock::MockServer) that
            /// adds a method for each operation. These are the equivalent of
            /// type-checked [`mock()`](::httpmock::MockServer::mock) calls.
            pub trait MockServerExt {
                #(
                    fn #op<F>(&self, config_fn: F) -> ::httpmock::Mock
                    where
                        F: FnOnce(operations::#when, operations::#then);
                )*
            }

            impl MockServerExt for ::httpmock::MockServer {
                #(
                    fn #op<F>(&self, config_fn: F) -> ::httpmock::Mock
                    where
                        F: FnOnce(operations::#when, operations::#then)
                    {
                        self.mock(|when, then| {
                            config_fn(
                                operations::#when::new(when),
                                operations::#then::new(then),
                            )
                        })
                    }
                )*
            }
        };
        Ok(code)
    }

    fn httpmock_method(&mut self, method: &crate::method::OperationMethod) -> MockOp {
        let when_name = sanitize(&format!("{}-when", method.operation_id), Case::Pascal);
        let when = format_ident!("{}", when_name).to_token_stream();
        let then_name = sanitize(&format!("{}-then", method.operation_id), Case::Pascal);
        let then = format_ident!("{}", then_name).to_token_stream();

        let http_method = match &method.method {
            HttpMethod::Get => quote! { ::httpmock::Method::GET },
            HttpMethod::Put => quote! { ::httpmock::Method::PUT },
            HttpMethod::Post => quote! { ::httpmock::Method::POST },
            HttpMethod::Delete => quote! { ::httpmock::Method::DELETE },
            HttpMethod::Options => quote! { ::httpmock::Method::OPTIONS },
            HttpMethod::Head => quote! { ::httpmock::Method::HEAD },
            HttpMethod::Patch => quote! { ::httpmock::Method::PATCH },
            HttpMethod::Trace => quote! { ::httpmock::Method::TRACE },
        };

        let path_re = method.path.as_wildcard();

        // Generate methods corresponding to each parameter so that callers
        // can specify a prescribed value for that parameter.
        let when_methods = method.params.iter().map(
            |OperationParameter {
                 name,
                 typ,
                 kind,
                 api_name,
                 description: _,
             }| {
                let arg_type_name = match typ {
                    OperationParameterType::Type(arg_type_id) => self
                        .type_space
                        .get_type(arg_type_id)
                        .unwrap()
                        .parameter_ident(),
                    OperationParameterType::RawBody => match kind {
                        OperationParameterKind::Body(BodyContentType::OctetStream) => quote! {
                            ::serde_json::Value
                        },
                        OperationParameterKind::Body(BodyContentType::Text(_)) => quote! {
                            String
                        },
                        _ => unreachable!(),
                    },
                };

                let name_ident = format_ident!("{}", name);
                let (required, handler) = match kind {
                    OperationParameterKind::Path => {
                        let re_fmt = method.path.as_wildcard_param(api_name);
                        (
                            true,
                            quote! {
                                let re = regex::Regex::new(
                                    &format!(#re_fmt, value.to_string())
                                ).unwrap();
                                Self(self.0.path_matches(re))
                            },
                        )
                    }
                    OperationParameterKind::Query(true) => (
                        true,
                        quote! {
                            Self(self.0.query_param(#api_name, value.to_string()))
                        },
                    ),
                    OperationParameterKind::Header(true) => (
                        true,
                        quote! {
                            Self(self.0.header(#api_name, value.to_string()))
                        },
                    ),

                    OperationParameterKind::Query(false) => (
                        false,
                        quote! {
                            if let Some(value) = value.into() {
                                Self(self.0.query_param(
                                    #api_name,
                                    value.to_string(),
                                ))
                            } else {
                                Self(self.0.matches(|req| {
                                    req.query_params
                                        .as_ref()
                                        .and_then(|qs| {
                                            qs.iter().find(
                                                |(key, _)| key == #api_name)
                                        })
                                        .is_none()
                                }))
                            }
                        },
                    ),
                    OperationParameterKind::Header(false) => (
                        false,
                        quote! {
                            if let Some(value) = value.into() {
                                Self(self.0.header(
                                    #api_name,
                                    value.to_string()
                                ))
                            } else {
                                Self(self.0.matches(|req| {
                                    req.headers
                                        .as_ref()
                                        .and_then(|hs| {
                                            hs.iter().find(
                                                |(key, _)| key == #api_name
                                            )
                                        })
                                        .is_none()
                                }))
                            }
                        },
                    ),
                    OperationParameterKind::Body(body_content_type) => match typ {
                        OperationParameterType::Type(_) => (
                            true,
                            quote! {
                                Self(self.0.json_body_obj(value))

                            },
                        ),
                        OperationParameterType::RawBody => match body_content_type {
                            BodyContentType::OctetStream => (
                                true,
                                quote! {
                                    Self(self.0.json_body(value))
                                },
                            ),
                            BodyContentType::Text(_) => (
                                true,
                                quote! {
                                    Self(self.0.body(value))
                                },
                            ),
                            _ => unreachable!(),
                        },
                    },
                };

                if required {
                    // The value is required so we just check for a simple
                    // match.
                    quote! {
                        pub fn #name_ident(self, value: #arg_type_name) -> Self {
                            #handler
                        }
                    }
                } else {
                    // For optional values we permit an input that's an
                    // `Into<Option<T>`. This allows callers to specify a value
                    // or specify that the parameter must be absent with None.

                    // If the type is a ref, augment it with a lifetime that
                    // we'll also use in the function
                    let (lifetime, arg_type_name) = if let syn::Type::Reference(mut rr) =
                        syn::parse2::<syn::Type>(arg_type_name.clone()).unwrap()
                    {
                        rr.lifetime =
                            Some(syn::Lifetime::new("'a", proc_macro2::Span::call_site()));
                        (Some(quote! { 'a, }), rr.to_token_stream())
                    } else {
                        (None, arg_type_name)
                    };

                    quote! {
                        pub fn #name_ident<#lifetime T>(
                            self,
                            value: T,
                        ) -> Self
                        where
                            T: Into<Option<#arg_type_name>>,
                        {
                            #handler
                        }
                    }
                }
            },
        );

        let when_impl = quote! {
            impl #when {
                pub fn new(inner: ::httpmock::When) -> Self {
                    Self(inner
                        .method(#http_method)
                        .path_matches(regex::Regex::new(#path_re).unwrap()))
                }

                pub fn into_inner(self) -> ::httpmock::When {
                    self.0
                }

                #(#when_methods)*
            }
        };

        // Methods for each discrete response. For specific status codes we use
        // the name of that code; for classes of codes we use the class name
        // and require a status code that must be within the prescribed range.
        let then_methods = method.responses.iter().map(
            |OperationResponse {
                 status_code, typ, ..
             }| {
                let (value_param, value_use) = match typ {
                    crate::method::OperationResponseKind::Type(arg_type_id) => {
                        let arg_type = self.type_space.get_type(arg_type_id).unwrap();
                        let arg_type_ident = arg_type.parameter_ident();
                        (
                            quote! {
                                value: #arg_type_ident,
                            },
                            quote! {
                                .header("content-type", "application/json")
                                .json_body_obj(value)
                            },
                        )
                    }
                    crate::method::OperationResponseKind::None => Default::default(),
                    crate::method::OperationResponseKind::Raw => (
                        quote! {
                            value: ::serde_json::Value,
                        },
                        quote! {
                            .header("content-type", "application/json")
                            .json_body(value)
                        },
                    ),
                    crate::method::OperationResponseKind::Upgrade => Default::default(),
                };

                match status_code {
                    OperationResponseStatus::Code(status_code) => {
                        let canonical_reason = http::StatusCode::from_u16(*status_code)
                            .unwrap()
                            .canonical_reason()
                            .unwrap();
                        let fn_name = format_ident!("{}", &sanitize(canonical_reason, Case::Snake));

                        quote! {
                            pub fn #fn_name(self, #value_param) -> Self {
                                Self(self.0
                                    .status(#status_code)
                                    #value_use
                                )
                            }
                        }
                    }
                    OperationResponseStatus::Range(status_type) => {
                        let status_string = match status_type {
                            1 => "informational",
                            2 => "success",
                            3 => "redirect",
                            4 => "client_error",
                            5 => "server_error",
                            _ => unreachable!(),
                        };
                        let fn_name = format_ident!("{}", status_string);
                        quote! {
                            pub fn #fn_name(self, status: u16, #value_param) -> Self {
                                assert_eq!(status / 100u16, #status_type);
                                Self(self.0
                                    .status(status)
                                    #value_use
                                )
                            }
                        }
                    }
                    OperationResponseStatus::Default => quote! {
                        pub fn default_response(self, status: u16, #value_param) -> Self {
                            Self(self.0
                                .status(status)
                                #value_use
                            )
                        }
                    },
                }
            },
        );

        let then_impl = quote! {
            impl #then {
                pub fn new(inner: ::httpmock::Then) -> Self {
                    Self(inner)
                }

                pub fn into_inner(self) -> ::httpmock::Then {
                    self.0
                }

                #(#then_methods)*
            }
        };

        MockOp {
            when,
            when_impl,
            then,
            then_impl,
        }
    }
}
