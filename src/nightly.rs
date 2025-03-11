use core::async_iter::AsyncIterator;
use core::marker::Tuple;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::Either::{self, Left, Right};

impl<A, L, R> FnOnce<A> for Either<L, R>
where
    A: Tuple,
    L: FnOnce<A>,
    R: FnOnce<A, Output = L::Output>,
{
    type Output = L::Output;

    #[inline]
    extern "rust-call" fn call_once(self, args: A) -> Self::Output {
        match self {
            Left(x) => x.call_once(args),
            Right(x) => x.call_once(args),
        }
    }
}

impl<A, L, R> FnMut<A> for Either<L, R>
where
    A: Tuple,
    L: FnMut<A>,
    R: FnMut<A, Output = L::Output>,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, args: A) -> Self::Output {
        match self {
            Left(x) => x.call_mut(args),
            Right(x) => x.call_mut(args),
        }
    }
}

impl<A, L, R> Fn<A> for Either<L, R>
where
    A: Tuple,
    L: Fn<A>,
    R: Fn<A, Output = L::Output>,
{
    #[inline]
    extern "rust-call" fn call(&self, args: A) -> Self::Output {
        match self {
            Left(x) => x.call(args),
            Right(x) => x.call(args),
        }
    }
}

impl<A, L, R> AsyncFnOnce<A> for Either<L, R>
where
    A: Tuple,
    L: AsyncFnOnce<A>,
    R: AsyncFnOnce<A, Output = L::Output>,
{
    type Output = L::Output;
    type CallOnceFuture = Either<L::CallOnceFuture, R::CallOnceFuture>;

    extern "rust-call" fn async_call_once(self, args: A) -> Self::CallOnceFuture {
        match self {
            Left(x) => Left(x.async_call_once(args)),
            Right(x) => Right(x.async_call_once(args)),
        }
    }
}

impl<A, L, R> AsyncFnMut<A> for Either<L, R>
where
    A: Tuple,
    L: AsyncFnMut<A>,
    R: AsyncFnMut<A, Output = L::Output>,
{
    type CallRefFuture<'a>
        = Either<L::CallRefFuture<'a>, R::CallRefFuture<'a>>
    where
        Self: 'a;

    #[inline]
    extern "rust-call" fn async_call_mut(&mut self, args: A) -> Self::CallRefFuture<'_> {
        match self {
            Left(x) => Left(x.async_call_mut(args)),
            Right(x) => Right(x.async_call_mut(args)),
        }
    }
}

impl<A, L, R> AsyncFn<A> for Either<L, R>
where
    A: Tuple,
    L: AsyncFn<A>,
    R: AsyncFn<A, Output = L::Output>,
{
    #[inline]
    extern "rust-call" fn async_call(&self, args: A) -> Self::CallRefFuture<'_> {
        match self {
            Left(x) => Left(x.async_call(args)),
            Right(x) => Right(x.async_call(args)),
        }
    }
}

impl<L, R> AsyncIterator for Either<L, R>
where
    L: AsyncIterator,
    R: AsyncIterator<Item = L::Item>,
{
    type Item = L::Item;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.as_pin_mut() {
            Left(x) => x.poll_next(cx),
            Right(x) => x.poll_next(cx),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Left(x) => x.size_hint(),
            Right(x) => x.size_hint(),
        }
    }
}
