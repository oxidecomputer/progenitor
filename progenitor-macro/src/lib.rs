// Copyright 2021 Oxide Computer Company

use std::path::Path;

use openapiv3::OpenAPI;
use proc_macro::TokenStream;
use progenitor_impl::Generator;
use syn::LitStr;

#[proc_macro]
pub fn generate_api(item: TokenStream) -> TokenStream {
    match do_generate_api(item) {
        Err(err) => err.to_compile_error().into(),
        Ok(out) => out,
    }
}

fn do_generate_api(item: TokenStream) -> Result<TokenStream, syn::Error> {
    let arg = syn::parse::<LitStr>(item)?;
    let dir = std::env::var("CARGO_MANIFEST_DIR").map_or_else(
        |_| std::env::current_dir().unwrap(),
        |s| Path::new(&s).to_path_buf(),
    );

    let path = dir.join(arg.value());

    let content = std::fs::read_to_string(&path).map_err(|e| {
        syn::Error::new(
            arg.span(),
            format!("couldn't read file {}: {}", arg.value(), e.to_string()),
        )
    })?;

    let spec = serde_json::from_str::<OpenAPI>(&content).map_err(|e| {
        syn::Error::new(
            arg.span(),
            format!("failed to parse {}: {}", arg.value(), e.to_string()),
        )
    })?;

    let mut builder = Generator::new();
    let ret = builder.generate_tokens(&spec).map_err(|e| {
        syn::Error::new(
            arg.span(),
            format!("generation error for {}: {}", arg.value(), e.to_string()),
        )
    })?;

    Ok(ret.into())
}
