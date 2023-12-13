use std::{fmt::Display, hash::Hasher, rc::Rc};

use bitvec::{bitarr, bitvec, order::LocalBits, view::BitView};
use bstr::ByteSlice;
use num_traits::{PrimInt, Unsigned};
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};

use crate::helper::{
    parsing::{BytesAsNumber, PartialConsume},
    util::{CollectToArray, Timer},
};

type Cache = FxHashMap<u64, u64>;

fn solve(mut springs: &[u8], lengths: &[u8], cache: &mut Cache) -> u64 {
    if springs.is_empty() {
        return lengths.is_empty() as u64;
    }

    fn calc(springs: &[u8], lengths: &[u8], cache: &mut Cache) -> u64 {
        let mut hasher = FxHasher::default();
        for spring in springs.iter() {
            hasher.write_u8(*spring);
        }
        for length in lengths.iter() {
            hasher.write_u8(*length);
        }
        let hash = hasher.finish();

        if lengths.is_empty() {
            return 0;
        }

        if let Some(&result) = cache.get(&hash) {
            return result;
        }

        let end = lengths[0] as usize;
        if springs.len() < end || springs[0..end].find_byte(b'.').is_some() {
            return 0;
        }

        if springs.len() == end {
            return (lengths.len() == 1) as u64;
        }

        if springs[end] == b'#' {
            return 0;
        }

        let out = solve(springs[(end + 1)..].into(), lengths[1..].into(), cache);

        cache.insert(hash, out);

        out
    }

    let mut ans = 0;
    if springs[0] != b'.' {
        ans += calc(springs, lengths, cache);
    }

    if springs[0] != b'#' {
        ans += solve(&springs[1..], lengths, cache);
    }

    ans
}

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut sum = 0;

    for (idx, mut line) in input.lines().enumerate() {
        let mut springs = line.skip_to_unit(b' ');
        let mut broken = 0u32;
        let mut normal = 0u32;
        let mut lengths: Vec<u32> = line.as_nums().collect::<Vec<_>>();
        let mut ones: u32 = lengths.iter().copied().sum();
        let mut start: u32 = (1u32 << (ones + lengths.len() as u32 - 2)) - 1;

        for (idx, &spring) in springs.iter().enumerate() {
            if spring == b'#' {
                broken |= 1 << idx;
            } else if spring == b'.' {
                normal |= 1 << idx;
            }
        }

        'outer: for mut comb in start..1 << springs.len() as u32 {
            if comb & normal != 0 {
                continue;
            }

            if comb & broken != broken {
                continue;
            }

            let mut num_secs = 0;

            while comb != 0 {
                if comb & 1 == 0 {
                    comb >>= comb.trailing_zeros();
                } else {
                    let chunk_size = comb.trailing_ones();
                    comb >>= chunk_size as usize;

                    if num_secs >= lengths.len() || chunk_size != lengths[num_secs] {
                        continue 'outer;
                    }
                    num_secs += 1;
                }
            }

            if num_secs != lengths.len() {
                continue;
            }

            sum += 1;
        }
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut sum: u128 = 0;

    let mut cache = Cache::default();

    let mut springs = Vec::with_capacity(104);

    for (idx, mut line) in input.lines().enumerate() {
        let springs_slice = line.skip_to_unit(b' ');

        springs.extend_from_slice(springs_slice);
        for i in 0..4 {
            springs.push(b'?');
            springs.extend_from_slice(springs_slice);
        }

        let mut lengths = line.as_nums().collect::<Vec<_>>().repeatn(5);

        sum += solve(springs.as_slice(), &lengths, &mut cache) as u128;

        unsafe {
            springs.set_len(0);
        }
    }

    sum
}
