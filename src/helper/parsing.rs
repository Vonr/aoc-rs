use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Mul, MulAssign},
    str::Lines,
};

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

pub trait BytesAsNumber {
    /// # Safety
    ///
    /// self should be a slice of ASCII bytes of characters between b'0' and b'9'
    unsafe fn as_num<T: From<u8> + AddAssign + MulAssign + Default>(&self) -> T;
}

impl BytesAsNumber for [u8] {
    unsafe fn as_num<T: From<u8> + AddAssign + MulAssign + Default>(&self) -> T {
        let mut out = T::default();
        for b in self {
            out *= 10.into();
            out += (b & 0xf).into();
        }
        out
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
        assert_eq!(unsafe { b"123".as_num::<usize>() }, 123);
    }
}
