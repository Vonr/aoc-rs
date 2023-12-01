use std::fmt::Display;

use bstr::ByteSlice;

pub fn part1(input: &str) -> impl Display {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut first = None;
        let mut last = None;
        for &b in line.as_bytes() {
            if b.is_ascii_digit() {
                let value = b & 0xf;
                last = Some(value);
                first.get_or_insert(value);
            }
        }
        sum += unsafe { (first.unwrap_unchecked() * 10 + last.unwrap_unchecked()) as u32 };
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let mut input = input.as_bytes();
    let mut sum: usize = 0;

    let mut options: [&[u8]; 18] = [
        b"1", b"one", b"2", b"two", b"3", b"three", b"4", b"four", b"5", b"five", b"6", b"six",
        b"7", b"seven", b"8", b"eight", b"9", b"nine",
    ];

    for line in input.lines() {
        let mut first = None;
        let mut last = None;

        for idx in (0..line.len()) {
            for (oidx, option) in options.iter().enumerate() {
                if line.get(idx..idx + option.len()) == Some(option) {
                    let value = (oidx >> 1) + 1;
                    last = Some(value);
                    first.get_or_insert(value);
                    break;
                }
            }
        }

        sum += unsafe { first.unwrap_unchecked() * 10 + last.unwrap_unchecked() };
    }

    sum
}
