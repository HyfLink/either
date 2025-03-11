use crate::Either::{self, Left, Right};

impl<L, R> Either<Option<L>, Option<R>> {
    /// Transposes an [`Either`] of [`Option`] into an [`Option`] of [`Either`].
    ///
    /// # Result
    ///
    /// | Input            | Output           |
    /// | ---------------- | ---------------- |
    /// | `Left(Some(x))`  | `Some(Left(x))`  |
    /// | `Right(Some(x))` | `Some(Right(x))` |
    /// | `Left(None)`     | `None`           |
    /// | `Right(None)`    | `None`           |
    #[inline]
    #[must_use]
    pub fn transpose(self) -> Option<Either<L, R>> {
        match self {
            Left(x) => x.map(Left),
            Right(x) => x.map(Right),
        }
    }
}
