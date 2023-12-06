use std::fmt::Display;

use bstr::ByteSlice;

use crate::helper::{
    parsing::BytesAsNumber,
    util::{binary_search_by, partition_point_high},
};

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split(|&b| b == b' ')
        .filter(|bs| !bs.is_empty())
        .skip(1)
        .map(|bs| bs.as_num::<u32>());

    let distances = lines
        .next()
        .unwrap()
        .split(|&b| b == b' ')
        .filter(|bs| !bs.is_empty())
        .skip(1)
        .map(|bs| bs.as_num::<u32>());

    let mut prod = 1;
    for (t, d) in times.zip(distances) {
        let high = partition_point_high(d / t..d.div_ceil(2), |i| (i + d / i) < t);
        let low = partition_point_high(d / t..high, |i| (i + d / i) >= t);

        prod *= high - low;
    }

    prod
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut lines = input.lines();

    let t: u64 = lines
        .next()
        .unwrap()
        .split(|&b| b == b' ')
        .filter(|bs| !bs.is_empty())
        .skip(1)
        .flatten()
        .copied()
        .fold(0, |acc, x| acc * 10 + (x - b'0') as u64);

    let d: u64 = lines
        .next()
        .unwrap()
        .split(|&b| b == b' ')
        .filter(|bs| !bs.is_empty())
        .skip(1)
        .flatten()
        .copied()
        .fold(0, |acc, x| acc * 10 + (x - b'0') as u64);

    let high = partition_point_high(d / t..d.div_ceil(2), |i| (i + d / i) < t);
    let low = partition_point_high(d / t..high, |i| (i + d / i) >= t);

    high - low
}
