//! The `enum` type [`Either`] with variants [`Left`] and [`Right`] is a
//! general purpose sum type with two cases.
//!
//! ```no_run
//! # #[allow(dead_code)]
//! enum Either<L, R> {
//!    Left(L),
//!    Right(R),
//! }
//! ```
//!
//! # Representation
//!
//! [`Either<L, R>`] has the same [representation] with [`Result<L, R>`].
//!
//! But different from [`Result`], which represents success or failure,
//! variants [`Left`] and [`Right`] are symmetric and represent values
//! without preference.
//!
//! [representation]: core::result#representation

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    feature = "nightly",
    feature(
        async_fn_traits,
        async_iterator,
        fn_traits,
        tuple_trait,
        unboxed_closures,
    )
)]

use core::fmt::Debug;
use core::hint::unreachable_unchecked;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr;

use crate::Either::{Left, Right};

pub mod borrow;
pub mod clone;
pub mod convert;
#[cfg(feature = "serde")]
pub mod de;
pub mod error;
pub mod fmt;
pub mod future;
pub mod hash;
#[cfg(feature = "std")]
pub mod io;
pub mod iter;
#[cfg(feature = "nightly")]
pub mod nightly;
pub mod ops;
pub mod option;
pub mod result;
#[cfg(feature = "serde")]
pub mod ser;

/// The `enum` type with variants [`Left`] and [`Right`] is a general purpose
/// sum type with two cases.
///
/// See the [crate-level documentation][crate] for more details.
#[derive(Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Either<L, R> {
    /// Contains a value of type `L` on the left side.
    Left(L),
    /// Contains a value of type `R` on the right side.
    Right(R),
}

impl<T> Either<T, T> {
    /// Returns the contained [`Left`] value or [`Right`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output |
    /// | ---------- | ------ |
    /// | `Left(x)`  | `x`    |
    /// | `Right(x)` | `x`    |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let x: Either<i32, i32> = Left(3);
    /// assert_eq!(x.into_inner(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T {
        match self {
            Left(x) | Right(x) => x,
        }
    }
}

impl<L, R> Either<L, R> {
    // /////////////////////////////////////////////////////////////////////////
    // Querying the contained variant
    // /////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the contained value is [`Left`].
    ///
    /// # Result
    ///
    /// | Input      | Output  |
    /// | ---------- | ------- |
    /// | `Left(x)`  | `true`  |
    /// | `Right(x)` | `false` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let x: Either<i32, i32> = Left(3);
    /// assert!(x.is_left());
    /// ```
    #[inline]
    #[must_use = "if you intended to assert that this has a value, consider `.left_unwrap()` instead"]
    pub const fn is_left(&self) -> bool {
        matches!(self, Left(_))
    }

    /// Returns `true` if the contained value is [`Right`].
    ///
    /// # Result
    ///
    /// | Input      | Output  |
    /// | ---------- | ------- |
    /// | `Left(x)`  | `false` |
    /// | `Right(x)` | `true`  |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Right};
    /// let x: Either<i32, i32> = Right(3);
    /// assert!(x.is_right());
    /// ```
    #[inline]
    #[must_use = "if you intended to assert that this has a value, consider `.right_unwrap()` instead"]
    pub const fn is_right(&self) -> bool {
        matches!(self, Right(_))
    }

    /// Returns `true` if the contained value is [`Left`] and matches the
    /// provided `predicate` function.
    ///
    /// # Result
    ///
    /// | Input      | Output         |
    /// | ---------- | -------------- |
    /// | `Left(x)`  | `predicate(x)` |
    /// | `Right(x)` | `false`        |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let x: Either<i32, i32> = Left(3);
    /// assert!(x.is_left_and(|x| x == 3));
    /// ```
    #[inline]
    #[must_use]
    pub fn is_left_and<F>(self, predicate: F) -> bool
    where
        F: FnOnce(L) -> bool,
    {
        match self {
            Left(x) => predicate(x),
            Right(_) => false,
        }
    }

    /// Returns `true` if the contained value is [`Right`] and matches the
    /// provided `predicate` function.
    ///
    /// # Result
    ///
    /// | Input      | Output         |
    /// | ---------- | -------------- |
    /// | `Left(x)`  | `false`        |
    /// | `Right(x)` | `predicate(x)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Right};
    /// let x: Either<i32, i32> = Right(3);
    /// assert!(x.is_right_and(|x| x == 3));
    /// ```
    #[inline]
    #[must_use]
    pub fn is_right_and<F>(self, predicate: F) -> bool
    where
        F: FnOnce(R) -> bool,
    {
        match self {
            Left(_) => false,
            Right(x) => predicate(x),
        }
    }

    // /////////////////////////////////////////////////////////////////////////
    // Working with references
    // /////////////////////////////////////////////////////////////////////////

    /// Converts from `&Either<L, R>` to `Either<&L, &R>`.
    ///
    /// # Result
    ///
    /// | Input      | Output     |
    /// | ---------- | ---------- |
    /// | `Left(x)`  | `Left(x)`  |
    /// | `Right(x)` | `Right(x)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let x: Either<i32, String> = Left(3);
    /// let y: Either<&i32, &String> = x.as_ref();
    /// # let _ = y;
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Left(x) => Left(x),
            Right(x) => Right(x),
        }
    }

    /// Converts from `&mut Either<L, R>` to `Either<&mut L, &mut R>`.
    ///
    /// # Result
    ///
    /// | Input      | Output     |
    /// | ---------- | ---------- |
    /// | `Left(x)`  | `Left(x)`  |
    /// | `Right(x)` | `Right(x)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let mut x: Either<i32, String> = Left(3);
    /// let y: Either<&mut i32, &mut String> = x.as_mut();
    /// # let _ = y;
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match self {
            Left(x) => Left(x),
            Right(x) => Right(x),
        }
    }

    /// Converts from `&Either<L, R>` to `Either<&L::Target, &R::Target>`.
    ///
    /// # Result
    ///
    /// | Input      | Output             |
    /// | ---------- | ------------------ |
    /// | `Left(x)`  | `Left(x.deref())`  |
    /// | `Right(x)` | `Right(x.deref())` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let x: Either<Vec<u8>, String> = Left(vec![1, 2, 3]);
    /// let y: Either<&[u8], &str> = x.as_deref();
    /// # let _ = y;
    /// ```
    #[inline]
    #[must_use]
    pub fn as_deref(&self) -> Either<&L::Target, &R::Target>
    where
        L: Deref,
        R: Deref,
    {
        match self {
            Left(x) => Left(x),
            Right(x) => Right(x),
        }
    }

    /// Converts from `&Either<L, R>` to `Either<&L::Target, &R::Target>`.
    ///
    /// # Result
    ///
    /// | Input      | Output             |
    /// | ---------- | ------------------ |
    /// | `Left(x)`  | `Left(x.deref())`  |
    /// | `Right(x)` | `Right(x.deref())` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let mut x: Either<Vec<u8>, String> = Left(vec![1, 2, 3]);
    /// let y: Either<&mut [u8], &mut str> = x.as_deref_mut();
    /// # let _ = y;
    /// ```
    #[inline]
    #[must_use]
    pub fn as_deref_mut(&mut self) -> Either<&mut L::Target, &mut R::Target>
    where
        L: DerefMut,
        R: DerefMut,
    {
        match self {
            Left(x) => Left(x),
            Right(x) => Right(x),
        }
    }

    /// Converts from [`Pin<&Either<L, R>>`] to `Either<Pin<&L>, Pin<&R>>`.
    ///
    /// Also see the [`as_pin_ref`] method defined on type [`Option`].
    ///
    /// [`as_pin_ref`]: Option::as_pin_ref
    #[inline]
    #[must_use]
    pub fn as_pin_ref(self: Pin<&Self>) -> Either<Pin<&L>, Pin<&R>> {
        // SAFETY: `x` is guaranteed to be pinned because it comes from `self`
        // which is pinned.
        unsafe {
            match self.get_ref() {
                Left(x) => Left(Pin::new_unchecked(x)),
                Right(x) => Right(Pin::new_unchecked(x)),
            }
        }
    }

    /// Converts from [`Pin<&mut Either<L, R>>`] to `Either<Pin<&mut L>, Pin<&mut R>>`.
    ///
    /// Also see the [`as_pin_mut`] method defined on type [`Option`].
    ///
    /// [`as_pin_mut`]: Option::as_pin_mut
    #[inline]
    #[must_use]
    pub fn as_pin_mut(self: Pin<&mut Self>) -> Either<Pin<&mut L>, Pin<&mut R>> {
        // SAFETY: `get_unchecked_mut` is never used to move the contained value
        // inside `self`. And `x` is guaranteed to be pinned because it comes
        // from `self` which is pinned.
        unsafe {
            match self.get_unchecked_mut() {
                Left(x) => Left(Pin::new_unchecked(x)),
                Right(x) => Right(Pin::new_unchecked(x)),
            }
        }
    }

    /// Returns the immutable reference to the contained [`Left`] value in
    /// [`Some`], otherwise returns [`None`].
    ///
    /// # Result
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `Some(x)` |
    /// | `Right(x)` | `None`    |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let x: Either<i32, u32> = Left(3);
    /// let y: Option<&i32> = x.as_left();
    /// assert_eq!(y, Some(&3));
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_left(&self) -> Option<&L> {
        match self {
            Left(x) => Some(x),
            Right(_) => None,
        }
    }

    /// Returns the immutable reference to the contained [`Right`] value in
    /// [`Some`], otherwise returns [`None`].
    ///
    /// # Result
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `None`    |
    /// | `Right(x)` | `Some(x)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Right};
    /// let x: Either<i32, u32> = Right(5);
    /// let y: Option<&u32> = x.as_right();
    /// assert_eq!(y, Some(&5));
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_right(&self) -> Option<&R> {
        match self {
            Left(_) => None,
            Right(x) => Some(x),
        }
    }

    /// Returns the mutable reference to the contained [`Left`] value in
    /// [`Some`], otherwise returns [`None`].
    ///
    /// # Result
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `Some(x)` |
    /// | `Right(x)` | `None`    |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let mut x: Either<i32, u32> = Left(3);
    /// let y: Option<&mut i32> = x.as_left_mut();
    /// assert_eq!(y.copied(), Some(3));
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_left_mut(&mut self) -> Option<&mut L> {
        match self {
            Left(x) => Some(x),
            Right(_) => None,
        }
    }

    /// Returns the mutable reference to the contained [`Right`] value in
    /// [`Some`], otherwise returns [`None`].
    ///
    /// # Result
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `None`    |
    /// | `Right(x)` | `Some(x)` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Right};
    /// let mut x: Either<i32, u32> = Right(5);
    /// let y: Option<&mut u32> = x.as_right_mut();
    /// assert_eq!(y.copied(), Some(5));
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_right_mut(&mut self) -> Option<&mut R> {
        match self {
            Left(_) => None,
            Right(x) => Some(x),
        }
    }

    /// Returns the immutable reference to the contained [`Left`] value,
    /// without checking that the value is not [`Right`].
    ///
    /// # Safety
    ///
    /// Calling this method on the [`Right`] variant is *undefined behavior*.
    ///
    /// # Result
    ///
    /// | Input      | Output               |
    /// | ---------- | -------------------- |
    /// | `Left(x)`  | `x`                  |
    /// | `Right(x)` | *undefined behavior* |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let x: Either<i32, u32> = Left(3);
    /// assert_eq!(unsafe { x.as_left_unchecked() }, &3);
    /// ```
    #[inline]
    #[must_use]
    pub const unsafe fn as_left_unchecked(&self) -> &L {
        match self {
            Left(x) => x,
            // SAFETY: the safety contract must be upheld by the caller.
            Right(_) => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the immutable reference to the contained [`Right`] value,
    /// without checking that the value is not [`Left`].
    ///
    /// # Safety
    ///
    /// Calling this method on the [`Left`] variant is *undefined behavior*.
    ///
    /// # Result
    ///
    /// | Input      | Output               |
    /// | ---------- | -------------------- |
    /// | `Left(x)`  | *undefined behavior* |
    /// | `Right(x)` | `x`                  |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Right};
    /// let x: Either<i32, u32> = Right(5);
    /// assert_eq!(unsafe { x.as_right_unchecked() }, &5);
    /// ```
    #[inline]
    #[must_use]
    pub const unsafe fn as_right_unchecked(&self) -> &R {
        match self {
            // SAFETY: the safety contract must be upheld by the caller.
            Left(_) => unsafe { unreachable_unchecked() },
            Right(x) => x,
        }
    }

    /// Returns the mutable reference to the contained [`Left`] value,
    /// without checking that the value is not [`Right`].
    ///
    /// # Safety
    ///
    /// Calling this method on the [`Right`] variant is *undefined behavior*.
    ///
    /// # Result
    ///
    /// | Input      | Output               |
    /// | ---------- | -------------------- |
    /// | `Left(x)`  | `x`                  |
    /// | `Right(x)` | *undefined behavior* |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let mut x: Either<i32, u32> = Left(3);
    /// unsafe { *x.as_left_unchecked_mut() = 4 };
    /// assert_eq!(x, Left(4));
    /// ```
    #[inline]
    #[must_use]
    pub const unsafe fn as_left_unchecked_mut(&mut self) -> &mut L {
        match self {
            Left(x) => x,
            // SAFETY: the safety contract must be upheld by the caller.
            Right(_) => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the mutable reference to the contained [`Right`] value,
    /// without checking that the value is not [`Left`].
    ///
    /// # Safety
    ///
    /// Calling this method on the [`Left`] variant is *undefined behavior*.
    ///
    /// # Result
    ///
    /// | Input      | Output               |
    /// | ---------- | -------------------- |
    /// | `Right(x)` | `x`                  |
    /// | `Left(x)`  | *undefined behavior* |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Right};
    /// let mut x: Either<i32, u32> = Right(5);
    /// unsafe { *x.as_right_unchecked_mut() = 4 };
    /// assert_eq!(x, Right(4));
    /// ```
    #[inline]
    #[must_use]
    pub const unsafe fn as_right_unchecked_mut(&mut self) -> &mut R {
        match self {
            // SAFETY: the safety contract must be upheld by the caller.
            Left(_) => unsafe { unreachable_unchecked() },
            Right(x) => x,
        }
    }

    // /////////////////////////////////////////////////////////////////////////
    // Extracting the contained value
    // /////////////////////////////////////////////////////////////////////////

    /// Returns the contained [`Left`] value in [`Some`],
    /// otherwise returns [`None`] and discards the [`Right`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `Some(x)` |
    /// | `Right(x)` | `None`    |
    #[inline]
    #[must_use]
    pub fn left(self) -> Option<L> {
        match self {
            Left(x) => Some(x),
            Right(_) => None,
        }
    }

    /// Returns the contained [`Left`] value in [`Some`],
    /// otherwise returns [`None`] and discards the [`Right`] value.
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `None`    |
    /// | `Right(x)` | `Some(x)` |
    #[inline]
    #[must_use]
    pub fn right(self) -> Option<R> {
        match self {
            Left(_) => None,
            Right(x) => Some(x),
        }
    }

    /// Returns the contained [`Left`] value in [`Ok`],
    /// otherwise returns the contained [`Right`] value in [`Err`].
    ///
    /// # Result
    ///
    /// | Input      | Output   |
    /// | ---------- | -------- |
    /// | `Left(x)`  | `Ok(x)`  |
    /// | `Right(x)` | `Err(x)` |
    #[inline]
    pub fn left_or_err(self) -> Result<L, R> {
        match self {
            Left(x) => Ok(x),
            Right(x) => Err(x),
        }
    }

    /// Returns the contained [`Right`] value in [`Ok`],
    /// otherwise returns the contained [`Left`] value in [`Err`].
    ///
    /// # Result
    ///
    /// | Input      | Output   |
    /// | ---------- | -------- |
    /// | `Left(x)`  | `Err(x)` |
    /// | `Right(x)` | `Ok(x)`  |
    #[inline]
    pub fn right_or_err(self) -> Result<R, L> {
        match self {
            Left(x) => Err(x),
            Right(x) => Ok(x),
        }
    }

    /// Returns the contained [`Left`] value, otherwise converts the [`Right`]
    /// value to `L`.
    ///
    /// # Result
    ///
    /// | Input      | Output     |
    /// | ---------- | ---------- |
    /// | `Left(x)`  | `x`        |
    /// | `Right(x)` | `x.into()` |
    #[inline]
    #[must_use]
    pub fn into_left(self) -> L
    where
        R: Into<L>,
    {
        match self {
            Left(x) => x,
            Right(x) => x.into(),
        }
    }

    /// Returns the contained [`Right`] value,
    /// otherwise converts the [`Left`] value to `L`.
    ///
    /// # Result
    ///
    /// | Input      | Output     |
    /// | ---------- | ---------- |
    /// | `Left(x)`  | `x.into()` |
    /// | `Right(x)` | `x`        |
    #[inline]
    #[must_use]
    pub fn into_right(self) -> R
    where
        L: Into<R>,
    {
        match self {
            Left(x) => x.into(),
            Right(x) => x,
        }
    }

    /// Returns the contained [`Left`] value in [`OK`],
    /// otherwise attempts to convert the [`Right`] value to `L`.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion from `R` to `L` failed.
    ///
    /// # Result
    ///
    /// | Input      | Output         |
    /// | ---------- | -------------- |
    /// | `Left(x)`  | `Ok(x)`        |
    /// | `Right(x)` | `x.try_into()` |
    #[inline]
    pub fn try_into_left(self) -> Result<L, R::Error>
    where
        R: TryInto<L>,
    {
        match self {
            Left(x) => Ok(x),
            Right(x) => x.try_into(),
        }
    }

    /// Returns the contained [`Right`] value in [`OK`],
    /// otherwise attempts to convert the [`Left`] value to `R`.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion from `L` to `R` failed.
    ///
    /// # Result
    ///
    /// | Input      | Output         |
    /// | ---------- | -------------- |
    /// | `Left(x)`  | `x.try_into()` |
    /// | `Right(x)` | `Ok(x)`        |
    #[inline]
    pub fn try_into_right(self) -> Result<R, L::Error>
    where
        L: TryInto<R>,
    {
        match self {
            Left(x) => x.try_into(),
            Right(x) => Ok(x),
        }
    }

    /// Returns the contained [`Left`] value, consuming the [`Either`] value.
    ///
    /// # Panics
    ///
    /// Panics if the value is the [`Right`] variant, with a panic message
    /// including the passed message, and the content of the [`Right`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output      |
    /// | ---------- | ----------- |
    /// | `Left(x)`  | `x`         |
    /// | `Right(x)` | *panicking* |
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn left_expect(self, message: &str) -> L
    where
        R: Debug,
    {
        match self {
            Left(x) => x,
            Right(ref x) => unwrap_failed(message, x),
        }
    }

    /// Returns the contained [`Right`] value, consuming the [`Either`] value.
    ///
    /// # Panics
    ///
    /// Panics if the value is the [`Left`] variant, with a panic message
    /// including the passed message, and the content of the [`Left`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output      |
    /// | ---------- | ----------- |
    /// | `Left(x)`  | *panicking* |
    /// | `Right(x)` | `x`         |
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn right_expect(self, message: &str) -> R
    where
        L: Debug,
    {
        match self {
            Left(ref x) => unwrap_failed(message, x),
            Right(x) => x,
        }
    }

    /// Returns the contained [`Left`] value, consuming the [`Either`] value.
    ///
    /// # Panics
    ///
    /// Panics if the value is the [`Right`] variant, with a panic message
    /// provided by the [`Right`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output      |
    /// | ---------- | ----------- |
    /// | `Left(x)`  | `x`         |
    /// | `Right(x)` | *panicking* |
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn left_unwrap(self) -> L
    where
        R: Debug,
    {
        self.left_expect("`Either::left_unwrap()` is called on `Right`")
    }

    /// Returns the contained [`Right`] value, consuming the [`Either`] value.
    ///
    /// # Panics
    ///
    /// Panics if the value is the [`Left`] variant, with a panic message
    /// provided by the [`Left`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output      |
    /// | ---------- | ----------- |
    /// | `Left(x)`  | *panicking* |
    /// | `Right(x)` | `x`         |
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn right_unwrap(self) -> R
    where
        L: Debug,
    {
        self.right_expect("`Either::right_unwrap()` is called on `Left`")
    }

    /// Returns the contained [`Left`] value, consuming the [`Either`] value,
    /// without checking that the value is not the [`Right`] variant.
    ///
    /// # Safety
    ///
    /// Calling this method on the [`Right`] variant is *undefined behavior*.
    ///
    /// # Result
    ///
    /// | Input      | Output               |
    /// | ---------- | -------------------- |
    /// | `Right(x)` | *undefined behavior* |
    /// | `Left(x)`  | `x`                  |
    #[inline]
    #[must_use]
    #[track_caller]
    pub unsafe fn left_unwrap_unchecked(self) -> L {
        match self {
            Left(x) => x,
            // SAFETY: the safety contract must be upheld by the caller.
            Right(_) => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns the contained [`Right`] value, consuming the [`Either`] value,
    /// without checking that the value is not the [`Left`] variant.
    ///
    /// # Safety
    ///
    /// Calling this method on the [`Left`] variant is *undefined behavior*.
    ///
    /// # Result
    ///
    /// | Input      | Output               |
    /// | ---------- | -------------------- |
    /// | `Left(x)`  | *undefined behavior* |
    /// | `Right(x)` | `x`                  |
    #[inline]
    #[must_use]
    #[track_caller]
    pub unsafe fn right_unwrap_unchecked(self) -> R {
        match self {
            // SAFETY: the safety contract must be upheld by the caller.
            Left(_) => unsafe { unreachable_unchecked() },
            Right(x) => x,
        }
    }

    /// Returns the contained [`Left`] value or a provided `default` value.
    ///
    /// Arguments passed to this function are eagerly evaluated; if you are
    /// passing the result of a function call, it is recommended to use
    /// [`left_unwrap_or_else`], which is lazily evaluated.
    ///
    /// [`left_unwrap_or_else`]: Either::left_unwrap_or_else
    ///
    /// # Result
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `x`       |
    /// | `Right(x)` | `default` |
    #[inline]
    #[must_use]
    pub fn left_unwrap_or(self, default: L) -> L {
        match self {
            Left(x) => x,
            Right(_) => default,
        }
    }

    /// Returns the contained [`Right`] value or a provided `default` value.
    ///
    /// Arguments passed to this function are eagerly evaluated; if you are
    /// passing the result of a function call, it is recommended to use
    /// [`right_unwrap_or_else`], which is lazily evaluated.
    ///
    /// [`right_unwrap_or_else`]: Either::right_unwrap_or_else
    ///
    /// # Result
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `default` |
    /// | `Right(x)` | `x`       |
    #[inline]
    #[must_use]
    pub fn right_unwrap_or(self, default: R) -> R {
        match self {
            Left(_) => default,
            Right(x) => x,
        }
    }

    /// Returns the contained [`Left`] value or the value computed from
    /// function `default`.
    ///
    /// # Result
    ///
    /// | Input      | Output       |
    /// | ---------- | ------------ |
    /// | `Left(x)`  | `x`          |
    /// | `Right(x)` | `default(x)` |
    #[inline]
    #[must_use]
    pub fn left_unwrap_or_else<F>(self, default: F) -> L
    where
        F: FnOnce(R) -> L,
    {
        match self {
            Left(x) => x,
            Right(x) => default(x),
        }
    }

    /// Returns the contained [`Right`] value or the value computed from
    /// function `default`.
    ///
    /// # Result
    ///
    /// | Input      | Output       |
    /// | ---------- | ------------ |
    /// | `Left(x)`  | `default(x)` |
    /// | `Right(x)` | `x`          |
    #[inline]
    #[must_use]
    pub fn right_unwrap_or_else<F>(self, default: F) -> R
    where
        F: FnOnce(L) -> R,
    {
        match self {
            Left(x) => default(x),
            Right(x) => x,
        }
    }

    /// Returns the contained [`Left`] value or the [`Default`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output         |
    /// | ---------- | -------------- |
    /// | `Left(x)`  | `x`            |
    /// | `Right(x)` | `L::default()` |
    #[inline]
    #[must_use]
    pub fn left_unwrap_or_default(self) -> L
    where
        L: Default,
    {
        match self {
            Left(x) => x,
            Right(_) => L::default(),
        }
    }

    /// Returns the contained [`Right`] value or the [`Default`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output         |
    /// | ---------- | -------------- |
    /// | `Right(x)` | `x`            |
    /// | `Left(x)`  | `R::default()` |
    #[inline]
    #[must_use]
    pub fn right_unwrap_or_default(self) -> R
    where
        R: Default,
    {
        match self {
            Left(_) => R::default(),
            Right(x) => x,
        }
    }

    // /////////////////////////////////////////////////////////////////////////
    // Transforming the contained value
    // /////////////////////////////////////////////////////////////////////////

    /// Converts between `Either<L, R>` and `Either<R, L>`.
    ///
    /// # Result
    ///
    /// | Input      | Output     |
    /// | ---------- | ---------- |
    /// | `Left(x)`  | `Right(x)` |
    /// | `Right(x)` | `Left(x)`  |
    #[inline]
    #[must_use]
    pub fn flip(self) -> Either<R, L> {
        match self {
            Left(x) => Right(x),
            Right(x) => Left(x),
        }
    }

    /// Applies one of two functions on the [`Left`] or [`Right`] variant,
    /// unifying the result.
    ///
    /// - If the value is [`Left`], then function `l` is called.
    /// - If it is [`Right`], then function `r` is called.
    ///
    /// # Result
    ///
    /// | Input      | Output |
    /// | ---------- | ------ |
    /// | `Left(x)`  | `f(x)` |
    /// | `Right(x)` | `r(x)` |
    #[inline]
    #[must_use]
    pub fn fold<T, F, G>(self, l: F, r: G) -> T
    where
        F: FnOnce(L) -> T,
        G: FnOnce(R) -> T,
    {
        match self {
            Left(x) => l(x),
            Right(x) => r(x),
        }
    }

    /// Applies one of two functions on the [`Left`] or [`Right`] variant,
    /// with provided `ctx` value, unifying the result.
    ///
    /// - If the value is [`Left`], then function `l` is called.
    /// - If it is [`Right`], then function `r` is called.
    ///
    /// NOTE: this function is the contextual version of the [`fold`] methods.
    ///
    /// [`fold`]: Either::fold
    ///
    /// # Result
    ///
    /// | Input      | Output      |
    /// | ---------- | ----------- |
    /// | `Left(x)`  | `l(x, ctx)` |
    /// | `Right(x)` | `r(x, ctx)` |
    #[inline]
    #[must_use]
    pub fn fold_with<C, T, F, G>(self, ctx: C, l: F, r: G) -> T
    where
        F: FnOnce(L, C) -> T,
        G: FnOnce(R, C) -> T,
    {
        match self {
            Left(x) => l(x, ctx),
            Right(x) => r(x, ctx),
        }
    }

    /// Applies one of two functions on the [`Left`] or [`Right`] variant,
    /// returning the result re-wrapped in [`Left`] or [`Right`].
    ///
    /// - If the value is [`Left`], then function `l` is called.
    /// - If it is [`Right`], then function `r` is called.
    ///
    /// # Result
    ///
    /// | Input      | Output        |
    /// | ---------- | ------------- |
    /// | `Left(x)`  | `Left(f(x))`  |
    /// | `Right(x)` | `Right(r(x))` |
    #[inline]
    #[must_use]
    pub fn map<T, U, F, G>(self, l: F, r: G) -> Either<T, U>
    where
        F: FnOnce(L) -> T,
        G: FnOnce(R) -> U,
    {
        match self {
            Left(x) => Left(l(x)),
            Right(y) => Right(r(y)),
        }
    }

    /// Applies one of two functions on the [`Left`] or [`Right`] variant,
    /// with provided `ctx` value, returning the result re-wrapped in [`Left`]
    /// or [`Right`].
    ///
    /// - If the value is [`Left`], then function `l` is called.
    /// - If it is [`Right`], then function `r` is called.
    ///
    /// # Result
    ///
    /// | Input      | Output             |
    /// | ---------- | ------------------ |
    /// | `Left(x)`  | `Left(l(x, ctx))`  |
    /// | `Right(x)` | `Right(r(x, ctx))` |
    ///
    /// NOTE: this function is the contextual version of the [`map`] methods.
    ///
    /// [`map`]: Either::map
    #[inline]
    #[must_use]
    pub fn map_with<C, T, U, F, G>(self, ctx: C, l: F, r: G) -> Either<T, U>
    where
        F: FnOnce(L, C) -> T,
        G: FnOnce(R, C) -> U,
    {
        match self {
            Left(x) => Left(l(x, ctx)),
            Right(y) => Right(r(y, ctx)),
        }
    }

    /// Applies function `f` on the contained [`Left`] value,
    /// and returns the result re-wrapped in [`Left`].
    /// Otherwise returns the contained [`Right`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output       |
    /// | ---------- | ------------ |
    /// | `Left(x)`  | `Left(f(x))` |
    /// | `Right(x)` | `Right(x)`   |
    #[inline]
    #[must_use]
    pub fn left_map<T, F: FnOnce(L) -> T>(self, f: F) -> Either<T, R> {
        match self {
            Left(x) => Left(f(x)),
            Right(x) => Right(x),
        }
    }

    /// Applies function `f` on the contained [`Right`] value,
    /// and returns the result re-wrapped in [`Right`].
    /// Otherwise returns the contained [`Left`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output        |
    /// | ---------- | ------------- |
    /// | `Left(x)`  | `Left(x)`     |
    /// | `Right(x)` | `Right(f(x))` |
    #[inline]
    #[must_use]
    pub fn right_map<T, F: FnOnce(R) -> T>(self, f: F) -> Either<L, T> {
        match self {
            Left(x) => Left(x),
            Right(x) => Right(f(x)),
        }
    }

    /// Applies function `f` on the contained [`Left`] value,
    /// otherwise returns the provided `default` value.
    ///
    /// Arguments passed to this function are eagerly evaluated; if you are
    /// passing the result of a function call, it is recommended to use
    /// [`fold`] and [`left_map_or_else`], which are lazily evaluated.
    ///
    /// [`fold`]: Either::fold
    /// [`left_map_or_else`]: Either::left_map_or_else
    ///
    /// # Result
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `f(x)`    |
    /// | `Right(x)` | `default` |
    #[inline]
    #[must_use]
    pub fn left_map_or<T, F: FnOnce(L) -> T>(self, default: T, f: F) -> T {
        match self {
            Left(x) => f(x),
            Right(_) => default,
        }
    }

    /// Applies function `f` on the contained [`Right`] value,
    /// otherwise returns the provided `default` value.
    ///
    /// Arguments passed to this function are eagerly evaluated; if you are
    /// passing the result of a function call, it is recommended to use
    /// [`fold`] and [`right_map_or_else`], which are lazily evaluated.
    ///
    /// [`fold`]: Either::fold
    /// [`right_map_or_else`]: Either::right_map_or_else
    ///
    /// # Result
    ///
    /// | Input      | Output    |
    /// | ---------- | --------- |
    /// | `Left(x)`  | `default` |
    /// | `Right(x)` | `f(x)`    |
    #[inline]
    #[must_use]
    pub fn right_map_or<T, F: FnOnce(R) -> T>(self, default: T, f: F) -> T {
        match self {
            Left(_) => default,
            Right(x) => f(x),
        }
    }

    /// Applies function `f` on the contained [`Left`] value,
    /// otherwise returns the value computed from function `default`.
    ///
    /// # Result
    ///
    /// | Input      | Output      |
    /// | ---------- | ----------- |
    /// | `Left(x)`  | `f(x)`      |
    /// | `Right(x)` | `default()` |
    #[inline]
    pub fn left_map_or_else<T, D, F>(self, default: D, f: F) -> T
    where
        D: FnOnce() -> T,
        F: FnOnce(L) -> T,
    {
        match self {
            Left(x) => f(x),
            Right(_) => default(),
        }
    }

    /// Applies function `f` on the contained [`Right`] value,
    /// otherwise returns the value computed from function `default`.
    ///
    /// # Result
    ///
    /// | Input      | Output      |
    /// | ---------- | ----------- |
    /// | `Left(x)`  | `default()` |
    /// | `Right(x)` | `f(x)`      |
    #[inline]
    pub fn right_map_or_else<T, D, F>(self, default: D, f: F) -> T
    where
        D: FnOnce() -> T,
        F: FnOnce(R) -> T,
    {
        match self {
            Left(_) => default(),
            Right(x) => f(x),
        }
    }

    /// Applies function `f` on the contained [`Left`] value,
    /// otherwise returns a [`Default`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output         |
    /// | ---------- | -------------- |
    /// | `Left(x)`  | `f(x)`         |
    /// | `Right(x)` | `T::default()` |
    #[inline]
    #[must_use]
    pub fn left_map_or_default<T, F>(self, f: F) -> T
    where
        T: Default,
        F: FnOnce(L) -> T,
    {
        self.left_map_or_else(T::default, f)
    }

    /// Applies function `f` on the contained [`Right`] value,
    /// otherwise returns a [`Default`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output         |
    /// | ---------- | -------------- |
    /// | `Left(x)`  | `T::default()` |
    /// | `Right(x)` | `f(x)`         |
    #[inline]
    #[must_use]
    pub fn right_map_or_default<T, F>(self, f: F) -> T
    where
        T: Default,
        F: FnOnce(R) -> T,
    {
        self.right_map_or_else(T::default, f)
    }

    // /////////////////////////////////////////////////////////////////////////
    // Boolean operators
    // /////////////////////////////////////////////////////////////////////////

    /// Returns `other` if the `self` value is [`Left`],
    /// otherwise returns the `self` value.
    ///
    /// Arguments passed to this function are eagerly evaluated; if you are
    /// passing the result of a function call, it is recommended to use
    /// [`left_and_then`] (alias `right_or_else`), which is lazily evaluated.
    ///
    /// [`left_and_then`]: Either::left_and_then
    ///   
    /// # Result
    ///
    /// | `self`     | `other`    | Output              |
    /// | ---------- | ---------- | ------------------- |
    /// | `Left(x)`  | `Left(y)`  | `Left(y)`  (`y`)    |
    /// | `Left(x)`  | `Right(y)` | `Right(y)` (`y`)    |
    /// | `Right(x)` | `Left(y)`  | `Right(x)` (`self`) |
    /// | `Right(x)` | `Right(y)` | `Right(x)` (`self`) |
    #[inline]
    #[must_use]
    #[doc(alias = "right_or")]
    pub fn left_and(self, other: Self) -> Self {
        match self {
            Left(_) => other,
            Right(x) => Right(x),
        }
    }

    /// Returns `other` if the `self` value is [`Right`],
    /// otherwise returns the `self` value.
    ///
    /// Arguments passed to this function are eagerly evaluated; if you are
    /// passing the result of a function call, it is recommended to use
    /// [`right_and_then`] (alias `left_or_else`), which is lazily evaluated.
    ///
    /// [`right_and_then`]: Either::right_and_then
    ///   
    /// # Result
    ///
    /// | `self`     | `other`    | Output              |
    /// | ---------- | ---------- | ------------------- |
    /// | `Left(x)`  | `Left(y)`  | `Left(x)` (`self`)  |
    /// | `Left(x)`  | `Right(y)` | `Left(x)` (`self`)  |
    /// | `Right(x)` | `Left(y)`  | `Left(y)` (`y`)     |
    /// | `Right(x)` | `Right(y)` | `Right(y)` (`y`)    |
    #[inline]
    #[must_use]
    #[doc(alias = "left_or")]
    pub fn right_and(self, other: Self) -> Self {
        match self {
            Left(x) => Left(x),
            Right(_) => other,
        }
    }

    /// Applies function `f` on the contained [`Left`] value,
    /// otherwise returns the contained [`Right`] value.
    ///    
    /// # Result
    ///
    /// | `self`     | Output     |
    /// | ---------- | ---------- |
    /// | `Left(x)`  | `f(x)`     |
    /// | `Right(x)` | `Right(x)` |
    #[inline]
    #[must_use]
    #[doc(alias = "right_or_else")]
    pub fn left_and_then<T, F: FnOnce(L) -> Either<T, R>>(self, f: F) -> Either<T, R> {
        match self {
            Left(x) => f(x),
            Right(x) => Right(x),
        }
    }

    /// Applies function `f` on the contained [`Right`] value,
    /// otherwise returns the contained [`Left`] value.
    ///
    /// # Result
    ///
    /// | `self`     | Output     |
    /// | ---------- | ---------- |
    /// | `Left(x)`  | `Left(x)`  |
    /// | `Right(x)` | `f(x)`     |
    #[inline]
    #[must_use]
    #[doc(alias = "left_or_else")]
    pub fn right_and_then<T, F: FnOnce(R) -> Either<L, T>>(self, f: F) -> Either<L, T> {
        match self {
            Left(x) => Left(x),
            Right(x) => f(x),
        }
    }

    // /////////////////////////////////////////////////////////////////////////
    // Modifying the contained value
    // /////////////////////////////////////////////////////////////////////////

    /// Inserts provided `value` into the [`Left`] variant,
    /// then returns a mutable reference to the contained value.
    ///
    /// NOTE: the old value contained in [`Either`] will be dropped.
    #[inline]
    #[must_use = "if you intended to set a value, consider assignment instead"]
    pub fn insert_left(&mut self, value: L) -> &mut L {
        *self = Left(value);
        // SAFETY: the code above just assigned `self` with `Left`.
        unsafe { self.as_left_unchecked_mut() }
    }

    /// Inserts provided `value` into the [`Right`] variant,
    /// then returns a mutable reference to the contained value.
    ///
    /// NOTE: the old value contained in [`Either`] will be dropped.
    #[inline]
    #[must_use = "if you intended to set a value, consider assignment instead"]
    pub fn insert_right(&mut self, value: R) -> &mut R {
        *self = Right(value);
        // SAFETY: the code above just assigned `self` with `Right`.
        unsafe { self.as_right_unchecked_mut() }
    }

    /// Inserts provided `value` into the [`Left`] variant if it is [`Right`],
    /// then returns a mutable reference to the contained value.
    ///
    /// Arguments passed to this function are eagerly evaluated; if you are
    /// passing the result of a function call, it is recommended to use
    /// [`left_or_insert_with`] and [`right_and_modify`], which are lazily
    /// evaluated.
    ///
    /// See also [`insert_left`], which updates the value even if the contained
    /// value is already [`Left`].
    ///
    /// [`left_or_insert_with`]: Either::left_or_insert_with
    /// [`right_and_modify`]: Either::right_and_modify
    /// [`insert_left`]: Either::insert_left
    #[inline]
    pub fn left_or_insert(&mut self, value: L) -> &mut L {
        match self {
            Left(x) => x,
            Right(_) => self.insert_left(value),
        }
    }

    /// Inserts provided `value` into the [`Right`] variant if it is [`Left`],
    /// then returns a mutable reference to the contained value.
    ///
    /// Arguments passed to this function are eagerly evaluated; if you are
    /// passing the result of a function call, it is recommended to use
    /// [`right_or_insert_with`] and [`left_and_modify`], which are lazily
    /// evaluated.
    ///
    /// See also [`insert_right`], which updates the value even if the contained
    /// value is already [`Right`].
    ///
    /// [`right_or_insert_with`]: Either::right_or_insert_with
    /// [`left_and_modify`]: Either::left_and_modify
    /// [`insert_right`]: Either::insert_right
    #[inline]
    pub fn right_or_insert(&mut self, value: R) -> &mut R {
        match self {
            Left(_) => self.insert_right(value),
            Right(x) => x,
        }
    }

    /// Inserts provided `value` into the [`Left`] variant if it is [`Right`],
    /// then returns a mutable reference to the contained value.
    #[inline]
    pub fn left_or_insert_with<F>(&mut self, f: F) -> &mut L
    where
        F: FnOnce(R) -> L,
    {
        match self {
            Left(x) => x,
            // SAFETY: This is safe because:
            // - `self` will be written with some value after it is read.
            // - after `self` is read and before it is written, only `f(x)` is
            //   called. And the function `f` would not access to `self` since
            //   `self` is mutably borrowed.
            // - the code above `as_right_unchecked_mut` assigned `self` with
            //   `Right`.
            Right(x) => unsafe {
                let x = ptr::read(x);
                ptr::write(self, Left(f(x)));
                self.as_left_unchecked_mut()
            },
        }
    }

    /// Inserts provided `value` into the [`Right`] variant if it is [`Left`],
    /// then returns a mutable reference to the contained value.
    #[inline]
    pub fn right_or_insert_with<F>(&mut self, f: F) -> &mut R
    where
        F: FnOnce(L) -> R,
    {
        match self {
            // SAFETY: This is safe because:
            // - `self` will be written with some value after it is read.
            // - after `self` is read and before it is written, only `f(x)` is
            //   called. And the function `f` would not access to `self` since
            //   `self` is mutably borrowed.
            // - the code above `as_right_unchecked_mut` assigned `self` with
            //   `Right`.
            Left(x) => unsafe {
                let x = ptr::read(x);
                ptr::write(self, Right(f(x)));
                self.as_right_unchecked_mut()
            },
            Right(x) => x,
        }
    }

    /// Applies function `f` on the contained [`Left`] value,
    /// assigning `self` with the result of the function.
    ///
    /// NOTE: this function does nothing if `self` is the [`Right`] variant.
    pub fn left_and_modify<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(L) -> Self,
    {
        if let Left(x) = self {
            // SAFETY: This is safe because:
            // - `self` will be written with some value after it is read.
            // - After `self` is read and before it is written, only `f(x)` is
            //   called. And the function `f` would not access to `self` since
            //   `self` is mutably borrowed.
            unsafe {
                let x = ptr::read(x);
                ptr::write(self, f(x));
            }
        }
        self
    }

    /// Applies function `f` on the contained [`Right`] value,
    /// assigning `self` with the result of the function.
    ///
    /// NOTE: this function does nothing if `self` is the [`Left`] variant.
    pub fn right_and_modify<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(R) -> Self,
    {
        if let Right(x) = self {
            // SAFETY: This is safe because:
            // - `self` will be written with some value after it is read.
            // - After `self` is read and before it is written, only `f(x)` is
            //   called. And the function `f` would not access to `self` since
            //   `self` is mutably borrowed.
            unsafe {
                let x = ptr::read(x);
                ptr::write(self, f(x));
            }
        }
        self
    }
}

impl<L, R> Either<&L, &R> {
    /// Converts from `Either<&L, &R>` to `Either<L, R>` by cloning the
    /// contained [`Left`] value or [`Right`] value.
    ///
    /// Also see [`copied`] if both `L` and `R` implement the [`Copy`] trait.
    ///
    /// [`copied`]: Either::copied
    ///
    /// # Result
    ///
    /// | Input      | Output             |
    /// | ---------- | ------------------ |
    /// | `Left(x)`  | `Left(x.clone())`  |
    /// | `Right(x)` | `Right(x.clone())` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// // `String` does not implement `Copy`, so `x.copied()` is not allowed.
    /// let x: Either<&i32, &String> = Left(&3);
    /// assert_eq!(x.cloned(), Left(3));
    /// ```
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn cloned(self) -> Either<L, R>
    where
        L: Clone,
        R: Clone,
    {
        match self {
            Left(x) => Left(x.clone()),
            Right(x) => Right(x.clone()),
        }
    }

    /// Converts from `Either<&L, &R>` to `Either<L, R>` by copying the
    /// contained [`Left`] value or [`Right`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output             |
    /// | ---------- | ------------------ |
    /// | `Left(x)`  | `Left(x.clone())`  |
    /// | `Right(x)` | `Right(x.clone())` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let x: Either<&i32, &f32> = Left(&3);
    /// assert_eq!(x.copied(), Left(3));
    /// ```
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn copied(self) -> Either<L, R>
    where
        L: Copy,
        R: Copy,
    {
        match self {
            Left(x) => Left(*x),
            Right(x) => Right(*x),
        }
    }
}

impl<L, R> Either<&mut L, &mut R> {
    /// Converts from `Either<&mut L, &mut R>` to `Either<L, R>` by cloning the
    /// contained [`Left`] value or [`Right`] value.
    ///
    /// Also see [`copied`] if both `L` and `R` implement the [`Copy`] trait.
    ///
    /// [`copied`]: Either::copied
    ///
    /// # Result
    ///
    /// | Input      | Output             |
    /// | ---------- | ------------------ |
    /// | `Left(x)`  | `Left(x.clone())`  |
    /// | `Right(x)` | `Right(x.clone())` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// // `String` does not implement `Copy`, so `x.copied()` is not allowed.
    /// let mut value = 3;
    /// let x: Either<&mut i32, &mut String> = Left(&mut value);
    /// assert_eq!(x.cloned(), Left(3));
    /// ```
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn cloned(self) -> Either<L, R>
    where
        L: Clone,
        R: Clone,
    {
        match self {
            Left(x) => Left(x.clone()),
            Right(x) => Right(x.clone()),
        }
    }

    /// Converts from `Either<&mut L, &mut R>` to `Either<L, R>` by copying the
    /// contained [`Left`] value or [`Right`] value.
    ///
    /// # Result
    ///
    /// | Input      | Output             |
    /// | ---------- | ------------------ |
    /// | `Left(x)`  | `Left(x.clone())`  |
    /// | `Right(x)` | `Right(x.clone())` |
    ///
    /// # Example
    ///
    /// ```
    /// # use either::Either::{self, Left};
    /// let mut value = 3;
    /// let x: Either<&mut i32, &mut f32> = Left(&mut value);
    /// assert_eq!(x.copied(), Left(3));
    /// ```
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn copied(self) -> Either<L, R>
    where
        L: Copy,
        R: Copy,
    {
        match self {
            Left(x) => Left(*x),
            Right(x) => Right(*x),
        }
    }
}

// This is a separate function to avoid constructing a `dyn Debug`
// that gets immediately thrown away, since vtables don't get cleaned up
// by dead code elimination if a trait object is constructed even if it goes
// unused
#[cfg(feature = "panic_immediate_abort")]
#[inline]
#[cold]
#[track_caller]
fn unwrap_failed<T>(_message: &str, _error: &T) -> ! {
    panic!()
}

// This is a separate function to reduce the code size of the methods
#[cfg(not(feature = "panic_immediate_abort"))]
#[inline(never)]
#[cold]
#[track_caller]
fn unwrap_failed(message: &str, value: &dyn Debug) -> ! {
    panic!("{message}: {value:?}")
}
