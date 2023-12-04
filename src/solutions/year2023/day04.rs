use std::{fmt::Display, ops::BitOr};

use bstr::ByteSlice;

use crate::helper::parsing::{BytesAsNumber, PartialConsume};

pub fn part1(input: &str) -> impl Display {
    let mut input = input.as_bytes();
    let mut sum: u32 = 0;

    for mut line in input.lines() {
        line.skip_to_unit(b':');
        line = &line[1..];
        let mut left = line.skip_to_unit(b'|');

        let mut lhs: u128 = 0;
        while left.len() > 2 {
            let [a, b]: [u8; 2] = left[..2].try_into().unwrap();

            if a != b' ' {
                lhs |= 1u128 << ((a - b'0') as u32 * 10 + (b - b'0') as u32);
            } else {
                lhs |= 1 << (b - b'0');
            }

            left = &left[3..];
        }

        let mut rhs = 0;
        let mut right = &line[1..];
        loop {
            let [a, b]: [u8; 2] = right[..2].try_into().unwrap();

            if a != b' ' {
                rhs |= 1u128 << ((a - b'0') as u32 * 10 + (b - b'0') as u32);
            } else {
                rhs |= 1 << (b - b'0');
            }

            if right.len() <= 3 {
                break;
            }
            right = &right[3..];
        }

        let z = (lhs & rhs).count_ones();
        if z != 0 {
            sum += 1u32 << (z - 1);
        }
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let mut input = input.as_bytes();
    let mut card_info: Vec<u128> = Vec::new();
    let mut cards: [u32; 256] = [0; 256];

    for (idx, mut line) in input.lines().enumerate() {
        cards[idx] = 1;
        line.skip_to_unit(b':');
        line = &line[1..];
        let mut left = line.skip_to_unit(b'|');

        let mut lhs: u128 = 0;
        while left.len() > 2 {
            let [a, b]: [u8; 2] = left[..2].try_into().unwrap();

            if a != b' ' {
                lhs |= 1u128 << ((a - b'0') as u32 * 10 + (b - b'0') as u32);
            } else {
                lhs |= 1 << (b - b'0');
            }

            left = &left[3..];
        }

        let mut rhs = 0;
        let mut right = &line[1..];
        loop {
            let [a, b]: [u8; 2] = right[..2].try_into().unwrap();

            if a != b' ' {
                rhs |= 1u128 << ((a - b'0') as u32 * 10 + (b - b'0') as u32);
            } else {
                rhs |= 1 << (b - b'0');
            }

            if right.len() <= 3 {
                break;
            }
            right = &right[3..];
        }

        let res = (lhs & rhs).count_ones();

        let mut info = 0;
        for card in 0..res as usize {
            info |= 1u128 << card;
        }

        card_info.push(info);
    }

    for idx in 0..card_info.len() {
        let num = cards[idx];
        let mut info = card_info[idx];

        let mut bit: u32 = 1;

        while info != 0 {
            cards[idx + bit as usize] += (info & 1) as u32 * num;
            let tz = info.trailing_zeros().max(1);
            info >>= tz;
            bit += tz;
        }
    }

    cards.iter().sum::<u32>()
}
