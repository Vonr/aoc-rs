use std::{collections::HashMap, fmt::Display, time::Instant};

use bstr::ByteSlice;
use rustc_hash::FxHashMap;

use crate::helper::{parsing::PartialConsume, util::IntegerIteratorExt};

type FakeHashMap<T> = [[[T; 26]; 26]; 26];

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut lines = input.lines();

    let instructions = lines.next().unwrap();

    lines.next();

    let mut map: FakeHashMap<([u8; 3], [u8; 3])> = Default::default();
    for mut line in lines {
        let start = line.skip_to_unit(b' ');

        line.skip_to_unit(b'(');
        let left = line.skip_to_unit(b',');
        line = &line[1..];
        let right = line.skip_to_unit(b')');

        map[(start[0] - b'A') as usize][(start[1] - b'A') as usize][(start[2] - b'A') as usize] =
            (left.try_into().unwrap(), right.try_into().unwrap());
    }

    let mut curr = *b"AAA";

    let mut steps = 0;
    while curr != *b"ZZZ" {
        let dirs =
            map[(curr[0] - b'A') as usize][(curr[1] - b'A') as usize][(curr[2] - b'A') as usize];
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

    let mut map: FakeHashMap<([u8; 3], [u8; 3])> = Default::default();
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

        map[(start[0] - b'A') as usize][(start[1] - b'A') as usize][(start[2] - b'A') as usize] =
            (left.try_into().unwrap(), right.try_into().unwrap());
    }

    let mut steps = 0;
    let mut reached: Vec<u64> = vec![0; starts.len()];
    let mut cycles: Vec<u64> = vec![0; starts.len()];
    let mut done = 0;

    let mut found_names: Vec<[u8; 3]> = Vec::with_capacity(starts.len());
    while done < starts.len() {
        for curr in starts.iter_mut() {
            let dirs = map[(curr[0] - b'A') as usize][(curr[1] - b'A') as usize]
                [(curr[2] - b'A') as usize];

            match instructions[steps % instructions.len()] {
                b'L' => *curr = dirs.0,
                b'R' => *curr = dirs.1,
                _ => unreachable!(),
            }

            if curr[2] == b'Z' {
                if let Some(idx) = found_names.iter().position(|n| n == curr) {
                    if cycles[idx] == 0 {
                        cycles[idx] = steps as u64 - reached[idx];
                        done += 1;
                    }
                } else {
                    reached[found_names.len()] = steps as u64;
                    found_names.push(*curr);
                }
            }
        }

        steps += 1;
    }

    cycles.into_iter().lcm().unwrap()
}
