// Copyright 2023 Oxide Computer Company

use heck::ToKebabCase;
use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use typify::{TypeSpaceImpl, TypeStructPropInfo};

use crate::{
    method::{
        OperationParameterKind, OperationParameterType, OperationResponseStatus,
    },
    space_out_items,
    to_schema::ToSchema,
    util::{sanitize, Case},
    validate_openapi, Generator, Result,
};

struct CliOperation {
    cli_fn: TokenStream,
    execute_fn: TokenStream,
    cli_trait: TokenStream,
    execute_trait: TokenStream,
}

impl Generator {
    pub fn cli_text(
        &mut self,
        spec: &OpenAPI,
        crate_name: &str,
    ) -> Result<String> {
        let output = self.cli(spec, crate_name)?;

        let content = rustfmt_wrapper::rustfmt_config(
            rustfmt_wrapper::config::Config {
                format_strings: Some(true),
                ..Default::default()
            },
            output,
        )
        .unwrap();

        space_out_items(content)
    }

    pub fn cli(
        &mut self,
        spec: &OpenAPI,
        crate_name: &str,
    ) -> Result<TokenStream> {
        validate_openapi(spec)?;

        // Convert our components dictionary to schemars
        let schemas = spec.components.iter().flat_map(|components| {
            components.schemas.iter().map(|(name, ref_or_schema)| {
                (name.clone(), ref_or_schema.to_schema())
            })
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
                self.process_operation(
                    operation,
                    &spec.components,
                    path,
                    method,
                    path_parameters,
                )
            })
            .collect::<Result<Vec<_>>>()?;

        let methods = raw_methods
            .iter()
            .map(|method| self.cli_method(method))
            .collect::<Vec<_>>();

        let cli_ops = methods.iter().map(|op| &op.cli_fn);
        let execute_ops = methods.iter().map(|op| &op.execute_fn);
        let ops = methods.iter().map(
            |CliOperation {
                 cli_fn,
                 execute_fn,
                 cli_trait: _,
                 execute_trait: _,
             }| {
                quote! {
                    #cli_fn
                    #execute_fn
                }
            },
        );

        let trait_ops = methods.iter().map(
            |CliOperation {
                 cli_fn: _,
                 execute_fn: _,
                 cli_trait,
                 execute_trait,
             }| {
                quote! {
                    // #cli_trait
                    #execute_trait
                }
            },
        );

        let cli_fns = raw_methods
            .iter()
            .map(|method| {
                format_ident!(
                    "cli_{}",
                    sanitize(&method.operation_id, Case::Snake)
                )
            })
            .collect::<Vec<_>>();
        let execute_fns = raw_methods
            .iter()
            .map(|method| {
                format_ident!(
                    "execute_{}",
                    sanitize(&method.operation_id, Case::Snake)
                )
            })
            .collect::<Vec<_>>();

        let cli_variants = raw_methods
            .iter()
            .map(|method| {
                format_ident!(
                    "{}",
                    sanitize(&method.operation_id, Case::Pascal)
                )
            })
            .collect::<Vec<_>>();

        let crate_ident = format_ident!("{}", crate_name);

        let code = quote! {
            pub struct Cli<T: CliOverride = ()> {
                client: #crate_ident::Client,
                over: T,
            }
            impl Cli {
                pub fn new(client: #crate_ident::Client) -> Self {
                    Self { client, over: () }
                }

                pub fn get_command(cmd: CliCommand) -> clap::Command {
                    match cmd {
                        #(
                            CliCommand::#cli_variants => Self::#cli_fns(),
                        )*
                    }
                }

                #(#cli_ops)*
            }

            impl<T: CliOverride> Cli<T> {
                pub fn new_with_override(
                    client: #crate_ident::Client,
                    over: T,
                ) -> Self {
                    Self { client, over }
                }

                pub async fn execute(
                    &self,
                    cmd: CliCommand,
                    matches: &clap::ArgMatches,
                ) {
                    let _ = match cmd {
                        #(
                            CliCommand::#cli_variants => {
                                // TODO ... do something with output
                                self.#execute_fns(matches).await;
                            }
                        )*
                    };
                }

                #(#execute_ops)*
            }

            pub trait CliOverride {
                #(#trait_ops)*
            }

            impl CliOverride for () {}

            #[derive(Copy, Clone, Debug)]
            pub enum CliCommand {
                #(#cli_variants,)*
            }

            impl CliCommand {
                pub fn iter() -> impl Iterator<Item = CliCommand> {
                    vec![
                        #(
                            CliCommand::#cli_variants,
                        )*
                    ].into_iter()
                }
            }

        };

        Ok(code)
    }

    fn cli_method(
        &mut self,
        method: &crate::method::OperationMethod,
    ) -> CliOperation {
        let fn_name = format_ident!("cli_{}", &method.operation_id);

        let args = method
            .params
            .iter()
            .filter(|param| {
                !matches!(&param.kind, OperationParameterKind::Body(_))
                    && (method.dropshot_paginated.is_none()
                        || (param.name.as_str() != "page_token"))
            })
            .map(|param| {
                let arg_name = param.name.to_kebab_case();

                let required = match &param.kind {
                    OperationParameterKind::Path => true,
                    OperationParameterKind::Query(required) => *required,
                    OperationParameterKind::Header(required) => *required,
                    OperationParameterKind::Body(_) => unreachable!(),
                };

                let OperationParameterType::Type(arg_type_id) = &param.typ else {
                    panic!()
                };
                let arg_type = self.type_space.get_type(arg_type_id).unwrap();
                let arg_details = arg_type.details();
                let arg_type_name = match &arg_details{
                    typify::TypeDetails::Option(opt_id) => {
                        let inner_type = self.type_space.get_type(opt_id).unwrap();
                        inner_type.ident()
                    }
                    _ => {
                        arg_type.ident()
                    }
                };

                let help = param.description.as_ref().map(|description| {
                    quote! {
                        .help(#description)
                    }
                });

                quote! {
                    clap::Arg::new(#arg_name)
                        .long(#arg_name)
                        .required(#required)
                        .value_parser(clap::value_parser!(#arg_type_name))
                        #help
                }
            });

        let maybe_body_param = method.params.iter().find(|param| {
            matches!(&param.kind, OperationParameterKind::Body(_))
                // TODO not sure how to deal with raw bodies right now
                    && matches!(&param.typ, OperationParameterType::Type(_))
        });

        let body_arg = maybe_body_param.map(|param| {
            let OperationParameterType::Type(type_id) = &param.typ else {
                    unreachable!();
                };

            let body_args = self.type_space.get_type(type_id).unwrap();

            let body_arg = match body_args.details() {
                typify::TypeDetails::Struct(s) => {
                    s.properties_info()
                        .filter_map(|prop_info| {
                            let TypeStructPropInfo {
                                name: prop_name,
                                description,
                                required,
                                type_id: prop_type_id,
                            } = prop_info;
                            let prop_type = self
                                .type_space
                                .get_type(&prop_type_id)
                                .unwrap();
                            let prop_type_ident = prop_type.ident();
                            let good =
                                prop_type.has_impl(TypeSpaceImpl::FromStr);
                            let prop_name = prop_name.to_kebab_case();
                            // assert!(good || !required);

                            good.then(|| {
                                let help =
                                    description.as_ref().map(|description| {
                                        quote! {
                                            .help(#description)
                                        }
                                    });
                                quote! {
                                    clap::Arg::new(#prop_name)
                                        .long(#prop_name)
                                        .required(#required)
                                        .value_parser(clap::value_parser!(
                                            #prop_type_ident
                                        ))
                                        #help
                                }
                            })
                        })
                        .collect::<Vec<_>>()
                }
                _ => Vec::new(),
            };

            quote! {
                #(
                    .arg(#body_arg)
                )*
            }
        });

        // TODO parameter for body as input json (--body-input?)
        // TODO parameter to output a body template (--body-template?)
        // TODO deal with all parameters?

        let about = method.summary.as_ref().map(|summary| {
            let mut about_str = summary.clone();
            if let Some(description) = &method.description {
                about_str.push_str("\n\n");
                about_str.push_str(description);
            }

            quote! {
                .about(#about_str)
            }
        });

        let cli_fn = quote! {
            pub fn #fn_name() -> clap::Command
            {
                clap::Command::new("")
                #(
                    .arg(#args)
                )*
                #body_arg
                #about
            }
        };

        let cli_trait = quote! {
            fn #fn_name(cmd: clap::Command) -> clap::Command {
                cmd
            }
        };

        let op_name = format_ident!("{}", &method.operation_id);

        let fn_name = format_ident!("execute_{}", &method.operation_id);

        let args = method
            .params
            .iter()
            .filter(|param| {
                !matches!(&param.kind, OperationParameterKind::Body(_))
                    && (method.dropshot_paginated.is_none()
                        || (param.name.as_str() != "page_token"))
            })
            .map(|param| {
                let arg_name = param.name.to_kebab_case();
                let arg_fn_name = sanitize(&param.name, Case::Snake);
                let arg_fn = format_ident!("{}", arg_fn_name);
                let OperationParameterType::Type(arg_type_id) = &param.typ else {
                    panic!()
                };
                let arg_type = self.type_space.get_type(arg_type_id).unwrap();

                // TODO this really should be simpler.
                let arg_details = arg_type.details();
                let arg_type_name = match &arg_details{
                    typify::TypeDetails::Option(opt_id) => {
                        let inner_type = self.type_space.get_type(opt_id).unwrap();
                        inner_type.ident()
                    }
                    _ => {
                        arg_type.ident()
                    }
                };

                quote! {
                    if let Some(value) =
                        matches.get_one::<#arg_type_name>(#arg_name)
                    {
                        // clone here in case the arg type doesn't impl
                        // From<&T>
                        request = request.#arg_fn(value.clone());
                    }
                }
            });

        let body_arg = maybe_body_param.map(|param| {
            let OperationParameterType::Type(type_id) = &param.typ else {
                    unreachable!();
                };

            let body_type = self.type_space.get_type(type_id).unwrap();

            let maybe_body_args = match body_type.details() {
                typify::TypeDetails::Struct(s) => {
                    let args = s
                        .properties_info()
                        .filter_map(|prop_info| {
                            let TypeStructPropInfo {
                                name: prop_name,
                                description: _,
                                required: _,
                                type_id: body_type_id,
                            } = prop_info;
                            let prop_type = self
                                .type_space
                                .get_type(&body_type_id)
                                .unwrap();
                            let prop_fn = format_ident!(
                                "{}",
                                sanitize(prop_name, Case::Snake)
                            );
                            let prop_name = prop_name.to_kebab_case();
                            let prop_type_ident = prop_type.ident();
                            let good =
                                prop_type.has_impl(TypeSpaceImpl::FromStr);
                            // assert!(good || !required);

                            good.then(|| {
                                quote! {
                                    if let Some(value) =
                                        matches.get_one::<#prop_type_ident>(
                                            #prop_name,
                                        )
                                    {
                                        // clone here in case the arg type
                                        // doesn't impl TryFrom<&T>
                                        request = request.body_map(|body| {
                                            body.#prop_fn(value.clone())
                                        })
                                    }
                                }
                            })
                        })
                        .collect::<Vec<_>>();
                    Some(args)
                }
                _ => None,
            };

            // TODO rework this.
            maybe_body_args.map(|body_args| {
                quote! {
                    #( #body_args )*
                }
            })
        });

        let (_, success_type) = self.extract_responses(
            method,
            OperationResponseStatus::is_success_or_default,
        );
        let (_, error_type) = self.extract_responses(
            method,
            OperationResponseStatus::is_error_or_default,
        );

        let success_output = match success_type {
            crate::method::OperationResponseType::Type(_) => {
                quote! { println!("success\n{:#?}", r) }
            }
            crate::method::OperationResponseType::None => {
                quote! { println!("success\n{:#?}", r) }
            }
            crate::method::OperationResponseType::Raw => quote! { todo!() },
            crate::method::OperationResponseType::Upgrade => quote! { todo!() },
        };

        let error_output = match error_type {
            crate::method::OperationResponseType::Type(_) => {
                quote! { println!("error\n{:#?}", r) }
            }
            crate::method::OperationResponseType::None => {
                quote! { println!("success\n{:#?}", r) }
            }
            crate::method::OperationResponseType::Raw => quote! { todo!() },
            crate::method::OperationResponseType::Upgrade => quote! { todo!() },
        };

        let execute_fn = quote! {
            pub async fn #fn_name(&self, matches: &clap::ArgMatches)
                // ->
                // Result<ResponseValue<#success_type>, Error<#error_type>>
            {
                let mut request = self.client.#op_name();
                #( #args )*
                #body_arg

                // TODO don't want to unwrap.
                self.over
                    .#fn_name(matches, &mut request)
                    .unwrap();

                let result = request.send().await;

                match result {
                    Ok(r) => {
                        #success_output
                    }
                    Err(r) => {
                        #error_output
                    }
                }
            }
        };

        // TODO this is copy-pasted--unwisely?
        let struct_name = sanitize(&method.operation_id, Case::Pascal);
        let struct_ident = format_ident!("{}", struct_name);

        let execute_trait = quote! {
            fn #fn_name(
                &self,
                matches: &clap::ArgMatches,
                request: &mut builder :: #struct_ident,
            ) -> Result<(), String> {
                Ok(())
            }
        };

        CliOperation {
            cli_fn,
            execute_fn,
            cli_trait,
            execute_trait,
        }
    }
}
