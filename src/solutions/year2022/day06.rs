use std::{
    collections::{BTreeSet, HashSet},
    fmt::Display,
};

pub fn part1(input: &str) -> impl Display {
    input
        .as_bytes()
        .array_windows::<4>()
        .position(|[a, b, c, d]| a != b && a != c && a != d && b != c && b != d && c != d)
        .unwrap()
        + 4
}

pub fn part2(input: &str) -> impl Display {
    input
        .as_bytes()
        .array_windows::<14>()
        .position(|arr| arr.iter().collect::<HashSet<_>>().len() == arr.len())
        .unwrap()
        + 14
}
