// Copyright 2021 Oxide Computer Company

use std::cmp::Ordering;

use openapiv3::{OpenAPI, ReferenceOr};
use proc_macro2::TokenStream;

use quote::{format_ident, quote};
use thiserror::Error;
use typify::TypeSpace;

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
}

impl Generator {
    pub fn new() -> Self {
        Self::default()
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

        enum ParamType {
            Path,
            Query,
            Body,
        }

        let methods = spec
            .operations()
            .map(|(path, method, operation)| {
                let mut query: Vec<(String, bool)> = Vec::new();
                let mut raw_params = operation
                    .parameters
                    .iter()
                    .map(|parameter| {
                        match parameter.item()? {
                            openapiv3::Parameter::Path {
                                parameter_data,
                                style: openapiv3::PathStyle::Simple,
                            } => {
                                // Path parameters MUST be required.
                                assert!(parameter_data.required);

                                let nam = parameter_data.name.clone();
                                let schema =
                                    parameter_data.schema()?.to_schema();
                                let typ = self
                                    .type_space
                                    .add_type_details(&schema)?
                                    .parameter;

                                Ok((ParamType::Path, nam, typ))
                            }
                            openapiv3::Parameter::Query {
                                parameter_data,
                                allow_reserved: _,
                                style: openapiv3::QueryStyle::Form,
                                allow_empty_value,
                            } => {
                                if let Some(aev) = allow_empty_value {
                                    if *aev {
                                        todo!("allow empty value is a no go");
                                    }
                                }

                                let nam = parameter_data.name.clone();
                                let schema =
                                    parameter_data.schema()?.to_schema();
                                let mut typ = self
                                    .type_space
                                    .add_type_details(&schema)?
                                    .parameter;
                                if !parameter_data.required {
                                    typ = quote! { Option<#typ> };
                                }
                                query.push((
                                    nam.to_string(),
                                    !parameter_data.required,
                                ));
                                Ok((ParamType::Query, nam, typ))
                            }
                            x => todo!("unhandled parameter type: {:#?}", x),
                        }
                    })
                    .collect::<Result<Vec<_>>>()?;

                let mut bounds = Vec::new();

                let (body_param, body_func) = if let Some(b) =
                    &operation.request_body
                {
                    let b = b.item()?;
                    if b.is_binary()? {
                        bounds.push(quote! {B: Into<reqwest::Body>});
                        (Some(quote! {B}), Some(quote! { .body(body) }))
                    } else {
                        let mt = b.content_json()?;
                        if !mt.encoding.is_empty() {
                            todo!("media type encoding not empty: {:#?}", mt);
                        }

                        if let Some(s) = &mt.schema {
                            let schema = s.to_schema();
                            let typ = self
                                .type_space
                                .add_type_details(&schema)?
                                .parameter;
                            (Some(typ), Some(quote! { .json(body) }))
                        } else {
                            todo!("media type encoding, no schema: {:#?}", mt);
                        }
                    }
                } else {
                    (None, None)
                };

                if let Some(body) = body_param {
                    raw_params.push((
                        ParamType::Body,
                        "body".to_string(),
                        body,
                    ));
                }

                let tmp = template::parse(path)?;
                let names = tmp.names();
                let url_path = tmp.compile();

                // Put parameters in a deterministic order.
                raw_params.sort_by(|a, b| match (&a.0, &b.0) {
                    // Path params are first and are in positional order.
                    (ParamType::Path, ParamType::Path) => {
                        let aa = names.iter().position(|x| x == &a.1).unwrap();
                        let bb = names.iter().position(|x| x == &b.1).unwrap();
                        aa.cmp(&bb)
                    }
                    (ParamType::Path, ParamType::Query) => Ordering::Less,
                    (ParamType::Path, ParamType::Body) => Ordering::Less,

                    // Query params are in lexicographic order.
                    (ParamType::Query, ParamType::Body) => Ordering::Less,
                    (ParamType::Query, ParamType::Query) => a.1.cmp(&b.1),
                    (ParamType::Query, ParamType::Path) => Ordering::Greater,

                    // Body params are last and should be unique
                    (ParamType::Body, ParamType::Path) => Ordering::Greater,
                    (ParamType::Body, ParamType::Query) => Ordering::Greater,
                    (ParamType::Body, ParamType::Body) => {
                        panic!("should only be one body")
                    }
                });

                let (response_type, decode_response) = if operation
                    .responses
                    .responses
                    .len()
                    == 1
                {
                    let only = operation.responses.responses.first().unwrap();
                    if !matches!(only.0, openapiv3::StatusCode::Code(200..=299))
                    {
                        todo!("code? {:#?}", only);
                    }

                    let i = only.1.item()?;
                    if !i.headers.is_empty() {
                        todo!("no response headers for now");
                    }

                    if !i.links.is_empty() {
                        todo!("no response links for now");
                    }

                    // Look at the response content.  For now, support a
                    // single JSON-formatted response.
                    let typ = match (
                        i.content.len(),
                        i.content.get("application/json"),
                    ) {
                        (0, _) => quote! { () },
                        (1, Some(mt)) => {
                            if !mt.encoding.is_empty() {
                                todo!(
                                    "media type encoding not empty: {:#?}",
                                    mt
                                );
                            }

                            if let Some(schema) = &mt.schema {
                                let schema = schema.to_schema();
                                self.type_space.add_type_details(&schema)?.ident
                            } else {
                                todo!(
                                    "media type encoding, no schema: {:#?}",
                                    mt
                                );
                            }
                        }
                        (1, None) => {
                            todo!(
                                "response content not JSON: {:#?}",
                                i.content
                            );
                        }
                        (_, _) => {
                            todo!(
                                "too many response contents: {:#?}",
                                i.content
                            );
                        }
                    };
                    (typ, quote! { res.json().await? })
                } else if operation.responses.responses.is_empty() {
                    (quote! { reqwest::Response }, quote! { res })
                } else {
                    todo!("responses? {:#?}", operation.responses);
                };

                let operation_id = format_ident!(
                    "{}",
                    operation.operation_id.as_deref().unwrap()
                );

                let bounds = if bounds.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        < #(#bounds),* >
                    }
                };

                let params = raw_params.into_iter().map(|(_, name, typ)| {
                    let name = format_ident!("{}", name);
                    quote! {
                        #name: #typ
                    }
                });

                let (query_build, query_use) = if query.is_empty() {
                    (quote! {}, quote! {})
                } else {
                    let query_items = query.iter().map(|(qn, opt)| {
                        if *opt {
                            let qn_ident = format_ident!("{}", qn);
                            quote! {
                                if let Some(v) = & #qn_ident {
                                    query.push((#qn, v.to_string()));
                                }
                            }
                        } else {
                            quote! {
                                query.push((#qn, #qn.to_string()));
                            }
                        }
                    });

                    let query_build = quote! {
                        let mut query = Vec::new();
                        #(#query_items)*
                    };
                    let query_use = quote! {
                        .query(&query)
                    };

                    (query_build, query_use)
                };

                let doc_comment = format!(
                    "{}: {} {}",
                    operation.operation_id.as_deref().unwrap(),
                    method.to_ascii_uppercase(),
                    path
                );

                let method_func = format_ident!("{}", method);

                let method = quote! {
                    #[doc = #doc_comment]
                    pub async fn #operation_id #bounds (
                        &self,
                        #(#params),*
                    ) -> Result<#response_type> {
                        #url_path
                        #query_build

                        let res = self.client
                            . #method_func (url)
                            #body_func
                            #query_use
                            .send()
                            .await?
                            .error_for_status()?;

                        Ok(#decode_response)
                    }
                };

                Ok(method)
            })
            .collect::<Result<Vec<_>>>()?;

        let mut types = self
            .type_space
            .iter_types()
            .map(|type_entry| {
                (
                    type_entry.type_name(&self.type_space),
                    type_entry.output(&self.type_space),
                )
            })
            .collect::<Vec<_>>();
        types.sort_by(|a, b| a.0.cmp(&b.0));
        let types = types.into_iter().map(|(_, def)| def);

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
            }

            impl Client {
                pub fn new(baseurl: &str) -> Client {
                    let dur = std::time::Duration::from_secs(15);
                    let client = reqwest::ClientBuilder::new()
                        .connect_timeout(dur)
                        .timeout(dur)
                        .build()
                        .unwrap();

                    Client::new_with_client(baseurl, client)
                }

                pub fn new_with_client(
                    baseurl: &str,
                    client: reqwest::Client,
                ) -> Client {
                    Client {
                        baseurl: baseurl.to_string(),
                        client,
                    }
                }

                #(#methods)*
            }
        };

        Ok(file)
    }

    pub fn generate_text(&mut self, spec: &OpenAPI) -> Result<String> {
        let output = self.generate_tokens(spec)?;

        // Format the file with rustfmt and some whitespace niceties.
        let content = rustfmt_wrapper::rustfmt(output).unwrap();
        let regex = regex::Regex::new(r#"(})(\n\s*[^} ])"#).unwrap();
        let content = regex.replace_all(&content, "$1\n$2").to_string();

        Ok(content)
    }

    pub fn dependencies(&self) -> Vec<String> {
        let mut deps = vec![
            "anyhow = \"1.0.44\"",
            "percent-encoding = \"2.1.0\"",
            "serde = { version = \"1.0.130\", features = [\"derive\"] }",
            "reqwest = { version = \"0.11.5\", features = [\"json\", \"stream\"] }",
        ];
        if self.type_space.uses_uuid() {
            deps.push(
                "uuid = { version = \"0.8.2\", features = [\"serde\", \"v4\"] }",
            )
        }
        if self.type_space.uses_chrono() {
            deps.push(
                "chrono = { version = \"0.4.19\", features = [\"serde\"] }",
            )
        }
        deps.sort_unstable();
        deps.iter().map(ToString::to_string).collect()
    }

    pub fn get_type_space(&self) -> &TypeSpace {
        &self.type_space
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
    fn is_binary(&self) -> Result<bool>;
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

    fn is_binary(&self) -> Result<bool> {
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

                let s = s.item()?;
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

    fn is_binary(&self) -> Result<bool> {
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

                let s = s.item()?;
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

trait ReferenceOrExt<T> {
    fn item(&self) -> Result<&T>;
}

impl<T> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
    fn item(&self) -> Result<&T> {
        match self {
            ReferenceOr::Reference { .. } => {
                Err(Error::BadConversion("unexpected reference".to_string()))
            }
            ReferenceOr::Item(item) => Ok(item),
        }
    }
}
