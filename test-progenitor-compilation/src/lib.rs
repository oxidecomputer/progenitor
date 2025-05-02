// Copyright 2022 Oxide Computer Company

extern crate proc_macro;

use openapiv3::OpenAPI;
use proc_macro::TokenStream;
use progenitor_impl::{GenerationSettings, Generator, InterfaceStyle, TagStyle};
use quote::quote;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[proc_macro]
pub fn cli_tokens(item: TokenStream) -> TokenStream {
    let arg = item.to_string().replace("\"", "");
    let mut in_path = PathBuf::from("sample_openapi");
    in_path.push(arg);

    let spec = load_api(in_path);

    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(InterfaceStyle::Builder)
            .with_tag(TagStyle::Separate)
            .with_derive("schemars::JsonSchema"),
    );

    // Builder generation with tags.
    let sdk = generator.generate_tokens(&spec).unwrap();

    // CLI generation.
    let cli = generator.cli(&spec, "sdk").unwrap();

    quote! {
        pub mod sdk {
            #sdk
        }
        use sdk::*;

        #cli
    }
    .into()
}

fn load_api<P>(p: P) -> OpenAPI
where
    P: AsRef<Path> + std::clone::Clone + std::fmt::Debug,
{
    let mut f = File::open(p.clone()).unwrap();
    match serde_json::from_reader(f) {
        Ok(json_value) => json_value,
        _ => {
            f = File::open(p).unwrap();
            serde_yaml::from_reader(f).unwrap()
        }
    }
}
