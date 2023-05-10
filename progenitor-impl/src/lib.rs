// Copyright 2023 Oxide Computer Company

use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use openapiv3::{OpenAPI, SecurityRequirement, SecurityScheme};
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use template::PathTemplate;
use thiserror::Error;
use typify::{TypeDetails, TypeId};
use typify::{TypeSpace, TypeSpaceSettings};
use util::ReferenceOrExt;

use crate::to_schema::ToSchema;

pub use typify::TypeSpaceImpl as TypeImpl;
pub use typify::TypeSpacePatch as TypePatch;

mod cli;
mod method;
mod template;
mod to_schema;
mod util;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unexpected value type {0}: {1}")]
    BadValue(String, serde_json::Value),
    #[error("type error {0}")]
    TypeError(#[from] typify::Error),
    #[error("unexpected or unhandled format in the OpenAPI document {0}")]
    UnexpectedFormat(String),
    #[error("invalid operation path {0}")]
    InvalidPath(String),
    #[error("invalid dropshot extension use: {0}")]
    InvalidExtension(String),
    #[error("internal error {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Generator {
    type_space: TypeSpace,
    forms: HashSet<TypeId>,
    settings: GenerationSettings,
    uses_futures: bool,
    uses_websockets: bool,
}

#[derive(Default, Clone)]
pub struct GenerationSettings {
    interface: InterfaceStyle,
    tag: TagStyle,
    inner_type: Option<TokenStream>,
    pre_hook: Option<TokenStream>,
    post_hook: Option<TokenStream>,
    extra_derives: Vec<String>,

    patch: HashMap<String, TypePatch>,
    replace: HashMap<String, (String, Vec<TypeImpl>)>,
    convert: Vec<(schemars::schema::SchemaObject, String, Vec<TypeImpl>)>,
}

#[derive(Clone, Deserialize, PartialEq, Eq)]
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

    pub fn with_derive(&mut self, derive: impl ToString) -> &mut Self {
        self.extra_derives.push(derive.to_string());
        self
    }

    pub fn with_patch<S: AsRef<str>>(
        &mut self,
        type_name: S,
        patch: &TypePatch,
    ) -> &mut Self {
        self.patch
            .insert(type_name.as_ref().to_string(), patch.clone());
        self
    }

    pub fn with_replacement<
        TS: ToString,
        RS: ToString,
        I: Iterator<Item = TypeImpl>,
    >(
        &mut self,
        type_name: TS,
        replace_name: RS,
        impls: I,
    ) -> &mut Self {
        self.replace.insert(
            type_name.to_string(),
            (replace_name.to_string(), impls.collect()),
        );
        self
    }

    pub fn with_conversion<S: ToString, I: Iterator<Item = TypeImpl>>(
        &mut self,
        schema: schemars::schema::SchemaObject,
        type_name: S,
        impls: I,
    ) -> &mut Self {
        self.convert
            .push((schema, type_name.to_string(), impls.collect()));
        self
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            type_space: TypeSpace::new(
                TypeSpaceSettings::default().with_type_mod("types"),
            ),
            forms: Default::default(),
            settings: Default::default(),
            uses_futures: Default::default(),
            uses_websockets: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Security {
    pub(crate) per_path: IndexMap<String, SecurityRequirement>, 
    /// The global available security requirements, the defaults, named, which must be ref'd by overrides
    pub(crate) global: Vec<SecurityRequirement>,
    /// Declares the scheme and which header to use, each one referenced above must exist in the below
    pub(crate) schemes: IndexMap<String, SecurityScheme>,
}

// usuful to derive the global fallback, if any
impl Security {
    pub(crate) fn resolve_for_path(&self, path: &PathTemplate) -> Option<SecurityScheme> {
        let path = path.to_string();
        let requirements = self.per_path.get(&path).cloned().or_else(|| { self.global.first().cloned() })?;
        let mut schemes = 
            requirements
                .iter()
                .map(|(name,requirements)| {
                    self.schemes
                        .get(name)
                        .expect("Contains that name, otherwise spec is buggy. qed")
                });
        // TODO let's start with exactly one or zero schemes
        assert!(schemes.len() <= 1);
        schemes.next().cloned()
    }

    pub(crate) fn from(spec: &OpenAPI) -> Self {
        Self {
            per_path: Default::default(), // TODO
            global: spec.security.clone().unwrap_or_default(),
            schemes: spec.components.as_ref().map(|c| {
                IndexMap::from_iter(
                    c.security_schemes
                        .iter()
                        .map(|(key, reference_or_sec_scheme)| {
                            (
                                key.to_owned(),
                                ReferenceOrExt::item(reference_or_sec_scheme, &spec.components).cloned().expect("Spec was checked for validity, so this must work. qed")
                            )
                        })
                )
            }).unwrap_or_default(),
        }
    }
}

impl Generator {
    pub fn new(settings: &GenerationSettings) -> Self {
        let mut type_settings = TypeSpaceSettings::default();
        type_settings
            .with_type_mod("types")
            .with_struct_builder(settings.interface == InterfaceStyle::Builder);
        settings.extra_derives.iter().for_each(|derive| {
            let _ = type_settings.with_derive(derive.clone());
        });
        settings.patch.iter().for_each(|(type_name, patch)| {
            type_settings.with_patch(type_name, patch);
        });
        settings.replace.iter().for_each(
            |(type_name, (replace_name, impls))| {
                type_settings.with_replacement(
                    type_name,
                    replace_name,
                    impls.iter().cloned(),
                );
            },
        );
        settings
            .convert
            .iter()
            .for_each(|(schema, type_name, impls)| {
                type_settings.with_conversion(
                    schema.clone(),
                    type_name,
                    impls.iter().cloned(),
                );
            });
        Self {
            type_space: TypeSpace::new(&type_settings),
            settings: settings.clone(),
            forms: Default::default(),
            uses_futures: false,
            uses_websockets: false,
        }
    }

    /// Generate the actual rust implementation from the specification
    pub fn generate_tokens(&mut self, spec: &OpenAPI) -> Result<TokenStream> {
        validate_openapi(spec)?;

        let security = Security::from(&spec);

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
                    &security,
                    path_parameters,
                )
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

        let types = self.type_space.to_stream();

        let extra_impl = TokenStream::from_iter(
            self.forms
                .iter()
                .map(|type_id| {
                    let typ = self.get_type_space().get_type(type_id).unwrap();
                    let form_name = typ.name();
                    let td = typ.details();
                    let TypeDetails::Struct(tstru) = td else { unreachable!() };
                    let properties = indexmap::IndexMap::<&'_ str, _>::from_iter(
                        tstru
                            .properties()
                            .filter_map(|(prop_name, prop_id)| {
                                self.get_type_space()
                                    .get_type(&prop_id).ok()
                                    .map(|prop_typ| (prop_name, prop_typ))
                            })
                        );
                    let properties = syn::punctuated::Punctuated::<_, syn::Token![,]>::from_iter(
                        properties
                            .into_iter()
                            .map(|(prop_name, prop_ty)| {
                                let ident = quote::format_ident!("{}", prop_name);
                                quote!{ (#prop_name, &self. #ident) }
                            }));

                    let form_name = quote::format_ident!("{}",typ.name());

                    quote! {
                        impl #form_name {
                            pub fn as_form<'f>(&'f self) -> impl std::iter::Iterator<Item=(&'static str, reqwest::multipart::Part)> + 'f {
                                [#properties]
                                    .into_iter()
                                    .filter_map(|(name, val)|{
                                        val.as_ref().map(|val| (name, val))
                                    })
                                    .map(|(name, val)| {
                                        let part = reqwest::multipart::Part::stream(val.to_vec())
                                            .file_name("sortme.pdf".to_owned()) // required for sevdesk, for "validation"
                                            .mime_str("application/pdf").unwrap();
                                        (name, part)
                                    })
                            }
                        }
                    }
                }));
        // Generate an implementation of a `Self::as_inner` method, if an inner
        // type is defined.
        let maybe_inner = self.settings.inner_type.as_ref().map(|inner| {
            quote! {
                /// Return a reference to the inner type stored in `self`.
                pub fn inner(&self) -> &#inner {
                    &self.inner
                }
            }
        });

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

        let client_docstring = {
            let mut doc = format!("Client for {}", spec.info.title);

            if let Some(desc) = &spec.info.description {
                doc.push_str("\n\n");
                doc.push_str(desc);
            }
            if let Some(tos) = &spec.info.terms_of_service {
                doc.push_str("\n\n");
                doc.push_str(tos);
            }

            doc.push_str(&format!("\n\nVersion: {}", &spec.info.version));

            doc
        };

        let version_str = &spec.info.version;

        let file = quote! {
            // Re-export ResponseValue and Error since those are used by the
            // public interface of Client.
            pub use progenitor_client::{ByteStream, Error, ResponseValue};
            #[allow(unused_imports)]
            use progenitor_client::{encode_path, RequestBuilderExt};
            #[allow(unused_imports)]
            use reqwest::header::{HeaderMap, HeaderValue};

            pub mod types {
                use serde::{Deserialize, Serialize};

                // This may be used by some impl Deserialize, but not all.
                #[allow(unused_imports)]
                use std::convert::TryFrom;

                #types

                #extra_impl
            }

            #[derive(Clone, Debug)]
            #[doc = #client_docstring]
            pub struct Client {
                pub(crate) baseurl: String,
                pub(crate) client: reqwest::Client,
                #inner_property
            }

            impl Client {
                /// Create a new client.
                ///
                /// `baseurl` is the base URL provided to the internal
                /// `reqwest::Client`, and should include a scheme and hostname,
                /// as well as port and a path stem if applicable.
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

                /// Construct a new client with an existing `reqwest::Client`,
                /// allowing more control over its configuration.
                ///
                /// `baseurl` is the base URL provided to the internal
                /// `reqwest::Client`, and should include a scheme and hostname,
                /// as well as port and a path stem if applicable.
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

                /// Get the base URL to which requests are made.
                pub fn baseurl(&self) -> &String {
                    &self.baseurl
                }

                /// Get the internal `reqwest::Client` used to make requests.
                pub fn client(&self) -> &reqwest::Client {
                    &self.client
                }

                /// Get the version of this API.
                ///
                /// This string is pulled directly from the source OpenAPI
                /// document and may be in any format the API selects.
                pub fn api_version(&self) -> &'static str {
                    #version_str
                }

                #maybe_inner
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
                #( #methods )*
            }

            pub mod prelude {
                pub use super::Client;
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
                    HeaderMap,
                    HeaderValue,
                    RequestBuilderExt,
                    ResponseValue,
                };

                #(#builder_struct)*
            }

            pub mod prelude {
                pub use self::super::Client;
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

        let (traits_and_impls, trait_preludes) =
            self.builder_tags(input_methods);

        let out = quote! {
            #traits_and_impls

            pub mod builder {
                use super::types;
                #[allow(unused_imports)]
                use super::{
                    encode_path,
                    ByteStream,
                    Error,
                    HeaderMap,
                    HeaderValue,
                    RequestBuilderExt,
                    ResponseValue,
                };

                #(#builder_struct)*

            }

            pub mod prelude {
                pub use super::Client;
                #trait_preludes
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

        space_out_items(content)
    }

    // TODO deprecate?
    pub fn get_type_space(&self) -> &TypeSpace {
        &self.type_space
    }

    pub fn uses_futures(&self) -> bool {
        self.uses_futures
    }

    pub fn uses_websockets(&self) -> bool {
        self.uses_websockets
    }
}

pub(crate) fn space_out_items(content: String) -> Result<String> {
    // Add newlines after end-braces at <= two levels of indentation.
    Ok(if cfg!(not(windows)) {
        let regex = regex::Regex::new(r#"(})(\n\s{0,8}[^} ])"#).unwrap();
        regex.replace_all(&content, "$1\n$2").to_string()
    } else {
        let regex = regex::Regex::new(r#"(})(\r\n\s{0,8}[^} ])"#).unwrap();
        regex.replace_all(&content, "$1\r\n$2").to_string()
    })
}

pub fn validate_openapi(spec: &OpenAPI) -> Result<()> {
    match spec.openapi.as_str() {
        "3.0.0" | "3.0.1" | "3.0.2" | "3.0.3" => (),
        v => {
            return Err(Error::UnexpectedFormat(format!(
                "invalid version: {}",
                v
            )))
        }
    }

    let mut opids = HashSet::new();
    spec.paths.paths.iter().try_for_each(|p| {
        match p.1 {
            openapiv3::ReferenceOr::Reference { reference: _ } => {
                Err(Error::UnexpectedFormat(format!(
                    "path {} uses reference, unsupported",
                    p.0,
                )))
            }
            openapiv3::ReferenceOr::Item(item) => {
                // Make sure every operation has an operation ID, and that each
                // operation ID is only used once in the document.
                item.iter().try_for_each(|(_, o)| {
                    if let Some(oid) = o.operation_id.as_ref() {
                        if !opids.insert(oid.to_string()) {
                            return Err(Error::UnexpectedFormat(format!(
                                "duplicate operation ID: {}",
                                oid,
                            )));
                        }
                    } else {
                        return Err(Error::UnexpectedFormat(format!(
                            "path {} is missing operation ID",
                            p.0,
                        )));
                    }
                    Ok(())
                })
            }
        }
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::Error;

    #[test]
    fn test_bad_value() {
        assert_eq!(
            Error::BadValue("nope".to_string(), json! { "nope"},).to_string(),
            "unexpected value type nope: \"nope\"",
        );
    }

    #[test]
    fn test_type_error() {
        assert_eq!(
            Error::UnexpectedFormat("nope".to_string()).to_string(),
            "unexpected or unhandled format in the OpenAPI document nope",
        );
    }

    #[test]
    fn test_invalid_path() {
        assert_eq!(
            Error::InvalidPath("nope".to_string()).to_string(),
            "invalid operation path nope",
        );
    }

    #[test]
    fn test_internal_error() {
        assert_eq!(
            Error::InternalError("nope".to_string()).to_string(),
            "internal error nope",
        );
    }
}
