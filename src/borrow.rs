//! This module implements the [`Borrow`] trait for [`Either`], which allows to
//! borrow slice, [`str`], [`CStr`], [`OsStr`] and [`Path`].

use core::borrow::{Borrow, BorrowMut};
use core::ffi::CStr;

#[cfg(feature = "std")]
use std::ffi::OsStr;
#[cfg(feature = "std")]
use std::path::Path;

use crate::Either::{self, Left, Right};

impl<T, L, R> Borrow<[T]> for Either<L, R>
where
    L: Borrow<[T]>,
    R: Borrow<[T]>,
{
    fn borrow(&self) -> &[T] {
        match self {
            Left(x) => x.borrow(),
            Right(x) => x.borrow(),
        }
    }
}

impl<T, L, R> BorrowMut<[T]> for Either<L, R>
where
    L: BorrowMut<[T]>,
    R: BorrowMut<[T]>,
{
    fn borrow_mut(&mut self) -> &mut [T] {
        match self {
            Left(x) => x.borrow_mut(),
            Right(x) => x.borrow_mut(),
        }
    }
}

impl<L, R> Borrow<str> for Either<L, R>
where
    L: Borrow<str>,
    R: Borrow<str>,
{
    fn borrow(&self) -> &str {
        match self {
            Left(x) => x.borrow(),
            Right(x) => x.borrow(),
        }
    }
}

impl<L, R> BorrowMut<str> for Either<L, R>
where
    L: BorrowMut<str>,
    R: BorrowMut<str>,
{
    fn borrow_mut(&mut self) -> &mut str {
        match self {
            Left(x) => x.borrow_mut(),
            Right(x) => x.borrow_mut(),
        }
    }
}

impl<L, R> Borrow<CStr> for Either<L, R>
where
    L: Borrow<CStr>,
    R: Borrow<CStr>,
{
    fn borrow(&self) -> &CStr {
        match self {
            Left(x) => x.borrow(),
            Right(x) => x.borrow(),
        }
    }
}

impl<L, R> BorrowMut<CStr> for Either<L, R>
where
    L: BorrowMut<CStr>,
    R: BorrowMut<CStr>,
{
    fn borrow_mut(&mut self) -> &mut CStr {
        match self {
            Left(x) => x.borrow_mut(),
            Right(x) => x.borrow_mut(),
        }
    }
}

#[cfg(feature = "std")]
impl<L, R> Borrow<OsStr> for Either<L, R>
where
    L: Borrow<OsStr>,
    R: Borrow<OsStr>,
{
    fn borrow(&self) -> &OsStr {
        match self {
            Left(x) => x.borrow(),
            Right(x) => x.borrow(),
        }
    }
}

#[cfg(feature = "std")]
impl<L, R> BorrowMut<OsStr> for Either<L, R>
where
    L: BorrowMut<OsStr>,
    R: BorrowMut<OsStr>,
{
    fn borrow_mut(&mut self) -> &mut OsStr {
        match self {
            Left(x) => x.borrow_mut(),
            Right(x) => x.borrow_mut(),
        }
    }
}

#[cfg(feature = "std")]
impl<L, R> Borrow<Path> for Either<L, R>
where
    L: Borrow<Path>,
    R: Borrow<Path>,
{
    fn borrow(&self) -> &Path {
        match self {
            Left(x) => x.borrow(),
            Right(x) => x.borrow(),
        }
    }
}

#[cfg(feature = "std")]
impl<L, R> BorrowMut<Path> for Either<L, R>
where
    L: BorrowMut<Path>,
    R: BorrowMut<Path>,
{
    fn borrow_mut(&mut self) -> &mut Path {
        match self {
            Left(x) => x.borrow_mut(),
            Right(x) => x.borrow_mut(),
        }
    }
}
