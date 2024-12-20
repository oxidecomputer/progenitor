// Copyright 2022 Oxide Computer Company

use std::collections::BTreeMap;

use indexmap::IndexMap;
use openapiv3::{
    Components, Parameter, ReferenceOr, RequestBody, Response, Schema,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use typify::TypeSpace;
use unicode_ident::{is_xid_continue, is_xid_start};

use crate::method::OperationResponseKind;
use crate::{Result};

pub(crate) trait ReferenceOrExt<T: ComponentLookup> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T>;
}
pub(crate) trait ComponentLookup: Sized {
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
                parameters
                    .get(key)
                    .unwrap_or_else(|| panic!("key {} is missing", key))
                    .item(components)
            }
        }
    }
}

pub(crate) fn items<'a, T>(
    refs: &'a [ReferenceOr<T>],
    components: &'a Option<Components>,
) -> impl Iterator<Item = Result<&'a T>>
where
    T: ComponentLookup,
{
    refs.iter().map(|r| r.item(components))
}

pub(crate) fn parameter_map<'a>(
    refs: &'a [ReferenceOr<Parameter>],
    components: &'a Option<Components>,
) -> Result<BTreeMap<&'a String, &'a Parameter>> {
    items(refs, components)
        .map(|res| res.map(|param| (&param.parameter_data_ref().name, param)))
        .collect()
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

pub(crate) enum Case {
    Pascal,
    Snake,
}

pub(crate) fn sanitize(input: &str, case: Case) -> String {
    use heck::{ToPascalCase, ToSnakeCase};
    let to_case = match case {
        Case::Pascal => str::to_pascal_case,
        Case::Snake => str::to_snake_case,
    };
    // If every case was special then none of them would be.
    let out = match input {
        "+1" => "plus1".to_string(),
        "-1" => "minus1".to_string(),
        _ => to_case(
            &input
                .replace('\'', "")
                .replace(|c: char| !is_xid_continue(c), "-"),
        ),
    };

    let out = match out.chars().next() {
        None => to_case("x"),
        Some(c) if is_xid_start(c) => out,
        Some(_) => format!("_{}", out),
    };

    // Make sure the string is a valid Rust identifier.
    if syn::parse_str::<syn::Ident>(&out).is_ok() {
        out
    } else {
        format!("{}_", out)
    }
}

/// Given a desired name and a slice of proc_macro2::Ident, generate a new
/// Ident that is unique from the slice.
pub(crate) fn unique_ident_from(
    name: &str,
    identities: &[proc_macro2::Ident],
) -> proc_macro2::Ident {
    let mut name = name.to_string();

    loop {
        let ident = quote::format_ident!("{}", name);

        if !identities.contains(&ident) {
            return ident;
        }

        name.insert_str(0, "_");
    }
}

/// Generates a unique identifier by concatenating the type identifiers for a collection of `OperationResponseKind`.
/// This function is used to dynamically create enum variant names or type combinations based on the types contained within a `Multi` response kind.
pub(crate) fn generate_multi_type_identifier(types: &Vec<Box<OperationResponseKind>>, type_space: &TypeSpace) -> TokenStream {
    let identifiers: Vec<TokenStream> = types.iter()
        .map(|type_kind| {
            match type_kind.as_ref() {
                OperationResponseKind::None => {
                    // Directly return a TokenStream representing 'None' if the type is None.
                    // This case handles the scenario where the generated tokens would have been ().
                    quote! { None }
                }
                OperationResponseKind::Upgrade => {
                    // Directly return a TokenStream representing 'Upgrade' if the type is Upgrade.
                    // This case handles the scenario where the generated tokens would have been reqwest::Upgraded.
                    quote! { Upgraded }
                }
                OperationResponseKind::Type(type_id) => {
                    // Directly use the Ident returned from TypeSpace, ensuring no invalid string manipulation
                    let type_name = format_ident!("{}", type_space.get_type(type_id).unwrap().ident().to_string().replace("types :: ", ""));
                    quote! { #type_name }
                }
                _ => {
                    // Otherwise, generate tokens normally using the `into_tokens` method.
                    type_kind.clone().into_tokens(type_space)
                }
            }
        })
        .collect();

    // Convert each TokenStream to string, concatenate them with "Or", and prepend with "types::"
    let concatenated_type = identifiers.iter()
        .map(|ts| ts.to_string())
        .collect::<Vec<_>>()
        .join("Or");

    // Parse the concatenated string back to a TokenStream to ensure that it can be used in code generation.
    // This step assumes that the concatenated string is a valid Rust identifier or code.
    let tokens = concatenated_type.parse::<TokenStream>().unwrap_or_else(|_| quote! { InvalidIdentifier });
    quote! { #tokens } // Return the new identifier as a TokenStream
}
pub(crate) fn generate_multi_type_for_types_stream(types: &Vec<Box<OperationResponseKind>>, type_space: &TypeSpace) -> TokenStream {
    let enum_name = generate_multi_type_identifier(types, type_space);

    // Generate enum variants and their `From` implementations
    let variants: Vec<TokenStream> = types.iter().map(|type_kind| {
        match type_kind.as_ref() {
            OperationResponseKind::None => {
                quote! { None }
            }
            OperationResponseKind::Upgrade => {
                quote! { Upgraded(reqwest::Upgraded) }
            }
            OperationResponseKind::Type(type_id) => {
                let type_ident = type_space.get_type(type_id).unwrap().ident().to_string().replace("types :: ", "").parse::<TokenStream>().unwrap();
                quote! { #type_ident(#type_ident) }
            }
            _ => quote! { Unknown },
        }
    }).collect();

    let from_impls: Vec<TokenStream> = types.iter().map(|type_kind| {
        match type_kind.as_ref() {
            OperationResponseKind::None => {
                quote! {
                    impl From<()> for #enum_name {
                        fn from(_: ()) -> Self {
                            #enum_name::None
                        }
                    }
                }
            }
            OperationResponseKind::Upgrade => {
                quote! {
                    impl From<reqwest::Upgraded> for #enum_name {
                        fn from(value: reqwest::Upgraded) -> Self {
                            #enum_name::Upgraded(value)
                        }
                    }
                }
            }
            OperationResponseKind::Type(type_id) => {
                let type_ident = type_space.get_type(type_id).unwrap().ident().to_string().replace("types :: ", "").parse::<TokenStream>().unwrap();
                quote! {
                    impl From<#type_ident> for #enum_name {
                        fn from(value: #type_ident) -> Self {
                            #enum_name::#type_ident(value)
                        }
                    }
                }
            }
            _ => {
                todo!() // Possibility of nested Multi types given openapi spec?
            }
        }
    }).collect();

    let tokens = quote! {
        // Define the enum
        #[derive(Debug)]
        pub enum #enum_name {
            #(#variants),*
        }

        // Define the From implementations
        #(#from_impls)*
    };

    println!("Tokens: {}", tokens);

    tokens
}

