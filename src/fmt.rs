//! This module implements the following formatting traits for [`Either`]:
//!
//! - [`Display`]
//! - [`Binary`]
//! - [`Octal`]
//! - [`LowerHex`] and [`UpperHex`]
//! - [`Pointer`]
//! - [`LowerExp`] and [`UpperExp`]
//!
//! if and only if, both `L` and `R` implement the corresponding trait.

use core::fmt::{
    self, Binary, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex, Write,
};

use crate::Either::{self, Left, Right};

impl<L, R> Write for Either<L, R>
where
    L: Write,
    R: Write,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self {
            Left(x) => x.write_str(s),
            Right(x) => x.write_str(s),
        }
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        match self {
            Left(x) => x.write_char(c),
            Right(x) => x.write_char(c),
        }
    }

    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        match self {
            Left(x) => x.write_fmt(args),
            Right(x) => x.write_fmt(args),
        }
    }
}

impl<L, R> Display for Either<L, R>
where
    L: Display,
    R: Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Left(x) => <L as Display>::fmt(x, f),
            Right(x) => <R as Display>::fmt(x, f),
        }
    }
}

impl<L, R> Binary for Either<L, R>
where
    L: Binary,
    R: Binary,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Left(x) => <L as Binary>::fmt(x, f),
            Right(x) => <R as Binary>::fmt(x, f),
        }
    }
}

impl<L, R> Octal for Either<L, R>
where
    L: Octal,
    R: Octal,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Left(x) => <L as Octal>::fmt(x, f),
            Right(x) => <R as Octal>::fmt(x, f),
        }
    }
}

impl<L, R> LowerHex for Either<L, R>
where
    L: LowerHex,
    R: LowerHex,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Left(x) => <L as LowerHex>::fmt(x, f),
            Right(x) => <R as LowerHex>::fmt(x, f),
        }
    }
}

impl<L, R> UpperHex for Either<L, R>
where
    L: UpperHex,
    R: UpperHex,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Left(x) => <L as UpperHex>::fmt(x, f),
            Right(x) => <R as UpperHex>::fmt(x, f),
        }
    }
}

impl<L, R> Pointer for Either<L, R>
where
    L: Pointer,
    R: Pointer,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Left(x) => <L as Pointer>::fmt(x, f),
            Right(x) => <R as Pointer>::fmt(x, f),
        }
    }
}

impl<L, R> LowerExp for Either<L, R>
where
    L: LowerExp,
    R: LowerExp,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Left(x) => <L as LowerExp>::fmt(x, f),
            Right(x) => <R as LowerExp>::fmt(x, f),
        }
    }
}

impl<L, R> UpperExp for Either<L, R>
where
    L: UpperExp,
    R: UpperExp,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Left(x) => <L as UpperExp>::fmt(x, f),
            Right(x) => <R as UpperExp>::fmt(x, f),
        }
    }
}
