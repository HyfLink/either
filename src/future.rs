//! This module implements the [`Future`] trait for [`Either`],
//! if and only if, both `L` and `R` are [`Future`]s.

use core::future::Future;
use core::pin::Pin;
use core::task;

use crate::Either::{self, Left, Right};

impl<L, R> Future for Either<L, R>
where
    L: Future,
    R: Future,
{
    type Output = Either<L::Output, R::Output>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        match self.as_pin_mut() {
            Left(x) => x.poll(cx).map(Left),
            Right(x) => x.poll(cx).map(Right),
        }
    }
}
