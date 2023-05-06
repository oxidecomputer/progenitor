// Copyright 2023 Oxide Computer Company

use heck::ToKebabCase;
use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use typify::{Type, TypeEnumVariant, TypeSpaceImpl, TypeStructPropInfo};

use crate::{
    method::{
        OperationParameterKind, OperationParameterType, OperationResponseStatus,
    },
    to_schema::ToSchema,
    util::{sanitize, Case},
    validate_openapi, Generator, Result,
};

struct CliOperation {
    cli_fn: TokenStream,
    execute_fn: TokenStream,
    execute_trait: TokenStream,
}

impl Generator {
    /// Generate a `clap`-based CLI.
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
        let trait_ops = methods.iter().map(|op| &op.execute_trait);

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
                    match cmd {
                        #(
                            CliCommand::#cli_variants => {
                                // TODO ... do something with output
                                self.#execute_fns(matches).await;
                            }
                        )*
                    }
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
        let maybe_body_arg = method.params.iter().find(|param| {
            matches!(&param.kind, OperationParameterKind::Body(_))
            // TODO not sure how to deal with raw bodies right now
            && matches!(&param.typ, OperationParameterType::Type(_))
        });

        let mut full_body = true;

        // Preprocess the body parameter (if there is one) to create an
        // iterator of top-level properties that can be represented as scalar
        // values. We use these to create `clap::Arg` structures and then to
        // build up the body parameter in the actual API call.
        let body_params = maybe_body_arg
            .into_iter()
            .flat_map(|param| {
                let OperationParameterType::Type(type_id) = &param.typ else {
                    unreachable!();
                };

                let body_arg_type = self.type_space.get_type(type_id).unwrap();
                let details = body_arg_type.details();

                match details {
                    typify::TypeDetails::Struct(struct_info) => {
                        struct_info
                            .properties_info()
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

                                // TODO this is maybe a kludge--not completely sure
                                // of the right way to handle option types. On one
                                // hand, we could want types from this interface to
                                // never show us Option<T> types--we could let the
                                // `required` field give us that information. On
                                // the other hand, there might be Option types that
                                // are required ... at least in the JSON sense,
                                // meaning that we need to include `"foo": null`
                                // rather than omitting the field. Back to the
                                // first hand: is that last point just a serde
                                // issue rather than an interface one?
                                let maybe_inner_type =
                                    if let typify::TypeDetails::Option(
                                        inner_type_id,
                                    ) = prop_type.details()
                                    {
                                        let inner_type = self
                                            .type_space
                                            .get_type(&inner_type_id)
                                            .unwrap();
                                        Some(inner_type)
                                    } else {
                                        None
                                    };

                                let prop_type = if let Some(inner_type) =
                                    maybe_inner_type
                                {
                                    inner_type
                                } else {
                                    prop_type
                                };

                                let scalar =
                                    prop_type.has_impl(TypeSpaceImpl::FromStr);
                                let prop_name = prop_name.to_kebab_case();

                                // If there's a required property that we can't
                                // represent as a scalar (and therefore as a
                                // CLI parameter), the user will be unable to
                                // specify the full body without a json file.
                                if required && !scalar {
                                    full_body = false;
                                }

                                // println!(
                                //     "{}::{}: {}; scalar: {}; required: {}",
                                //     body_args.name(),
                                //     prop_name,
                                //     prop_type.name(),
                                //     scalar,
                                //     required,
                                // );

                                scalar.then(|| {
                                    (
                                        prop_name.clone(),
                                        required,
                                        description.map(str::to_string),
                                        prop_type,
                                    )
                                })
                            })
                            .collect::<Vec<_>>()
                    }
                    _ => Vec::new(),
                }
            })
            .collect::<Vec<_>>();
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
                    OperationParameterKind::Path => Volitionality::Required,
                    OperationParameterKind::Query(true)
                    | OperationParameterKind::Header(true) => {
                        Volitionality::Required
                    }
                    OperationParameterKind::Query(false)
                    | OperationParameterKind::Header(false) => {
                        Volitionality::Optional
                    }
                    OperationParameterKind::Body(_) => unreachable!(),
                };

                let OperationParameterType::Type(arg_type_id) = &param.typ
                else {
                    panic!()
                };
                let arg_type = self.type_space.get_type(arg_type_id).unwrap();

                clap_arg(&arg_name, required, &param.description, &arg_type)
            });

        let body_args = body_params.iter().map(
            |(prop_name, required, description, prop_type)| {
                let volitionality = if *required {
                    Volitionality::RequiredIfNoBody
                } else {
                    Volitionality::Optional
                };
                clap_arg(prop_name, volitionality, description, prop_type)
            },
        );

        // TODO parameter for body as input json (--body-input?)
        // TODO parameter to output a body template (--body-template?)
        // TODO deal with all parameters?

        let body_json_args = maybe_body_arg.map(|_| {
            let help = "Path to a file that contains the full json body.";
            let required = !full_body;

            quote! {
                .arg(
                    clap::Arg::new("json-body")
                        .long("json-body")
                        .value_name("JSON-FILE")
                        // Required if we can't turn the body into individual
                        // parameters.
                        .required(#required)
                        .value_parser(clap::value_parser!(std::path::PathBuf))
                        .help(#help)
                )
                .arg(
                    clap::Arg::new("json-body-template")
                        .long("json-body-template")
                        .action(clap::ArgAction::SetTrue)
                        .help("XXX")
                )
            }
        });

        let about = method.summary.as_ref().map(|summary| {
            quote! {
                .about(#summary)
            }
        });

        let long_about = method.description.as_ref().map(|description| {
            quote! {
                .long_about(#description)
            }
        });

        let cli_fn = quote! {
            pub fn #fn_name() -> clap::Command
            {
                clap::Command::new("")
                #(
                    .arg(#args)
                )*
                #(
                    .arg(#body_args)
                )*
                #body_json_args
                #about
                #long_about
            }
        };

        let op_name = format_ident!("{}", &method.operation_id);
        let fn_name = format_ident!("execute_{}", &method.operation_id);

        // Build up the iterator processing each top-level parameter.
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
                let arg_type_name = arg_type.ident();

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

        // Build up the iterator processing each body property we can handle.
        let body_args =
            body_params.iter().map(|(prop_name, _, _, prop_type)| {
                let prop_fn =
                    format_ident!("{}", sanitize(prop_name, Case::Snake));
                let prop_type_ident = prop_type.ident();
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

        let execute_and_output = match method.dropshot_paginated {
            None => {
                quote! {
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
            }
            Some(_) => {
                quote! {
                    let mut stream = request.stream();

                    loop {
                        match futures::TryStreamExt::try_next(&mut stream).await {
                            Err(r) => {
                                #error_output;
                                break;
                            }
                            Ok(None) => {
                                break;
                            }
                            Ok(Some(value)) => {
                                println!("{:#?}", value);
                            }
                        }
                    }
                }
            }
        };

        let body_json_args = maybe_body_arg.map(|body_param| {
            let OperationParameterType::Type(body_type_id) = &body_param.typ
            else {
                unreachable!();
            };
            let body_type = self.type_space.get_type(body_type_id).unwrap();
            let body_type_ident = body_type.ident();
            quote! {
                if let Some(value) =
                    matches.get_one::<std::path::PathBuf>("json-body")
                {
                    let body_txt = std::fs::read_to_string(value).unwrap();
                    let body_value =
                        serde_json::from_str::<#body_type_ident>(
                            &body_txt,
                        )
                        .unwrap();
                    request = request.body(body_value);
                }
            }
        });

        let execute_fn = quote! {
            pub async fn #fn_name(&self, matches: &clap::ArgMatches)
                // ->
                // Result<ResponseValue<#success_type>, Error<#error_type>>
            {
                let mut request = self.client.#op_name();
                #body_json_args
                #( #args )*
                #( #body_args )*

                // Call the override function.
                // TODO don't want to unwrap.
                self.over
                    .#fn_name(matches, &mut request)
                    .unwrap();

                #execute_and_output
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
            execute_trait,
        }
    }
}

enum Volitionality {
    Optional,
    Required,
    RequiredIfNoBody,
}

fn clap_arg(
    arg_name: &str,
    volitionality: Volitionality,
    description: &Option<String>,
    arg_type: &Type,
) -> TokenStream {
    let help = description.as_ref().map(|description| {
        quote! {
            .help(#description)
        }
    });
    let arg_type_name = arg_type.ident();

    // For enums that have **only** simple variants, we do some slightly
    // fancier argument handling to expose the possible values. In particular,
    // we use clap's `PossibleValuesParser` with each variant converted to a
    // string. Then we use TypedValueParser::map to translate that into the
    // actual type of the enum.
    let maybe_enum_parser =
        if let typify::TypeDetails::Enum(e) = arg_type.details() {
            let maybe_var_names = e
                .variants()
                .map(|(var_name, var_details)| {
                    if let TypeEnumVariant::Simple = var_details {
                        Some(format_ident!("{}", var_name))
                    } else {
                        None
                    }
                })
                .collect::<Option<Vec<_>>>();

            maybe_var_names.map(|var_names| {
                quote! {
                    clap::builder::TypedValueParser::map(
                        clap::builder::PossibleValuesParser::new([
                            #( #arg_type_name :: #var_names.to_string(), )*
                        ]),
                        |s| #arg_type_name :: try_from(s).unwrap()
                    )
                }
            })
        } else {
            None
        };

    let value_parser = if let Some(enum_parser) = maybe_enum_parser {
        enum_parser
    } else {
        // Let clap pick a value parser for us. This has the benefit of
        // allowing for override implementations. A generated client may
        // implement ValueParserFactory for a type to create a custom parser.
        quote! {
            clap::value_parser!(#arg_type_name)
        }
    };

    let required = match volitionality {
        Volitionality::Optional => quote! { .required(false) },
        Volitionality::Required => quote! { .required(true) },
        Volitionality::RequiredIfNoBody => {
            quote! { .required_unless_present("json-body") }
        }
    };

    quote! {
        clap::Arg::new(#arg_name)
            .long(#arg_name)
            .value_parser(#value_parser)
            #required
            #help
    }
}
