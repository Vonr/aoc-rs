use core::slice::SlicePattern;
use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub},
    str::Lines,
};

use bstr::ByteSlice;
use num_traits::{AsPrimitive, PrimInt, Signed};

pub struct Columns<T, U, V, I: Iterator<Item = T>, F: Fn(T, usize) -> Option<U>> {
    iter: I,
    indexer: F,
    idx: usize,
    max: usize,
    phantom: PhantomData<V>,
}

impl<T, U, V: FromIterator<U>, I: Iterator<Item = T> + Clone, F: Fn(T, usize) -> Option<U>> Iterator
    for Columns<T, U, V, I, F>
{
    type Item = V;

    fn next(&mut self) -> Option<V> {
        if self.idx >= self.max {
            return None;
        }
        let out = Some(
            self.iter
                .clone()
                .filter_map(|l| (self.indexer)(l, self.idx))
                .collect::<V>(),
        );
        self.idx += 1;
        out
    }
}

pub trait IntoColumns<T, U, V, F: Fn(T, usize) -> Option<U>> {
    fn into_columns(self) -> Columns<T, U, V, Self, F>
    where
        Self: Iterator<Item = T> + Sized;
}

impl<'a, I> IntoColumns<&'a str, char, String, fn(&str, usize) -> Option<char>> for I
where
    I: Iterator<Item = &'a str> + Clone,
{
    fn into_columns(self) -> Columns<&'a str, char, String, Self, fn(&str, usize) -> Option<char>> {
        let min_len = self.clone().map(|s| s.len()).min().unwrap_or(0);
        Columns {
            iter: self,
            indexer: |s, i| s.chars().nth(i),
            idx: 0,
            max: min_len,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoColumns<&'a [u8], u8, Vec<u8>, fn(&[u8], usize) -> Option<u8>> for I
where
    I: Iterator<Item = &'a [u8]> + Clone,
{
    fn into_columns(self) -> Columns<&'a [u8], u8, Vec<u8>, Self, fn(&[u8], usize) -> Option<u8>> {
        let min_len = self.clone().map(|s| s.len()).min().unwrap_or(0);
        Columns {
            iter: self,
            indexer: |s, i| s.get(i).copied(),
            idx: 0,
            max: min_len,
            phantom: PhantomData,
        }
    }
}

pub trait BytesAsNumber {
    fn as_num<T: PrimInt + 'static>(&self) -> T;
    fn as_signed_num<T: PrimInt + Signed + 'static>(&self) -> T;
    fn as_num_checked<T: PrimInt + 'static>(&self) -> T;
    fn as_signed_num_checked<T: PrimInt + Signed + 'static>(&self) -> T;
    fn as_nums<T: PrimInt + 'static>(&self) -> SeparatedNumbers<T>;
    fn as_signed_nums<T: PrimInt + Signed + 'static>(&self) -> SeparatedSignedNumbers<T>;
}

impl BytesAsNumber for [u8] {
    fn as_num<T: PrimInt + 'static>(&self) -> T {
        let mut out = T::zero();
        for b in self {
            out = out * T::from(10).unwrap() + T::from(b - b'0').unwrap();
        }
        out
    }

    fn as_signed_num<T: PrimInt + Signed + 'static>(&self) -> T {
        let mut out = T::zero();
        if self[0] == b'-' {
            for b in &self[1..] {
                out = out * T::from(10).unwrap() + T::from(b - b'0').unwrap();
            }
            -out
        } else {
            for b in self {
                out = out * T::from(10).unwrap() + T::from(b - b'0').unwrap();
            }
            out
        }
    }

    fn as_num_checked<T: PrimInt + 'static>(&self) -> T {
        let mut out = T::zero();
        for b in self {
            if b.is_ascii_digit() {
                out = out * T::from(10).unwrap() + T::from(b - b'0').unwrap();
            }
        }
        out
    }

    fn as_signed_num_checked<T: PrimInt + Signed + 'static>(&self) -> T {
        let mut out = T::zero();
        if self[0] == b'-' {
            for b in &self[1..] {
                if b.is_ascii_digit() {
                    out = out * T::from(10).unwrap() + T::from(b - b'0').unwrap();
                }
            }
            -out
        } else {
            for b in self {
                if b.is_ascii_digit() {
                    out = out * T::from(10).unwrap() + T::from(b - b'0').unwrap();
                }
            }
            out
        }
    }

    fn as_nums<T: PrimInt + 'static>(&self) -> SeparatedNumbers<T> {
        SeparatedNumbers {
            slice: self,
            _output: PhantomData,
        }
    }

    fn as_signed_nums<T: PrimInt + 'static>(&self) -> SeparatedSignedNumbers<T> {
        SeparatedSignedNumbers {
            slice: self,
            _output: PhantomData,
        }
    }
}

pub trait StripPrefixUnchecked<T> {
    /// # Safety
    ///
    /// Caller must ensure that the length of prefix is less than the length of self
    unsafe fn strip_prefix_unchecked<P: SlicePattern<Item = T> + ?Sized>(
        &self,
        prefix: &P,
    ) -> Option<&[T]>
    where
        T: PartialEq;
}

impl StripPrefixUnchecked<u8> for [u8] {
    unsafe fn strip_prefix_unchecked<P: SlicePattern<Item = u8> + ?Sized>(
        &self,
        prefix: &P,
    ) -> Option<&[u8]> {
        let prefix = prefix.as_slice();
        let (head, tail) = self.split_at_unchecked(prefix.len());
        if head == prefix {
            return Some(tail);
        }
        None
    }
}

pub trait PartialConsume<T> {
    fn next(self: &mut &Self) -> Option<T>;
    fn skip_n(self: &mut &Self, n: usize) -> &[T];
    fn skip_to_unit<'l, 'r: 'l>(self: &mut &'r Self, unit: T) -> &'l [T];
    fn skip_to_group<'l, 'r: 'l>(self: &mut &'r Self, group: impl AsRef<[T]>) -> &'l [T];
}

impl PartialConsume<u8> for [u8] {
    #[inline]
    fn next(self: &mut &Self) -> Option<u8> {
        let l = self.skip_n(1);
        l.first().copied()
    }

    #[inline]
    fn skip_n(self: &mut &Self, n: usize) -> &[u8] {
        let (l, r) = unsafe { self.split_at_unchecked(n.min(self.len().saturating_sub(1))) };
        *self = unsafe { r.get_unchecked(1..) };
        l
    }

    #[inline]
    fn skip_to_unit<'l, 'r: 'l>(self: &mut &'r Self, unit: u8) -> &'l [u8] {
        let idx = self.find_byte(unit);
        let ret = match idx {
            Some(idx) => {
                let (l, r) = unsafe { self.split_at_unchecked(idx) };
                *self = unsafe { r.get_unchecked(1..) };
                l
            }
            None => {
                let l = *self;
                *self = &[];
                l
            }
        };

        ret
    }

    #[inline]
    fn skip_to_group<'l, 'r: 'l>(self: &mut &'r Self, group: impl AsRef<[u8]>) -> &'l [u8] {
        let idx = self.find(group.as_ref());
        let ret = match idx {
            Some(idx) => {
                let (l, r) = unsafe { self.split_at_unchecked(idx) };
                *self = unsafe { r.get_unchecked(1..) };
                l
            }
            None => {
                let l = *self;
                *self = &[];
                l
            }
        };

        ret
    }
}

pub struct SeparatedNumbers<'a, Output> {
    slice: &'a [u8],
    _output: PhantomData<Output>,
}

impl<'a, Output> Iterator for SeparatedNumbers<'a, Output>
where
    Output: PrimInt + 'static + std::fmt::Display,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.slice.first()?.is_ascii_digit() {
            if !self.slice.is_empty() {
                self.slice = &self.slice[1..];
            }
        }

        let mut out: Self::Item = Self::Item::zero();
        while self
            .slice
            .first()
            .map(|f| f.is_ascii_digit())
            .unwrap_or(false)
        {
            out = (out * Self::Item::from(10).unwrap())
                + Self::Item::from(self.slice[0] - b'0').unwrap();

            if !self.slice.is_empty() {
                self.slice = &self.slice[1..];
            }
        }

        Some(out)
    }
}

pub struct SeparatedSignedNumbers<'a, Output> {
    slice: &'a [u8],
    _output: PhantomData<Output>,
}

impl<'a, Output> Iterator for SeparatedSignedNumbers<'a, Output>
where
    Output: PrimInt + Signed + 'static + std::fmt::Display,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        let mut neg = false;
        let mut first = *self.slice.first()?;
        while first != b'-' && !first.is_ascii_digit() {
            if !self.slice.is_empty() {
                self.slice = &self.slice[1..];
            }
            first = *self.slice.first()?;
        }

        let mut out: Self::Item = Self::Item::zero();
        while self
            .slice
            .first()
            .map(|&f| f == b'-' || f.is_ascii_digit())
            .unwrap_or(false)
        {
            if self.slice[0] == b'-' {
                neg = !neg;

                if !self.slice.is_empty() {
                    self.slice = &self.slice[1..];
                }
                continue;
            }

            out = (out * Self::Item::from(10).unwrap())
                + Self::Item::from(self.slice[0] - b'0').unwrap();
            if !self.slice.is_empty() {
                self.slice = &self.slice[1..];
            }
        }

        Some(if neg { -out } else { out })
    }
}

mod tests {
    use super::{BytesAsNumber, IntoColumns};

    #[test]
    pub fn into_columns() {
        assert_eq!(
            ["a b c", "d e f"]
                .into_iter()
                .into_columns()
                .collect::<Vec<String>>(),
            vec!["ad", "  ", "be", "  ", "cf"]
        );
    }

    #[test]
    pub fn filter_columns() {
        assert_eq!(
            ["a b c", "d e f"]
                .into_iter()
                .into_columns()
                .filter(|c| !c.trim().is_empty())
                .collect::<Vec<String>>(),
            vec!["ad", "be", "cf"]
        );
    }

    #[test]
    pub fn ascii_as_num() {
        assert_eq!(b"123".as_num::<usize>(), 123);
    }
}
