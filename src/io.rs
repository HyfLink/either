//! This module implements the [`Seek`], [`Read`], [`BufRead`] and [`Write`]
//! traits for [`Either`], if and only if, both `L` and `R` implement the
//! corresponding trait.

use std::fmt::Arguments;
use std::io::{self, BufRead, Read, Seek, Write};

use crate::Either::{self, Left, Right};

impl<L, R> Seek for Either<L, R>
where
    L: Seek,
    R: Seek,
{
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match self {
            Left(x) => x.seek(pos),
            Right(x) => x.seek(pos),
        }
    }

    #[inline]
    fn rewind(&mut self) -> io::Result<()> {
        match self {
            Left(x) => x.rewind(),
            Right(x) => x.rewind(),
        }
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        match self {
            Left(x) => x.stream_position(),
            Right(x) => x.stream_position(),
        }
    }

    #[inline]
    fn seek_relative(&mut self, offset: i64) -> io::Result<()> {
        match self {
            Left(x) => x.seek_relative(offset),
            Right(x) => x.seek_relative(offset),
        }
    }
}

impl<L, R> Read for Either<L, R>
where
    L: Read,
    R: Read,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Left(x) => x.read(buf),
            Right(x) => x.read(buf),
        }
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        match self {
            Left(x) => x.read_vectored(bufs),
            Right(x) => x.read_vectored(bufs),
        }
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        match self {
            Left(x) => x.read_to_end(buf),
            Right(x) => x.read_to_end(buf),
        }
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        match self {
            Left(x) => x.read_to_string(buf),
            Right(x) => x.read_to_string(buf),
        }
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        match self {
            Left(x) => x.read_exact(buf),
            Right(x) => x.read_exact(buf),
        }
    }
}

impl<L, R> BufRead for Either<L, R>
where
    L: BufRead,
    R: BufRead,
{
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            Left(x) => x.fill_buf(),
            Right(x) => x.fill_buf(),
        }
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        match self {
            Left(x) => x.consume(amt),
            Right(x) => x.consume(amt),
        }
    }

    #[inline]
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
        match self {
            Left(x) => x.read_until(byte, buf),
            Right(x) => x.read_until(byte, buf),
        }
    }

    #[inline]
    fn skip_until(&mut self, byte: u8) -> io::Result<usize> {
        match self {
            Left(x) => x.skip_until(byte),
            Right(x) => x.skip_until(byte),
        }
    }

    #[inline]
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        match self {
            Left(x) => x.read_line(buf),
            Right(x) => x.read_line(buf),
        }
    }
}

impl<L, R> Write for Either<L, R>
where
    L: Write,
    R: Write,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Left(x) => x.write(buf),
            Right(x) => x.write(buf),
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match self {
            Left(x) => x.flush(),
            Right(x) => x.flush(),
        }
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        match self {
            Left(x) => x.write_vectored(bufs),
            Right(x) => x.write_vectored(bufs),
        }
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self {
            Left(x) => x.write_all(buf),
            Right(x) => x.write_all(buf),
        }
    }

    #[inline]
    fn write_fmt(&mut self, fmt: Arguments<'_>) -> io::Result<()> {
        match self {
            Left(x) => x.write_fmt(fmt),
            Right(x) => x.write_fmt(fmt),
        }
    }
}
