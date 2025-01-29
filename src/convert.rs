//! This module implements the [`AsRef`], [`AsMut`], [`From`] traits for
//! [`Either`].
//!
//! # Working with reference
//!
//! [`Either<L, R>`] implements [`AsRef<T>`] (or [`AsMut<T>`]), if and only if
//! both `L` and `R` implements [`AsRef<T>`] (or [`AsMut<T>`]).
//!
//! # Factorize the variant
//!
//! The [`From`] trait allows to
//!
//! - convert from `Either<(T, L), (T, R)>` to `(T, Either<L, R>)`
//! - convert from `Either<(L, T), (R, T)>` to `(Either<L, R>, T)`
//! - convert from `Either<Option<L>, Option<R>>` to `Option<Either<L, R>>`
//! - convert from `Either<Result<T, L>, Result<T, R>>` to `Result<T, Either<L, R>>`
//! - convert from `Either<Result<L, E>, Result<R, E>>` to `Result<Either<L, R>, E>`
//! - transpose between `Either<Result<T1, E1>, Result<T2, E2>>` and `Result<Either<T1, T2>, Either<E1, E2>>`

use crate::Either::{self, Left, Right};

impl<L, R, T: ?Sized> AsRef<T> for Either<L, R>
where
    L: AsRef<T>,
    R: AsRef<T>,
{
    #[inline]
    fn as_ref(&self) -> &T {
        match self {
            Left(x) => x.as_ref(),
            Right(x) => x.as_ref(),
        }
    }
}

impl<L, R, T: ?Sized> AsMut<T> for Either<L, R>
where
    L: AsMut<T>,
    R: AsMut<T>,
{
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        match self {
            Left(x) => x.as_mut(),
            Right(x) => x.as_mut(),
        }
    }
}

impl<L, R, T> From<Either<(T, L), (T, R)>> for (T, Either<L, R>) {
    #[inline]
    fn from(value: Either<(T, L), (T, R)>) -> Self {
        match value {
            Left((x, y)) => (x, Left(y)),
            Right((x, y)) => (x, Right(y)),
        }
    }
}

impl<L, R, T> From<Either<(L, T), (R, T)>> for (Either<L, R>, T) {
    #[inline]
    fn from(value: Either<(L, T), (R, T)>) -> Self {
        match value {
            Left((x, y)) => (Left(x), y),
            Right((x, y)) => (Right(x), y),
        }
    }
}

impl<L, R> From<Either<Option<L>, Option<R>>> for Option<Either<L, R>> {
    #[inline]
    fn from(value: Either<Option<L>, Option<R>>) -> Self {
        match value {
            Left(Some(x)) => Some(Left(x)),
            Right(Some(x)) => Some(Right(x)),
            Left(None) | Right(None) => None,
        }
    }
}

impl<L, R, T> From<Either<Result<T, L>, Result<T, R>>> for Result<T, Either<L, R>> {
    #[inline]
    fn from(value: Either<Result<T, L>, Result<T, R>>) -> Self {
        match value {
            Left(Ok(x)) | Right(Ok(x)) => Ok(x),
            Left(Err(x)) => Err(Left(x)),
            Right(Err(x)) => Err(Right(x)),
        }
    }
}

impl<L, R, E> From<Either<Result<L, E>, Result<R, E>>> for Result<Either<L, R>, E> {
    #[inline]
    fn from(value: Either<Result<L, E>, Result<R, E>>) -> Self {
        match value {
            Left(Ok(x)) => Ok(Left(x)),
            Right(Ok(x)) => Ok(Right(x)),
            Left(Err(x)) | Right(Err(x)) => Err(x),
        }
    }
}

impl<T1, T2, E1, E2> From<Either<Result<T1, E1>, Result<T2, E2>>>
    for Result<Either<T1, T2>, Either<E1, E2>>
{
    #[inline]
    fn from(value: Either<Result<T1, E1>, Result<T2, E2>>) -> Self {
        match value {
            Left(Ok(x)) => Ok(Left(x)),
            Left(Err(x)) => Err(Left(x)),
            Right(Ok(x)) => Ok(Right(x)),
            Right(Err(x)) => Err(Right(x)),
        }
    }
}

impl<T1, T2, E1, E2> From<Result<Either<T1, T2>, Either<E1, E2>>>
    for Either<Result<T1, E1>, Result<T2, E2>>
{
    #[inline]
    fn from(value: Result<Either<T1, T2>, Either<E1, E2>>) -> Self {
        match value {
            Ok(Left(x)) => Left(Ok(x)),
            Ok(Right(x)) => Right(Ok(x)),
            Err(Left(x)) => Left(Err(x)),
            Err(Right(x)) => Right(Err(x)),
        }
    }
}
