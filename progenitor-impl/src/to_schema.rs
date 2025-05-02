// Copyright 2024 Oxide Computer Company

use indexmap::IndexMap;
use openapiv3::AnySchema;
use schemars::schema::SingleOrVec;
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

impl Convert<schemars::schema::Schema> for openapiv3::ReferenceOr<openapiv3::Schema> {
    fn convert(&self) -> schemars::schema::Schema {
        match self {
            openapiv3::ReferenceOr::Reference { reference } => {
                schemars::schema::SchemaObject::new_ref(reference.clone()).into()
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
        // require chasing references. For each subschema we can then make it
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
            extensions,
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
        let extensions = extensions.into_iter().collect();

        match &self.schema_kind {
            openapiv3::SchemaKind::Type(openapiv3::Type::String(openapiv3::StringType {
                format,
                pattern,
                enumeration,
                min_length,
                max_length,
            })) => schemars::schema::SchemaObject {
                metadata,
                instance_type: instance_type(schemars::schema::InstanceType::String, nullable),
                format: format.convert(),
                enum_values: enumeration.convert(),
                string: Some(Box::new(schemars::schema::StringValidation {
                    max_length: max_length.convert(),
                    min_length: min_length.convert(),
                    pattern: pattern.clone(),
                }))
                .reduce(),
                extensions,
                ..Default::default()
            },
            openapiv3::SchemaKind::Type(openapiv3::Type::Number(openapiv3::NumberType {
                format,
                multiple_of,
                exclusive_minimum,
                exclusive_maximum,
                minimum,
                maximum,
                enumeration,
            })) => {
                let (maximum, exclusive_maximum) = match (maximum, exclusive_maximum) {
                    (v, true) => (None, *v),
                    (v, false) => (*v, None),
                };
                let (minimum, exclusive_minimum) = match (minimum, exclusive_minimum) {
                    (v, true) => (None, *v),
                    (v, false) => (*v, None),
                };
                schemars::schema::SchemaObject {
                    metadata,
                    instance_type: instance_type(schemars::schema::InstanceType::Number, nullable),
                    format: format.convert(),
                    enum_values: enumeration.convert(),
                    number: Some(Box::new(schemars::schema::NumberValidation {
                        multiple_of: *multiple_of,
                        maximum,
                        exclusive_maximum,
                        minimum,
                        exclusive_minimum,
                    }))
                    .reduce(),
                    extensions,
                    ..Default::default()
                }
            }

            openapiv3::SchemaKind::Type(openapiv3::Type::Integer(openapiv3::IntegerType {
                format,
                multiple_of,
                exclusive_minimum,
                exclusive_maximum,
                minimum,
                maximum,
                enumeration,
            })) => {
                let (maximum, exclusive_maximum) = match (maximum, exclusive_maximum) {
                    (Some(v), true) => (None, Some(*v as f64)),
                    (Some(v), false) => (Some(*v as f64), None),
                    (None, _) => (None, None),
                };
                let (minimum, exclusive_minimum) = match (minimum, exclusive_minimum) {
                    (Some(v), true) => (None, Some(*v as f64)),
                    (Some(v), false) => (Some(*v as f64), None),
                    (None, _) => (None, None),
                };
                schemars::schema::SchemaObject {
                    metadata,
                    instance_type: instance_type(schemars::schema::InstanceType::Integer, nullable),
                    format: format.convert(),
                    enum_values: enumeration.convert(),
                    number: Some(Box::new(schemars::schema::NumberValidation {
                        multiple_of: multiple_of.map(|v| v as f64).convert(),
                        maximum,
                        exclusive_maximum,
                        minimum,
                        exclusive_minimum,
                    }))
                    .reduce(),
                    extensions,
                    ..Default::default()
                }
            }

            openapiv3::SchemaKind::Type(openapiv3::Type::Object(openapiv3::ObjectType {
                properties,
                required,
                additional_properties,
                min_properties,
                max_properties,
            })) => schemars::schema::SchemaObject {
                metadata,
                instance_type: instance_type(schemars::schema::InstanceType::Object, nullable),
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
                extensions,
                ..Default::default()
            },

            openapiv3::SchemaKind::Type(openapiv3::Type::Array(openapiv3::ArrayType {
                items,
                min_items,
                max_items,
                unique_items,
            })) => schemars::schema::SchemaObject {
                metadata,
                instance_type: instance_type(schemars::schema::InstanceType::Array, nullable),
                array: Some(Box::new(schemars::schema::ArrayValidation {
                    items: items.as_ref().map(|items| {
                        schemars::schema::SingleOrVec::Single(Box::new(items.convert()))
                    }),
                    additional_items: None,
                    max_items: max_items.convert(),
                    min_items: min_items.convert(),
                    unique_items: if *unique_items { Some(true) } else { None },
                    contains: None,
                }))
                .reduce(),
                extensions,
                ..Default::default()
            },

            openapiv3::SchemaKind::Type(openapiv3::Type::Boolean(openapiv3::BooleanType {
                enumeration,
            })) => schemars::schema::SchemaObject {
                metadata,
                instance_type: instance_type(schemars::schema::InstanceType::Boolean, nullable),
                enum_values: enumeration.convert(),
                extensions,
                ..Default::default()
            },

            openapiv3::SchemaKind::OneOf { one_of } => oneof_nullable_wrapper(
                schemars::schema::SchemaObject {
                    metadata,
                    subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                        one_of: Some(one_of.convert()),
                        ..Default::default()
                    })),
                    extensions,
                    ..Default::default()
                },
                nullable,
            ),

            openapiv3::SchemaKind::AllOf { all_of } => oneof_nullable_wrapper(
                schemars::schema::SchemaObject {
                    metadata,
                    subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                        all_of: Some(all_of.convert()),
                        ..Default::default()
                    })),
                    extensions,
                    ..Default::default()
                },
                nullable,
            ),

            openapiv3::SchemaKind::AnyOf { any_of } => oneof_nullable_wrapper(
                schemars::schema::SchemaObject {
                    metadata,
                    subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                        any_of: Some(any_of.convert()),
                        ..Default::default()
                    })),
                    extensions,
                    ..Default::default()
                },
                nullable,
            ),

            openapiv3::SchemaKind::Not { not } => oneof_nullable_wrapper(
                schemars::schema::SchemaObject {
                    metadata,
                    subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                        not: Some(Box::new(not.convert())),
                        ..Default::default()
                    })),
                    extensions,
                    ..Default::default()
                },
                nullable,
            ),

            // This is the permissive schema that allows anything to match.
            // We can ignore `nullable` because--sure--null matches already.
            openapiv3::SchemaKind::Any(AnySchema {
                typ: None,
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
                enumeration,
                min_length: None,
                max_length: None,
                one_of,
                all_of,
                any_of,
                not: None,
            }) if properties.is_empty()
                && required.is_empty()
                && enumeration.is_empty()
                && one_of.is_empty()
                && all_of.is_empty()
                && any_of.is_empty() =>
            {
                schemars::schema::SchemaObject {
                    metadata,
                    extensions,
                    ..Default::default()
                }
            }

            // A simple null value. (We can ignore `nullable` as it would be
            // redundant)
            openapiv3::SchemaKind::Any(AnySchema {
                typ: None,
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
                enumeration,
                min_length: None,
                max_length: None,
                one_of,
                all_of,
                any_of,
                not: None,
            }) if properties.is_empty()
                && required.is_empty()
                && enumeration.len() == 1
                && enumeration[0] == serde_json::Value::Null
                && one_of.is_empty()
                && all_of.is_empty()
                && any_of.is_empty() =>
            {
                schemars::schema::SchemaObject {
                    metadata,
                    instance_type: Some(schemars::schema::InstanceType::Null.into()),
                    extensions,
                    ..Default::default()
                }
            }

            openapiv3::SchemaKind::Any(AnySchema {
                typ,
                pattern,
                multiple_of,
                exclusive_minimum,
                exclusive_maximum,
                minimum,
                maximum,
                properties,
                required,
                additional_properties,
                min_properties,
                max_properties,
                items,
                min_items,
                max_items,
                unique_items,
                enumeration,
                format,
                min_length,
                max_length,
                one_of,
                all_of,
                any_of,
                not,
            }) => {
                let mut so = schemars::schema::SchemaObject {
                    metadata,
                    extensions,
                    ..Default::default()
                };

                // General
                if let Some(format) = format {
                    so.format = Some(format.clone());
                }
                if !enumeration.is_empty() {
                    so.enum_values = Some(enumeration.clone());
                }

                // String
                if let Some(pattern) = pattern {
                    so.string().pattern = Some(pattern.clone());
                }
                if let Some(min_length) = min_length {
                    so.string().min_length = Some(*min_length as u32);
                }
                if let Some(max_length) = max_length {
                    so.string().max_length = Some(*max_length as u32);
                }

                // Number
                if let Some(multiple_of) = multiple_of {
                    so.number().multiple_of = Some(*multiple_of);
                }
                match (minimum, exclusive_minimum) {
                    (None, Some(true)) => {
                        todo!("exclusive_minimum set without minimum");
                    }
                    (None, _) => (),
                    (Some(minimum), Some(true)) => {
                        so.number().exclusive_minimum = Some(*minimum);
                    }
                    (Some(minimum), _) => {
                        so.number().minimum = Some(*minimum);
                    }
                }
                match (maximum, exclusive_maximum) {
                    (None, Some(true)) => {
                        todo!("exclusive_maximum set without maximum");
                    }
                    (None, _) => (),
                    (Some(maximum), Some(true)) => {
                        so.number().exclusive_maximum = Some(*maximum);
                    }
                    (Some(maximum), _) => {
                        so.number().maximum = Some(*maximum);
                    }
                }

                // Object
                if !properties.is_empty() {
                    so.object().properties = properties.convert();
                }
                if !required.is_empty() {
                    so.object().required = required.convert();
                }
                if additional_properties.is_some() {
                    so.object().additional_properties = additional_properties.convert();
                }
                if let Some(min_properties) = min_properties {
                    so.object().min_properties = Some(*min_properties as u32);
                }
                if let Some(max_properties) = max_properties {
                    so.object().max_properties = Some(*max_properties as u32);
                }

                // Array
                if items.is_some() {
                    so.array().items = items.convert().clone().map(SingleOrVec::Single);
                }
                if let Some(min_items) = min_items {
                    so.array().min_items = Some(*min_items as u32);
                }
                if let Some(max_items) = max_items {
                    so.array().max_items = Some(*max_items as u32);
                }
                if let Some(unique_items) = unique_items {
                    so.array().unique_items = Some(*unique_items);
                }

                // Subschemas
                if !one_of.is_empty() {
                    so.subschemas().one_of = one_of.convert();
                }
                if !all_of.is_empty() {
                    so.subschemas().all_of = all_of.convert();
                }
                if !any_of.is_empty() {
                    so.subschemas().any_of = any_of.convert();
                }
                if not.is_some() {
                    so.subschemas().not = not.convert();
                }

                // We do this last so we can infer types if none are specified.
                match (typ.as_deref(), nullable) {
                    (Some("boolean"), _) => {
                        so.instance_type =
                            instance_type(schemars::schema::InstanceType::Boolean, nullable);
                    }
                    (Some("object"), _) => {
                        so.instance_type =
                            instance_type(schemars::schema::InstanceType::Object, nullable);
                    }
                    (Some("array"), _) => {
                        so.instance_type =
                            instance_type(schemars::schema::InstanceType::Array, nullable);
                    }
                    (Some("number"), _) => {
                        so.instance_type =
                            instance_type(schemars::schema::InstanceType::Number, nullable);
                    }
                    (Some("string"), _) => {
                        so.instance_type =
                            instance_type(schemars::schema::InstanceType::String, nullable);
                    }
                    (Some("integer"), _) => {
                        so.instance_type =
                            instance_type(schemars::schema::InstanceType::Integer, nullable);
                    }

                    (Some(typ), _) => todo!("invalid type: {}", typ),

                    // No types
                    (None, false) => (),

                    // We only try to infer types if we need to add in an
                    // additional null type; otherwise we can leave the type
                    // implied.
                    (None, true) => {
                        let instance_types = [
                            so.object
                                .is_some()
                                .then_some(schemars::schema::InstanceType::Object),
                            so.array
                                .is_some()
                                .then_some(schemars::schema::InstanceType::Array),
                            so.number
                                .is_some()
                                .then_some(schemars::schema::InstanceType::Array),
                            so.string
                                .is_some()
                                .then_some(schemars::schema::InstanceType::Array),
                            nullable.then_some(schemars::schema::InstanceType::Null),
                        ]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>();

                        // TODO we could also look at enumerated values to
                        // infer a type.

                        so.instance_type = match (instance_types.first(), instance_types.len()) {
                            (Some(typ), 1) => Some(SingleOrVec::Single(Box::new(*typ))),
                            (Some(_), _) => Some(SingleOrVec::Vec(instance_types)),
                            (None, _) => None,
                        };
                    }
                };

                // If we have exactly one type, and it's null, and we have
                // subschemas that means that we must have had a bunch of
                // subschemas *and* nullable = true. In such a case, we remove
                // the type and inject the oneof wrapper.
                match &so.instance_type {
                    Some(SingleOrVec::Single(it))
                        if **it == schemars::schema::InstanceType::Null
                            && so.subschemas.is_some() =>
                    {
                        oneof_nullable_wrapper(
                            schemars::schema::SchemaObject {
                                instance_type: None,
                                ..so
                            },
                            nullable,
                        )
                    }
                    _ => so,
                }
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

impl Convert<Value> for Option<f64> {
    fn convert(&self) -> Value {
        match self {
            Some(value) => Value::Number(serde_json::Number::from_f64(*value).unwrap()),
            None => Value::Null,
        }
    }
}
impl Convert<Value> for Option<i64> {
    fn convert(&self) -> Value {
        match self {
            Some(value) => Value::Number(serde_json::Number::from(*value)),
            None => Value::Null,
        }
    }
}
impl Convert<Value> for Option<bool> {
    fn convert(&self) -> Value {
        match self {
            Some(value) => Value::Bool(*value),
            None => Value::Null,
        }
    }
}

fn instance_type(
    instance_type: schemars::schema::InstanceType,
    nullable: bool,
) -> Option<schemars::schema::SingleOrVec<schemars::schema::InstanceType>> {
    if nullable {
        Some(vec![instance_type, schemars::schema::InstanceType::Null].into())
    } else {
        Some(instance_type.into())
    }
}

fn oneof_nullable_wrapper(
    mut schema: schemars::schema::SchemaObject,
    nullable: bool,
) -> schemars::schema::SchemaObject {
    if nullable {
        let metadata = schema.metadata;
        let extensions = schema.extensions;

        schema.metadata = None;
        schema.extensions = Default::default();

        schemars::schema::SchemaObject {
            metadata,
            extensions,
            subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                one_of: Some(vec![
                    schemars::schema::SchemaObject {
                        instance_type: Some(schemars::schema::InstanceType::Null.into()),
                        ..Default::default()
                    }
                    .into(),
                    schema.into(),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        }
    } else {
        schema
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
            openapiv3::AdditionalProperties::Any(b) => schemars::schema::Schema::Bool(*b),
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::to_schema::Convert;

    #[test]
    fn test_null() {
        let schema_value = json!({ "enum": [null] });
        let oa_schema = serde_json::from_value::<openapiv3::Schema>(schema_value).unwrap();

        let schema = oa_schema.convert();
        assert_eq!(
            schema.into_object().instance_type,
            Some(schemars::schema::SingleOrVec::Single(Box::new(
                schemars::schema::InstanceType::Null
            )))
        );
    }

    #[test]
    fn test_weird_integer() {
        let schema_value = json!({
            "type": "integer",
            "minimum": 0.0,
        });
        let oa_schema = serde_json::from_value::<openapiv3::Schema>(schema_value.clone()).unwrap();
        let js_schema = serde_json::from_value::<schemars::schema::Schema>(schema_value).unwrap();

        let conv_schema = oa_schema.convert();
        assert_eq!(conv_schema, js_schema);
    }

    #[test]
    fn test_object_no_type() {
        let schema_value = json!({
            "properties": {
                "foo": {}
            }
        });
        let oa_schema = serde_json::from_value::<openapiv3::Schema>(schema_value.clone()).unwrap();
        let js_schema = serde_json::from_value::<schemars::schema::Schema>(schema_value).unwrap();

        let conv_schema = oa_schema.convert();
        assert_eq!(conv_schema, js_schema);
    }

    #[test]
    fn test_array_no_type() {
        let schema_value = json!({
            "items": {}
        });
        let oa_schema = serde_json::from_value::<openapiv3::Schema>(schema_value.clone()).unwrap();
        let js_schema = serde_json::from_value::<schemars::schema::Schema>(schema_value).unwrap();

        let conv_schema = oa_schema.convert();
        assert_eq!(conv_schema, js_schema);
    }

    #[test]
    fn test_number_no_type() {
        let schema_value = json!({
            "minimum": 100.0
        });
        let oa_schema = serde_json::from_value::<openapiv3::Schema>(schema_value.clone()).unwrap();
        let js_schema = serde_json::from_value::<schemars::schema::Schema>(schema_value).unwrap();

        let conv_schema = oa_schema.convert();
        assert_eq!(conv_schema, js_schema);
    }

    #[test]
    fn test_solo_enum() {
        let schema_value = json!({
            "enum": ["one"]
        });
        let oa_schema = serde_json::from_value::<openapiv3::Schema>(schema_value.clone()).unwrap();
        let js_schema = serde_json::from_value::<schemars::schema::Schema>(schema_value).unwrap();

        let conv_schema = oa_schema.convert();
        assert_eq!(conv_schema, js_schema);
    }
}
