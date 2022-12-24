// Copyright 2022 Oxide Computer Company

use std::{collections::HashMap, fmt::Display, path::Path};

use openapiv3::OpenAPI;
use proc_macro::TokenStream;
use progenitor_impl::{
    GenerationSettings, Generator, InterfaceStyle, TagStyle, TypePatch,
};
use quote::{quote, ToTokens};
use schemars::schema::SchemaObject;
use serde::Deserialize;
use serde_tokenstream::{OrderedMap, ParseWrapper};
use syn::LitStr;

/// Generates a client from the given OpenAPI document
///
/// `generate_api!` can be invoked in two ways. The simple form, takes a path
/// to the OpenAPI document:
/// ```ignore
/// generate_api!("path/to/spec.json");
/// ```
///
/// The more complex form accepts the following key-value pairs in any order:
/// ```ignore
/// generate_api!(
///     spec = "path/to/spec.json",
///     [ interface = ( Positional | Builder ), ]
///     [ tags = ( Merged | Separate ), ]
///     [ pre_hook = closure::or::path::to::function, ]
///     [ post_hook = closure::or::path::to::function, ]
///     [ derives = [ path::to::DeriveMacro ], ]
/// );
/// ```
///
/// The `spec` key is required; it is the OpenAPI document from which the
/// client is derived.
///
/// The optional `interface` lets you specify either a `Positional` argument or
/// `Builder` argument style; `Positional` is the default.
///
/// The optional `tags` may be `Merged` in which case all operations are
/// methods on the `Client` struct or `Separate` in which case each tag is
/// represented by an "extension trait" that `Client` implements. The default
/// is `Merged`.
///
/// The optional `inner_type` is for ancillary data, stored with the generated
/// client that can be usd by the pre and post hooks.
///
/// The optional `pre_hook` is either a closure (that must be within
/// parentheses: `(fn |inner, request| { .. })`) or a path to a function. The
/// closure or function must take two parameters: the inner type and a
/// `&reqwest::Request`. This allows clients to examine requests before they're
/// sent to the server, for example to log them.
///
/// The optional `post_hook` is either a closure (that must be within
/// parentheses: `(fn |inner, result| { .. })`) or a path to a function. The
/// closure or function must take two parameters: the inner type and a
/// `&Result<reqwest::Response, reqwest::Error>`. This allows clients to
/// examine responses, for example to log them.
///
/// The optional `derives` array allows consumers to specify additional derive
/// macros to apply to generated types.
#[proc_macro]
pub fn generate_api(item: TokenStream) -> TokenStream {
    match do_generate_api(item) {
        Err(err) => err.to_compile_error().into(),
        Ok(out) => out,
    }
}

#[derive(Deserialize)]
struct MacroSettings {
    spec: ParseWrapper<LitStr>,
    #[serde(default)]
    interface: InterfaceStyle,
    #[serde(default)]
    tags: TagStyle,
    inner_type: Option<ParseWrapper<syn::Type>>,
    pre_hook: Option<ParseWrapper<ClosureOrPath>>,
    post_hook: Option<ParseWrapper<ClosureOrPath>>,
    #[serde(default)]
    derives: Vec<ParseWrapper<syn::Path>>,
    #[serde(default)]
    patch: HashMap<ParseWrapper<syn::Type>, MacroPatch>,
    #[serde(default)]
    convert: OrderedMap<
        SchemaObject,
        (ParseWrapper<syn::Path>, Vec<MacroSettingsImpl>),
    >,
}

#[derive(Deserialize)]
enum MacroSettingsImpl {
    Display,
}

impl Display for MacroSettingsImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacroSettingsImpl::Display => f.write_str("Display"),
        }
    }
}

#[derive(Deserialize)]
struct MacroPatch {
    #[serde(default)]
    rename: Option<String>,
    #[serde(default)]
    derives: Vec<ParseWrapper<syn::Path>>,
}

impl From<MacroPatch> for TypePatch {
    fn from(a: MacroPatch) -> Self {
        let mut s = Self::default();
        a.rename.iter().for_each(|rename| {
            s.with_rename(rename);
        });
        a.derives.iter().for_each(|derive| {
            s.with_derive(derive.to_token_stream().to_string());
        });
        s
    }
}

#[derive(Deserialize)]
enum GenerationStyle {
    Positional,
    Builder,
}

impl Default for GenerationStyle {
    fn default() -> Self {
        Self::Positional
    }
}

#[derive(Debug)]
struct ClosureOrPath(proc_macro2::TokenStream);

impl syn::parse::Parse for ClosureOrPath {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(syn::token::Paren) {
            let group: proc_macro2::Group = input.parse()?;
            return syn::parse2::<Self>(group.stream());
        }

        if let Ok(closure) = input.parse::<syn::ExprClosure>() {
            return Ok(Self(closure.to_token_stream()));
        }

        input
            .parse::<syn::Path>()
            .map(|path| Self(path.to_token_stream()))
    }
}

fn do_generate_api(item: TokenStream) -> Result<TokenStream, syn::Error> {
    let (spec, settings) = if let Ok(spec) = syn::parse::<LitStr>(item.clone())
    {
        (spec, GenerationSettings::default())
    } else {
        let MacroSettings {
            spec,
            interface,
            tags,
            inner_type,
            pre_hook,
            post_hook,
            derives,
            patch,
            convert,
        } = serde_tokenstream::from_tokenstream(&item.into())?;
        let mut settings = GenerationSettings::default();
        settings.with_interface(interface);
        settings.with_tag(tags);
        inner_type.map(|inner_type| {
            settings.with_inner_type(inner_type.to_token_stream())
        });
        pre_hook
            .map(|pre_hook| settings.with_pre_hook(pre_hook.into_inner().0));
        post_hook
            .map(|post_hook| settings.with_post_hook(post_hook.into_inner().0));

        derives.into_iter().for_each(|derive| {
            settings.with_derive(derive.to_token_stream());
        });
        patch.into_iter().for_each(|(type_name, patch)| {
            settings.with_patch(
                type_name.to_token_stream().to_string(),
                &patch.into(),
            );
        });
        convert
            .into_iter()
            .for_each(|(schema, (type_name, impls))| {
                settings.with_conversion(
                    schema,
                    type_name.to_token_stream(),
                    impls.into_iter(),
                );
            });
        (spec.into_inner(), settings)
    };

    let dir = std::env::var("CARGO_MANIFEST_DIR").map_or_else(
        |_| std::env::current_dir().unwrap(),
        |s| Path::new(&s).to_path_buf(),
    );

    let path = dir.join(spec.value());
    let path_str = path.to_string_lossy();

    let oapi: OpenAPI =
        serde_json::from_reader(std::fs::File::open(&path).map_err(|e| {
            syn::Error::new(
                spec.span(),
                format!("couldn't read file {}: {}", path_str, e),
            )
        })?)
        .map_err(|e| {
            syn::Error::new(
                spec.span(),
                format!("failed to parse {}: {}", path_str, e),
            )
        })?;

    let mut builder = Generator::new(&settings);

    let code = builder.generate_tokens(&oapi).map_err(|e| {
        syn::Error::new(
            spec.span(),
            format!("generation error for {}: {}", spec.value(), e),
        )
    })?;

    let output = quote! {
        // The progenitor_client is tautologically visible from macro
        // consumers.
        use progenitor::progenitor_client;

        #code

        // Force a rebuild when the given file is modified.
        const _: &str = include_str!(#path_str);
    };

    Ok(output.into())
}
