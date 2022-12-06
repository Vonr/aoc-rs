use std::fmt::Display;

use crate::helper::util::Unique;

pub fn solve(input: &str, size: usize) -> impl Display {
    input
        .as_bytes()
        .windows(size)
        .position(Unique::unique)
        .unwrap()
        + size
}

pub fn part1(input: &str) -> impl Display {
    solve(input, 4)
}

pub fn part2(input: &str) -> impl Display {
    solve(input, 14)
}
