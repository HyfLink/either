//! This module implements the following trait for [`Either`]:
//!
//! - [`Deref`] and [`DerefMut`]
//! - [`Index`] and [`IndexMut`]
//! - [`Neg`]
//! - [`Not`]
//! - [`Add`] and [`AddAssign`]
//! - [`Sub`] and [`SubAssign`]
//! - [`Mul`] and [`MulAssign`]
//! - [`Div`] and [`DivAssign`]
//! - [`Rem`] and [`RemAssign`]
//! - [`Shl`] and [`ShlAssign`]
//! - [`Shr`] and [`ShrAssign`]
//! - [`BitAnd`] and [`BitAndAssign`]
//! - [`BitOr`] and [`BitOrAssign`]
//! - [`BitXor`] and [`BitXorAssign`]
//!
//! if and only if, both `L` and `R` implement the corresponding trait.

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
    DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::Either::{self, Left, Right};

impl<L, R> Deref for Either<L, R>
where
    L: Deref,
    R: Deref<Target = L::Target>,
{
    type Target = L::Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Left(x) => x,
            Right(x) => x,
        }
    }
}

impl<L, R> DerefMut for Either<L, R>
where
    L: DerefMut,
    R: DerefMut<Target = L::Target>,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Left(x) => x,
            Right(x) => x,
        }
    }
}

impl<L, R, Idx> Index<Idx> for Either<L, R>
where
    L: Index<Idx>,
    R: Index<Idx, Output = L::Output>,
{
    type Output = L::Output;

    #[inline]
    fn index(&self, index: Idx) -> &Self::Output {
        match self {
            Left(x) => x.index(index),
            Right(x) => x.index(index),
        }
    }
}

impl<L, R, Idx> IndexMut<Idx> for Either<L, R>
where
    L: IndexMut<Idx>,
    R: IndexMut<Idx, Output = L::Output>,
{
    #[inline]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        match self {
            Left(x) => x.index_mut(index),
            Right(x) => x.index_mut(index),
        }
    }
}

impl<L, R> Neg for Either<L, R>
where
    L: Neg,
    R: Neg,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Left(x) => Left(x.neg()),
            Right(x) => Right(x.neg()),
        }
    }
}

impl<L, R> Not for Either<L, R>
where
    L: Not,
    R: Not,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Left(x) => Left(x.not()),
            Right(x) => Right(x.not()),
        }
    }
}

impl<L, R, T> Add<T> for Either<L, R>
where
    L: Add<T>,
    R: Add<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.add(rhs)),
            Right(x) => Right(x.add(rhs)),
        }
    }
}

impl<L, R, T> Sub<T> for Either<L, R>
where
    L: Sub<T>,
    R: Sub<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.sub(rhs)),
            Right(x) => Right(x.sub(rhs)),
        }
    }
}

impl<L, R, T> Mul<T> for Either<L, R>
where
    L: Mul<T>,
    R: Mul<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.mul(rhs)),
            Right(x) => Right(x.mul(rhs)),
        }
    }
}

impl<L, R, T> Div<T> for Either<L, R>
where
    L: Div<T>,
    R: Div<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.div(rhs)),
            Right(x) => Right(x.div(rhs)),
        }
    }
}

impl<L, R, T> Rem<T> for Either<L, R>
where
    L: Rem<T>,
    R: Rem<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn rem(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.rem(rhs)),
            Right(x) => Right(x.rem(rhs)),
        }
    }
}

impl<L, R, T> Shl<T> for Either<L, R>
where
    L: Shl<T>,
    R: Shl<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn shl(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.shl(rhs)),
            Right(x) => Right(x.shl(rhs)),
        }
    }
}

impl<L, R, T> Shr<T> for Either<L, R>
where
    L: Shr<T>,
    R: Shr<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn shr(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.shr(rhs)),
            Right(x) => Right(x.shr(rhs)),
        }
    }
}

impl<L, R, T> BitAnd<T> for Either<L, R>
where
    L: BitAnd<T>,
    R: BitAnd<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn bitand(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.bitand(rhs)),
            Right(x) => Right(x.bitand(rhs)),
        }
    }
}

impl<L, R, T> BitOr<T> for Either<L, R>
where
    L: BitOr<T>,
    R: BitOr<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn bitor(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.bitor(rhs)),
            Right(x) => Right(x.bitor(rhs)),
        }
    }
}

impl<L, R, T> BitXor<T> for Either<L, R>
where
    L: BitXor<T>,
    R: BitXor<T>,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn bitxor(self, rhs: T) -> Self::Output {
        match self {
            Left(x) => Left(x.bitxor(rhs)),
            Right(x) => Right(x.bitxor(rhs)),
        }
    }
}

impl<L, R, T> AddAssign<T> for Either<L, R>
where
    L: AddAssign<T>,
    R: AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.add_assign(rhs),
            Right(x) => x.add_assign(rhs),
        }
    }
}

impl<L, R, T> SubAssign<T> for Either<L, R>
where
    L: SubAssign<T>,
    R: SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.sub_assign(rhs),
            Right(x) => x.sub_assign(rhs),
        }
    }
}

impl<L, R, T> MulAssign<T> for Either<L, R>
where
    L: MulAssign<T>,
    R: MulAssign<T>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.mul_assign(rhs),
            Right(x) => x.mul_assign(rhs),
        }
    }
}

impl<L, R, T> DivAssign<T> for Either<L, R>
where
    L: DivAssign<T>,
    R: DivAssign<T>,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.div_assign(rhs),
            Right(x) => x.div_assign(rhs),
        }
    }
}

impl<L, R, T> RemAssign<T> for Either<L, R>
where
    L: RemAssign<T>,
    R: RemAssign<T>,
{
    #[inline]
    fn rem_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.rem_assign(rhs),
            Right(x) => x.rem_assign(rhs),
        }
    }
}

impl<L, R, T> ShlAssign<T> for Either<L, R>
where
    L: ShlAssign<T>,
    R: ShlAssign<T>,
{
    #[inline]
    fn shl_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.shl_assign(rhs),
            Right(x) => x.shl_assign(rhs),
        }
    }
}

impl<L, R, T> ShrAssign<T> for Either<L, R>
where
    L: ShrAssign<T>,
    R: ShrAssign<T>,
{
    #[inline]
    fn shr_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.shr_assign(rhs),
            Right(x) => x.shr_assign(rhs),
        }
    }
}

impl<L, R, T> BitAndAssign<T> for Either<L, R>
where
    L: BitAndAssign<T>,
    R: BitAndAssign<T>,
{
    #[inline]
    fn bitand_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.bitand_assign(rhs),
            Right(x) => x.bitand_assign(rhs),
        }
    }
}

impl<L, R, T> BitOrAssign<T> for Either<L, R>
where
    L: BitOrAssign<T>,
    R: BitOrAssign<T>,
{
    #[inline]
    fn bitor_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.bitor_assign(rhs),
            Right(x) => x.bitor_assign(rhs),
        }
    }
}

impl<L, R, T> BitXorAssign<T> for Either<L, R>
where
    L: BitXorAssign<T>,
    R: BitXorAssign<T>,
{
    #[inline]
    fn bitxor_assign(&mut self, rhs: T) {
        match self {
            Left(x) => x.bitxor_assign(rhs),
            Right(x) => x.bitxor_assign(rhs),
        }
    }
}
