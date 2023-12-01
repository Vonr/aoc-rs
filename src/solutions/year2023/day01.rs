use std::{
    fmt::Display,
    num::{NonZeroU8, NonZeroUsize},
};

use bstr::ByteSlice;

pub fn part1(input: &str) -> impl Display {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut iter = line.bytes();
        let first = unsafe { iter.find(|&b| b <= b'9').unwrap_unchecked() & 0xf };
        let last = iter.rfind(|&b| b <= b'9').unwrap_or(first) & 0xf;
        sum += (first * 10 + last) as u32;
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let mut input = input.as_bytes();
    let mut sum: usize = 0;

    let mut options: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    for line in input.lines() {
        let mut first = None;
        let mut last = None;

        for idx in (0..line.len()) {
            for (oidx, option) in options.iter().enumerate() {
                if line[idx].is_ascii_digit() {
                    let value = unsafe { NonZeroUsize::new_unchecked((line[idx] & 0xf) as usize) };
                    last = Some(value);
                    first.get_or_insert(value);
                    break;
                }
                if line.get(idx..idx + option.len()) == Some(option) {
                    let value = unsafe { NonZeroUsize::new_unchecked(oidx + 1) };
                    last = Some(value);
                    first.get_or_insert(value);
                    break;
                }
            }
        }

        sum += unsafe { first.unwrap_unchecked().get() * 10 + last.unwrap_unchecked().get() };
    }

    sum
}
