// Copyright 2022 Oxide Computer Company

use indexmap::IndexMap;
use openapiv3::{
    Components, Parameter, ReferenceOr, RequestBody, Response, Schema,
};
use unicode_ident::{is_xid_continue, is_xid_start};

use crate::Result;

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
                .replace("'", "")
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
