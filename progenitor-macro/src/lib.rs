// Copyright 2021 Oxide Computer Company

use std::path::Path;

use openapiv3::OpenAPI;
use proc_macro::TokenStream;
use progenitor_impl::Generator;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    ExprClosure, LitStr, Token,
};

#[proc_macro]
pub fn generate_api(item: TokenStream) -> TokenStream {
    match do_generate_api(item) {
        Err(err) => err.to_compile_error().into(),
        Ok(out) => out,
    }
}

struct Settings {
    file: LitStr,
    inner: Option<proc_macro2::TokenStream>,
    pre: Option<proc_macro2::TokenStream>,
    post: Option<proc_macro2::TokenStream>,
}

impl Parse for Settings {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let file = input.parse::<LitStr>()?;
        let inner = parse_inner(input)?;
        let pre = parse_hook(input)?;
        let post = parse_hook(input)?;

        // Optional trailing comma.
        if input.peek(Token!(,)) {
            let _ = input.parse::<Token!(,)>();
        }

        Ok(Settings {
            file,
            inner,
            pre,
            post,
        })
    }
}

fn parse_inner(
    input: ParseStream,
) -> Result<Option<proc_macro2::TokenStream>, syn::Error> {
    if input.is_empty() {
        return Ok(None);
    }
    let _: Token!(,) = input.parse()?;
    if input.is_empty() {
        return Ok(None);
    }
    Ok(Some(input.parse::<syn::Type>()?.to_token_stream()))
}

fn parse_hook(
    input: ParseStream,
) -> Result<Option<proc_macro2::TokenStream>, syn::Error> {
    if input.is_empty() {
        return Ok(None);
    }
    let _: Token!(,) = input.parse()?;
    if input.is_empty() {
        return Ok(None);
    }
    if let Ok(closure) = input.parse::<ExprClosure>() {
        Ok(Some(closure.to_token_stream()))
    } else {
        Ok(Some(input.parse::<syn::Path>()?.to_token_stream()))
    }
}

fn do_generate_api(item: TokenStream) -> Result<TokenStream, syn::Error> {
    let Settings {
        file,
        inner,
        pre,
        post,
    } = syn::parse::<Settings>(item)?;
    let dir = std::env::var("CARGO_MANIFEST_DIR").map_or_else(
        |_| std::env::current_dir().unwrap(),
        |s| Path::new(&s).to_path_buf(),
    );

    let path = dir.join(file.value());

    let content = std::fs::read_to_string(&path).map_err(|e| {
        syn::Error::new(
            file.span(),
            format!("couldn't read file {}: {}", file.value(), e.to_string()),
        )
    })?;

    let spec = serde_json::from_str::<OpenAPI>(&content).map_err(|e| {
        syn::Error::new(
            file.span(),
            format!("failed to parse {}: {}", file.value(), e.to_string()),
        )
    })?;

    let mut builder = Generator::new();
    inner.map(|inner_type| builder.with_inner_type(inner_type));
    pre.map(|pre_hook| builder.with_pre_hook(pre_hook));
    post.map(|post_hook| builder.with_post_hook(post_hook));
    let ret = builder.generate_tokens(&spec).map_err(|e| {
        syn::Error::new(
            file.span(),
            format!("generation error for {}: {}", file.value(), e.to_string()),
        )
    })?;

    Ok(ret.into())
}
