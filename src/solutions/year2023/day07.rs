use std::{cmp::Ordering, fmt::Display};

use bstr::ByteSlice;

use crate::helper::parsing::{BytesAsNumber, PartialConsume};

fn p1_valuator(card: u8) -> u8 {
    match card {
        b'2'..=b'9' => (card - b'2'),
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!(),
    }
}

fn p2_valuator(card: u8) -> u8 {
    match card {
        b'J' => 0,
        b'2'..=b'9' => (card - b'1'),
        b'T' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!(),
    }
}

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
struct Cards([u8; 5]);

impl PartialEq for Cards {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        for (&a, &b) in self.0.iter().zip(other.0.iter()) {
            if a > b {
                return Some(Ordering::Less);
            }

            if a < b {
                return Some(Ordering::Greater);
            }
        }

        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    ty: Type,
    cards: Cards,
}

impl Hand {
    fn from_bytes(
        bytes: [u8; 5],
        valuator: impl Fn(u8) -> u8,
        typer: impl Fn([u8; 5]) -> Type,
    ) -> Self {
        let valued = bytes.map(valuator);
        Self {
            ty: typer(valued),
            cards: Cards(valued),
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

        self.cards.partial_cmp(&other.cards)
    }
}

fn p1_typer(valued: [u8; 5]) -> Type {
    use Type::*;

    let mut cards: [u8; 14] = [0; 14];
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

    let mut cards: [u8; 14] = [0; 14];
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
        let max_idx = cards[1..]
            .iter()
            .position(|&num| num == *cards[1..].iter().max().unwrap())
            .unwrap()
            + 1;

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
        let hand = Hand::from_bytes(hand, p1_valuator, p1_typer);
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
        let hand = Hand::from_bytes(hand, p2_valuator, p2_typer);
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
