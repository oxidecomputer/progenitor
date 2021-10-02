use indexmap::IndexMap;
use openapiv3::AnySchema;
use serde_json::Value;

pub trait ToSchema {
    fn to_schema(&self) -> schemars::schema::Schema;
}

trait Convert<T> {
    fn convert(&self) -> T;
}

impl ToSchema for openapiv3::Schema {
    fn to_schema(&self) -> schemars::schema::Schema {
        self.convert()
    }
}

impl ToSchema for openapiv3::ReferenceOr<openapiv3::Schema> {
    fn to_schema(&self) -> schemars::schema::Schema {
        self.convert()
    }
}

impl<I, T> Convert<Vec<T>> for Vec<I>
where
    I: Convert<T>,
{
    fn convert(&self) -> Vec<T> {
        self.iter().map(Convert::convert).collect()
    }
}

impl<I, T> Convert<Option<Vec<T>>> for Vec<I>
where
    I: Convert<T>,
{
    fn convert(&self) -> Option<Vec<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self.iter().map(Convert::convert).collect())
        }
    }
}

impl Convert<schemars::schema::Schema>
    for openapiv3::ReferenceOr<openapiv3::Schema>
{
    fn convert(&self) -> schemars::schema::Schema {
        match self {
            openapiv3::ReferenceOr::Reference { reference } => {
                schemars::schema::SchemaObject::new_ref(reference.clone())
                    .into()
            }
            openapiv3::ReferenceOr::Item(schema) => schema.convert(),
        }
    }
}

impl<T, TT> Convert<TT> for openapiv3::ReferenceOr<Box<T>>
where
    openapiv3::ReferenceOr<T>: Convert<TT>,
    T: Clone,
{
    fn convert(&self) -> TT {
        self.clone().unbox().convert()
    }
}

impl Convert<schemars::schema::Schema> for openapiv3::Schema {
    fn convert(&self) -> schemars::schema::Schema {
        // TODO the discriminator field is used in a way that seems both
        // important and unfortunately redundant. It corresponds to the same
        // regime as internally tagged enums in the serde sense: a field that
        // the discriminator defines is used to determine which schema is
        // valid. This can base used in two ways:

        // 1. It can be used within a struct to identify a particular, required
        // field. This is typically done on a "base" class in an OOP hierarchy.
        // Child class structs "extend" that base class by using an allOf
        // construction where the parent is one of the subschemas. To recognize
        // this case, we need to check all subschemas in an allOf to see if any
        // of them have a discriminator. If they do, we can simply construct an
        // additional structure for the allOf that has a fixed value for that
        // field.

        // 2. It can be used within a oneOf or anyOf schema to determine which
        // subschema is relevant. This is easier to detect because it doesn't
        // required chasing references. For each subschema we can then make it
        // an allOf union of the actual subschema along with a fixed-field
        // structure.

        let openapiv3::SchemaData {
            nullable,
            discriminator: _, // TODO: see above
            external_docs: _, // TODO: append to description?

            title,
            description,
            default,
            deprecated,
            read_only,
            write_only,
            example,
        } = self.schema_data.clone();

        let metadata = schemars::schema::Metadata {
            id: None,
            title,
            description,
            default,
            deprecated,
            read_only,
            write_only,
            examples: example.into_iter().collect::<Vec<_>>(),
        };

        let metadata = Some(Box::new(metadata)).reduce();

        match &self.schema_kind {
            openapiv3::SchemaKind::Type(openapiv3::Type::String(
                openapiv3::StringType {
                    format,
                    pattern,
                    enumeration,
                    min_length,
                    max_length,
                },
            )) => schemars::schema::SchemaObject {
                metadata,
                instance_type: instance_type(
                    schemars::schema::InstanceType::String,
                    nullable,
                ),
                format: format.convert(),
                enum_values: enumeration.convert(),
                string: Some(Box::new(schemars::schema::StringValidation {
                    max_length: max_length.convert(),
                    min_length: min_length.convert(),
                    pattern: pattern.clone(),
                }))
                .reduce(),
                ..Default::default()
            },
            openapiv3::SchemaKind::Type(openapiv3::Type::Number(
                openapiv3::NumberType {
                    format,
                    multiple_of,
                    exclusive_minimum,
                    exclusive_maximum,
                    minimum,
                    maximum,
                    enumeration,
                },
            )) => {
                let (maximum, exclusive_maximum) =
                    match (maximum, exclusive_maximum) {
                        (None, _) => (None, None),
                        (Some(v), false) => (Some(*v), None),
                        (Some(v), true) => (None, Some(*v)),
                    };
                let (minimum, exclusive_minimum) =
                    match (minimum, exclusive_minimum) {
                        (None, _) => (None, None),
                        (Some(v), false) => (Some(*v), None),
                        (Some(v), true) => (None, Some(*v)),
                    };
                schemars::schema::SchemaObject {
                    metadata,
                    instance_type: instance_type(
                        schemars::schema::InstanceType::Number,
                        nullable,
                    ),
                    format: format.convert(),
                    enum_values: enumeration.convert(),
                    number: Some(Box::new(
                        schemars::schema::NumberValidation {
                            multiple_of: multiple_of.convert(),
                            maximum,
                            exclusive_maximum,
                            minimum,
                            exclusive_minimum,
                        },
                    ))
                    .reduce(),
                    ..Default::default()
                }
            }

            openapiv3::SchemaKind::Type(openapiv3::Type::Integer(
                openapiv3::IntegerType {
                    format,
                    multiple_of,
                    exclusive_minimum,
                    exclusive_maximum,
                    minimum,
                    maximum,
                    enumeration,
                },
            )) => {
                let (maximum, exclusive_maximum) =
                    match (maximum, exclusive_maximum) {
                        (None, _) => (None, None),
                        (Some(v), false) => (Some(*v as f64), None),
                        (Some(v), true) => (None, Some(*v as f64)),
                    };
                let (minimum, exclusive_minimum) =
                    match (minimum, exclusive_minimum) {
                        (None, _) => (None, None),
                        (Some(v), false) => (Some(*v as f64), None),
                        (Some(v), true) => (None, Some(*v as f64)),
                    };
                schemars::schema::SchemaObject {
                    metadata,
                    instance_type: instance_type(
                        schemars::schema::InstanceType::Integer,
                        nullable,
                    ),
                    format: format.convert(),
                    enum_values: enumeration.convert(),
                    number: Some(Box::new(
                        schemars::schema::NumberValidation {
                            multiple_of: multiple_of
                                .map(|v| v as f64)
                                .convert(),
                            maximum,
                            exclusive_maximum,
                            minimum,
                            exclusive_minimum,
                        },
                    ))
                    .reduce(),
                    ..Default::default()
                }
            }

            openapiv3::SchemaKind::Type(openapiv3::Type::Object(
                openapiv3::ObjectType {
                    properties,
                    required,
                    additional_properties,
                    min_properties,
                    max_properties,
                },
            )) => schemars::schema::SchemaObject {
                metadata,
                instance_type: instance_type(
                    schemars::schema::InstanceType::Object,
                    nullable,
                ),
                object: Some(Box::new(schemars::schema::ObjectValidation {
                    max_properties: max_properties.convert(),
                    min_properties: min_properties.convert(),
                    required: required.convert(),
                    properties: properties.convert(),
                    pattern_properties: schemars::Map::default(),
                    additional_properties: additional_properties.convert(),
                    property_names: None,
                }))
                .reduce(),
                ..Default::default()
            },

            openapiv3::SchemaKind::Type(openapiv3::Type::Array(
                openapiv3::ArrayType {
                    items,
                    min_items,
                    max_items,
                    unique_items,
                },
            )) => schemars::schema::SchemaObject {
                metadata,
                instance_type: instance_type(
                    schemars::schema::InstanceType::Array,
                    nullable,
                ),
                array: Some(Box::new(schemars::schema::ArrayValidation {
                    items: items.as_ref().map(|items| {
                        schemars::schema::SingleOrVec::Single(Box::new(
                            items.convert(),
                        ))
                    }),
                    additional_items: None,
                    max_items: max_items.convert(),
                    min_items: min_items.convert(),
                    unique_items: if *unique_items { Some(true) } else { None },
                    contains: None,
                }))
                .reduce(),
                ..Default::default()
            },

            openapiv3::SchemaKind::Type(openapiv3::Type::Boolean {}) => {
                schemars::schema::SchemaObject {
                    metadata,
                    instance_type: instance_type(
                        schemars::schema::InstanceType::Boolean,
                        nullable,
                    ),
                    ..Default::default()
                }
            }

            openapiv3::SchemaKind::OneOf { one_of } => {
                schemars::schema::SchemaObject {
                    subschemas: Some(Box::new(
                        schemars::schema::SubschemaValidation {
                            one_of: Some(one_of.convert()),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }
            }

            openapiv3::SchemaKind::AllOf { all_of } => {
                schemars::schema::SchemaObject {
                    subschemas: Some(Box::new(
                        schemars::schema::SubschemaValidation {
                            all_of: Some(all_of.convert()),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }
            }

            openapiv3::SchemaKind::AnyOf { any_of } => {
                schemars::schema::SchemaObject {
                    subschemas: Some(Box::new(
                        schemars::schema::SubschemaValidation {
                            any_of: Some(any_of.convert()),
                            ..Default::default()
                        },
                    )),
                    ..Default::default()
                }
            }

            // This is the permissive schema that allows anything to match.
            openapiv3::SchemaKind::Any(AnySchema {
                pattern: None,
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                properties,
                required,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
                items: None,
                min_items: None,
                max_items: None,
                unique_items: None,
                format: None,
            }) if properties.is_empty() && required.is_empty() => {
                schemars::schema::Schema::Bool(true).into_object()
            }

            openapiv3::SchemaKind::Any(_) => {
                panic!("not clear what we could usefully do here {:#?}", self)
            }
        }
        .into()
    }
}

impl<T> Convert<Option<String>> for openapiv3::VariantOrUnknownOrEmpty<T>
where
    T: Convert<String>,
{
    fn convert(&self) -> Option<String> {
        match self {
            openapiv3::VariantOrUnknownOrEmpty::Item(i) => Some(i.convert()),
            openapiv3::VariantOrUnknownOrEmpty::Unknown(s) => Some(s.clone()),
            openapiv3::VariantOrUnknownOrEmpty::Empty => None,
        }
    }
}

impl Convert<String> for openapiv3::StringFormat {
    fn convert(&self) -> String {
        match self {
            openapiv3::StringFormat::Date => "date",
            openapiv3::StringFormat::DateTime => "date-time",
            openapiv3::StringFormat::Password => "password",
            openapiv3::StringFormat::Byte => "byte",
            openapiv3::StringFormat::Binary => "binary",
        }
        .to_string()
    }
}

impl Convert<String> for openapiv3::NumberFormat {
    fn convert(&self) -> String {
        match self {
            openapiv3::NumberFormat::Float => "float",
            openapiv3::NumberFormat::Double => "double",
        }
        .to_string()
    }
}

impl Convert<String> for openapiv3::IntegerFormat {
    fn convert(&self) -> String {
        match self {
            openapiv3::IntegerFormat::Int32 => "int32",
            openapiv3::IntegerFormat::Int64 => "int64",
        }
        .to_string()
    }
}

impl Convert<Value> for Option<String> {
    fn convert(&self) -> Value {
        match self {
            Some(value) => Value::String(value.clone()),
            None => Value::Null,
        }
    }
}

impl Convert<Value> for f64 {
    fn convert(&self) -> Value {
        Value::Number(serde_json::Number::from_f64(*self).unwrap())
    }
}
impl Convert<Value> for i64 {
    fn convert(&self) -> Value {
        Value::Number(serde_json::Number::from(*self))
    }
}

fn instance_type(
    instance_type: schemars::schema::InstanceType,
    nullable: bool,
) -> Option<schemars::schema::SingleOrVec<schemars::schema::InstanceType>> {
    if nullable {
        Some(schemars::schema::SingleOrVec::Vec(vec![
            instance_type,
            schemars::schema::InstanceType::Null,
        ]))
    } else {
        Some(schemars::schema::SingleOrVec::Single(Box::new(
            instance_type,
        )))
    }
}

impl Convert<Option<u32>> for Option<usize> {
    fn convert(&self) -> Option<u32> {
        (*self).map(|m| m as u32)
    }
}

impl Convert<Option<f64>> for Option<f64> {
    fn convert(&self) -> Option<f64> {
        *self
    }
}

impl Convert<schemars::Set<String>> for Vec<String> {
    fn convert(&self) -> schemars::Set<String> {
        self.iter().cloned().collect()
    }
}

impl Convert<schemars::Map<String, schemars::schema::Schema>>
    for IndexMap<String, openapiv3::ReferenceOr<Box<openapiv3::Schema>>>
{
    fn convert(&self) -> schemars::Map<String, schemars::schema::Schema> {
        self.iter().map(|(k, v)| (k.clone(), v.convert())).collect()
    }
}

impl<T, TT> Convert<TT> for Box<T>
where
    T: Convert<TT>,
{
    fn convert(&self) -> TT {
        self.as_ref().convert()
    }
}

impl<T, TT> Convert<Option<Box<TT>>> for Option<T>
where
    T: Convert<TT>,
{
    fn convert(&self) -> Option<Box<TT>> {
        self.as_ref().map(|t| Box::new(t.convert()))
    }
}

impl Convert<schemars::schema::Schema> for openapiv3::AdditionalProperties {
    fn convert(&self) -> schemars::schema::Schema {
        match self {
            openapiv3::AdditionalProperties::Any(b) => {
                schemars::schema::Schema::Bool(*b)
            }
            openapiv3::AdditionalProperties::Schema(schema) => schema.convert(),
        }
    }
}

trait OptionReduce {
    fn reduce(self) -> Self;
}

// If an Option is `Some` of it's default value, we can simplify that to `None`
impl<T> OptionReduce for Option<T>
where
    T: Default + PartialEq + std::fmt::Debug,
{
    fn reduce(self) -> Self {
        match &self {
            Some(s) if s != &T::default() => self,
            _ => None,
        }
    }
}
