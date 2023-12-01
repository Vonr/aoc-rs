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
    let mut input = input.as_bytes();
    let mut lsum: u32 = 0;
    let mut rsum: u32 = 0;

    let mut options: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    let mut compensations_needed: u32 = 0;
    for line in input.lines() {
        let mut first = MaybeUninit::uninit();

        let mut first_idx = 0;
        'outer: while first_idx < line.len() {
            let b = line[first_idx];
            if b <= b'9' {
                first.write(b);
                compensations_needed += 10;
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
                lsum += unsafe { first.assume_init() } as u32;
                rsum += b as u32;
                compensations_needed += 1;
                break;
            }

            let mut oidx = 0;
            #[allow(clippy::explicit_counter_loop)]
            for option in options.iter() {
                if line.len() - idx >= 3 && line.get(idx..idx + option.len()) == Some(option) {
                    let value = oidx + 1;
                    lsum += unsafe { first.assume_init() } as u32;
                    rsum += value as u32;
                    break 'outer;
                }
                oidx += 1
            }
        }
    }

    const COMPENSATION: u32 = b'0' as u32;

    lsum * 10 + rsum - compensations_needed * COMPENSATION
}
