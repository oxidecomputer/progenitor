// Copyright 2022 Oxide Computer Company

use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::quote;
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
    #[error("invalid operation path")]
    InternalError(String),
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

        let methods = raw_methods
            .iter()
            .map(|method| self.positional_method(method))
            .collect::<Result<Vec<_>>>()?;

        let mut types = self
            .type_space
            .iter_types()
            .map(|t| (t.name(), t.definition()))
            .collect::<Vec<_>>();
        types.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));
        let types = types.into_iter().map(|(_, def)| def);
        let shared = self.type_space.common_code();

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
            // Re-export ResponseValue and Error since those are used by the
            // public interface of Client.
            pub use progenitor_client::{ByteStream, Error, ResponseValue};

            pub mod types {
                use serde::{Deserialize, Serialize};
                #shared
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
            "bytes = \"1.1.0\"",
            "futures-core = \"0.3.21\"",
            "percent-encoding = \"2.1\"",
            "serde = { version = \"1.0\", features = [\"derive\"] }",
            "reqwest = { version = \"0.11\", features = [\"json\", \"stream\"] }",
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
