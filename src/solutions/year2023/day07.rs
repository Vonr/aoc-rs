use std::{cmp::Ordering, fmt::Display};

use bstr::ByteSlice;
use num_traits::FloatConst;

use crate::helper::parsing::{BytesAsNumber, PartialConsume};

const P1_VALUES: [u8; 91] = {
    let mut values = [0; 91];
    values[b'2' as usize] = 0;
    values[b'3' as usize] = 1;
    values[b'4' as usize] = 2;
    values[b'5' as usize] = 3;
    values[b'6' as usize] = 4;
    values[b'7' as usize] = 5;
    values[b'8' as usize] = 6;
    values[b'9' as usize] = 7;
    values[b'T' as usize] = 8;
    values[b'J' as usize] = 9;
    values[b'Q' as usize] = 10;
    values[b'K' as usize] = 11;
    values[b'A' as usize] = 12;

    values
};

const P2_VALUES: [u8; 91] = {
    let mut values = [0; 91];
    values[b'J' as usize] = 0;
    values[b'2' as usize] = 1;
    values[b'3' as usize] = 2;
    values[b'4' as usize] = 3;
    values[b'5' as usize] = 4;
    values[b'6' as usize] = 5;
    values[b'7' as usize] = 6;
    values[b'8' as usize] = 7;
    values[b'9' as usize] = 8;
    values[b'T' as usize] = 9;
    values[b'Q' as usize] = 10;
    values[b'K' as usize] = 11;
    values[b'A' as usize] = 12;

    values
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Type {
    FiveOak,
    FourOak,
    FullH,
    ThreeOak,
    TwoP,
    OneP,
    HighC,
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    ty: Type,
    cards: [u8; 5],
}

impl Hand {
    fn from_bytes(bytes: [u8; 5], values: [u8; 91], typer: impl Fn([u8; 5]) -> Type) -> Self {
        let valued = bytes.map(|b| values[b as usize]);
        Self {
            ty: typer(valued),
            cards: valued,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ty.partial_cmp(&other.ty) {
            Some(core::cmp::Ordering::Equal) => (),
            ord => return ord,
        }

        other.cards.partial_cmp(&self.cards)
    }
}

fn p1_typer(valued: [u8; 5]) -> Type {
    use Type::*;

    let mut cards: [u8; 13] = [0; 13];
    let mut kinds = 0;
    for (idx, card) in valued.iter().copied().enumerate() {
        let v = card as usize;
        cards[v] += 1;
        let num = cards[v];
        if num == 1 {
            kinds += 1;
        }

        if num == 5 {
            return FiveOak;
        }
    }

    for num in cards {
        if num == 4 {
            return FourOak;
        }

        if kinds == 2 && (num == 2 || num == 3) {
            return FullH;
        }

        if kinds == 3 && num == 3 {
            return ThreeOak;
        }

        if kinds == 3 && num == 2 {
            return TwoP;
        }

        if kinds == 4 && num == 2 {
            return OneP;
        }
    }

    HighC
}

fn p2_typer(valued: [u8; 5]) -> Type {
    use Type::*;

    let mut cards: [u8; 13] = [0; 13];
    let mut kinds = 0;
    for (idx, card) in valued.iter().copied().enumerate() {
        let v = card as usize;
        cards[v] += 1;
        let num = cards[v];
        if num == 1 {
            kinds += 1;
        }
    }

    if cards[0] != 0 {
        let max = *cards[1..].iter().max().unwrap();
        let max_idx = cards[1..].iter().position(|&num| num == max).unwrap() + 1;

        cards[max_idx] += cards[0];
        cards[0] = 0;
        kinds -= 1;
    }

    for num in cards {
        if num == 5 {
            return FiveOak;
        }

        if num == 4 {
            return FourOak;
        }

        if kinds == 2 && (num == 2 || num == 3) {
            return FullH;
        }

        if kinds == 3 && num == 3 {
            return ThreeOak;
        }

        if kinds == 3 && num == 2 {
            return TwoP;
        }

        if kinds == 4 && num == 2 {
            return OneP;
        }
    }

    HighC
}

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut hands = Vec::new();
    for mut line in input.lines() {
        let hand: [u8; 5] = line.skip_to_unit(b' ').try_into().unwrap();
        let hand = Hand::from_bytes(hand, P1_VALUES, p1_typer);
        let bid = line.as_num::<u32>();

        hands.push((hand, bid));
    }

    hands.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut ans = 0;
    let len = hands.len();
    for (idx, (hand, bid)) in hands.into_iter().enumerate() {
        ans += bid * (len - idx) as u32;
    }

    ans
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut hands = Vec::new();
    for mut line in input.lines() {
        let hand: [u8; 5] = line.skip_to_unit(b' ').try_into().unwrap();
        let hand = Hand::from_bytes(hand, P2_VALUES, p2_typer);
        let bid = line.as_num::<u32>();

        hands.push((hand, bid));
    }

    hands.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut ans = 0;
    let len = hands.len();
    for (idx, (hand, bid)) in hands.into_iter().enumerate() {
        ans += bid * (len - idx) as u32;
    }

    ans
}
