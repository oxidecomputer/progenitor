// Copyright 2025 Oxide Computer Company

use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    error::Error,
};

use progenitor_client::{encode_path, QueryParam};
use serde::Serialize;

#[test]
fn test_path_segment_encoding() {
    assert_eq!(encode_path("192.168.0.0/24"), "192.168.0.0%2F24");
}

fn encode_query_param<T: Serialize>(param_name: &str, value: &T) -> Result<String, Box<dyn Error>> {
    let mut url = url::Url::parse("https://localhost")?;
    let mut pairs = url.query_pairs_mut();
    let serializer = serde_urlencoded::Serializer::new(&mut pairs);

    QueryParam::new(param_name, value).serialize(serializer)?;
    drop(pairs);

    Ok(url.query().unwrap().to_owned())
}

#[test]
fn test_query_scalars() {
    let value = "xyz".to_string();
    let result = encode_query_param("param_name", &value).unwrap();
    assert_eq!(result, "param_name=xyz");

    let value = 42;
    let result = encode_query_param("param_name", &value).unwrap();
    assert_eq!(result, "param_name=42");

    let value = -0.05;
    let result = encode_query_param("param_name", &value).unwrap();
    assert_eq!(result, "param_name=-0.05");
}

#[test]
fn test_query_arrays() {
    let value = vec!["a", "b", "c"];
    let result = encode_query_param("paramName", &value).unwrap();

    assert_eq!(result, "paramName=a&paramName=b&paramName=c");

    let value = ["a", "b", "c"].into_iter().collect::<BTreeSet<_>>();
    let result = encode_query_param("paramName", &value).unwrap();

    assert_eq!(result, "paramName=a&paramName=b&paramName=c");

    let value = ["a", "b", "c"].into_iter().collect::<HashSet<_>>();
    let result = encode_query_param("paramName", &value).unwrap();

    // Handle hash ordering.
    assert_eq!(
        result,
        value
            .iter()
            .map(|v| format!("paramName={v}"))
            .collect::<Vec<_>>()
            .join("&")
    );
}

#[test]
fn test_query_object() {
    #[derive(Serialize)]
    struct Value {
        a: String,
        b: String,
    }
    let value = Value {
        a: "a value".to_string(),
        b: "b value".to_string(),
    };
    let result = encode_query_param("ignored", &value).unwrap();

    assert_eq!(result, "a=a+value&b=b+value");
}

#[test]
fn test_query_map() {
    let value = [("a", "a value"), ("b", "b value")]
        .into_iter()
        .collect::<BTreeMap<_, _>>();
    let result = encode_query_param("ignored", &value).unwrap();
    assert_eq!(result, "a=a+value&b=b+value");
}

#[test]
fn test_query_enum_external() {
    #[derive(Serialize)]
    #[serde(rename_all = "snake_case")]
    enum Value {
        Simple,
        Newtype(u64),
        Tuple(u64, u64),
        Object { a: u64, b: u64 },
    }
    let value = Value::Simple;
    let result = encode_query_param("paramValue", &value).unwrap();
    assert_eq!(result, "paramValue=simple");

    let value = Value::Newtype(42);
    let result = encode_query_param("ignored", &value).unwrap();
    assert_eq!(result, "newtype=42");

    let value = Value::Tuple(1, 2);
    encode_query_param("ignored", &value).expect_err("variant not supported");

    let value = Value::Object { a: 3, b: 4 };
    encode_query_param("ignored", &value).expect_err("variant not supported");
}

#[test]
fn test_query_enum_internal() {
    #[derive(Serialize)]
    #[serde(tag = "tag_name")]
    #[serde(rename_all = "snake_case")]
    enum Value {
        Simple,
        Object { a: u64, b: u64 },
    }
    let value = Value::Simple;
    let result = encode_query_param("ignored", &value).unwrap();
    assert_eq!(result, "tag_name=simple");

    let value = Value::Object { a: 3, b: 4 };
    let result = encode_query_param("ignored", &value).unwrap();
    assert_eq!(result, "tag_name=object&a=3&b=4");
}

#[test]
fn test_query_enum_adjacent() {
    #[derive(Serialize)]
    #[serde(tag = "tag_name", content = "content_name")]
    #[serde(rename_all = "snake_case")]
    enum Value {
        Simple,
        Newtype(u64),
        Tuple(u64, u64),
        Object { a: u64, b: u64 },
    }
    let value = Value::Simple;
    let result = encode_query_param("ignored", &value).unwrap();
    assert_eq!(result, "tag_name=simple");

    let value = Value::Newtype(42);
    let result = encode_query_param("ignored", &value).unwrap();
    assert_eq!(result, "tag_name=newtype&content_name=42");

    let value = Value::Tuple(1, 2);
    encode_query_param("ignored", &value).expect_err("invalid variant");

    let value = Value::Object { a: 3, b: 4 };
    encode_query_param("ignored", &value).expect_err("invalid variant");
}

#[test]
fn test_query_enum_untagged() {
    #[derive(Serialize)]
    #[serde(untagged)]
    #[serde(rename_all = "snake_case")]
    enum Value {
        Simple,
        Newtype(u64),
        Tuple(u64, u64),
        Object { a: u64, b: u64 },
        Array(Vec<u64>),
    }
    let value = Value::Simple;
    let result = encode_query_param("ignored", &value).unwrap();
    assert_eq!(result, "");

    let value = Value::Newtype(42);
    let result = encode_query_param("paramName", &value).unwrap();
    assert_eq!(result, "paramName=42");

    let value = Value::Tuple(1, 2);
    encode_query_param("ignored", &value).expect_err("invalid variant");

    let value = Value::Object { a: 3, b: 4 };
    let result = encode_query_param("ignored", &value).unwrap();
    assert_eq!(result, "a=3&b=4");

    let value = Value::Array(vec![1, 2, 3, 4]);
    let result = encode_query_param("paramName", &value).unwrap();
    assert_eq!(result, "paramName=1&paramName=2&paramName=3&paramName=4");

    #[derive(Serialize)]
    #[serde(transparent)]
    struct Name(String);
    #[derive(Serialize)]
    #[serde(untagged)]
    enum NameOrId {
        Name(Name),
        Id(uuid::Uuid),
    }
    let value = Some(NameOrId::Name(Name("xyz".to_string())));
    let result = encode_query_param("paramName", &value).unwrap();
    assert_eq!(result, "paramName=xyz");

    let id = uuid::Uuid::new_v4();
    let value = Some(NameOrId::Id(id));
    let result = encode_query_param("paramName", &value).unwrap();
    assert_eq!(result, format!("paramName={id}"));
}

#[test]
fn test_query_option() {
    let value = Option::<u64>::None;
    let result = encode_query_param("ignored", &value).unwrap();
    assert_eq!(result, "");

    let value = Some(42);
    let result = encode_query_param("paramName", &value).unwrap();
    assert_eq!(result, "paramName=42");
}
