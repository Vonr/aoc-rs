use std::{collections::HashMap, fmt::Display};

use bstr::ByteSlice;

use crate::helper::parsing::PartialConsume;

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut lines = input.lines();

    let instructions = lines.next().unwrap();

    lines.next();

    let mut map: HashMap<[u8; 3], ([u8; 3], [u8; 3])> = HashMap::new();
    for mut line in lines {
        let start = line.skip_to_unit(b' ');

        line.skip_to_unit(b'(');
        let left = line.skip_to_unit(b',');
        line = &line[1..];
        let right = line.skip_to_unit(b')');

        map.insert(
            start.try_into().unwrap(),
            (left.try_into().unwrap(), right.try_into().unwrap()),
        );
    }

    let mut curr = *b"AAA";

    let mut steps = 0;
    while curr != *b"ZZZ" {
        let dirs = map[&curr];
        match instructions[steps % instructions.len()] {
            b'L' => curr = dirs.0,
            b'R' => curr = dirs.1,
            _ => unreachable!(),
        }

        steps += 1;
    }

    steps
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut lines = input.lines();

    let instructions = lines.next().unwrap();

    lines.next();

    let mut map: HashMap<[u8; 3], ([u8; 3], [u8; 3])> = HashMap::new();
    let mut starts: Vec<[u8; 3]> = Vec::new();
    for mut line in lines {
        let start = line.skip_to_unit(b' ');

        line.skip_to_unit(b'(');
        let left = line.skip_to_unit(b',');
        line = &line[1..];
        let right = line.skip_to_unit(b')');

        if start[2] == b'A' {
            starts.push(start.try_into().unwrap());
        }

        map.insert(
            start.try_into().unwrap(),
            (left.try_into().unwrap(), right.try_into().unwrap()),
        );
    }

    let mut steps = 0;
    let mut reached = HashMap::new();
    let mut cycles = HashMap::new();

    while cycles.len() < starts.len() {
        for curr in starts.iter_mut() {
            let dirs = map[curr];
            match instructions[steps % instructions.len()] {
                b'L' => *curr = dirs.0,
                b'R' => *curr = dirs.1,
                _ => unreachable!(),
            }

            if curr[2] == b'Z' {
                if reached.contains_key(curr) {
                    if !cycles.contains_key(curr) {
                        cycles.insert(*curr, steps - reached[curr]);
                    }
                } else {
                    reached.insert(*curr, steps);
                }
            }
        }

        steps += 1;
    }

    cycles
        .values()
        .map(|&v| v as u128)
        .reduce(num::integer::lcm)
        .unwrap()
}
