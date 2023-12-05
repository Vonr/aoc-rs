use bstr::ByteSlice;
use rayon::iter::{IntoParallelIterator, ParallelDrainRange, ParallelIterator};
use std::{fmt::Display, ops::Range};

use crate::helper::parsing::{BytesAsNumber, PartialConsume};

pub fn part1(input: &str) -> impl Display {
    let mut input = input.as_bytes();

    let mut line_iter = input.lines();
    let mut curr = line_iter.next().unwrap()[7..]
        .split(|&b| b == b' ')
        .map(|bs| bs.as_num::<u64>())
        .collect::<Vec<_>>();

    let mut ranges: Vec<(u64, u64, u64)> = Vec::new();

    for mut line in line_iter.skip(1) {
        if line.is_empty() {
            for thing in curr.iter_mut() {
                for &(src, dst, len) in &ranges {
                    if (src..src + len).contains(thing) {
                        *thing = *thing - src + dst;
                        break;
                    }
                }
            }
            ranges.clear();
            continue;
        }

        if !line[0].is_ascii_digit() {
            continue;
        }

        let dst: u64 = line.skip_to_unit(b' ').as_num();
        let src: u64 = line.skip_to_unit(b' ').as_num();
        let len: u64 = line.skip_to_unit(b' ').as_num();

        ranges.push((src, dst, len));
    }

    let mut result = curr[0];
    for thing in curr.iter_mut() {
        for &(src, dst, len) in &ranges {
            if (src..src + len).contains(thing) {
                result = result.min(*thing - src + dst);
                break;
            }
        }
        result = result.min(*thing);
    }

    result
}

// This is an abomination
pub fn part2(input: &str) -> impl Display {
    let mut input = input.as_bytes();

    let mut line_iter = input.lines();
    let mut sets = line_iter.next().unwrap()[7..]
        .split(|&b| b == b' ')
        .map(|bs| bs.as_num::<u64>())
        .array_chunks()
        .map(|[a, b]| a..a + b)
        .collect::<Box<[_]>>();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.install(move || {
        sets.into_par_iter()
            .map(|set| {
                let mut subresult = u64::MAX;
                let mut curr = set.clone().collect::<Vec<_>>();
                let mut ranges: Vec<(u64, u64, u64)> = Vec::new();

                for mut line in line_iter.clone().skip(1) {
                    if line.is_empty() {
                        for thing in curr.iter_mut() {
                            for &(src, dst, len) in &ranges {
                                if (src..src + len).contains(thing) {
                                    *thing = *thing - src + dst;
                                    break;
                                }
                            }
                        }
                        ranges.clear();
                        continue;
                    }

                    if !line[0].is_ascii_digit() {
                        continue;
                    }

                    let dst: u64 = line.skip_to_unit(b' ').as_num();
                    let src: u64 = line.skip_to_unit(b' ').as_num();
                    let len: u64 = line.skip_to_unit(b' ').as_num();

                    ranges.push((src, dst, len));
                }

                for thing in curr.iter_mut() {
                    for &(src, dst, len) in &ranges {
                        if (src..src + len).contains(thing) {
                            subresult = subresult.min(*thing - src + dst);
                            break;
                        }
                    }
                    subresult = subresult.min(*thing);
                }

                subresult
            })
            .min()
            .unwrap()
    })
}
