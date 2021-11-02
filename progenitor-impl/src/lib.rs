// Copyright 2021 Oxide Computer Company

use std::cmp::Ordering;

use convert_case::{Case, Casing};
use indexmap::IndexMap;
use openapiv3::{
    Components, OpenAPI, Parameter, ReferenceOr, RequestBody, Response, Schema,
};
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
    inner_type: Option<TokenStream>,
    pre_hook: Option<TokenStream>,
    post_hook: Option<TokenStream>,
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

        let methods = spec
            .paths
            .iter()
            .flat_map(|(path, ref_or_item)| {
                let item = ref_or_item.as_item().unwrap();
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
                )
            })
            .collect::<Result<Vec<_>>>()?;

        let mut types = self
            .type_space
            .iter_types()
            .map(|t| (t.name(), t.definition()))
            .collect::<Vec<_>>();
        types.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));
        let types = types.into_iter().map(|(_, def)| def);

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

                #(#methods)*
            }
        };

        Ok(file)
    }

    fn process_operation(
        &mut self,
        operation: &openapiv3::Operation,
        components: &Option<Components>,
        path: &str,
        method: &str,
    ) -> Result<TokenStream> {
        enum ParamType {
            Path,
            Query,
            Body,
        }

        let mut query: Vec<(String, bool)> = Vec::new();
        let mut raw_params = operation
            .parameters
            .iter()
            .map(|parameter| {
                match parameter.item(components)? {
                    openapiv3::Parameter::Path {
                        parameter_data,
                        style: openapiv3::PathStyle::Simple,
                    } => {
                        // Path parameters MUST be required.
                        assert!(parameter_data.required);

                        let nam = parameter_data.name.clone();
                        let schema = parameter_data.schema()?.to_schema();
                        let name = format!(
                            "{}{}",
                            sanitize(
                                operation.operation_id.as_ref().unwrap(),
                                Case::Pascal
                            ),
                            sanitize(&nam, Case::Pascal),
                        );
                        let typ = self
                            .type_space
                            .add_type_with_name(&schema, Some(name))?
                            .parameter_ident();

                        Ok((ParamType::Path, nam, typ))
                    }
                    openapiv3::Parameter::Query {
                        parameter_data,
                        allow_reserved: _,
                        style: openapiv3::QueryStyle::Form,
                        allow_empty_value,
                    } => {
                        if let Some(true) = allow_empty_value {
                            todo!("allow empty value is a no go");
                        }

                        let nam = parameter_data.name.clone();
                        let mut schema = parameter_data.schema()?.to_schema();
                        let name = format!(
                            "{}{}",
                            sanitize(
                                operation.operation_id.as_ref().unwrap(),
                                Case::Pascal
                            ),
                            sanitize(&nam, Case::Pascal),
                        );

                        if !parameter_data.required {
                            schema = make_optional(schema);
                        }

                        let typ = self
                            .type_space
                            .add_type_with_name(&schema, Some(name))?
                            .parameter_ident();

                        query.push((nam.to_string(), !parameter_data.required));
                        Ok((ParamType::Query, nam, typ))
                    }
                    x => todo!("unhandled parameter type: {:#?}", x),
                }
            })
            .collect::<Result<Vec<_>>>()?;
        let mut bounds = Vec::new();
        let (body_param, body_func) = if let Some(b) = &operation.request_body {
            let b = b.item(components)?;
            if b.is_binary(components)? {
                bounds.push(quote! {B: Into<reqwest::Body>});
                (Some(quote! {B}), Some(quote! { .body(body) }))
            } else {
                let mt = b.content_json()?;
                if !mt.encoding.is_empty() {
                    todo!("media type encoding not empty: {:#?}", mt);
                }

                if let Some(s) = &mt.schema {
                    let schema = s.to_schema();
                    let name = format!(
                        "{}Body",
                        sanitize(
                            operation.operation_id.as_ref().unwrap(),
                            Case::Pascal
                        )
                    );
                    let typ = self
                        .type_space
                        .add_type_with_name(&schema, Some(name))?
                        .parameter_ident();
                    (Some(typ), Some(quote! { .json(body) }))
                } else {
                    todo!("media type encoding, no schema: {:#?}", mt);
                }
            }
        } else {
            (None, None)
        };
        if let Some(body) = body_param {
            raw_params.push((ParamType::Body, "body".to_string(), body));
        }
        let tmp = template::parse(path)?;
        let names = tmp.names();
        let url_path = tmp.compile();
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

        let (response_type, decode_response) =
        // TODO let's consider how we handle multiple responses
            if operation.responses.responses.len() >= 1 {
                let only =
                    operation.responses.responses.first().unwrap();
                if !matches!(
                    only.0,
                    openapiv3::StatusCode::Code(200..=299)
                ) {
                    todo!("code? {:#?}", only);
                }

                let i = only.1.item(components)?;
                // TODO handle response headers.

                // Look at the response content.  For now, support a
                // single JSON-formatted response.
                match (
                    i.content.len(),
                    i.content.get("application/json"),
                ) {
                    (0, _) => (quote! { () }, quote! { res.json().await? }),
                    (1, Some(mt)) => {
                        if !mt.encoding.is_empty() {
                            todo!(
                                "media type encoding not empty: {:#?}",
                                mt
                            );
                        }

                        let typ = if let Some(schema) = &mt.schema {
                            let schema = schema.to_schema();
                            let name = format!(
                                "{}Response",
                                sanitize(
                                    operation
                                        .operation_id
                                        .as_ref()
                                        .unwrap(),
                                    Case::Pascal
                                )
                            );
                            self.type_space
                                .add_type_with_name(
                                    &schema,
                                    Some(name),
                                )?
                                .ident()
                        } else {
                            todo!(
                                "media type encoding, no schema: {:#?}",
                                mt
                            );
                        };
                        (typ, quote! { res.json().await? })
                    }
                    (1, None) => {
                        // Non-JSON response.
                        (quote! { reqwest::Response }, quote! { res })
                    }
                    (_, _) => {
                        todo!(
                            "too many response contents: {:#?}",
                            i.content
                        );
                    }
                }
            } else if operation.responses.responses.is_empty() {
                (quote! { reqwest::Response }, quote! { res })
            } else {
                todo!("responses? {:#?}", operation.responses);
            };
        let operation_id = format_ident!(
            "{}",
            sanitize(operation.operation_id.as_deref().unwrap(), Case::Snake)
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

        let pre_hook = self.pre_hook.as_ref().map(|hook| {
            quote! {
                (#hook)(&self.inner, &request);
            }
        });
        let post_hook = self.post_hook.as_ref().map(|hook| {
            quote! {
                (#hook)(&self.inner, &result);
            }
        });

        // TODO validate that method is one of the expected methods.
        let method_func = format_ident!("{}", method.to_lowercase());
        let method = quote! {
            #[doc = #doc_comment]
            pub async fn #operation_id #bounds (
                &self,
                #(#params),*
            ) -> Result<#response_type> {
                #url_path
                #query_build

                let request = self.client
                    . #method_func (url)
                    #body_func
                    #query_use
                    .build()?;
                #pre_hook
                let result = self.client
                    .execute(request)
                    .await;
                #post_hook

                // TODO we should do a match here for result?.status().as_u16()
                let res = result?.error_for_status()?;

                Ok(#decode_response)
            }
        };
        Ok(method)
    }

    pub fn generate_text(&mut self, spec: &OpenAPI) -> Result<String> {
        let output = self.generate_tokens(spec)?;

        // Format the file with rustfmt and some whitespace niceties.
        let content = rustfmt_wrapper::rustfmt(output).unwrap();

        Ok(if cfg!(not(windows)) {
            let regex = regex::Regex::new(r#"(})(\n\s*[^} ])"#).unwrap();
            regex.replace_all(&content, "$1\n$2").to_string()
        } else {
            let regex = regex::Regex::new(r#"(})(\r\n\s*[^} ])"#).unwrap();
            regex.replace_all(&content, "$1\r\n$2").to_string()
        })
    }

    pub fn dependencies(&self) -> Vec<String> {
        let mut deps = vec![
            "anyhow = \"1.0\"",
            "percent-encoding = \"2.1\"",
            "serde = { version = \"1.0\", features = [\"derive\"] }",
            "reqwest = { version = \"0.11\", features = [\"json\", \"stream\"] }",
        ];
        if self.type_space.uses_uuid() {
            deps.push(
                "uuid = { version = \"0.8\", features = [\"serde\", \"v4\"] }",
            )
        }
        if self.type_space.uses_chrono() {
            deps.push("chrono = { version = \"0.4\", features = [\"serde\"] }")
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

/// Make the schema optional if it isn't already.
pub fn make_optional(
    schema: schemars::schema::Schema,
) -> schemars::schema::Schema {
    match &schema {
        // If the instance_type already includes Null then this is already
        // optional.
        schemars::schema::Schema::Object(schemars::schema::SchemaObject {
            instance_type: Some(schemars::schema::SingleOrVec::Vec(types)),
            ..
        }) if types.contains(&schemars::schema::InstanceType::Null) => schema,

        // Otherwise, create a oneOf where one of the branches is the null
        // type. We could potentially check to see if the schema already
        // conforms to this pattern as well, but it doesn't hurt as typify will
        // already reduce nested Options to a single Option.
        _ => {
            let null_schema = schemars::schema::Schema::Object(
                schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::SingleOrVec::Single(
                        Box::new(schemars::schema::InstanceType::Null),
                    )),
                    ..Default::default()
                },
            );
            schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                subschemas: Some(Box::new(
                    schemars::schema::SubschemaValidation {
                        one_of: Some(vec![schema, null_schema]),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            })
        }
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
    fn is_binary(&self, components: &Option<Components>) -> Result<bool>;
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

    fn is_binary(&self, _components: &Option<Components>) -> Result<bool> {
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

                let s = s.item(&None)?;
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

    fn is_binary(&self, components: &Option<Components>) -> Result<bool> {
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

                let s = s.item(components)?;
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

trait ReferenceOrExt<T: ComponentLookup> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T>;
}
trait ComponentLookup: Sized {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>>;
}

impl<T: ComponentLookup> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T> {
        match self {
            ReferenceOr::Item(item) => Ok(item),
            ReferenceOr::Reference { reference } => {
                let idx = reference.rfind('/').unwrap();
                let key = &reference[idx + 1..];
                let parameters =
                    T::get_components(components.as_ref().unwrap());
                parameters.get(key).unwrap().item(components)
            }
        }
    }
}

impl ComponentLookup for Parameter {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.parameters
    }
}

impl ComponentLookup for RequestBody {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.request_bodies
    }
}

impl ComponentLookup for Response {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.responses
    }
}

impl ComponentLookup for Schema {
    fn get_components(
        components: &Components,
    ) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.schemas
    }
}

fn sanitize(input: &str, case: Case) -> String {
    input.replace('/', "-").to_case(case)
}
