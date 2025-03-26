// Copyright 2024 Oxide Computer Company

use std::collections::BTreeMap;

use heck::ToKebabCase;
use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use typify::{Type, TypeEnumVariant, TypeSpaceImpl, TypeStructPropInfo};

use crate::{
    method::{OperationParameterKind, OperationParameterType, OperationResponseStatus},
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
    pub fn cli(&mut self, spec: &OpenAPI, crate_name: &str) -> Result<TokenStream> {
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
            .map(|method| self.cli_method(method))
            .collect::<Vec<_>>();

        let cli_ops = methods.iter().map(|op| &op.cli_fn);
        let execute_ops = methods.iter().map(|op| &op.execute_fn);
        let trait_ops = methods.iter().map(|op| &op.execute_trait);

        let cli_fns = raw_methods
            .iter()
            .map(|method| format_ident!("cli_{}", sanitize(&method.operation_id, Case::Snake)))
            .collect::<Vec<_>>();
        let execute_fns = raw_methods
            .iter()
            .map(|method| format_ident!("execute_{}", sanitize(&method.operation_id, Case::Snake)))
            .collect::<Vec<_>>();

        let cli_variants = raw_methods
            .iter()
            .map(|method| format_ident!("{}", sanitize(&method.operation_id, Case::Pascal)))
            .collect::<Vec<_>>();

        let crate_path = syn::TypePath {
            qself: None,
            path: syn::parse_str(crate_name).unwrap(),
        };

        let code = quote! {
            use #crate_path::*;

            pub struct Cli<T: CliConfig> {
                client: Client,
                config: T,
            }
            impl<T: CliConfig> Cli<T> {
                pub fn new(
                    client: Client,
                    config: T,
                ) -> Self {
                    Self { client, config }
                }

                pub fn get_command(cmd: CliCommand) -> ::clap::Command {
                    match cmd {
                        #(
                            CliCommand::#cli_variants => Self::#cli_fns(),
                        )*
                    }
                }

                #(#cli_ops)*

                pub async fn execute(
                    &self,
                    cmd: CliCommand,
                    matches: &::clap::ArgMatches,
                ) -> anyhow::Result<()> {
                    match cmd {
                        #(
                            CliCommand::#cli_variants => {
                                // TODO ... do something with output
                                self.#execute_fns(matches).await
                            }
                        )*
                    }
                }

                #(#execute_ops)*
            }

            pub trait CliConfig {
                fn success_item<T>(&self, value: &ResponseValue<T>)
                where
                    T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
                fn success_no_item(&self, value: &ResponseValue<()>);
                fn error<T>(&self, value: &Error<T>)
                where
                    T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;

                fn list_start<T>(&self)
                where
                    T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
                fn list_item<T>(&self, value: &T)
                where
                    T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
                fn list_end_success<T>(&self)
                where
                    T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
                fn list_end_error<T>(&self, value: &Error<T>)
                where
                    T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;

                #(#trait_ops)*
            }

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

    fn cli_method(&mut self, method: &crate::method::OperationMethod) -> CliOperation {
        let CliArg {
            parser: parser_args,
            consumer: consumer_args,
        } = self.cli_method_args(method);

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

        let fn_name = format_ident!("cli_{}", &method.operation_id);

        let cli_fn = quote! {
            pub fn #fn_name() -> ::clap::Command
            {
                ::clap::Command::new("")
                #parser_args
                #about
                #long_about
            }
        };

        let fn_name = format_ident!("execute_{}", &method.operation_id);
        let op_name = format_ident!("{}", &method.operation_id);

        let (_, success_kind) =
            self.extract_responses(method, OperationResponseStatus::is_success_or_default);
        let (_, error_kind) =
            self.extract_responses(method, OperationResponseStatus::is_error_or_default);

        let execute_and_output = match method.dropshot_paginated {
            // Normal, one-shot API calls.
            None => {
                let success_output = match success_kind {
                    crate::method::OperationResponseKind::Type(_) => {
                        quote! {
                            {
                                self.config.success_item(&r);
                                Ok(())
                            }
                        }
                    }
                    crate::method::OperationResponseKind::None => {
                        quote! {
                            {
                                self.config.success_no_item(&r);
                                Ok(())
                            }
                        }
                    }
                    crate::method::OperationResponseKind::Raw
                    | crate::method::OperationResponseKind::Upgrade => {
                        quote! {
                            {
                                todo!()
                            }
                        }
                    }
                };

                let error_output = match error_kind {
                    crate::method::OperationResponseKind::Type(_)
                    | crate::method::OperationResponseKind::None => {
                        quote! {
                            {
                                self.config.error(&r);
                                Err(anyhow::Error::new(r))
                            }
                        }
                    }
                    crate::method::OperationResponseKind::Raw
                    | crate::method::OperationResponseKind::Upgrade => {
                        quote! {
                            {
                                todo!()
                            }
                        }
                    }
                };

                quote! {
                    let result = request.send().await;

                    match result {
                        Ok(r) => #success_output
                        Err(r) => #error_output
                    }
                }
            }

            // Paginated APIs for which we iterate over each item.
            Some(_) => {
                let success_type = match success_kind {
                    crate::method::OperationResponseKind::Type(type_id) => {
                        self.type_space.get_type(&type_id).unwrap().ident()
                    }
                    crate::method::OperationResponseKind::None => quote! { () },
                    crate::method::OperationResponseKind::Raw => todo!(),
                    crate::method::OperationResponseKind::Upgrade => todo!(),
                };
                let error_output = match error_kind {
                    crate::method::OperationResponseKind::Type(_)
                    | crate::method::OperationResponseKind::None => {
                        quote! {
                            {
                                self.config.list_end_error(&r);
                                return Err(anyhow::Error::new(r))
                            }
                        }
                    }
                    crate::method::OperationResponseKind::Raw
                    | crate::method::OperationResponseKind::Upgrade => {
                        quote! {
                            {
                                todo!()
                            }
                        }
                    }
                };
                quote! {
                    self.config.list_start::<#success_type>();

                    // We're using "limit" as both the maximum page size and
                    // as the full limit. It's not ideal in that we could
                    // reduce the limit with each iteration and we might get a
                    // bunch of results we don't display... but it's fine.
                    let mut stream = futures::StreamExt::take(
                        request.stream(),
                        matches
                            .get_one::<std::num::NonZeroU32>("limit")
                            .map_or(usize::MAX, |x| x.get() as usize));

                    loop {
                        match futures::TryStreamExt::try_next(&mut stream).await {
                            Err(r) => #error_output
                            Ok(None) => {
                                self.config.list_end_success::<#success_type>();
                                return Ok(());
                            }
                            Ok(Some(value)) => {
                                self.config.list_item(&value);
                            }
                        }
                    }
                }
            }
        };

        let execute_fn = quote! {
            pub async fn #fn_name(&self, matches: &::clap::ArgMatches)
                -> anyhow::Result<()>
            {
                let mut request = self.client.#op_name();
                #consumer_args

                // Call the override function.
                self.config.#fn_name(matches, &mut request)?;

                #execute_and_output
            }
        };

        // TODO this is copy-pasted--unwisely?
        let struct_name = sanitize(&method.operation_id, Case::Pascal);
        let struct_ident = format_ident!("{}", struct_name);

        let execute_trait = quote! {
            fn #fn_name(
                &self,
                matches: &::clap::ArgMatches,
                request: &mut builder :: #struct_ident,
            ) -> anyhow::Result<()> {
                Ok(())
            }
        };

        CliOperation {
            cli_fn,
            execute_fn,
            execute_trait,
        }
    }

    fn cli_method_args(&self, method: &crate::method::OperationMethod) -> CliArg {
        let mut args = CliOperationArgs::default();

        let first_page_required_set = method
            .dropshot_paginated
            .as_ref()
            .map(|d| &d.first_page_params);

        for param in &method.params {
            let innately_required = match &param.kind {
                // We're not interetested in the body parameter yet.
                OperationParameterKind::Body(_) => continue,

                OperationParameterKind::Path => true,
                OperationParameterKind::Query(required) => *required,
                OperationParameterKind::Header(required) => *required,
            };

            // For paginated endpoints, we don't generate 'page_token' args.
            if method.dropshot_paginated.is_some() && param.name.as_str() == "page_token" {
                continue;
            }

            let first_page_required = first_page_required_set
                .map_or(false, |required| required.contains(&param.api_name));

            let volitionality = if innately_required || first_page_required {
                Volitionality::Required
            } else {
                Volitionality::Optional
            };

            let OperationParameterType::Type(arg_type_id) = &param.typ else {
                unreachable!("query and path parameters must be typed")
            };
            let arg_type = self.type_space.get_type(arg_type_id).unwrap();

            let arg_name = param.name.to_kebab_case();

            // There should be no conflicting path or query parameters.
            assert!(!args.has_arg(&arg_name));

            let parser = clap_arg(&arg_name, volitionality, &param.description, &arg_type);

            let arg_fn_name = sanitize(&param.name, Case::Snake);
            let arg_fn = format_ident!("{}", arg_fn_name);
            let OperationParameterType::Type(arg_type_id) = &param.typ else {
                panic!()
            };
            let arg_type = self.type_space.get_type(arg_type_id).unwrap();
            let arg_type_name = arg_type.ident();

            let consumer = quote! {
                if let Some(value) =
                    matches.get_one::<#arg_type_name>(#arg_name)
                {
                    // clone here in case the arg type doesn't impl
                    // From<&T>
                    request = request.#arg_fn(value.clone());
                }
            };

            args.add_arg(arg_name, CliArg { parser, consumer })
        }

        let maybe_body_type_id = method
            .params
            .iter()
            .find(|param| matches!(&param.kind, OperationParameterKind::Body(_)))
            .and_then(|param| match &param.typ {
                // TODO not sure how to deal with raw bodies, but we definitely
                // need **some** input so we shouldn't just ignore it... as we
                // are currently...
                OperationParameterType::RawBody => None,

                OperationParameterType::Type(body_type_id) => Some(body_type_id),
            });

        if let Some(body_type_id) = maybe_body_type_id {
            args.body_present();
            let body_type = self.type_space.get_type(body_type_id).unwrap();
            let details = body_type.details();

            match details {
                typify::TypeDetails::Struct(struct_info) => {
                    for prop_info in struct_info.properties_info() {
                        self.cli_method_body_arg(&mut args, prop_info)
                    }
                }

                _ => {
                    // If the body is not a struct, we don't know what's
                    // required or how to generate it
                    args.body_required()
                }
            }
        }

        let parser_args = args.args.values().map(|CliArg { parser, .. }| parser);

        // TODO do this as args we add in.
        let body_json_args = (match args.body {
            CliBodyArg::None => None,
            CliBodyArg::Required => Some(true),
            CliBodyArg::Optional => Some(false),
        })
        .map(|required| {
            let help = "Path to a file that contains the full json body.";

            quote! {
                .arg(
                    ::clap::Arg::new("json-body")
                        .long("json-body")
                        .value_name("JSON-FILE")
                        // Required if we can't turn the body into individual
                        // parameters.
                        .required(#required)
                        .value_parser(::clap::value_parser!(std::path::PathBuf))
                        .help(#help)
                )
                .arg(
                    ::clap::Arg::new("json-body-template")
                        .long("json-body-template")
                        .action(::clap::ArgAction::SetTrue)
                        .help("XXX")
                )
            }
        });

        let parser = quote! {
            #(
                .arg(#parser_args)
            )*
            #body_json_args
        };

        let consumer_args = args.args.values().map(|CliArg { consumer, .. }| consumer);

        let body_json_consumer = maybe_body_type_id.map(|body_type_id| {
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

        let consumer = quote! {
            #(
                #consumer_args
            )*
            #body_json_consumer
        };

        CliArg { parser, consumer }
    }

    fn cli_method_body_arg(&self, args: &mut CliOperationArgs, prop_info: TypeStructPropInfo<'_>) {
        let TypeStructPropInfo {
            name,
            description,
            required,
            type_id,
        } = prop_info;

        let prop_type = self.type_space.get_type(&type_id).unwrap();

        // TODO this is maybe a kludge--not completely sure of the right way to
        // handle option types. On one hand, we could want types from this
        // interface to never show us Option<T> types--we could let the
        // `required` field give us that information. On the other hand, there
        // might be Option types that are required ... at least in the JSON
        // sense, meaning that we need to include `"foo": null` rather than
        // omitting the field. Back to the first hand: is that last point just
        // a serde issue rather than an interface one?
        let maybe_inner_type =
            if let typify::TypeDetails::Option(inner_type_id) = prop_type.details() {
                let inner_type = self.type_space.get_type(&inner_type_id).unwrap();
                Some(inner_type)
            } else {
                None
            };

        let prop_type = if let Some(inner_type) = maybe_inner_type {
            inner_type
        } else {
            prop_type
        };

        let scalar = prop_type.has_impl(TypeSpaceImpl::FromStr);

        let prop_name = name.to_kebab_case();
        if scalar && !args.has_arg(&prop_name) {
            let volitionality = if required {
                Volitionality::RequiredIfNoBody
            } else {
                Volitionality::Optional
            };
            let parser = clap_arg(
                &prop_name,
                volitionality,
                &description.map(str::to_string),
                &prop_type,
            );

            let prop_fn = format_ident!("{}", sanitize(name, Case::Snake));
            let prop_type_ident = prop_type.ident();
            let consumer = quote! {
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
            };
            args.add_arg(prop_name, CliArg { parser, consumer })
        } else if required {
            args.body_required()
        }

        // Cases
        // 1. If the type can be represented as a string, great
        //
        // 2. If it's a substruct then we can try to glue the names together
        // and hope?
        //
        // 3. enums
        // 3.1 simple enums (should be covered by 1 above)
        //   e.g. enum { A, B }
        //   args for --a and --b that are in a group
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
    let maybe_enum_parser = if let typify::TypeDetails::Enum(e) = arg_type.details() {
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
                ::clap::builder::TypedValueParser::map(
                    ::clap::builder::PossibleValuesParser::new([
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
            ::clap::value_parser!(#arg_type_name)
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
        ::clap::Arg::new(#arg_name)
            .long(#arg_name)
            .value_parser(#value_parser)
            #required
            #help
    }
}

#[derive(Debug)]
struct CliArg {
    /// Code to parse the argument
    parser: TokenStream,

    /// Code to consume the argument
    consumer: TokenStream,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum CliBodyArg {
    #[default]
    None,
    Required,
    Optional,
}

#[derive(Default, Debug)]
struct CliOperationArgs {
    args: BTreeMap<String, CliArg>,
    body: CliBodyArg,
}

impl CliOperationArgs {
    fn has_arg(&self, name: &String) -> bool {
        self.args.contains_key(name)
    }
    fn add_arg(&mut self, name: String, arg: CliArg) {
        self.args.insert(name, arg);
    }

    fn body_present(&mut self) {
        assert_eq!(self.body, CliBodyArg::None);
        self.body = CliBodyArg::Optional;
    }

    fn body_required(&mut self) {
        assert!(self.body == CliBodyArg::Optional || self.body == CliBodyArg::Required);
        self.body = CliBodyArg::Required;
    }
}
