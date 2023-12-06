use std::fmt::Display;

use bstr::ByteSlice;

use crate::helper::parsing::BytesAsNumber;

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split(|&b| b == b' ')
        .filter(|bs| !bs.is_empty())
        .skip(1)
        .map(|bs| bs.as_num::<u16>());
    let distances = lines
        .next()
        .unwrap()
        .split(|&b| b == b' ')
        .filter(|bs| !bs.is_empty())
        .skip(1)
        .map(|bs| bs.as_num::<u16>());

    let mut prod = 1;
    for (t, d) in times.zip(distances) {
        let mut ways = 0;

        let mut i = d / t;
        while i < d.div_ceil(2) {
            let n = (i + d / i);
            if n > t * 2 {
                i *= 2;
                continue;
            }
            if n < t {
                ways += 1;
            }
            i += 1;
        }

        prod *= ways;
    }

    prod
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut lines = input.lines();
    let t = lines
        .next()
        .unwrap()
        .split(|&b| b == b' ')
        .filter(|bs| !bs.is_empty())
        .skip(1)
        .flatten()
        .copied()
        .collect::<Vec<u8>>()
        .as_num::<u64>();

    let d = lines
        .next()
        .unwrap()
        .split(|&b| b == b' ')
        .filter(|bs| !bs.is_empty())
        .skip(1)
        .flatten()
        .copied()
        .collect::<Vec<u8>>()
        .as_num::<u64>();

    let mut ways: u64 = 0;

    let mut i = d / t;
    while i < d.div_ceil(2) {
        let n = (i + d / i);
        if n > t * 2 {
            i *= 2;
            continue;
        }
        if n < t {
            ways += 1;
        }
        i += 1;
    }

    ways
}
