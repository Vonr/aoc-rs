use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
    mem::MaybeUninit,
    ops::{Add, Div, Range, Shr, Sub},
    simd::{u16x2, u16x4, u16x8, SimdUint},
    time::{Duration, Instant},
};

use num::Integer;
use num_traits::PrimInt;

pub trait Unique {
    fn unique(&self) -> bool;
}

impl<const N: usize> Unique for [u8; N] {
    fn unique(&self) -> bool {
        let mut found = (0u128, 0u128);
        let mut ones = 0;

        for (i, c) in self.iter().enumerate() {
            if *c < 128 {
                let val = 1 << c;
                ones += 1 - ((found.0 & val) >> c);
                found.0 |= val;
            } else {
                let val = 1 << (c - 128);
                ones += 1 - ((found.1 & val) >> (c - 128));
                found.1 |= val;
            }
            if ones as usize != i + 1 {
                return false;
            }
        }
        true
    }
}

impl Unique for [u8] {
    fn unique(&self) -> bool {
        let mut found = (0u128, 0u128);
        let mut ones = 0;

        for (i, c) in self.iter().enumerate() {
            if *c < 128 {
                let val = 1 << c;
                ones += 1 - ((found.0 & val) >> c);
                found.0 |= val;
            } else {
                let val = 1 << (c - 128);
                ones += 1 - ((found.1 & val) >> (c - 128));
                found.1 |= val;
            }
            if ones as usize != i + 1 {
                return false;
            }
        }
        true
    }
}

#[inline(always)]
pub fn hash_ascii_digit_pair(digits: [u8; 2]) -> u8 {
    let n = u16::from_le_bytes(digits) as u32;
    (((n * 0x10a) >> 8) as u8) & 0x7f
}

#[inline(always)]
pub fn hash_4_separated_ascii_digit_pairs(digits: [u8; 11]) -> [u8; 4] {
    use std::arch::is_x86_feature_detected;
    use std::arch::x86_64::*;

    #[target_feature(enable = "sse2")]
    unsafe fn inner(digits: [u8; 11]) -> [u8; 4] {
        let init = _mm_set_epi32(
            i16::from_le_bytes([digits[9], digits[10]]) as i32,
            i16::from_le_bytes([digits[6], digits[7]]) as i32,
            i16::from_le_bytes([digits[3], digits[4]]) as i32,
            i16::from_le_bytes([digits[0], digits[1]]) as i32,
        );

        let mul = _mm_mullo_epi32(init, _mm_set1_epi32(0x10a));
        let shr = _mm_srai_epi32(mul, 8);
        let and = _mm_and_si128(shr, _mm_set1_epi32(0x7f));
        let shuffled = _mm_shuffle_epi8(
            and,
            _mm_set_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 12, 8, 4, 0),
        );

        let mut out = [0; 4];
        _mm_storeu_epi8(out.as_mut_ptr(), shuffled);

        std::mem::transmute(out)
    }

    // if is_x86_feature_detected!("sse") && cfg!(target_feature = "sse2") {
    // unsafe { inner(digits) }
    // } else {
    [
        hash_ascii_digit_pair([digits[0], digits[1]]),
        hash_ascii_digit_pair([digits[3], digits[4]]),
        hash_ascii_digit_pair([digits[6], digits[7]]),
        hash_ascii_digit_pair([digits[9], digits[10]]),
    ]
    // }
}

pub fn binary_search_by<T>(mut low: T, mut high: T, pred: impl Fn(T) -> Ordering) -> Option<T>
where
    T: PrimInt,
{
    while low < high {
        let mid = (low + high) >> 1;
        match pred(mid) {
            Ordering::Less => low = mid + T::one(),
            Ordering::Equal => return Some(mid),
            Ordering::Greater => high = mid - T::one(),
        }
    }

    None
}

pub fn partition_point_high<T>(mut range: Range<T>, pred: impl Fn(T) -> bool) -> T
where
    T: PrimInt,
{
    while range.start != range.end {
        let mid = range.start + ((range.end - range.start) >> 1);
        if pred(mid) {
            range.start = mid + T::one();
        } else {
            range.end = mid;
        }
    }
    range.start
}

pub trait IntegerIteratorExt<T> {
    fn gcd(self) -> Option<T>;
    fn lcm(self) -> Option<T>;
}

impl<T, I> IntegerIteratorExt<T> for I
where
    T: Integer,
    I: Iterator<Item = T>,
{
    fn gcd(self) -> Option<T> {
        self.reduce(num::integer::gcd)
    }

    fn lcm(self) -> Option<T> {
        self.reduce(num::integer::lcm)
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct Timer(Instant);

impl Timer {
    pub fn new() -> Self {
        Self(Instant::now())
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        eprintln!("{:?}", self.0.elapsed());
    }
}

/// Taken from nils https://github.com/Nilstrieb/advent-of-code/blob/45b91e97187f9d59b068c9803b2756239a41a397/helper/src/ext.rs
/// i will not use itertools i will not use itertools i will not use itertools i will not use itertools
pub trait CollectToArray: Iterator {
    /// Collect an iterator into a new array.
    /// If `next` panics, collected items are leaked. Too bad!
    fn collect_array<const N: usize>(&mut self) -> Option<[Self::Item; N]> {
        // SAFETY: Uninit is valid for MaybeUninit
        let mut array: [MaybeUninit<Self::Item>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for a in &mut array {
            a.write(self.next()?);
        }

        // SAFETY: All elements have been initialized
        Some(array.map(|elem| unsafe { elem.assume_init() }))
    }

    /// Collect an iterator into a new array.
    fn collect_array_default<const N: usize>(&mut self) -> [Self::Item; N]
    where
        Self::Item: Default + Copy,
    {
        let mut array: [Self::Item; N] = [Default::default(); N];

        for a in &mut array {
            let Some(elem) = self.next() else {
                break;
            };
            *a = elem;
        }

        array
    }

    /// Collect an iterator into an existing array.
    fn collect_into_array<const N: usize>(&mut self, array: &mut [Self::Item; N])
    where
        Self::Item: Default + Copy,
    {
        for a in array {
            let Some(elem) = self.next() else {
                break;
            };
            *a = elem;
        }
    }
}

impl<I: Iterator> CollectToArray for I {}

#[cfg(test)]
mod tests {
    use std::array;

    use super::*;

    #[test]
    #[cfg(not(miri))]
    fn unique() {
        let mut all: [u8; 256] = array::from_fn(|i| i as u8);
        assert!(all.unique());

        for i in 0..=255 {
            for j in 0..=255u8 {
                if i as u8 == j {
                    continue;
                }
                all[i] = j;
                assert!(!all.unique());
            }
            all[i] = i as u8;
            assert!(all.unique());
        }
    }

    #[test]
    fn ascii_digit_pairs() {
        let input = *b"88 52 87  9";
        let input_two = [*b"88", *b"52", *b"87", *b" 9"];
        let hash = hash_4_separated_ascii_digit_pairs(input);
        for (input, hash) in input_two.into_iter().zip(hash.into_iter()) {
            assert_eq!(hash_ascii_digit_pair(input), hash);
        }
    }
}
