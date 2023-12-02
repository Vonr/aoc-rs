use std::{
    fmt::Display,
    mem::MaybeUninit,
    num::{NonZeroU8, NonZeroUsize},
};

use bstr::ByteSlice;

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut lsum: u32 = 0;
    let mut rsum: u32 = 0;

    let mut num_lines: u32 = 0;
    for line in input.lines() {
        num_lines += 1;
        let mut iter = line.iter().copied();
        let first = unsafe { iter.find(|&b| b <= b'9').unwrap_unchecked() };
        let last = iter.rfind(|&b| b <= b'9').unwrap_or(first);
        lsum += first as u32;
        rsum += last as u32;
    }

    const COMPENSATION: u32 = 11 * b'0' as u32;

    lsum * 10 + rsum - num_lines * COMPENSATION
}

pub fn part2(input: &str) -> impl Display {
    const OPTIONS: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    let input = input.as_bytes();
    let mut lsum = 0;
    let mut rsum = 0;
    let mut lines = 0;

    for mut line in input.lines() {
        let mut first = b'0';
        while !line.is_empty() {
            if line[0] <= b'9' {
                first = line[0];
                line = &line[1..];
                break;
            }

            if line.len() >= 3 {
                if let Some(val) = OPTIONS
                    .iter()
                    .position(|&opt| line.len() >= opt.len() && line[..opt.len()] == *opt)
                {
                    first = val as u8 + b'1';
                    break;
                }
            }

            line = &line[1..];
        }

        let mut last = first;
        while !line.is_empty() {
            let last_idx = line.len() - 1;
            if line[last_idx] <= b'9' {
                last = line[last_idx];
                break;
            }

            if line.len() >= 3 {
                if let Some(val) = OPTIONS.iter().position(|&opt| line.ends_with(opt)) {
                    last = val as u8 + b'1';
                    break;
                }
            }

            line = &line[..last_idx];
        }

        lsum += first as u32;
        rsum += last as u32;
        lines += 1;
    }

    const COMPENSATION: u32 = 11 * b'0' as u32;

    lsum * 10 + rsum - lines * COMPENSATION
}
