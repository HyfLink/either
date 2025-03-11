//! This module implements the [`Deserializer`] trait for [`Either`].
//!
//! That means, `Either<L, R>` is deserializer, if and only if, both `L` and `R`
//! are deserializers.

use core::fmt::{self, Display, Formatter};

use serde::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, Error, Expected, MapAccess, SeqAccess,
    Unexpected, VariantAccess, Visitor,
};

use crate::Either::{self, Left, Right};

impl<L, R> Error for Either<L, R>
where
    L: Error,
    R: Error,
{
    #[inline]
    fn custom<T: Display>(msg: T) -> Self {
        Left(L::custom(msg))
    }

    #[inline]
    fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Left(L::invalid_type(unexp, exp))
    }

    #[inline]
    fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Left(L::invalid_value(unexp, exp))
    }

    #[inline]
    fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
        Left(L::invalid_length(len, exp))
    }

    #[inline]
    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Left(L::unknown_variant(variant, expected))
    }

    #[inline]
    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Left(L::unknown_field(field, expected))
    }

    #[inline]
    fn missing_field(field: &'static str) -> Self {
        Left(L::missing_field(field))
    }

    #[inline]
    fn duplicate_field(field: &'static str) -> Self {
        Left(L::duplicate_field(field))
    }
}

impl<'de, L, R> Deserializer<'de> for Either<L, R>
where
    L: Deserializer<'de>,
    R: Deserializer<'de>,
{
    type Error = Either<L::Error, R::Error>;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_any(visitor).map_err(Left),
            Right(x) => x.deserialize_any(visitor).map_err(Right),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_bool(visitor).map_err(Left),
            Right(x) => x.deserialize_bool(visitor).map_err(Right),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_i8(visitor).map_err(Left),
            Right(x) => x.deserialize_i8(visitor).map_err(Right),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_i16(visitor).map_err(Left),
            Right(x) => x.deserialize_i16(visitor).map_err(Right),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_i32(visitor).map_err(Left),
            Right(x) => x.deserialize_i32(visitor).map_err(Right),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_i64(visitor).map_err(Left),
            Right(x) => x.deserialize_i64(visitor).map_err(Right),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_i128(visitor).map_err(Left),
            Right(x) => x.deserialize_i128(visitor).map_err(Right),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_u8(visitor).map_err(Left),
            Right(x) => x.deserialize_u8(visitor).map_err(Right),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_u16(visitor).map_err(Left),
            Right(x) => x.deserialize_u16(visitor).map_err(Right),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_u32(visitor).map_err(Left),
            Right(x) => x.deserialize_u32(visitor).map_err(Right),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_u64(visitor).map_err(Left),
            Right(x) => x.deserialize_u64(visitor).map_err(Right),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_u128(visitor).map_err(Left),
            Right(x) => x.deserialize_u128(visitor).map_err(Right),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_f32(visitor).map_err(Left),
            Right(x) => x.deserialize_f32(visitor).map_err(Right),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_f64(visitor).map_err(Left),
            Right(x) => x.deserialize_f64(visitor).map_err(Right),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_char(visitor).map_err(Left),
            Right(x) => x.deserialize_char(visitor).map_err(Right),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_str(visitor).map_err(Left),
            Right(x) => x.deserialize_str(visitor).map_err(Right),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_string(visitor).map_err(Left),
            Right(x) => x.deserialize_string(visitor).map_err(Right),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_bytes(visitor).map_err(Left),
            Right(x) => x.deserialize_bytes(visitor).map_err(Right),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_byte_buf(visitor).map_err(Left),
            Right(x) => x.deserialize_byte_buf(visitor).map_err(Right),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_option(visitor).map_err(Left),
            Right(x) => x.deserialize_option(visitor).map_err(Right),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_unit(visitor).map_err(Left),
            Right(x) => x.deserialize_unit(visitor).map_err(Right),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_unit_struct(name, visitor).map_err(Left),
            Right(x) => x.deserialize_unit_struct(name, visitor).map_err(Right),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_newtype_struct(name, visitor).map_err(Left),
            Right(x) => x.deserialize_newtype_struct(name, visitor).map_err(Right),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_seq(visitor).map_err(Left),
            Right(x) => x.deserialize_seq(visitor).map_err(Right),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_tuple(len, visitor).map_err(Left),
            Right(x) => x.deserialize_tuple(len, visitor).map_err(Right),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_tuple_struct(name, len, visitor).map_err(Left),
            Right(x) => x
                .deserialize_tuple_struct(name, len, visitor)
                .map_err(Right),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_map(visitor).map_err(Left),
            Right(x) => x.deserialize_map(visitor).map_err(Right),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_struct(name, fields, visitor).map_err(Left),
            Right(x) => x.deserialize_struct(name, fields, visitor).map_err(Right),
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_enum(name, variants, visitor).map_err(Left),
            Right(x) => x.deserialize_enum(name, variants, visitor).map_err(Right),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_identifier(visitor).map_err(Left),
            Right(x) => x.deserialize_identifier(visitor).map_err(Right),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.deserialize_ignored_any(visitor).map_err(Left),
            Right(x) => x.deserialize_ignored_any(visitor).map_err(Right),
        }
    }

    fn is_human_readable(&self) -> bool {
        match self {
            Left(x) => x.is_human_readable(),
            Right(x) => x.is_human_readable(),
        }
    }
}

impl<'de, L, R> DeserializeSeed<'de> for Either<L, R>
where
    L: DeserializeSeed<'de>,
    R: DeserializeSeed<'de>,
{
    type Value = Either<L::Value, R::Value>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self {
            Left(x) => x.deserialize(deserializer).map(Left),
            Right(x) => x.deserialize(deserializer).map(Right),
        }
    }
}

impl<'de, L, R> Visitor<'de> for Either<L, R>
where
    L: Visitor<'de>,
    R: Visitor<'de>,
{
    type Value = Either<L::Value, R::Value>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Left(x) => x.expecting(formatter),
            Right(x) => x.expecting(formatter),
        }
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_bool(v).map(Left),
            Right(x) => x.visit_bool(v).map(Right),
        }
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_i8(v).map(Left),
            Right(x) => x.visit_i8(v).map(Right),
        }
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_i16(v).map(Left),
            Right(x) => x.visit_i16(v).map(Right),
        }
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_i32(v).map(Left),
            Right(x) => x.visit_i32(v).map(Right),
        }
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_i64(v).map(Left),
            Right(x) => x.visit_i64(v).map(Right),
        }
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_i128(v).map(Left),
            Right(x) => x.visit_i128(v).map(Right),
        }
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_u8(v).map(Left),
            Right(x) => x.visit_u8(v).map(Right),
        }
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_u16(v).map(Left),
            Right(x) => x.visit_u16(v).map(Right),
        }
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_u32(v).map(Left),
            Right(x) => x.visit_u32(v).map(Right),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_u64(v).map(Left),
            Right(x) => x.visit_u64(v).map(Right),
        }
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_u128(v).map(Left),
            Right(x) => x.visit_u128(v).map(Right),
        }
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_f32(v).map(Left),
            Right(x) => x.visit_f32(v).map(Right),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_f64(v).map(Left),
            Right(x) => x.visit_f64(v).map(Right),
        }
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_char(v).map(Left),
            Right(x) => x.visit_char(v).map(Right),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_str(v).map(Left),
            Right(x) => x.visit_str(v).map(Right),
        }
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_borrowed_str(v).map(Left),
            Right(x) => x.visit_borrowed_str(v).map(Right),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_string(v).map(Left),
            Right(x) => x.visit_string(v).map(Right),
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_bytes(v).map(Left),
            Right(x) => x.visit_bytes(v).map(Right),
        }
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_borrowed_bytes(v).map(Left),
            Right(x) => x.visit_borrowed_bytes(v).map(Right),
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_byte_buf(v).map(Left),
            Right(x) => x.visit_byte_buf(v).map(Right),
        }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_none().map(Left),
            Right(x) => x.visit_none().map(Right),
        }
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self {
            Left(x) => x.visit_some(deserializer).map(Left),
            Right(x) => x.visit_some(deserializer).map(Right),
        }
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match self {
            Left(x) => x.visit_unit().map(Left),
            Right(x) => x.visit_unit().map(Right),
        }
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self {
            Left(x) => x.visit_newtype_struct(deserializer).map(Left),
            Right(x) => x.visit_newtype_struct(deserializer).map(Right),
        }
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        match self {
            Left(x) => x.visit_seq(seq).map(Left),
            Right(x) => x.visit_seq(seq).map(Right),
        }
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        match self {
            Left(x) => x.visit_map(map).map(Left),
            Right(x) => x.visit_map(map).map(Right),
        }
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        match self {
            Left(x) => x.visit_enum(data).map(Left),
            Right(x) => x.visit_enum(data).map(Right),
        }
    }
}

impl<'de, L, R> SeqAccess<'de> for Either<L, R>
where
    L: SeqAccess<'de>,
    R: SeqAccess<'de>,
{
    type Error = Either<L::Error, R::Error>;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self {
            Left(x) => x.next_element_seed(seed).map_err(Left),
            Right(x) => x.next_element_seed(seed).map_err(Right),
        }
    }

    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        match self {
            Left(x) => x.next_element().map_err(Left),
            Right(x) => x.next_element().map_err(Right),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self {
            Left(x) => x.size_hint(),
            Right(x) => x.size_hint(),
        }
    }
}

impl<'de, L, R> MapAccess<'de> for Either<L, R>
where
    L: MapAccess<'de>,
    R: MapAccess<'de>,
{
    type Error = Either<L::Error, R::Error>;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self {
            Left(x) => x.next_key_seed(seed).map_err(Left),
            Right(x) => x.next_key_seed(seed).map_err(Right),
        }
    }

    fn next_key<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        match self {
            Left(x) => x.next_key().map_err(Left),
            Right(x) => x.next_key().map_err(Right),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self {
            Left(x) => x.next_value_seed(seed).map_err(Left),
            Right(x) => x.next_value_seed(seed).map_err(Right),
        }
    }

    fn next_value<T>(&mut self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>,
    {
        match self {
            Left(x) => x.next_value().map_err(Left),
            Right(x) => x.next_value().map_err(Right),
        }
    }

    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        match self {
            Left(x) => x.next_entry_seed(kseed, vseed).map_err(Left),
            Right(x) => x.next_entry_seed(kseed, vseed).map_err(Right),
        }
    }

    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        match self {
            Left(x) => x.next_entry().map_err(Left),
            Right(x) => x.next_entry().map_err(Right),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self {
            Left(x) => x.size_hint(),
            Right(x) => x.size_hint(),
        }
    }
}

impl<'de, L, R> EnumAccess<'de> for Either<L, R>
where
    L: EnumAccess<'de>,
    R: EnumAccess<'de>,
{
    type Error = Either<L::Error, R::Error>;
    type Variant = Either<L::Variant, R::Variant>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self {
            Left(x) => match x.variant_seed(seed) {
                Ok((value, variant)) => Ok((value, Left(variant))),
                Err(err) => Err(Left(err)),
            },
            Right(x) => match x.variant_seed(seed) {
                Ok((value, variant)) => Ok((value, Right(variant))),
                Err(err) => Err(Right(err)),
            },
        }
    }

    fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
    where
        V: Deserialize<'de>,
    {
        match self {
            Left(x) => match x.variant() {
                Ok((value, variant)) => Ok((value, Left(variant))),
                Err(err) => Err(Left(err)),
            },
            Right(x) => match x.variant() {
                Ok((value, variant)) => Ok((value, Right(variant))),
                Err(err) => Err(Right(err)),
            },
        }
    }
}

impl<'de, L, R> VariantAccess<'de> for Either<L, R>
where
    L: VariantAccess<'de>,
    R: VariantAccess<'de>,
{
    type Error = Either<L::Error, R::Error>;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self {
            Left(x) => x.unit_variant().map_err(Left),
            Right(x) => x.unit_variant().map_err(Right),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self {
            Left(x) => x.newtype_variant_seed(seed).map_err(Left),
            Right(x) => x.newtype_variant_seed(seed).map_err(Right),
        }
    }

    fn newtype_variant<T>(self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>,
    {
        match self {
            Left(x) => x.newtype_variant().map_err(Left),
            Right(x) => x.newtype_variant().map_err(Right),
        }
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.tuple_variant(len, visitor).map_err(Left),
            Right(x) => x.tuple_variant(len, visitor).map_err(Right),
        }
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Left(x) => x.struct_variant(fields, visitor).map_err(Left),
            Right(x) => x.struct_variant(fields, visitor).map_err(Right),
        }
    }
}
