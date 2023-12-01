use std::{
    fmt::Display,
    num::{NonZeroU8, NonZeroUsize},
};

use bstr::ByteSlice;

pub fn part1(input: &str) -> impl Display {
    let mut input = input.as_bytes();
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut iter = line.iter().copied();
        let first = unsafe { iter.find(|&b| b <= b'9').unwrap_unchecked() & 0xf };
        let last = iter.rfind(|&b| b <= b'9').unwrap_or(first) & 0xf;
        sum += (first * 10 + last) as u32;
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let mut input = input.as_bytes();
    let mut sum: u32 = 0;

    let mut options: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    for line in input.lines() {
        let mut first = None;

        let mut first_idx = 0;
        'outer: while first_idx < line.len() {
            let b = unsafe { *line.get_unchecked(first_idx) };
            if b <= b'9' {
                let value = unsafe { NonZeroU8::new_unchecked(b - b'0') };
                first = Some(value);
                break;
            }

            for (oidx, option) in options.iter().enumerate() {
                if line.get(first_idx..first_idx + option.len()) == Some(option) {
                    let value = unsafe { NonZeroU8::new_unchecked(oidx as u8 + 1) };
                    first = Some(value);
                    break 'outer;
                }
            }
            first_idx += 1;
        }

        'outer: for idx in (first_idx..line.len()).rev() {
            let b = unsafe { *line.get_unchecked(idx) };
            if b <= b'9' {
                let value = (b - b'0');
                sum += unsafe { first.unwrap_unchecked().get() * 10 + value } as u32;
                break;
            }

            let mut oidx = 0;
            #[allow(clippy::explicit_counter_loop)]
            for option in options.iter() {
                if line.get(idx..idx + option.len()) == Some(option) {
                    let value = oidx + 1;
                    sum += unsafe { first.unwrap_unchecked().get() * 10 + value } as u32;
                    break 'outer;
                }
                oidx += 1
            }
        }
    }

    sum
}
