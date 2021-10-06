#![allow(unused_imports)]
#![allow(clippy::single_match)]

use anyhow::{anyhow, bail, Context, Result};
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaData, SchemaKind};
use serde::Deserialize;
use std::cell::Ref;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use quote::{format_ident, quote};
use typify::{TypeEntryIdentifier, TypeSpace};

use crate::to_schema::ToSchema;

mod template;
mod to_schema;

fn save<P>(p: P, data: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let p = p.as_ref();
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(p)?;
    f.write_all(data.as_bytes())?;
    f.flush()?;
    Ok(())
}

fn load<P, T>(p: P) -> Result<T>
where
    P: AsRef<Path>,
    for<'de> T: Deserialize<'de>,
{
    let p = p.as_ref();
    let f = File::open(p)?;
    Ok(serde_json::from_reader(f)?)
}

fn load_api<P>(p: P) -> Result<OpenAPI>
where
    P: AsRef<Path>,
{
    let api: OpenAPI = load(p)?;

    if api.openapi != "3.0.3" {
        /*
         * XXX During development we are being very strict, but this should
         * probably be relaxed.
         */
        bail!("unexpected version {}", api.openapi);
    }

    if !api.servers.is_empty() {
        bail!("servers not presently supported");
    }

    if api.security.is_some() {
        bail!("security not presently supported");
    }

    if !api.tags.is_empty() {
        bail!("tags not presently supported");
    }

    if let Some(components) = api.components.as_ref() {
        if !components.security_schemes.is_empty() {
            bail!("component security schemes not supported");
        }

        if !components.responses.is_empty() {
            bail!("component responses not supported");
        }

        if !components.parameters.is_empty() {
            bail!("component parameters not supported");
        }

        if !components.request_bodies.is_empty() {
            bail!("component request bodies not supported");
        }

        if !components.headers.is_empty() {
            bail!("component headers not supported");
        }

        if !components.links.is_empty() {
            bail!("component links not supported");
        }

        if !components.callbacks.is_empty() {
            bail!("component callbacks not supported");
        }

        /*
         * XXX Ignoring "examples" and "extensions" for now.
         * Explicitly allowing "schemas" through.
         */
    }

    /*
     * XXX Ignoring "external_docs" and "extensions" for now, as they seem not
     * to immediately affect our code generation.
     */

    let mut opids = HashSet::new();
    for p in api.paths.paths.iter() {
        match p.1 {
            openapiv3::ReferenceOr::Reference { reference: _ } => {
                bail!("path {} uses reference, unsupported", p.0);
            }
            openapiv3::ReferenceOr::Item(item) => {
                /*
                 * Make sure every operation has an operation ID, and that each
                 * operation ID is only used once in the document.
                 */
                let mut id = |o: Option<&openapiv3::Operation>| -> Result<()> {
                    if let Some(o) = o {
                        if let Some(oid) = o.operation_id.as_ref() {
                            if !opids.insert(oid.to_string()) {
                                bail!("duplicate operation ID: {}", oid);
                            }

                            if !o.tags.is_empty() {
                                bail!("op {}: tags, unsupported", oid);
                            }

                            if !o.servers.is_empty() {
                                bail!("op {}: servers, unsupported", oid);
                            }

                            if o.security.is_some() {
                                bail!("op {}: security, unsupported", oid);
                            }

                            if o.responses.default.is_some() {
                                bail!("op {}: has response default", oid);
                            }
                        } else {
                            bail!("path {} is missing operation ID", p.0);
                        }
                    }

                    Ok(())
                };

                id(item.get.as_ref())?;
                id(item.put.as_ref())?;
                id(item.post.as_ref())?;
                id(item.delete.as_ref())?;
                id(item.options.as_ref())?;
                id(item.head.as_ref())?;
                id(item.patch.as_ref())?;
                id(item.trace.as_ref())?;

                if !item.servers.is_empty() {
                    bail!("path {} has servers; unsupported", p.0);
                }
            }
        }
    }

    Ok(api)
}

trait ParameterDataExt {
    fn render_type(&self) -> Result<String>;
    fn schema(&self) -> Result<&openapiv3::ReferenceOr<openapiv3::Schema>>;
}

impl ParameterDataExt for openapiv3::ParameterData {
    fn schema(&self) -> Result<&openapiv3::ReferenceOr<openapiv3::Schema>> {
        match &self.format {
            openapiv3::ParameterSchemaOrContent::Schema(s) => Ok(s),
            x => bail!("XXX param format {:#?}", x),
        }
    }

    fn render_type(&self) -> Result<String> {
        use openapiv3::{SchemaKind, Type};

        Ok(match &self.format {
            openapiv3::ParameterSchemaOrContent::Schema(s) => {
                let s = s.item().context("parameter data render type")?;
                match &s.schema_kind {
                    SchemaKind::Type(Type::String(st)) => {
                        if !st.format.is_empty() {
                            bail!("XXX format");
                        }
                        if st.pattern.is_some() {
                            bail!("XXX pattern");
                        }
                        if !st.enumeration.is_empty() {
                            bail!("XXX enumeration");
                        }
                        if st.min_length.is_some() || st.max_length.is_some() {
                            bail!("XXX min/max length");
                        }
                        "&str".to_string()
                    }
                    SchemaKind::Type(Type::Integer(it)) => {
                        let mut uint;
                        let width;

                        use openapiv3::VariantOrUnknownOrEmpty::Unknown;
                        if let Unknown(f) = &it.format {
                            match f.as_str() {
                                "uint" | "uint32" => {
                                    uint = true;
                                    width = 32;
                                }
                                "uint64" => {
                                    uint = true;
                                    width = 32;
                                }
                                f => bail!("XXX unknown integer format {}", f),
                            }
                        } else {
                            bail!("XXX format {:?}", it.format);
                        }

                        if it.multiple_of.is_some() {
                            bail!("XXX multiple_of");
                        }
                        if it.exclusive_minimum || it.exclusive_maximum {
                            bail!("XXX exclusive");
                        }

                        if let Some(min) = it.minimum {
                            if min == 0 {
                                uint = true;
                            } else {
                                bail!("XXX invalid minimum: {}", min);
                            }
                        }

                        if it.maximum.is_some() {
                            bail!("XXX maximum");
                        }
                        if !it.enumeration.is_empty() {
                            bail!("XXX enumeration");
                        }
                        if uint {
                            format!("u{}", width)
                        } else {
                            format!("i{}", width)
                        }
                    }
                    x => bail!("unexpected type {:#?}", x),
                }
            }
            x => bail!("XXX param format {:#?}", x),
        })
    }
}

trait ExtractJsonMediaType {
    fn is_binary(&self) -> Result<bool>;
    fn content_json(&self) -> Result<openapiv3::MediaType>;
}

impl ExtractJsonMediaType for openapiv3::Response {
    fn content_json(&self) -> Result<openapiv3::MediaType> {
        if self.content.len() != 1 {
            bail!("expected one content entry, found {}", self.content.len());
        }

        if let Some(mt) = self.content.get("application/json") {
            Ok(mt.clone())
        } else {
            bail!(
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
            bail!("expected one content entry, found {}", self.content.len());
        }

        if let Some(mt) = self.content.get("application/octet-stream") {
            if !mt.encoding.is_empty() {
                bail!("XXX encoding");
            }

            if let Some(s) = &mt.schema {
                use openapiv3::{
                    SchemaKind, StringFormat, Type,
                    VariantOrUnknownOrEmpty::Item,
                };

                let s = s.item()?;
                if s.schema_data.nullable {
                    bail!("XXX nullable binary?");
                }
                if s.schema_data.default.is_some() {
                    bail!("XXX default binary?");
                }
                if s.schema_data.discriminator.is_some() {
                    bail!("XXX binary discriminator?");
                }
                match &s.schema_kind {
                    SchemaKind::Type(Type::String(st)) => {
                        if st.min_length.is_some() || st.max_length.is_some() {
                            bail!("binary min/max length");
                        }
                        if !matches!(st.format, Item(StringFormat::Binary)) {
                            bail!(
                                "expected binary format string, got {:?}",
                                st.format
                            );
                        }
                        if st.pattern.is_some() {
                            bail!("XXX pattern");
                        }
                        if !st.enumeration.is_empty() {
                            bail!("XXX enumeration");
                        }
                        return Ok(true);
                    }
                    x => {
                        bail!("XXX schemakind type {:?}", x);
                    }
                }
            } else {
                bail!("binary thing had no schema?");
            }
        }

        Ok(false)
    }
}

impl ExtractJsonMediaType for openapiv3::RequestBody {
    fn content_json(&self) -> Result<openapiv3::MediaType> {
        if self.content.len() != 1 {
            bail!("expected one content entry, found {}", self.content.len());
        }

        if let Some(mt) = self.content.get("application/json") {
            Ok(mt.clone())
        } else {
            bail!(
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
            bail!("expected one content entry, found {}", self.content.len());
        }

        if let Some(mt) = self.content.get("application/octet-stream") {
            if !mt.encoding.is_empty() {
                bail!("XXX encoding");
            }

            if let Some(s) = &mt.schema {
                use openapiv3::{
                    SchemaKind, StringFormat, Type,
                    VariantOrUnknownOrEmpty::Item,
                };

                let s = s.item()?;
                if s.schema_data.nullable {
                    bail!("XXX nullable binary?");
                }
                if s.schema_data.default.is_some() {
                    bail!("XXX default binary?");
                }
                if s.schema_data.discriminator.is_some() {
                    bail!("XXX binary discriminator?");
                }
                match &s.schema_kind {
                    SchemaKind::Type(Type::String(st)) => {
                        if st.min_length.is_some() || st.max_length.is_some() {
                            bail!("binary min/max length");
                        }
                        if !matches!(st.format, Item(StringFormat::Binary)) {
                            bail!(
                                "expected binary format string, got {:?}",
                                st.format
                            );
                        }
                        if st.pattern.is_some() {
                            bail!("XXX pattern");
                        }
                        if !st.enumeration.is_empty() {
                            bail!("XXX enumeration");
                        }
                        return Ok(true);
                    }
                    x => {
                        bail!("XXX schemakind type {:?}", x);
                    }
                }
            } else {
                bail!("binary thing had no schema?");
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
            openapiv3::ReferenceOr::Item(i) => Ok(i),
            openapiv3::ReferenceOr::Reference { reference: _ } => {
                bail!("reference not supported here");
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum TypeDetails {
    Unknown,
    Basic,
    Array(TypeId),
    Optional(TypeId),
    /*
     * Object property names are sorted lexicographically to ensure a stable
     * order in the generated code.
     */
    Object(BTreeMap<String, TypeId>),
    NewType(TypeId),
    Enumeration(Vec<String>),
    /*
     * A map with string keys and values of a specific type:
     */
    Dictionary(TypeId),
}

#[derive(Debug)]
struct TypeEntry {
    id: TypeId,
    name: Option<String>,
    details: TypeDetails,
}

#[derive(Debug, Eq, Clone)]
struct TypeId(u64);

impl PartialOrd for TypeId {
    fn partial_cmp(&self, other: &TypeId) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypeId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialEq for TypeId {
    fn eq(&self, other: &TypeId) -> bool {
        self.0 == other.0
    }
}

enum ParamType {
    Path,
    Query,
    Body,
}

fn generate(api: &OpenAPI, ts: &mut TypeSpace) -> Result<String> {
    let methods = api
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
                            /*
                             * Path parameters MUST be required.
                             */
                            assert!(parameter_data.required);

                            let nam = parameter_data.name.clone();
                            let schema = parameter_data.schema()?.to_schema();
                            let typ = ts.add_type_details(&schema)?.parameter;

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
                                    bail!("allow empty value is a no go");
                                }
                            }

                            let nam = parameter_data.name.clone();
                            let schema = parameter_data.schema()?.to_schema();
                            let mut typ =
                                ts.add_type_details(&schema)?.parameter;
                            if !parameter_data.required {
                                typ = quote! { Option<#typ> };
                            }
                            query.push((
                                nam.to_string(),
                                !parameter_data.required,
                            ));
                            Ok((ParamType::Query, nam, typ))
                        }
                        x => bail!("unhandled parameter type: {:#?}", x),
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            let mut bounds = Vec::new();

            let (body_param, body_func) =
                if let Some(b) = &operation.request_body {
                    let b = b.item()?;
                    if b.is_binary()? {
                        bounds.push(quote! {B: Into<reqwest::Body>});
                        (Some(quote! {B}), Some(quote! { .body(body) }))
                    } else {
                        let mt = b
                            .content_json()
                            .with_context(|| anyhow!("{} {}", method, path))?;
                        if !mt.encoding.is_empty() {
                            bail!("media type encoding not empty: {:#?}", mt);
                        }

                        if let Some(s) = &mt.schema {
                            let schema = s.to_schema();
                            let typ = ts.add_type_details(&schema)?.parameter;
                            (Some(typ), Some(quote! { .json(body) }))
                        } else {
                            bail!("media type encoding, no schema: {:#?}", mt);
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
                if !matches!(only.0, openapiv3::StatusCode::Code(200..=299)) {
                    bail!("code? {:#?}", only);
                }

                let i = only.1.item()?;
                if !i.headers.is_empty() {
                    bail!("no response headers for now");
                }

                if !i.links.is_empty() {
                    bail!("no response links for now");
                }

                /*
                 * Look at the response content.  For now, support a single
                 * JSON-formatted response.
                 */
                let typ = match (
                    i.content.len(),
                    i.content.get("application/json"),
                ) {
                    (0, _) => quote! { () },
                    (1, Some(mt)) => {
                        if !mt.encoding.is_empty() {
                            bail!("media type encoding not empty: {:#?}", mt);
                        }

                        if let Some(schema) = &mt.schema {
                            let schema = schema.to_schema();
                            ts.add_type_details(&schema)?.ident
                        } else {
                            bail!("media type encoding, no schema: {:#?}", mt);
                        }
                    }
                    (1, None) => {
                        bail!("response content not JSON: {:#?}", i.content);
                    }
                    (_, _) => {
                        bail!("too many response contents: {:#?}", i.content);
                    }
                };
                (typ, quote! { res.json().await? })
            } else if operation.responses.responses.is_empty() {
                (quote! { reqwest::Response }, quote! { res })
            } else {
                bail!("responses? {:#?}", operation.responses);
            };

            let operation_id =
                format_ident!("{}", operation.operation_id.as_deref().unwrap());

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

    let mut types = ts
        .iter_types()
        .map(|type_entry| (type_entry.type_name(ts), type_entry.output(ts)))
        .collect::<Vec<_>>();
    types.sort_by(|a, b| a.0.cmp(&b.0));
    let types = types.into_iter().map(|(_, def)| def);

    println!("-----------------------------------------------------");
    println!(" TYPE SPACE");
    println!("-----------------------------------------------------");
    for (idx, type_entry) in ts.iter_types().enumerate() {
        let n = type_entry.describe();
        println!("{:>4}  {}", idx, n);
    }
    println!("-----------------------------------------------------");
    println!();

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
    let regex = regex::Regex::new(r#"(})(\n\s*[^} ])"#).unwrap();
    let file = rustfmt_wrapper::rustfmt(file).unwrap();

    let file = regex.replace_all(&file, "$1\n$2").to_string();

    Ok(file)
}

fn main() -> Result<()> {
    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.reqopt("i", "", "OpenAPI definition document (JSON)", "INPUT");
    opts.reqopt("o", "", "Generated Rust crate directory", "OUTPUT");
    opts.reqopt("n", "", "Target Rust crate name", "CRATE");
    opts.reqopt("v", "", "Target Rust crate version", "VERSION");

    let args = match opts.parse(std::env::args().skip(1)) {
        Ok(args) => {
            if !args.free.is_empty() {
                eprintln!("{}", opts.usage("progenitor"));
                bail!("unexpected positional arguments");
            }
            args
        }
        Err(e) => {
            eprintln!("{}", opts.usage("progenitor"));
            bail!(e);
        }
    };

    let api = load_api(&args.opt_str("i").unwrap())?;

    // Convert our components dictionary to schemars
    let schemas = api
        .components
        .iter()
        .flat_map(|components| {
            components.schemas.iter().map(|(name, ref_or_schema)| {
                (name.clone(), ref_or_schema.to_schema())
            })
        })
        .collect::<Vec<(String, _)>>();

    // Create a new type space, prepopulated with our referenced schemas.
    let mut ts = TypeSpace::default();
    ts.set_type_mod("types");

    ts.add_ref_types(schemas)?;

    let fail = match generate(&api, &mut ts) {
        Ok(out) => {
            let name = args.opt_str("n").unwrap();
            let version = args.opt_str("v").unwrap();

            /*
             * Create the top-level crate directory:
             */
            let root = PathBuf::from(args.opt_str("o").unwrap());
            std::fs::create_dir_all(&root)?;

            /*
             * Write the Cargo.toml file:
             */
            let mut toml = root.clone();
            toml.push("Cargo.toml");
            let chrono = if ts.uses_chrono() {
                "chrono = { version = \"0.4\", features = [\"serde\"] }\n"
            } else {
                ""
            };
            let uuid = if ts.uses_uuid() {
                "uuid = { version = \"0.8\", features = [\"serde\", \"v4\"] }\n"
            } else {
                ""
            };
            let serde_json = if ts.uses_uuid() {
                "serde_json = \"1\"\n"
            } else {
                ""
            };
            let tomlout = format!(
                "[package]\n\
                name = \"{}\"\n\
                version = \"{}\"\n\
                edition = \"2018\"\n\
                \n\
                [dependencies]\n\
                anyhow = \"1\"\n\
                {}\
                {}\
                percent-encoding = \"2.1\"\n\
                reqwest = {{ version = \"0.11\", features = [\"json\"] }}\n\
                serde = {{ version = \"1\", features = [\"derive\"] }}\n\
                {}",
                name, version, chrono, uuid, serde_json,
            );
            save(&toml, tomlout.as_str())?;

            /*
             * Create the src/ directory:
             */
            let mut src = root;
            src.push("src");
            std::fs::create_dir_all(&src)?;

            /*
             * Create the Rust source file containing the generated client:
             */
            let mut librs = src;
            librs.push("lib.rs");
            save(librs, out.as_str())?;
            false
        }
        Err(e) => {
            println!("gen fail: {:?}", e);
            true
        }
    };

    if fail {
        bail!("generation experienced errors");
    }

    Ok(())
}
