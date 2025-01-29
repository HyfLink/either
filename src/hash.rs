//! This module implements the [`Hasher`] and [`BuildHasher`] traits for
//! [`Either`], if and only if, both `L` and `R` implement the corresponding
//! trait.

use core::hash::{BuildHasher, Hasher};

use crate::Either::{self, Left, Right};

impl<L, R> Hasher for Either<L, R>
where
    L: Hasher,
    R: Hasher,
{
    #[inline]
    fn finish(&self) -> u64 {
        match self {
            Left(x) => x.finish(),
            Right(x) => x.finish(),
        }
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        match self {
            Left(x) => x.write(bytes),
            Right(x) => x.write(bytes),
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        match self {
            Left(x) => x.write_u8(i),
            Right(x) => x.write_u8(i),
        }
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        match self {
            Left(x) => x.write_u16(i),
            Right(x) => x.write_u16(i),
        }
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        match self {
            Left(x) => x.write_u32(i),
            Right(x) => x.write_u32(i),
        }
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        match self {
            Left(x) => x.write_u64(i),
            Right(x) => x.write_u64(i),
        }
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        match self {
            Left(x) => x.write_u128(i),
            Right(x) => x.write_u128(i),
        }
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        match self {
            Left(x) => x.write_usize(i),
            Right(x) => x.write_usize(i),
        }
    }

    #[inline]
    fn write_i8(&mut self, i: i8) {
        match self {
            Left(x) => x.write_i8(i),
            Right(x) => x.write_i8(i),
        }
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        match self {
            Left(x) => x.write_i16(i),
            Right(x) => x.write_i16(i),
        }
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        match self {
            Left(x) => x.write_i32(i),
            Right(x) => x.write_i32(i),
        }
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        match self {
            Left(x) => x.write_i64(i),
            Right(x) => x.write_i64(i),
        }
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        match self {
            Left(x) => x.write_i128(i),
            Right(x) => x.write_i128(i),
        }
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        match self {
            Left(x) => x.write_isize(i),
            Right(x) => x.write_isize(i),
        }
    }
}

impl<L, R> BuildHasher for Either<L, R>
where
    L: BuildHasher,
    R: BuildHasher,
{
    type Hasher = Either<L::Hasher, R::Hasher>;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        match self {
            Left(x) => Left(x.build_hasher()),
            Right(x) => Right(x.build_hasher()),
        }
    }
}
