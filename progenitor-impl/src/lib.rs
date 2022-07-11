// Copyright 2022 Oxide Computer Company

use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use thiserror::Error;
use typify::TypeSpace;

use crate::to_schema::ToSchema;

mod method;
mod template;
mod to_schema;
mod util;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected value type")]
    BadValue(String, serde_json::Value),
    #[error("type error")]
    TypeError(#[from] typify::Error),
    #[error("unexpected or unhandled format in the OpenAPI document")]
    UnexpectedFormat(String),
    #[error("invalid operation path")]
    InvalidPath(String),
    #[error("internal error")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Generator {
    type_space: TypeSpace,
    settings: GenerationSettings,
    uses_futures: bool,
}

#[derive(Default, Clone)]
pub struct GenerationSettings {
    interface: InterfaceStyle,
    tag: TagStyle,
    inner_type: Option<TokenStream>,
    pre_hook: Option<TokenStream>,
    post_hook: Option<TokenStream>,
    extra_derives: Vec<TokenStream>,
}

#[derive(Clone, Deserialize)]
pub enum InterfaceStyle {
    Positional,
    Builder,
}

impl Default for InterfaceStyle {
    fn default() -> Self {
        Self::Positional
    }
}

#[derive(Clone, Deserialize)]
pub enum TagStyle {
    Merged,
    Separate,
}

impl Default for TagStyle {
    fn default() -> Self {
        Self::Merged
    }
}

impl GenerationSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_interface(&mut self, interface: InterfaceStyle) -> &mut Self {
        self.interface = interface;
        self
    }

    pub fn with_tag(&mut self, tag: TagStyle) -> &mut Self {
        self.tag = tag;
        self
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

    // TODO maybe change to a typify::Settings or something
    pub fn with_derive(&mut self, derive: TokenStream) -> &mut Self {
        self.extra_derives.push(derive);
        self
    }
}

impl Generator {
    pub fn new(settings: &GenerationSettings) -> Self {
        Self {
            type_space: TypeSpace::default(),
            settings: settings.clone(),
            uses_futures: false,
        }
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
        self.settings
            .extra_derives
            .iter()
            .for_each(|derive| self.type_space.add_derive(derive.clone()));

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
            .map(|(path, method, operation, inherited_parameters)| {
                let new_params = operation
                    .parameters
                    .iter()
                    .chain(inherited_parameters.iter())
                    .cloned()
                    .collect();
                let op = openapiv3::Operation {
                    parameters: new_params,
                    ..operation.clone()
                };
                self.process_operation(&op, &spec.components, path, method)
            })
            .collect::<Result<Vec<_>>>()?;

        let operation_code = match (
            &self.settings.interface,
            &self.settings.tag,
        ) {
            (InterfaceStyle::Positional, TagStyle::Merged) => {
                self.generate_tokens_positional_merged(&raw_methods)
            }
            (InterfaceStyle::Positional, TagStyle::Separate) => {
                unimplemented!("positional arguments with separate tags are currently unsupported")
            }
            (InterfaceStyle::Builder, TagStyle::Merged) => {
                self.generate_tokens_builder_merged(&raw_methods)
            }
            (InterfaceStyle::Builder, TagStyle::Separate) => {
                self.generate_tokens_builder_separate(&raw_methods)
            }
        }?;

        let mut types = self
            .type_space
            .iter_types()
            .map(|t| (t.name(), t.definition()))
            .collect::<Vec<_>>();
        types.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));
        let types = types.into_iter().map(|(_, def)| def);
        let shared = self.type_space.common_code();

        let inner_property = self.settings.inner_type.as_ref().map(|inner| {
            quote! {
                pub (crate) inner: #inner,
            }
        });
        let inner_parameter = self.settings.inner_type.as_ref().map(|inner| {
            quote! {
                inner: #inner,
            }
        });
        let inner_value = self.settings.inner_type.as_ref().map(|_| {
            quote! {
                inner
            }
        });

        let file = quote! {
            // Re-export ResponseValue and Error since those are used by the
            // public interface of Client.
            pub use progenitor_client::{ByteStream, Error, ResponseValue};
            #[allow(unused_imports)]
            use progenitor_client::{encode_path, RequestBuilderExt};

            pub mod types {
                use serde::{Deserialize, Serialize};
                #shared
                #(#types)*
            }

            #[derive(Clone)]
            pub struct Client {
                pub(crate) baseurl: String,
                pub(crate) client: reqwest::Client,
                #inner_property
            }

            impl Client {
                pub fn new(
                    baseurl: &str,
                    #inner_parameter
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
                    #inner_parameter
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
            }

            #operation_code
        };

        Ok(file)
    }

    fn generate_tokens_positional_merged(
        &mut self,
        input_methods: &[method::OperationMethod],
    ) -> Result<TokenStream> {
        let methods = input_methods
            .iter()
            .map(|method| self.positional_method(method))
            .collect::<Result<Vec<_>>>()?;
        let out = quote! {
            impl Client {
                #(#methods)*
            }
        };
        Ok(out)
    }

    fn generate_tokens_builder_merged(
        &mut self,
        input_methods: &[method::OperationMethod],
    ) -> Result<TokenStream> {
        let builder_struct = input_methods
            .iter()
            .map(|method| self.builder_struct(method, TagStyle::Merged))
            .collect::<Result<Vec<_>>>()?;

        let builder_methods = input_methods
            .iter()
            .map(|method| self.builder_impl(method))
            .collect::<Vec<_>>();

        let out = quote! {
            impl Client {
                #(#builder_methods)*
            }

            pub mod builder {
                use super::types;
                #[allow(unused_imports)]
                use super::{
                    encode_path,
                    ByteStream,
                    Error,
                    RequestBuilderExt,
                    ResponseValue,
                };

                #(#builder_struct)*
            }
        };

        Ok(out)
    }

    fn generate_tokens_builder_separate(
        &mut self,
        input_methods: &[method::OperationMethod],
    ) -> Result<TokenStream> {
        let builder_struct = input_methods
            .iter()
            .map(|method| self.builder_struct(method, TagStyle::Separate))
            .collect::<Result<Vec<_>>>()?;

        let traits_and_impls = self.builder_tags(input_methods);

        let out = quote! {
            #traits_and_impls

            pub mod builder {
                use super::types;
                #[allow(unused_imports)]
                use super::{
                    encode_path,
                    ByteStream,
                    Error,
                    RequestBuilderExt,
                    ResponseValue,
                };

                #(#builder_struct)*
            }
        };

        Ok(out)
    }

    /// Render text output.
    pub fn generate_text(&mut self, spec: &OpenAPI) -> Result<String> {
        self.generate_text_impl(
            spec,
            rustfmt_wrapper::config::Config::default(),
        )
    }

    /// Render text output and normalize doc comments
    ///
    /// Requires a nightly install of `rustfmt` (even if the target project is
    /// not using nightly).
    pub fn generate_text_normalize_comments(
        &mut self,
        spec: &OpenAPI,
    ) -> Result<String> {
        self.generate_text_impl(
            spec,
            rustfmt_wrapper::config::Config {
                normalize_doc_attributes: Some(true),
                wrap_comments: Some(true),
                ..Default::default()
            },
        )
    }

    fn generate_text_impl(
        &mut self,
        spec: &OpenAPI,
        config: rustfmt_wrapper::config::Config,
    ) -> Result<String> {
        let output = self.generate_tokens(spec)?;

        // Format the file with rustfmt.
        let content = rustfmt_wrapper::rustfmt_config(config, output).unwrap();

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
            "bytes = \"1.1.0\"",
            "futures-core = \"0.3.21\"",
            "percent-encoding = \"2.1\"",
            "reqwest = { version = \"0.11\", features = [\"json\", \"stream\"] }",
            "serde = { version = \"1.0\", features = [\"derive\"] }",
            "serde_urlencoded = 0.7",
        ];
        if self.type_space.uses_uuid() {
            deps.push(
                "uuid = { version = \">=0.8.0, <2.0.0\", features = [\"serde\", \"v4\"] }",
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
