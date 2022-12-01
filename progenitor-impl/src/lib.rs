// Copyright 2022 Oxide Computer Company

use std::collections::HashSet;

use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::quote;
use security::SecurityRequirements;
use serde::Deserialize;
use thiserror::Error;
use typify::{TypeSpace, TypeSpaceSettings};

use crate::{
    security::SecuritySchemeAuthenticator, to_schema::ToSchema,
    util::ReferenceOrExt,
};

mod method;
mod security;
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
    #[error("internal error {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

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
    extra_derives: Vec<String>,
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
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            type_space: TypeSpace::new(
                TypeSpaceSettings::default().with_type_mod("types"),
            ),
            settings: Default::default(),
            uses_futures: Default::default(),
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
        Self {
            type_space: TypeSpace::new(&type_settings),
            settings: settings.clone(),
            uses_futures: false,
        }
    }

    pub fn generate_tokens(&mut self, spec: &OpenAPI) -> Result<TokenStream> {
        validate_openapi(spec)?;

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

        // Construct the list of global security requirements
        let global_security_requirements: Option<SecurityRequirements> =
            spec.security.as_ref().map(|req| req.into());

        // Construct a list of security schemes. Given a valid spec file, elements in this list are
        // uniquely identified by their name
        let security_schemes = spec
            .components
            .iter()
            .flat_map(|components| {
                components.security_schemes.iter().filter_map(
                    |(name, ref_or_scheme)| {
                        ref_or_scheme.item(&spec.components).ok().map(
                            |scheme| {
                                SecuritySchemeAuthenticator::new(name, scheme)
                            },
                        )
                    },
                )
            })
            .collect::<Vec<SecuritySchemeAuthenticator>>();

        self.type_space.add_ref_types(schemas)?;

        let raw_methods = spec
            .paths
            .iter()
            .flat_map(|(path, ref_or_item)| {
                // Exclude externally defined path items.
                let item = ref_or_item.as_item().unwrap();
                // TODO punt on parameters that apply to all path items for now.
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
                    &global_security_requirements,
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

        let security_scheme_tokens = security_schemes
            .iter()
            .map(SecuritySchemeAuthenticator::generate_tokens)
            .collect::<Result<Vec<_>>>()?;

        // Only generate client methods for managing security scheme implementations if the spec
        // actually defines security schemes
        // TODO: Move + cleanup all of this
        let (
            security_schemes_impl,
            (client_security_property, client_security_value),
        ) = if !security_scheme_tokens.is_empty() {
            let client_security_tokens =
                SecuritySchemeAuthenticator::generate_client_impls();

            let security_trait =
                SecuritySchemeAuthenticator::generate_security_trait();

            (
                Some(quote! {
                    use std::{
                        any::TypeId,
                        collections::HashMap,
                        sync::Arc,
                    };

                    pub mod security {
                        use oauth2::{
                            AuthUrl,
                            AuthorizationCode,
                            basic::{
                                BasicClient,
                                BasicErrorResponseType,
                            },
                            ClientId,
                            ClientSecret,
                            CsrfToken,
                            PkceCodeChallenge,
                            PkceCodeVerifier,
                            RedirectUrl,
                            RequestTokenError,
                            reqwest::async_http_client,
                            ResourceOwnerUsername,
                            ResourceOwnerPassword,
                            Scope,
                            StandardErrorResponse,
                            TokenResponse,
                            TokenUrl,
                            url::Url,
                        };

                        use crate::Error;

                        struct AccessToken {
                            secret: String,
                            expires_at: std::time::Instant,
                        }

                        impl std::fmt::Debug for AccessToken {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                f.debug_struct("AccessToken")
                                    .field("secret", &"<redacted>".to_string())
                                    .field("expires_at", &self.expires_at)
                                    .finish()
                            }
                        }

                        #[derive(Debug)]
                        pub enum OAuthClientBuildError {
                            InvalidCsrfToken,
                            MissingCsrfToken,
                            MissingPkceVerifier,
                            Token(OAuthTokenError),
                            Url(oauth2::url::ParseError),
                        }

                        impl From<oauth2::url::ParseError> for OAuthClientBuildError {
                            fn from(err: oauth2::url::ParseError) -> Self {
                                OAuthClientBuildError::Url(err)
                            }
                        }

                        type TokenClientError = RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>;

                        #[derive(Debug)]
                        pub enum OAuthTokenError {
                            Client(TokenClientError),
                            ExpirationOutOfBounds,
                        }

                        impl From<OAuthTokenError> for OAuthClientBuildError {
                            fn from(err: OAuthTokenError) -> Self {
                                OAuthClientBuildError::Token(err)
                            }
                        }

                        impl From<TokenClientError> for OAuthTokenError {
                            fn from(err: TokenClientError) -> Self {
                                OAuthTokenError::Client(err)
                            }
                        }

                        impl From<TokenClientError> for OAuthClientBuildError {
                            fn from(err: TokenClientError) -> Self {
                                OAuthClientBuildError::Token(err.into())
                            }
                        }

                        #security_trait
                        #(#security_scheme_tokens)*
                    }

                    use security::ApplySecurity;

                    #client_security_tokens
                }),
                (
                    Some(quote! {
                        security_schemes: HashMap<TypeId, Arc<dyn ApplySecurity>>,
                    }),
                    Some(quote! {
                        security_schemes: HashMap::default(),
                    }),
                ),
            )
        } else {
            (None, (None, None))
        };

        let types = self.type_space.to_stream();

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

                // This may be used by some impl Deserialize, but not all.
                #[allow(unused_imports)]
                use std::convert::TryFrom;

                #types
            }

            #security_schemes_impl

            #[derive(Clone, Debug)]
            pub struct Client {
                pub(crate) baseurl: String,
                pub(crate) client: reqwest::Client,
                #client_security_property
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
                        #client_security_value
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
            "bytes = \"1.1\"",
            "futures-core = \"0.3\"",
            "percent-encoding = \"2.1\"",
            "reqwest = { version = \"0.11\", features = [\"json\", \"stream\"] }",
            "serde = { version = \"1.0\", features = [\"derive\"] }",
            "serde_urlencoded = \"0.7\"",
        ];
        if self.type_space.uses_regress() {
            deps.push("regress = \"0.4\"")
        }
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

    // TODO deprecate?
    pub fn get_type_space(&self) -> &TypeSpace {
        &self.type_space
    }
}

fn validate_openapi(spec: &OpenAPI) -> Result<()> {
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
