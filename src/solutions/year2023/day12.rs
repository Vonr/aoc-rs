use std::fmt::Display;

use bitvec::{bitarr, bitvec, order::LocalBits, view::BitView};
use bstr::ByteSlice;
use num_traits::{PrimInt, Unsigned};

use crate::helper::{
    parsing::{BytesAsNumber, PartialConsume},
    util::CollectToArray,
};

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut sum = 0;

    for (idx, mut line) in input.lines().enumerate() {
        let mut springs = line.skip_to_unit(b' ');
        let mut broken = 0u32;
        let mut normal = 0u32;
        let mut lengths: Vec<u32> = line.as_nums().collect::<Vec<_>>();

        for (idx, &spring) in springs.iter().enumerate() {
            if spring == b'#' {
                broken |= 1 << idx;
            } else if spring == b'.' {
                normal |= 1 << idx;
            }
        }

        'outer: for comb in 0..1 << springs.len() as u32 {
            if comb & normal != 0 {
                continue;
            }

            if comb & broken != broken {
                continue;
            }

            let mut num_secs = 0;
            for (idx, sec) in comb
                .view_bits::<LocalBits>()
                .split(|_pos, bit| !bit)
                .filter(|sec| !sec.is_empty())
                .enumerate()
            {
                num_secs += 1;
                if idx >= lengths.len() || sec.len() as u32 != lengths[idx] {
                    continue 'outer;
                }
            }

            if num_secs != lengths.len() {
                continue;
            }

            sum += 1;
        }
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut sum = 0;

    for (idx, mut line) in input.lines().enumerate() {
        println!("line {idx}");
        let mut springs = line.skip_to_unit(b' ').repeatn(5);
        let mut broken = bitarr![0; 100];
        let mut normal = bitarr![0; 100];
        let mut lengths: Vec<u32> = line.as_nums().collect::<Vec<_>>().repeat(5);

        for (idx, &spring) in springs.iter().enumerate() {
            if spring == b'#' {
                broken.set(idx, true);
            } else if spring == b'.' {
                normal.set(idx, true);
            }
        }

        'outer: for mut comb in 0u128..1 << springs.len() as u128 {
            if comb & 65535 == 0 {
                println!("{comb:0130b}");
            }
            if comb & unsafe { std::mem::transmute::<_, u128>(normal.data) } != 0 {
                continue;
            }

            if comb & unsafe { std::mem::transmute::<_, u128>(broken.data) }
                != unsafe { std::mem::transmute::<_, u128>(broken.data) }
            {
                continue;
            }

            let mut num_secs = 0;

            while comb != 0 {
                if comb & 1 == 0 {
                    comb = comb >> comb.trailing_zeros() as usize;
                } else {
                    num_secs += 1;
                    let chunk_size = comb.trailing_ones();

                    if num_secs >= lengths.len() || chunk_size != lengths[num_secs] {
                        continue 'outer;
                    }

                    comb >>= chunk_size as usize;
                }
            }

            if num_secs != lengths.len() {
                continue;
            }

            sum += 1;
        }
    }

    sum
}
