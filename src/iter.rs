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
    R: IntoIterator<Item = L::Item>,
{
    type Item = L::Item;
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
    &'a R: IntoIterator<Item = <&'a L as IntoIterator>::Item>,
{
    type Item = <&'a L as IntoIterator>::Item;
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
    &'a mut R: IntoIterator<Item = <&'a mut L as IntoIterator>::Item>,
{
    type Item = <&'a mut L as IntoIterator>::Item;
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
    R: Iterator<Item = L::Item>,
{
    type Item = L::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Left(ref mut x) => x.next(),
            Right(ref mut x) => x.next(),
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
            Left(x) => x.last(),
            Right(x) => x.last(),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Left(ref mut x) => x.nth(n),
            Right(ref mut x) => x.nth(n),
        }
    }

    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        match self.0 {
            Left(x) => x.fold(init, f),
            Right(x) => x.fold(init, f),
        }
    }

    #[inline]
    fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item),
    {
        match self.0 {
            Left(x) => x.for_each(f),
            Right(x) => x.for_each(f),
        }
    }

    #[inline]
    fn collect<B: FromIterator<Self::Item>>(self) -> B {
        match self.0 {
            Left(x) => x.collect(),
            Right(x) => x.collect(),
        }
    }

    #[inline]
    fn partition<B, F>(self, f: F) -> (B, B)
    where
        B: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> bool,
    {
        match self.0 {
            Left(x) => x.partition(f),
            Right(x) => x.partition(f),
        }
    }

    #[inline]
    fn all<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        match self.0 {
            Left(ref mut x) => x.all(f),
            Right(ref mut x) => x.all(f),
        }
    }

    #[inline]
    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        match self.0 {
            Left(ref mut x) => x.any(f),
            Right(ref mut x) => x.any(f),
        }
    }

    #[inline]
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        match self.0 {
            Left(ref mut x) => x.find(predicate),
            Right(ref mut x) => x.find(predicate),
        }
    }

    #[inline]
    fn find_map<B, F>(&mut self, f: F) -> Option<B>
    where
        F: FnMut(Self::Item) -> Option<B>,
    {
        match self.0 {
            Left(ref mut x) => x.find_map(f),
            Right(ref mut x) => x.find_map(f),
        }
    }

    #[inline]
    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        match self.0 {
            Left(ref mut x) => x.position(predicate),
            Right(ref mut x) => x.position(predicate),
        }
    }

    #[inline]
    fn max(self) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        match self.0 {
            Left(x) => x.max(),
            Right(x) => x.max(),
        }
    }

    #[inline]
    fn min(self) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        match self.0 {
            Left(x) => x.min(),
            Right(x) => x.min(),
        }
    }

    #[inline]
    fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> B,
    {
        match self.0 {
            Left(x) => x.max_by_key(f),
            Right(x) => x.max_by_key(f),
        }
    }

    #[inline]
    fn max_by<F>(self, compare: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> core::cmp::Ordering,
    {
        match self.0 {
            Left(x) => x.max_by(compare),
            Right(x) => x.max_by(compare),
        }
    }

    #[inline]
    fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> B,
    {
        match self.0 {
            Left(x) => x.min_by_key(f),
            Right(x) => x.min_by_key(f),
        }
    }

    #[inline]
    fn min_by<F>(self, compare: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> core::cmp::Ordering,
    {
        match self.0 {
            Left(x) => x.min_by(compare),
            Right(x) => x.min_by(compare),
        }
    }

    #[inline]
    fn sum<S>(self) -> S
    where
        S: core::iter::Sum<Self::Item>,
    {
        match self.0 {
            Left(x) => x.sum(),
            Right(x) => x.sum(),
        }
    }

    #[inline]
    fn product<P>(self) -> P
    where
        P: core::iter::Product<Self::Item>,
    {
        match self.0 {
            Left(x) => x.product(),
            Right(x) => x.product(),
        }
    }

    #[inline]
    fn cmp<I>(self, other: I) -> core::cmp::Ordering
    where
        I: IntoIterator<Item = Self::Item>,
        Self::Item: Ord,
    {
        match self.0 {
            Left(x) => x.cmp(other),
            Right(x) => x.cmp(other),
        }
    }

    #[inline]
    fn partial_cmp<I>(self, other: I) -> Option<core::cmp::Ordering>
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
    {
        match self.0 {
            Left(x) => x.partial_cmp(other),
            Right(x) => x.partial_cmp(other),
        }
    }

    #[inline]
    fn eq<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
    {
        match self.0 {
            Left(x) => x.eq(other),
            Right(x) => x.eq(other),
        }
    }

    #[inline]
    fn is_sorted_by<F>(self, compare: F) -> bool
    where
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        match self.0 {
            Left(x) => x.is_sorted_by(compare),
            Right(x) => x.is_sorted_by(compare),
        }
    }

    #[inline]
    fn is_sorted_by_key<F, K>(self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> K,
        K: PartialOrd,
    {
        match self.0 {
            Left(x) => x.is_sorted_by_key(f),
            Right(x) => x.is_sorted_by_key(f),
        }
    }
}

impl<L, R> DoubleEndedIterator for IntoIter<L, R>
where
    L: DoubleEndedIterator,
    R: DoubleEndedIterator<Item = L::Item>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            Left(ref mut x) => x.next_back(),
            Right(ref mut x) => x.next_back(),
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self.0 {
            Left(ref mut x) => x.nth_back(n),
            Right(ref mut x) => x.nth_back(n),
        }
    }

    #[inline]
    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        match self.0 {
            Left(x) => x.rfold(init, f),
            Right(x) => x.rfold(init, f),
        }
    }

    #[inline]
    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        match self.0 {
            Left(ref mut x) => x.rfind(predicate),
            Right(ref mut x) => x.rfind(predicate),
        }
    }
}

impl<L, R> ExactSizeIterator for IntoIter<L, R>
where
    L: ExactSizeIterator,
    R: ExactSizeIterator<Item = L::Item>,
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
    R: FusedIterator<Item = L::Item>,
{
}
