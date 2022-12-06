use std::fmt::Display;

use crate::helper::parsing::IntoColumns;

pub fn part1(input: &str) -> impl Display {
    let mut blocks = input
        .lines()
        .take_while(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .into_columns()
        .filter(|c| !c.chars().all(|c| matches!(c, ' ' | '[' | ']')))
        .map(|s| s.trim_start().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for insn in input.lines().skip(blocks.len() + 1) {
        let split = insn.split(' ').collect::<Vec<_>>();
        let num: usize = split[1].parse().unwrap();
        let fr: usize = split[3].parse::<usize>().unwrap() - 1;
        let to: usize = split[5].parse::<usize>().unwrap() - 1;
        for _ in 0..num {
            let popped = blocks[fr].remove(0);
            blocks[to].insert(0, popped);
        }
    }
    blocks
        .into_iter()
        .map(|v| v.first().cloned().unwrap_or(' '))
        .collect::<String>()
}

pub fn part2(input: &str) -> impl Display {
    let mut blocks = input
        .lines()
        .take_while(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .into_columns()
        .filter(|c| !c.chars().all(|c| matches!(c, ' ' | '[' | ']')))
        .map(|s| s.trim_start().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for insn in input.lines().skip(blocks.len() + 1) {
        let split = insn.split(' ').collect::<Vec<_>>();
        let num: usize = split[1].parse().unwrap();
        let fr: usize = split[3].parse::<usize>().unwrap() - 1;
        let to: usize = split[5].parse::<usize>().unwrap() - 1;
        let mut popped = Vec::new();
        for _ in 0..num {
            popped.insert(0, blocks[fr].remove(0));
        }
        for pop in popped {
            blocks[to].insert(0, pop);
        }
    }
    blocks
        .into_iter()
        .map(|v| v.first().cloned().unwrap_or(' '))
        .collect::<String>()
}
