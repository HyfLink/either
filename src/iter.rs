//! This module implements the [`IntoIterator`] trait for [`Either`],
//! if and only if, both `L` and `R` are [`Iterator`]s.
//!
//! Also, the [`IntoIterator`] trait is implemented for `&Either` and
//! `&mut Either`.

use core::iter::FusedIterator;

use crate::Either::{self, Left, Right};

impl<L, R> IntoIterator for Either<L, R>
where
    L: IntoIterator,
    R: IntoIterator,
{
    type Item = Either<L::Item, R::Item>;
    type IntoIter = IntoIter<L::IntoIter, R::IntoIter>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(match self {
            Left(x) => Left(x.into_iter()),
            Right(x) => Right(x.into_iter()),
        })
    }
}

impl<'a, L, R> IntoIterator for &'a Either<L, R>
where
    &'a L: IntoIterator,
    &'a R: IntoIterator,
{
    type Item = Either<<&'a L as IntoIterator>::Item, <&'a R as IntoIterator>::Item>;
    type IntoIter = IntoIter<<&'a L as IntoIterator>::IntoIter, <&'a R as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(match self {
            Left(x) => Left(x.into_iter()),
            Right(x) => Right(x.into_iter()),
        })
    }
}

impl<'a, L, R> IntoIterator for &'a mut Either<L, R>
where
    &'a mut L: IntoIterator,
    &'a mut R: IntoIterator,
{
    type Item = Either<<&'a mut L as IntoIterator>::Item, <&'a mut R as IntoIterator>::Item>;
    type IntoIter =
        IntoIter<<&'a mut L as IntoIterator>::IntoIter, <&'a mut R as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(match self {
            Left(x) => Left(x.into_iter()),
            Right(x) => Right(x.into_iter()),
        })
    }
}

impl<A, L, R> Extend<A> for Either<L, R>
where
    L: Extend<A>,
    R: Extend<A>,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        match self {
            Left(x) => x.extend(iter),
            Right(x) => x.extend(iter),
        }
    }
}

/// An iterator that is either `L` or `R`.
///
/// This `struct` can be constructed by
/// <code>\<[Either<L, R>] as [IntoIterator]\>::into_iter</code>
/// (which requires both `L` and `R` implement [`IntoIterator`]).
#[derive(Clone, Copy, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IntoIter<L, R>(pub(crate) Either<L, R>);

impl<L, R> IntoIter<L, R> {
    /// Returns the contained [`Either<L, R>`] iterator.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Either<L, R> {
        self.0
    }
}

impl<L, R> Iterator for IntoIter<L, R>
where
    L: Iterator,
    R: Iterator,
{
    type Item = Either<L::Item, R::Item>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Left(ref mut x) => x.next().map(Left),
            Right(ref mut x) => x.next().map(Right),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            Left(ref x) => x.size_hint(),
            Right(ref x) => x.size_hint(),
        }
    }

    #[inline]
    fn count(self) -> usize {
        match self.0 {
            Left(x) => x.count(),
            Right(x) => x.count(),
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        match self.0 {
            Left(x) => x.last().map(Left),
            Right(x) => x.last().map(Right),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Left(ref mut x) => x.nth(n).map(Left),
            Right(ref mut x) => x.nth(n).map(Right),
        }
    }
}

impl<L, R> DoubleEndedIterator for IntoIter<L, R>
where
    L: DoubleEndedIterator,
    R: DoubleEndedIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            Left(ref mut x) => x.next_back().map(Left),
            Right(ref mut x) => x.next_back().map(Right),
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Left(ref mut x) => x.nth_back(n).map(Left),
            Right(ref mut x) => x.nth_back(n).map(Right),
        }
    }
}

impl<L, R> ExactSizeIterator for IntoIter<L, R>
where
    L: ExactSizeIterator,
    R: ExactSizeIterator,
{
    #[inline]
    fn len(&self) -> usize {
        match self.0 {
            Left(ref x) => x.len(),
            Right(ref x) => x.len(),
        }
    }
}

impl<L, R> FusedIterator for IntoIter<L, R>
where
    L: FusedIterator,
    R: FusedIterator,
{
}
