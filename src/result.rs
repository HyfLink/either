use crate::Either::{self, Left, Right};

impl<L, R, E> Either<Result<L, E>, Result<R, E>> {
    /// Transposes an [`Either`] of [`Result`] into a [`Result`] of [`Either`].
    ///
    /// # Result
    ///
    /// | Input           | Output         |
    /// | --------------- | -------------- |
    /// | `Left(Ok(x))`   | `Ok(Left(x))`  |
    /// | `Right(Ok(x))`  | `Ok(Right(x))` |
    /// | `Left(Err(x))`  | `Err(x)`       |
    /// | `Right(Err(x))` | `Err(x)`       |
    #[inline]
    pub fn transpose(self) -> Result<Either<L, R>, E> {
        match self {
            Left(x) => x.map(Left),
            Right(x) => x.map(Right),
        }
    }
}
