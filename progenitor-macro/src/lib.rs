// Copyright 2022 Oxide Computer Company

use std::path::Path;

use openapiv3::OpenAPI;
use proc_macro::TokenStream;
use progenitor_impl::Generator;
use quote::{quote, ToTokens};
use serde::Deserialize;
use serde_tokenstream::ParseWrapper;
use syn::LitStr;

#[proc_macro]
pub fn generate_api(item: TokenStream) -> TokenStream {
    match do_generate_api(item) {
        Err(err) => err.to_compile_error().into(),
        Ok(out) => out,
    }
}

#[derive(Deserialize)]
struct Settings {
    spec: ParseWrapper<LitStr>,
    inner_type: Option<ParseWrapper<syn::Type>>,
    pre_hook: Option<ParseWrapper<ClosureOrPath>>,
    post_hook: Option<ParseWrapper<ClosureOrPath>>,
    #[serde(default)]
    derives: Vec<ParseWrapper<syn::Path>>,
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
    let (spec, inner_type, pre_hook, post_hook, derives) =
        if let Ok(spec) = syn::parse::<LitStr>(item.clone()) {
            (spec, None, None, None, Vec::new())
        } else {
            let Settings {
                spec,
                inner_type,
                pre_hook,
                post_hook,
                derives,
            } = serde_tokenstream::from_tokenstream(&item.into())?;
            (
                spec.into_inner(),
                inner_type.map(|x| x.into_inner()),
                pre_hook.map(|x| x.into_inner()),
                post_hook.map(|x| x.into_inner()),
                derives.into_iter().map(ParseWrapper::into_inner).collect(),
            )
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
                format!("couldn't read file {}: {}", path_str, e.to_string()),
            )
        })?)
        .map_err(|e| {
            syn::Error::new(
                spec.span(),
                format!("failed to parse {}: {}", path_str, e.to_string()),
            )
        })?;

    let mut builder = Generator::new();
    inner_type.map(|inner_type| {
        builder.with_inner_type(inner_type.to_token_stream())
    });
    pre_hook.map(|pre_hook| builder.with_pre_hook(pre_hook.0));
    post_hook.map(|post_hook| builder.with_post_hook(post_hook.0));

    derives.into_iter().for_each(|derive| {
        builder.with_derive(derive.to_token_stream());
    });

    let code = builder.generate_tokens(&oapi).map_err(|e| {
        syn::Error::new(
            spec.span(),
            format!("generation error for {}: {}", spec.value(), e.to_string()),
        )
    })?;

    let output = quote! {
        #code

        // Force a rebuild when the given file is modified.
        const _: &str = include_str!(#path_str);
    };

    Ok(output.into())
}
