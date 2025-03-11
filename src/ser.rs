//! This module implements the [`Serializer`] trait for [`Either`].
//!
//! That means, `Either<L, R>` is serializer, if and only if, both `L` and `R`
//! are serializers.

use core::fmt::Display;

use serde::ser::{
    Error, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

use crate::Either::{self, Left, Right};

impl<L, R> Error for Either<L, R>
where
    L: Error,
    R: Error,
{
    fn custom<T: Display>(msg: T) -> Self {
        Left(L::custom(msg))
    }
}

impl<L, R> Serializer for Either<L, R>
where
    L: Serializer,
    R: Serializer,
{
    type Ok = Either<L::Ok, R::Ok>;
    type Error = Either<L::Error, R::Error>;
    type SerializeSeq = Either<L::SerializeSeq, R::SerializeSeq>;
    type SerializeTuple = Either<L::SerializeTuple, R::SerializeTuple>;
    type SerializeTupleStruct = Either<L::SerializeTupleStruct, R::SerializeTupleStruct>;
    type SerializeTupleVariant = Either<L::SerializeTupleVariant, R::SerializeTupleVariant>;
    type SerializeMap = Either<L::SerializeMap, R::SerializeMap>;
    type SerializeStruct = Either<L::SerializeStruct, R::SerializeStruct>;
    type SerializeStructVariant = Either<L::SerializeStructVariant, R::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_bool(v)),
            Right(x) => Right(x.serialize_bool(v)),
        })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_i8(v)),
            Right(x) => Right(x.serialize_i8(v)),
        })
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_i16(v)),
            Right(x) => Right(x.serialize_i16(v)),
        })
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_i32(v)),
            Right(x) => Right(x.serialize_i32(v)),
        })
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_i64(v)),
            Right(x) => Right(x.serialize_i64(v)),
        })
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_i128(v)),
            Right(x) => Right(x.serialize_i128(v)),
        })
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_u8(v)),
            Right(x) => Right(x.serialize_u8(v)),
        })
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_u16(v)),
            Right(x) => Right(x.serialize_u16(v)),
        })
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_u32(v)),
            Right(x) => Right(x.serialize_u32(v)),
        })
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_u64(v)),
            Right(x) => Right(x.serialize_u64(v)),
        })
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_u128(v)),
            Right(x) => Right(x.serialize_u128(v)),
        })
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_f32(v)),
            Right(x) => Right(x.serialize_f32(v)),
        })
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_f64(v)),
            Right(x) => Right(x.serialize_f64(v)),
        })
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_char(v)),
            Right(x) => Right(x.serialize_char(v)),
        })
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_str(v)),
            Right(x) => Right(x.serialize_str(v)),
        })
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_bytes(v)),
            Right(x) => Right(x.serialize_bytes(v)),
        })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_none()),
            Right(x) => Right(x.serialize_none()),
        })
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Result::from(match self {
            Left(x) => Left(x.serialize_some(value)),
            Right(x) => Right(x.serialize_some(value)),
        })
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_unit()),
            Right(x) => Right(x.serialize_unit()),
        })
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_unit_struct(name)),
            Right(x) => Right(x.serialize_unit_struct(name)),
        })
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_unit_variant(name, variant_index, variant)),
            Right(x) => Right(x.serialize_unit_variant(name, variant_index, variant)),
        })
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Result::from(match self {
            Left(x) => Left(x.serialize_newtype_struct(name, value)),
            Right(x) => Right(x.serialize_newtype_struct(name, value)),
        })
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Result::from(match self {
            Left(x) => Left(x.serialize_newtype_variant(name, variant_index, variant, value)),
            Right(x) => Right(x.serialize_newtype_variant(name, variant_index, variant, value)),
        })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_seq(len)),
            Right(x) => Right(x.serialize_seq(len)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_tuple(len)),
            Right(x) => Right(x.serialize_tuple(len)),
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_tuple_struct(name, len)),
            Right(x) => Right(x.serialize_tuple_struct(name, len)),
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_tuple_variant(name, variant_index, variant, len)),
            Right(x) => Right(x.serialize_tuple_variant(name, variant_index, variant, len)),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_map(len)),
            Right(x) => Right(x.serialize_map(len)),
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_struct(name, len)),
            Right(x) => Right(x.serialize_struct(name, len)),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.serialize_struct_variant(name, variant_index, variant, len)),
            Right(x) => Right(x.serialize_struct_variant(name, variant_index, variant, len)),
        })
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        Result::from(match self {
            Left(x) => Left(x.collect_seq(iter)),
            Right(x) => Right(x.collect_seq(iter)),
        })
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        Result::from(match self {
            Left(x) => Left(x.collect_map(iter)),
            Right(x) => Right(x.collect_map(iter)),
        })
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        Result::from(match self {
            Left(x) => Left(x.collect_str(value)),
            Right(x) => Right(x.collect_str(value)),
        })
    }

    fn is_human_readable(&self) -> bool {
        match self {
            Left(x) => x.is_human_readable(),
            Right(x) => x.is_human_readable(),
        }
    }
}

impl<L, R> SerializeSeq for Either<L, R>
where
    L: SerializeSeq,
    R: SerializeSeq,
{
    type Ok = Either<L::Ok, R::Ok>;
    type Error = Either<L::Error, R::Error>;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Left(x) => x.serialize_element(value).map_err(Left),
            Right(x) => x.serialize_element(value).map_err(Right),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.end()),
            Right(x) => Right(x.end()),
        })
    }
}

impl<L, R> SerializeTuple for Either<L, R>
where
    L: SerializeTuple,
    R: SerializeTuple,
{
    type Ok = Either<L::Ok, R::Ok>;
    type Error = Either<L::Error, R::Error>;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Left(x) => x.serialize_element(value).map_err(Left),
            Right(x) => x.serialize_element(value).map_err(Right),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.end()),
            Right(x) => Right(x.end()),
        })
    }
}

impl<L, R> SerializeTupleStruct for Either<L, R>
where
    L: SerializeTupleStruct,
    R: SerializeTupleStruct,
{
    type Ok = Either<L::Ok, R::Ok>;
    type Error = Either<L::Error, R::Error>;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Left(x) => x.serialize_field(value).map_err(Left),
            Right(x) => x.serialize_field(value).map_err(Right),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.end()),
            Right(x) => Right(x.end()),
        })
    }
}

impl<L, R> SerializeTupleVariant for Either<L, R>
where
    L: SerializeTupleVariant,
    R: SerializeTupleVariant,
{
    type Ok = Either<L::Ok, R::Ok>;
    type Error = Either<L::Error, R::Error>;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Left(x) => x.serialize_field(value).map_err(Left),
            Right(x) => x.serialize_field(value).map_err(Right),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.end()),
            Right(x) => Right(x.end()),
        })
    }
}

impl<L, R> SerializeMap for Either<L, R>
where
    L: SerializeMap,
    R: SerializeMap,
{
    type Ok = Either<L::Ok, R::Ok>;
    type Error = Either<L::Error, R::Error>;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Left(x) => x.serialize_key(key).map_err(Left),
            Right(x) => x.serialize_key(key).map_err(Right),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Left(x) => x.serialize_value(value).map_err(Left),
            Right(x) => x.serialize_value(value).map_err(Right),
        }
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        match self {
            Left(x) => x.serialize_entry(key, value).map_err(Left),
            Right(x) => x.serialize_entry(key, value).map_err(Right),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.end()),
            Right(x) => Right(x.end()),
        })
    }
}

impl<L, R> SerializeStruct for Either<L, R>
where
    L: SerializeStruct,
    R: SerializeStruct,
{
    type Ok = Either<L::Ok, R::Ok>;
    type Error = Either<L::Error, R::Error>;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Left(x) => x.serialize_field(key, value).map_err(Left),
            Right(x) => x.serialize_field(key, value).map_err(Right),
        }
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        match self {
            Left(x) => x.skip_field(key).map_err(Left),
            Right(x) => x.skip_field(key).map_err(Right),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.end()),
            Right(x) => Right(x.end()),
        })
    }
}

impl<L, R> SerializeStructVariant for Either<L, R>
where
    L: SerializeStructVariant,
    R: SerializeStructVariant,
{
    type Ok = Either<L::Ok, R::Ok>;
    type Error = Either<L::Error, R::Error>;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self {
            Left(x) => x.serialize_field(key, value).map_err(Left),
            Right(x) => x.serialize_field(key, value).map_err(Right),
        }
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        match self {
            Left(x) => x.skip_field(key).map_err(Left),
            Right(x) => x.skip_field(key).map_err(Right),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Result::from(match self {
            Left(x) => Left(x.end()),
            Right(x) => Right(x.end()),
        })
    }
}
