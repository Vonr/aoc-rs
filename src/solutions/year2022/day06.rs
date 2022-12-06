use std::{collections::BTreeSet, fmt::Display};

pub fn part1(input: &str) -> impl Display {
    input
        .as_bytes()
        .array_windows::<4>()
        .enumerate()
        .find(|(i, [a, b, c, d])| a != b && a != c && a != d && b != c && b != d && c != d)
        .unwrap()
        .0
        + 4
}

pub fn part2(input: &str) -> impl Display {
    input
        .as_bytes()
        .array_windows::<14>()
        .enumerate()
        .find(|(i, arr)| BTreeSet::from_iter(arr.iter()).len() == arr.len())
        .unwrap()
        .0
        + 14
}
