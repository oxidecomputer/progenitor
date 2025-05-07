// Copyright 2025 Oxide Computer Company

//! Core implementation for the progenitor OpenAPI client generator.

#![deny(missing_docs)]

use std::collections::{BTreeMap, HashMap, HashSet};

use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use thiserror::Error;
use typify::{TypeSpace, TypeSpaceSettings};

use crate::to_schema::ToSchema;

pub use typify::CrateVers;
pub use typify::TypeSpaceImpl as TypeImpl;
pub use typify::TypeSpacePatch as TypePatch;
pub use typify::UnknownPolicy;

mod cli;
mod httpmock;
mod method;
mod template;
mod to_schema;
mod util;

#[allow(missing_docs)]
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

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, Error>;

/// OpenAPI generator.
pub struct Generator {
    type_space: TypeSpace,
    settings: GenerationSettings,
    uses_futures: bool,
    uses_websockets: bool,
}

/// Settings for [Generator].
#[derive(Default, Clone)]
pub struct GenerationSettings {
    interface: InterfaceStyle,
    tag: TagStyle,
    inner_type: Option<TokenStream>,
    pre_hook: Option<TokenStream>,
    pre_hook_async: Option<TokenStream>,
    post_hook: Option<TokenStream>,
    post_hook_async: Option<TokenStream>,
    extra_derives: Vec<String>,

    map_type: Option<String>,
    unknown_crates: UnknownPolicy,
    crates: BTreeMap<String, CrateSpec>,

    patch: HashMap<String, TypePatch>,
    replace: HashMap<String, (String, Vec<TypeImpl>)>,
    convert: Vec<(schemars::schema::SchemaObject, String, Vec<TypeImpl>)>,
}

#[derive(Debug, Clone)]
struct CrateSpec {
    version: CrateVers,
    rename: Option<String>,
}

/// Style of generated client.
#[derive(Clone, Deserialize, PartialEq, Eq)]
pub enum InterfaceStyle {
    /// Use positional style.
    Positional,
    /// Use builder style.
    Builder,
}

impl Default for InterfaceStyle {
    fn default() -> Self {
        Self::Positional
    }
}

/// Style for using the OpenAPI tags when generating names in the client.
#[derive(Clone, Deserialize)]
pub enum TagStyle {
    /// Merge tags to create names in the generated client.
    Merged,
    /// Use each tag name to create separate names in the generated client.
    Separate,
}

impl Default for TagStyle {
    fn default() -> Self {
        Self::Merged
    }
}

impl GenerationSettings {
    /// Create new generator settings with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the [InterfaceStyle].
    pub fn with_interface(&mut self, interface: InterfaceStyle) -> &mut Self {
        self.interface = interface;
        self
    }

    /// Set the [TagStyle].
    pub fn with_tag(&mut self, tag: TagStyle) -> &mut Self {
        self.tag = tag;
        self
    }

    /// Client inner type available to pre and post hooks.
    pub fn with_inner_type(&mut self, inner_type: TokenStream) -> &mut Self {
        self.inner_type = Some(inner_type);
        self
    }

    /// Hook invoked before issuing the HTTP request.
    pub fn with_pre_hook(&mut self, pre_hook: TokenStream) -> &mut Self {
        self.pre_hook = Some(pre_hook);
        self
    }

    /// Hook invoked before issuing the HTTP request.
    pub fn with_pre_hook_async(&mut self, pre_hook: TokenStream) -> &mut Self {
        self.pre_hook_async = Some(pre_hook);
        self
    }

    /// Hook invoked prior to receiving the HTTP response.
    pub fn with_post_hook(&mut self, post_hook: TokenStream) -> &mut Self {
        self.post_hook = Some(post_hook);
        self
    }

    /// Hook invoked prior to receiving the HTTP response.
    pub fn with_post_hook_async(&mut self, post_hook: TokenStream) -> &mut Self {
        self.post_hook_async = Some(post_hook);
        self
    }

    /// Additional derive macros applied to generated types.
    pub fn with_derive(&mut self, derive: impl ToString) -> &mut Self {
        self.extra_derives.push(derive.to_string());
        self
    }

    /// Modify a type with the given name.
    /// See [typify::TypeSpaceSettings::with_patch].
    pub fn with_patch<S: AsRef<str>>(&mut self, type_name: S, patch: &TypePatch) -> &mut Self {
        self.patch
            .insert(type_name.as_ref().to_string(), patch.clone());
        self
    }

    /// Replace a referenced type with a named type.
    /// See [typify::TypeSpaceSettings::with_replacement].
    pub fn with_replacement<TS: ToString, RS: ToString, I: Iterator<Item = TypeImpl>>(
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

    /// Replace a given schema with a named type.
    /// See [typify::TypeSpaceSettings::with_conversion].
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

    /// Policy regarding crates referenced by the schema extension
    /// `x-rust-type` not explicitly specified via [Self::with_crate].
    /// See [typify::TypeSpaceSettings::with_unknown_crates].
    pub fn with_unknown_crates(&mut self, policy: UnknownPolicy) -> &mut Self {
        self.unknown_crates = policy;
        self
    }

    /// Explicitly named crates whose types may be used during generation
    /// rather than generating new types based on their schemas (base on the
    /// presence of the x-rust-type extension).
    /// See [typify::TypeSpaceSettings::with_crate].
    pub fn with_crate<S1: ToString>(
        &mut self,
        crate_name: S1,
        version: CrateVers,
        rename: Option<&String>,
    ) -> &mut Self {
        self.crates.insert(
            crate_name.to_string(),
            CrateSpec {
                version,
                rename: rename.cloned(),
            },
        );
        self
    }

    /// Set the type used for key-value maps. Common examples:
    /// - [`std::collections::HashMap`] - **Default**
    /// - [`std::collections::BTreeMap`]
    /// - [`indexmap::IndexMap`]
    ///
    /// The requiremnets for a map type can be found in the
    /// [typify::TypeSpaceSettings::with_map_type] documentation.
    pub fn with_map_type<MT: ToString>(&mut self, map_type: MT) -> &mut Self {
        self.map_type = Some(map_type.to_string());
        self
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            type_space: TypeSpace::new(TypeSpaceSettings::default().with_type_mod("types")),
            settings: Default::default(),
            uses_futures: Default::default(),
            uses_websockets: Default::default(),
        }
    }
}

impl Generator {
    /// Create a new generator with default values.
    pub fn new(settings: &GenerationSettings) -> Self {
        let mut type_settings = TypeSpaceSettings::default();
        type_settings
            .with_type_mod("types")
            .with_struct_builder(settings.interface == InterfaceStyle::Builder);
        settings.extra_derives.iter().for_each(|derive| {
            let _ = type_settings.with_derive(derive.clone());
        });

        // Control use of crates found in x-rust-type extension
        type_settings.with_unknown_crates(settings.unknown_crates);
        settings
            .crates
            .iter()
            .for_each(|(crate_name, CrateSpec { version, rename })| {
                type_settings.with_crate(crate_name, version.clone(), rename.as_ref());
            });

        // Adjust generation by type, name, or schema.
        settings.patch.iter().for_each(|(type_name, patch)| {
            type_settings.with_patch(type_name, patch);
        });
        settings
            .replace
            .iter()
            .for_each(|(type_name, (replace_name, impls))| {
                type_settings.with_replacement(type_name, replace_name, impls.iter().cloned());
            });
        settings
            .convert
            .iter()
            .for_each(|(schema, type_name, impls)| {
                type_settings.with_conversion(schema.clone(), type_name, impls.iter().cloned());
            });

        // Set the map type if specified.
        if let Some(map_type) = &settings.map_type {
            type_settings.with_map_type(map_type.clone());
        }

        Self {
            type_space: TypeSpace::new(&type_settings),
            settings: settings.clone(),
            uses_futures: false,
            uses_websockets: false,
        }
    }

    /// Emit a [TokenStream] containing the generated client code.
    pub fn generate_tokens(&mut self, spec: &OpenAPI) -> Result<TokenStream> {
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

        let operation_code = match (&self.settings.interface, &self.settings.tag) {
            (InterfaceStyle::Positional, TagStyle::Merged) => self
                .generate_tokens_positional_merged(
                    &raw_methods,
                    self.settings.inner_type.is_some(),
                ),
            (InterfaceStyle::Positional, TagStyle::Separate) => {
                unimplemented!("positional arguments with separate tags are currently unsupported")
            }
            (InterfaceStyle::Builder, TagStyle::Merged) => self
                .generate_tokens_builder_merged(&raw_methods, self.settings.inner_type.is_some()),
            (InterfaceStyle::Builder, TagStyle::Separate) => {
                let tag_info = spec
                    .tags
                    .iter()
                    .map(|tag| (&tag.name, tag))
                    .collect::<BTreeMap<_, _>>();
                self.generate_tokens_builder_separate(
                    &raw_methods,
                    tag_info,
                    self.settings.inner_type.is_some(),
                )
            }
        }?;

        let types = self.type_space.to_stream();

        let (inner_type, inner_fn_value) = match self.settings.inner_type.as_ref() {
            Some(inner_type) => (inner_type.clone(), quote! { &self.inner }),
            None => (quote! { () }, quote! { &() }),
        };

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
            let mut s = format!("Client for {}", spec.info.title);

            if let Some(ss) = &spec.info.description {
                s.push_str("\n\n");
                s.push_str(ss);
            }
            if let Some(ss) = &spec.info.terms_of_service {
                s.push_str("\n\n");
                s.push_str(ss);
            }

            s.push_str(&format!("\n\nVersion: {}", &spec.info.version));

            s
        };

        let version_str = &spec.info.version;

        // The allow(unused_imports) on the `pub use` is necessary with Rust
        // 1.76+, in case the generated file is not at the top level of the
        // crate.

        let file = quote! {
            // Re-export types that are used by the public interface of Client.
            #[allow(unused_imports)]
            pub use progenitor_client::{
                ByteStream,
                ClientInfo,
                Error,
                ResponseValue,
            };
            #[allow(unused_imports)]
            use progenitor_client::{
                encode_path,
                ClientHooks,
                OperationInfo,
                RequestBuilderExt,
            };

            /// Types used as operation parameters and responses.
            #[allow(clippy::all)]
            pub mod types {
                #types
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
                    #[cfg(not(target_arch = "wasm32"))]
                    let client = {
                        let dur = std::time::Duration::from_secs(15);

                        reqwest::ClientBuilder::new()
                            .connect_timeout(dur)
                            .timeout(dur)
                    };
                    #[cfg(target_arch = "wasm32")]
                    let client = reqwest::ClientBuilder::new();

                    Self::new_with_client(baseurl, client.build().unwrap(), #inner_value)
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
            }

            impl ClientInfo<#inner_type> for Client {
                fn api_version() -> &'static str {
                    #version_str
                }

                fn baseurl(&self) -> &str {
                    self.baseurl.as_str()
                }

                fn client(&self) -> &reqwest::Client {
                    &self.client
                }

                fn inner(&self) -> &#inner_type {
                    #inner_fn_value
                }
            }

            impl ClientHooks<#inner_type> for &Client {}

            #operation_code
        };

        Ok(file)
    }

    fn generate_tokens_positional_merged(
        &mut self,
        input_methods: &[method::OperationMethod],
        has_inner: bool,
    ) -> Result<TokenStream> {
        let methods = input_methods
            .iter()
            .map(|method| self.positional_method(method, has_inner))
            .collect::<Result<Vec<_>>>()?;

        // The allow(unused_imports) on the `pub use` is necessary with Rust
        // 1.76+, in case the generated file is not at the top level of the
        // crate.

        let out = quote! {
            #[allow(clippy::all)]
            #[allow(elided_named_lifetimes)]
            impl Client {
                #(#methods)*
            }

            /// Items consumers will typically use such as the Client.
            pub mod prelude {
                #[allow(unused_imports)]
                pub use super::Client;
            }
        };
        Ok(out)
    }

    fn generate_tokens_builder_merged(
        &mut self,
        input_methods: &[method::OperationMethod],
        has_inner: bool,
    ) -> Result<TokenStream> {
        let builder_struct = input_methods
            .iter()
            .map(|method| self.builder_struct(method, TagStyle::Merged, has_inner))
            .collect::<Result<Vec<_>>>()?;

        let builder_methods = input_methods
            .iter()
            .map(|method| self.builder_impl(method))
            .collect::<Vec<_>>();

        let out = quote! {
            impl Client {
                #(#builder_methods)*
            }

            /// Types for composing operation parameters.
            #[allow(clippy::all)]
            pub mod builder {
                use super::types;
                #[allow(unused_imports)]
                use super::{
                    encode_path,
                    ByteStream,
                    ClientInfo,
                    ClientHooks,
                    Error,
                    OperationInfo,
                    RequestBuilderExt,
                    ResponseValue,
                };

                #(#builder_struct)*
            }

            /// Items consumers will typically use such as the Client.
            pub mod prelude {
                pub use self::super::Client;
            }
        };

        Ok(out)
    }

    fn generate_tokens_builder_separate(
        &mut self,
        input_methods: &[method::OperationMethod],
        tag_info: BTreeMap<&String, &openapiv3::Tag>,
        has_inner: bool,
    ) -> Result<TokenStream> {
        let builder_struct = input_methods
            .iter()
            .map(|method| self.builder_struct(method, TagStyle::Separate, has_inner))
            .collect::<Result<Vec<_>>>()?;

        let (traits_and_impls, trait_preludes) = self.builder_tags(input_methods, &tag_info);

        // The allow(unused_imports) on the `pub use` is necessary with Rust
        // 1.76+, in case the generated file is not at the top level of the
        // crate.

        let out = quote! {
            #traits_and_impls

            /// Types for composing operation parameters.
            #[allow(clippy::all)]
            pub mod builder {
                use super::types;
                #[allow(unused_imports)]
                use super::{
                    encode_path,
                    ByteStream,
                    ClientInfo,
                    ClientHooks,
                    Error,
                    OperationInfo,
                    RequestBuilderExt,
                    ResponseValue,
                };

                #(#builder_struct)*
            }

            /// Items consumers will typically use such as the Client and
            /// extension traits.
            pub mod prelude {
                #[allow(unused_imports)]
                pub use super::Client;
                #trait_preludes
            }
        };

        Ok(out)
    }

    /// Get the [TypeSpace] for schemas present in the OpenAPI specification.
    pub fn get_type_space(&self) -> &TypeSpace {
        &self.type_space
    }

    /// Whether the generated client needs to use additional crates to support
    /// futures.
    pub fn uses_futures(&self) -> bool {
        self.uses_futures
    }

    /// Whether the generated client needs to use additional crates to support
    /// websockets.
    pub fn uses_websockets(&self) -> bool {
        self.uses_websockets
    }
}

/// Add newlines after end-braces at <= two levels of indentation.
pub fn space_out_items(content: String) -> Result<String> {
    Ok(if cfg!(not(windows)) {
        let regex = regex::Regex::new(r#"(\n\s*})(\n\s{0,8}[^} ])"#).unwrap();
        regex.replace_all(&content, "$1\n$2").to_string()
    } else {
        let regex = regex::Regex::new(r#"(\n\s*})(\r\n\s{0,8}[^} ])"#).unwrap();
        regex.replace_all(&content, "$1\r\n$2").to_string()
    })
}

fn validate_openapi_spec_version(spec_version: &str) -> Result<()> {
    // progenitor currenlty only support OAS 3.0.x
    if spec_version.trim().starts_with("3.0.") {
        Ok(())
    } else {
        Err(Error::UnexpectedFormat(format!(
            "invalid version: {}",
            spec_version
        )))
    }
}

/// Do some very basic checks of the OpenAPI documents.
pub fn validate_openapi(spec: &OpenAPI) -> Result<()> {
    validate_openapi_spec_version(spec.openapi.as_str())?;

    let mut opids = HashSet::new();
    spec.paths.paths.iter().try_for_each(|p| {
        match p.1 {
            openapiv3::ReferenceOr::Reference { reference: _ } => Err(Error::UnexpectedFormat(
                format!("path {} uses reference, unsupported", p.0,),
            )),
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

    use crate::{validate_openapi_spec_version, Error};

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

    #[test]
    fn test_validate_openapi_spec_version() {
        assert!(validate_openapi_spec_version("3.0.0").is_ok());
        assert!(validate_openapi_spec_version("3.0.1").is_ok());
        assert!(validate_openapi_spec_version("3.0.4").is_ok());
        assert!(validate_openapi_spec_version("3.0.5-draft").is_ok());
        assert_eq!(
            validate_openapi_spec_version("3.1.0")
                .unwrap_err()
                .to_string(),
            "unexpected or unhandled format in the OpenAPI document invalid version: 3.1.0"
        );
    }
}
