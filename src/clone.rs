//! This module implements the [`Clone`] trait for [`Either`].

use crate::Either::{self, Left, Right};

impl<L, R> Clone for Either<L, R>
where
    L: Clone,
    R: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Left(x) => Left(x.clone()),
            Right(x) => Right(x.clone()),
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Left(x), Left(y)) => x.clone_from(y),
            (Right(x), Right(y)) => x.clone_from(y),
            (x, y) => *x = y.clone(),
        }
    }
}
