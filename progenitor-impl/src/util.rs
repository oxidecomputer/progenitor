// Copyright 2022 Oxide Computer Company

use std::collections::BTreeMap;

use indexmap::IndexMap;
use openapiv3::{Components, Parameter, ReferenceOr, RequestBody, Response, Schema};
use unicode_ident::{is_xid_continue, is_xid_start};

use crate::Result;

pub(crate) trait ReferenceOrExt<T: ComponentLookup> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T>;
}
pub(crate) trait ComponentLookup: Sized {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>>;
}

impl<T: ComponentLookup> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T> {
        match self {
            ReferenceOr::Item(item) => Ok(item),
            ReferenceOr::Reference { reference } => {
                let idx = reference.rfind('/').unwrap();
                let key = &reference[idx + 1..];
                let parameters = T::get_components(components.as_ref().unwrap());
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
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.parameters
    }
}

impl ComponentLookup for RequestBody {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.request_bodies
    }
}

impl ComponentLookup for Response {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.responses
    }
}

impl ComponentLookup for Schema {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
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
    if typify::accept_as_ident(&out) {
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

/// Name `lifetime` on every reference in a rendered parameter identifier.
///
/// A parameter identifier introduces a reference wherever the caller
/// borrows rather than owns, and every one of those needs the method's
/// lifetime.
// ATTN REVIEWER: typespace renders a parameter identifier with a module
// scope (`parameter_ident_in`) or with a lifetime
// (`parameter_ident_with_lifetime`), but has no query for both at once,
// and progenitor needs both. `&` appears in a parameter identifier only
// where typespace would have placed the lifetime, so inserting it after
// each one reproduces the identifier typespace would have rendered.
pub(crate) fn with_lifetime(
    tokens: proc_macro2::TokenStream,
    lifetime: &str,
) -> proc_macro2::TokenStream {
    use proc_macro2::{Group, TokenStream, TokenTree};

    let lifetime_tok = syn::Lifetime::new(&format!("'{lifetime}"), proc_macro2::Span::call_site());

    tokens
        .into_iter()
        .flat_map(|tree| match tree {
            TokenTree::Group(group) => {
                let inner = with_lifetime(group.stream(), lifetime);
                vec![TokenTree::Group(Group::new(group.delimiter(), inner))]
            }
            TokenTree::Punct(punct) if punct.as_char() == '&' => {
                let mut out = vec![TokenTree::Punct(punct)];
                out.extend(quote::ToTokens::to_token_stream(&lifetime_tok));
                out
            }
            tree => vec![tree],
        })
        .collect::<TokenStream>()
}
