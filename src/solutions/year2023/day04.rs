use std::fmt::Display;

use bstr::ByteSlice;

use crate::helper::parsing::{BytesAsNumber, PartialConsume};

pub fn part1(input: &str) -> impl Display {
    let mut input = input.as_bytes();
    let mut lhs = Vec::new();
    let mut sum: u32 = 0;

    for mut line in input.lines() {
        line.skip_to_unit(b':');
        line.skip_to_unit(b' ');

        let left = line.skip_to_unit(b'|');

        lhs.extend(
            left.split(|&b| b == b' ')
                .filter(|bs| !bs.is_empty())
                .map(|bs| bs.as_num::<u32>()),
        );

        let mut res = 0;

        for num in line
            .split(|&b| b == b' ')
            .filter(|bs| !bs.is_empty())
            .map(|bs| bs.as_num::<u32>())
        {
            if !lhs.contains(&num) {
                continue;
            }
            res += res.max(1)
        }

        lhs.clear();

        sum += res;
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
            .split(|&b| b == b' ')
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
