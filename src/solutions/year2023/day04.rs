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
    let mut card_info: Vec<[u32; 256]> = Vec::new();
    let mut cards: [u32; 256] = [0; 256];

    for (idx, mut line) in input.lines().enumerate() {
        line.skip_to_unit(b' ');
        let card: usize = line.skip_to_unit(b':').as_num();
        line.skip_to_unit(b' ');

        let left = line.skip_to_unit(b'|');

        let mut lhs: Vec<_> = left
            .split_str(b" ")
            .filter(|bs| !bs.is_empty())
            .map(|bs| bs.as_num::<u32>())
            .collect();

        let mut res = 0;

        let mut info = [0; 256];
        let mut rhs = Vec::new();
        for num in line
            .split(|&b| b == b' ')
            .filter(|bs| !bs.is_empty())
            .map(|bs| bs.as_num::<u32>())
        {
            rhs.push(num);
            if lhs.contains(&num) {
                res += 1;
            }
        }

        cards[card - 1] = 1;

        for info in info.iter_mut().skip(card).take(res as usize) {
            *info += 1;
        }

        card_info.push(info);

        lhs.clear();
    }

    for idx in (0..cards.len().min(card_info.len())) {
        let num = cards[idx];
        for (i, card) in card_info[idx].iter().enumerate() {
            cards[i] += card * num;
        }
    }

    cards.iter().sum::<u32>()
}
