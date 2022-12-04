use std::{collections::BTreeSet, fmt::Display};

pub fn part1(input: &str) -> impl Display {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let a = (a1.parse::<u8>().unwrap()..=a2.parse().unwrap()).collect::<BTreeSet<_>>();
            let (b1, b2) = b.split_once('-').unwrap();
            let b = (b1.parse::<u8>().unwrap()..=b2.parse().unwrap()).collect::<BTreeSet<_>>();
            (a.is_subset(&b) || b.is_subset(&a)) as u64
        })
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl Display {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let a = (a1.parse::<u8>().unwrap()..=a2.parse().unwrap()).collect::<BTreeSet<_>>();
            let (b1, b2) = b.split_once('-').unwrap();
            let b = (b1.parse::<u8>().unwrap()..=b2.parse().unwrap()).collect::<BTreeSet<_>>();
            a.intersection(&b).any(|_| true) as u64
        })
        .sum::<u64>()
}
