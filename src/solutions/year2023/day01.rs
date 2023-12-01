use std::{
    fmt::Display,
    mem::MaybeUninit,
    num::{NonZeroU8, NonZeroUsize},
};

use bstr::ByteSlice;

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut sums: [u32; 2] = [0, 0];

    let mut num_lines: u32 = 0;
    for line in input.lines() {
        num_lines += 1;
        let mut iter = line.iter().copied();
        let first = unsafe { iter.find(|&b| b <= b'9').unwrap_unchecked() };
        let last = iter.rfind(|&b| b <= b'9').unwrap_or(first);
        sums[0] += first as u32;
        sums[1] += last as u32;
    }

    const COMPENSATION: u32 = 11 * b'0' as u32;

    sums[0] * 10 + sums[1] - num_lines * COMPENSATION
}

pub fn part2(input: &str) -> impl Display {
    let mut input = input.as_bytes();
    let mut sum: u32 = 0;

    let mut options: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    for line in input.lines() {
        let mut first = MaybeUninit::uninit();

        let mut first_idx = 0;
        'outer: while first_idx < line.len() {
            let b = line[first_idx];
            if b <= b'9' {
                let value = b - b'0';
                first.write(value);
                break;
            }

            for (oidx, option) in options.iter().enumerate() {
                if line.get(first_idx..first_idx + option.len()) == Some(option) {
                    let value = oidx as u8 + 1;
                    first.write(value);
                    break 'outer;
                }
            }
            first_idx += 1;
        }

        'outer: for idx in (first_idx..line.len()).rev() {
            let b = line[idx];
            if b <= b'9' {
                let value = b - b'0';
                sum += unsafe { first.assume_init() * 10 + value } as u32;
                break;
            }

            let mut oidx = 0;
            #[allow(clippy::explicit_counter_loop)]
            for option in options.iter() {
                if line.len() - idx >= 3 && line.get(idx..idx + option.len()) == Some(option) {
                    let value = oidx + 1;
                    sum += unsafe { first.assume_init() * 10 + value } as u32;
                    break 'outer;
                }
                oidx += 1
            }
        }
    }

    sum
}
