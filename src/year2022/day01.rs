use std::{collections::BinaryHeap, fmt::Display};

pub fn part1(input: &str) -> impl Display {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<i32>().unwrap()).sum::<i32>())
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> impl Display {
    let mut heap: BinaryHeap<i32> = input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<i32>().unwrap()).sum())
        .collect();
    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}
