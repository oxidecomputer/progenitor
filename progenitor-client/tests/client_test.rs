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

mod form_part_tests {
    use assert_matches::assert_matches;
    use bytes::Bytes;
    use progenitor_client::{BinaryFormPart, ContentType, Filename, FormPart, TextFormPart};
    use serde::Serialize;

    #[test]
    fn test_form_part_binary() {
        let data = vec![1u8, 2, 3, 4];
        let part = FormPart::binary(data.clone());

        assert_matches!(part, FormPart::Binary(BinaryFormPart { data: d, filename: None, content_type: None }) => {
            assert_eq!(d.as_ref(), &data[..]);
        });
    }

    #[test]
    fn test_form_part_binary_from_bytes() {
        let data = Bytes::from_static(b"hello world");
        let part = FormPart::binary(data.clone());

        assert_matches!(part, FormPart::Binary(BinaryFormPart { data: d, .. }) => {
            assert_eq!(d, data);
        });
    }

    #[test]
    fn test_form_part_binary_with_filename() {
        let part =
            FormPart::binary_with_filename(vec![1u8, 2, 3], Filename::new("test.bin").unwrap());

        assert_matches!(part, FormPart::Binary(BinaryFormPart { filename: Some(f), content_type: None, .. }) => {
            assert_eq!(f.as_str(), "test.bin");
        });
    }

    #[test]
    fn test_form_part_binary_with_metadata() {
        let part = FormPart::binary_with_metadata(
            vec![1u8, 2, 3],
            Some(Filename::new("document.pdf").unwrap()),
            Some(ContentType::application("pdf")),
        );

        assert_matches!(part, FormPart::Binary(BinaryFormPart { filename: Some(f), content_type: Some(ct), .. }) => {
            assert_eq!(f.as_str(), "document.pdf");
            assert_eq!(ct.as_str(), "application/pdf");
        });
    }

    #[test]
    fn test_form_part_binary_with_metadata_none() {
        let part = FormPart::binary_with_metadata(vec![1u8, 2, 3], None, None);
        assert_matches!(
            part,
            FormPart::Binary(BinaryFormPart {
                filename: None,
                content_type: None,
                ..
            })
        );
    }

    #[test]
    fn test_form_part_text() {
        let part = FormPart::text("hello world");

        assert_matches!(part, FormPart::Text(TextFormPart { value, content_type: None }) => {
            assert_eq!(value, "hello world");
        });
    }

    #[test]
    fn test_form_part_text_from_string() {
        let part = FormPart::text(String::from("hello world"));

        assert_matches!(part, FormPart::Text(TextFormPart { value, .. }) => {
            assert_eq!(value, "hello world");
        });
    }

    #[test]
    fn test_form_part_text_with_content_type() {
        let part = FormPart::text_with_content_type("{\"key\": \"value\"}", ContentType::json());

        assert_matches!(part, FormPart::Text(TextFormPart { value, content_type: Some(ct) }) => {
            assert_eq!(value, "{\"key\": \"value\"}");
            assert_eq!(ct.as_str(), "application/json");
        });
    }

    #[test]
    fn test_form_part_json() {
        #[derive(Serialize)]
        struct TestData {
            name: String,
            count: i32,
        }

        let data = TestData {
            name: "test".to_string(),
            count: 42,
        };
        let part = FormPart::json(&data).unwrap();

        assert_matches!(part, FormPart::Text(TextFormPart { value, content_type: Some(ct) }) => {
            assert_eq!(value, r#"{"name":"test","count":42}"#);
            assert_eq!(ct.as_str(), "application/json");
        });
    }

    #[test]
    fn test_form_part_json_array() {
        let part = FormPart::json(&vec![1, 2, 3, 4, 5]).unwrap();

        assert_matches!(part, FormPart::Text(TextFormPart { value, content_type: Some(ct) }) => {
            assert_eq!(value, "[1,2,3,4,5]");
            assert_eq!(ct.as_str(), "application/json");
        });
    }

    #[test]
    fn test_form_part_json_nested() {
        #[derive(Serialize)]
        struct Inner {
            value: String,
        }

        #[derive(Serialize)]
        struct Outer {
            inner: Inner,
            tags: Vec<String>,
        }

        let data = Outer {
            inner: Inner {
                value: "nested".to_string(),
            },
            tags: vec!["a".to_string(), "b".to_string()],
        };
        let part = FormPart::json(&data).unwrap();

        assert_matches!(part, FormPart::Text(TextFormPart { value, .. }) => {
            assert!(value.contains("\"inner\""));
            assert!(value.contains("\"nested\""));
            assert!(value.contains("\"tags\""));
        });
    }

    #[test]
    fn test_form_part_clone() {
        let part1 =
            FormPart::binary_with_filename(vec![1, 2, 3], Filename::new("test.bin").unwrap());
        let part2 = part1.clone();

        assert_matches!(
            (part1, part2),
            (FormPart::Binary(BinaryFormPart { data: d1, filename: f1, .. }), FormPart::Binary(BinaryFormPart { data: d2, filename: f2, .. })) => {
                assert_eq!(d1, d2);
                assert_eq!(f1, f2);
            }
        );
    }

    #[test]
    fn test_form_part_debug() {
        let part = FormPart::text("test");
        let debug_str = format!("{:?}", part);
        assert!(debug_str.contains("Text"));
        assert!(debug_str.contains("test"));
    }

    // Tests for the new type-safe newtypes
    #[test]
    fn test_filename_validation() {
        // Valid filenames
        assert!(Filename::new("test.bin").is_ok());
        assert!(Filename::new("my-file_123.txt").is_ok());
        assert!(Filename::new("file with spaces.pdf").is_ok());

        // Invalid filenames (path separators)
        assert!(Filename::new("path/to/file.txt").is_err());
        assert!(Filename::new("path\\to\\file.txt").is_err());
        assert!(Filename::new("file\0name.txt").is_err());
        assert!(Filename::new("").is_err());
    }

    #[test]
    fn test_filename_unchecked() {
        // new_unchecked bypasses validation (useful for trusted sources)
        let f = Filename::new_unchecked("any/path/works");
        assert_eq!(f.as_str(), "any/path/works");
    }

    #[test]
    fn test_content_type_json() {
        assert_eq!(ContentType::json().as_str(), "application/json");
    }

    #[test]
    fn test_content_type_new_unchecked() {
        assert_eq!(
            ContentType::new_unchecked("text/html").as_str(),
            "text/html"
        );
    }

    #[test]
    fn test_content_type_application() {
        assert_eq!(ContentType::application("pdf").as_str(), "application/pdf");
        assert_eq!(
            ContentType::application("xml").as_str(),
            "application/xml"
        );
    }

    #[test]
    fn test_content_type_image() {
        assert_eq!(ContentType::image("png").as_str(), "image/png");
        assert_eq!(ContentType::image("jpeg").as_str(), "image/jpeg");
    }

    #[test]
    fn test_content_type_octet_stream() {
        assert_eq!(
            ContentType::octet_stream().as_str(),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_filename_display() {
        let f = Filename::new("test.bin").unwrap();
        assert_eq!(format!("{}", f), "test.bin");
    }

    #[test]
    fn test_content_type_display() {
        let ct = ContentType::json();
        assert_eq!(format!("{}", ct), "application/json");
    }

    #[test]
    fn test_filename_as_ref() {
        let f = Filename::new("test.bin").unwrap();
        let s: &str = f.as_ref();
        assert_eq!(s, "test.bin");
    }

    #[test]
    fn test_content_type_as_ref() {
        let ct = ContentType::json();
        let s: &str = ct.as_ref();
        assert_eq!(s, "application/json");
    }

    // Builder pattern tests
    #[test]
    fn test_binary_builder_minimal() {
        let part = FormPart::binary_builder(vec![1u8, 2, 3]).build();
        assert_matches!(
            part,
            BinaryFormPart {
                filename: None,
                content_type: None,
                ..
            }
        );
    }

    #[test]
    fn test_binary_builder_with_filename() {
        let part = FormPart::binary_builder(vec![1u8, 2, 3])
            .filename(Filename::new("test.bin").unwrap())
            .build();

        assert_matches!(part, BinaryFormPart { filename: Some(f), content_type: None, .. } => {
            assert_eq!(f.as_str(), "test.bin");
        });
    }

    #[test]
    fn test_binary_builder_with_content_type() {
        let part = FormPart::binary_builder(vec![1u8, 2, 3])
            .content_type(ContentType::image("png"))
            .build();

        assert_matches!(part, BinaryFormPart { filename: None, content_type: Some(ct), .. } => {
            assert_eq!(ct.as_str(), "image/png");
        });
    }

    #[test]
    fn test_binary_builder_full() {
        let part = FormPart::binary_builder(vec![1u8, 2, 3])
            .filename(Filename::new("document.pdf").unwrap())
            .content_type(ContentType::application("pdf"))
            .build();

        assert_matches!(part, BinaryFormPart { data, filename: Some(f), content_type: Some(ct) } => {
            assert_eq!(data.as_ref(), &[1u8, 2, 3]);
            assert_eq!(f.as_str(), "document.pdf");
            assert_eq!(ct.as_str(), "application/pdf");
        });
    }

    #[test]
    fn test_text_builder_minimal() {
        let part = FormPart::text_builder("hello").build();

        assert_matches!(part, TextFormPart { value, content_type: None } => {
            assert_eq!(value, "hello");
        });
    }

    #[test]
    fn test_text_builder_with_content_type() {
        let part = FormPart::text_builder("{}")
            .content_type(ContentType::json())
            .build();

        assert_matches!(part, TextFormPart { value, content_type: Some(ct) } => {
            assert_eq!(value, "{}");
            assert_eq!(ct.as_str(), "application/json");
        });
    }

    #[test]
    fn test_builder_into_form_part() {
        // Test that builders can be converted directly to FormPart
        let part: FormPart = FormPart::binary_builder(vec![1u8, 2, 3])
            .filename(Filename::new("test.bin").unwrap())
            .into_form_part();
        assert_matches!(
            part,
            FormPart::Binary(BinaryFormPart {
                filename: Some(_),
                ..
            })
        );

        let part: FormPart = FormPart::text_builder("hello")
            .content_type(ContentType::json())
            .into_form_part();
        assert_matches!(
            part,
            FormPart::Text(TextFormPart {
                content_type: Some(_),
                ..
            })
        );
    }

    #[test]
    fn test_inner_types_into_form_part() {
        // Test that BinaryFormPart and TextFormPart can be converted to FormPart via From
        let binary = BinaryFormPart::new(vec![1u8, 2, 3]);
        let part: FormPart = binary.into();
        assert_matches!(part, FormPart::Binary(_));

        let text = TextFormPart::new("hello");
        let part: FormPart = text.into();
        assert_matches!(part, FormPart::Text(_));
    }
}
