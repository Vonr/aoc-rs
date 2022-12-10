use std::{
    fmt::Display,
    io::{BufWriter, Stderr, Write},
};

use bstr::{io::BufReadExt, ByteSlice, B};

use crate::helper::parsing::{BytesAsNumber, StripPrefixUnchecked};

pub fn part1(input: &str) -> impl Display {
    let mut cycle = 1;
    let mut reg = 1;
    let mut sum = 0;

    input.as_bytes().lines().for_each(|line| {
        if (cycle - 20) % 40 == 0 {
            sum += cycle * reg;
        }
        if let Some(x) = unsafe { line.strip_prefix_unchecked(b"addx") } {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                sum += cycle * reg;
            }
            reg += unsafe { x[1..].as_signed_num::<i32>() };
        }
        cycle += 1;
    });
    sum
}

pub fn part2(input: &str) -> impl Display {
    let mut cycle: i32 = 0;
    let mut reg: i32 = 1;
    let mut crt = [32u8; 40 * 6 + 6];

    let mut idx = 0;
    unsafe {
        input.as_bytes().lines().for_each(|line| {
            if reg.abs_diff(cycle) <= 1 {
                *crt.get_unchecked_mut(idx) = b'#';
            }
            idx += 1;
            if let Some(x) = line.strip_prefix(b"addx ") {
                cycle += 1;
                if cycle == 40 {
                    cycle = 0;
                    *crt.get_unchecked_mut(idx) = b'\n';
                    idx += 1;
                }
                if reg.abs_diff(cycle) <= 1 {
                    *crt.get_unchecked_mut(idx) = b'#';
                }
                idx += 1;
                reg += x.as_signed_num::<i32>();
            }
            cycle += 1;
            if cycle == 40 {
                cycle = 0;
                *crt.get_unchecked_mut(idx) = b'\n';
                idx += 1;
            }
        });
        let stdout = std::io::stderr().lock();
        let mut writer = BufWriter::with_capacity(40 * 6 + 6, stdout);
        writer.write_all(&crt);
    }

    ""
}
