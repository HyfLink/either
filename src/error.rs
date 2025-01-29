//! This module implements the [`Error`] trait for [`Either`],
//! if and only if, both `L` and `R` are [`Error`]s.

use crate::Either::{self, Left, Right};

use core::error::Error;

impl<L, R> Error for Either<L, R>
where
    L: Error,
    R: Error,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Left(x) => x.source(),
            Right(x) => x.source(),
        }
    }
}
