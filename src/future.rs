//! This module implements the [`Future`] trait for [`Either`],
//! if and only if, both `L` and `R` are [`Future`]s.

use core::pin::Pin;
use core::task::{Context, Poll};

use crate::Either::{self, Left, Right};

impl<L, R> Future for Either<L, R>
where
    L: Future,
    R: Future<Output = L::Output>,
{
    type Output = L::Output;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.as_pin_mut() {
            Left(x) => x.poll(cx),
            Right(x) => x.poll(cx),
        }
    }
}
