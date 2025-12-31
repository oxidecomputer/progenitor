// Copyright 2025 Oxide Computer Company

//! Shared code used by both reqwest and gloo-net backends.

use serde::{ser::SerializeStruct, Serialize};

/// Information about an operation, consumed by hook implementations.
pub struct OperationInfo {
    /// The corresponding operationId from the source OpenAPI document.
    pub operation_id: &'static str,
}

// See https://url.spec.whatwg.org/#url-path-segment-string
const PATH_SET: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b'%');

#[doc(hidden)]
/// Percent encode input string.
pub fn encode_path(pc: &str) -> String {
    percent_encoding::utf8_percent_encode(pc, PATH_SET).to_string()
}

#[doc(hidden)]
pub struct QueryParam<'a, T> {
    name: &'a str,
    value: &'a T,
}

impl<'a, T> QueryParam<'a, T> {
    #[doc(hidden)]
    pub fn new(name: &'a str, value: &'a T) -> Self {
        Self { name, value }
    }
}

impl<T> Serialize for QueryParam<'_, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, inner: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serializer = QuerySerializer {
            inner,
            name: self.name,
        };
        self.value.serialize(serializer)
    }
}

pub(crate) struct QuerySerializer<'a, S> {
    inner: S,
    name: &'a str,
}

macro_rules! serialize_scalar {
    ($f:ident, $t:ty) => {
        fn $f(self, v: $t) -> Result<Self::Ok, Self::Error> {
            [(self.name, v)].serialize(self.inner)
        }
    };
}

impl<'a, S> serde::Serializer for QuerySerializer<'a, S>
where
    S: serde::Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = QuerySeq<'a, S::SerializeSeq>;
    type SerializeTuple = S::SerializeTuple;
    type SerializeTupleStruct = S::SerializeTupleStruct;
    type SerializeTupleVariant = S::SerializeTupleVariant;
    type SerializeMap = S::SerializeMap;
    type SerializeStruct = S::SerializeStruct;
    type SerializeStructVariant = S::SerializeStructVariant;

    serialize_scalar!(serialize_bool, bool);
    serialize_scalar!(serialize_i8, i8);
    serialize_scalar!(serialize_i16, i16);
    serialize_scalar!(serialize_i32, i32);
    serialize_scalar!(serialize_i64, i64);
    serialize_scalar!(serialize_u8, u8);
    serialize_scalar!(serialize_u16, u16);
    serialize_scalar!(serialize_u32, u32);
    serialize_scalar!(serialize_u64, u64);
    serialize_scalar!(serialize_f32, f32);
    serialize_scalar!(serialize_f64, f64);
    serialize_scalar!(serialize_char, char);
    serialize_scalar!(serialize_str, &str);

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // Serialize the value through self which will proxy into the inner
        // Serializer as appropriate.
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        // A query parameter with a list of enumerated values will produce an
        // enum with unit variants. We treat these as scalar values, ignoring
        // the unit variant wrapper.
        variant.serialize(self)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // As with serde_json, we treat a newtype variant like a struct with a
        // single field. This may seem a little weird, but if an OpenAPI
        // document were to specify a query parameter whose schema was a oneOf
        // whose elements were objects with a single field, the user would end
        // up with an enum like this as a parameter.
        let mut map = self.inner.serialize_struct(name, 1)?;
        map.serialize_field(variant, value)?;
        map.end()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let Self { inner, name, .. } = self;
        Ok(QuerySeq {
            inner: inner.serialize_seq(len)?,
            name,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.inner.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.inner.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.inner
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.inner.serialize_map(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.inner.serialize_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.inner
            .serialize_struct_variant(name, variant_index, variant, len)
    }
}

#[doc(hidden)]
pub struct QuerySeq<'a, S> {
    inner: S,
    name: &'a str,
}

impl<S> serde::ser::SerializeSeq for QuerySeq<'_, S>
where
    S: serde::ser::SerializeSeq,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let v = (self.name, value);
        self.inner.serialize_element(&v)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}
