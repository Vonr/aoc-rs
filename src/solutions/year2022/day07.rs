use std::{collections::HashMap, fmt::Display};

use crate::helper::parsing::{BytesAsNumber, Position};

pub fn part1(input: &str) -> impl Display {
    let mut sizes: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut dir = Vec::new();
    let mut len: usize = 0;
    let mut size: usize = 0;

    for (i, line) in input.lines().enumerate() {
        if line.starts_with('$') {
            if len != 0 {
                sizes
                    .iter_mut()
                    .filter(|(k, _)| dir.starts_with(k))
                    .for_each(|(k, v)| *v += size);
                sizes.insert(dir.clone(), size);
                dir.splice(len.., []);
                len = 0;
                size = 0;
            }

            if let Some(after) = line.strip_prefix("$ cd ") {
                match after {
                    "/" => {
                        dir.clear();
                        dir.push(b'/');
                    }
                    ".." => {
                        dir.splice(dir.rposition(|c| *c == b'/').unwrap().., []);
                        if dir.is_empty() {
                            dir.push(b'/');
                        }
                    }
                    other => {
                        if dir.last().map(|l| *l != b'/').unwrap_or_default() {
                            dir.push(b'/');
                        }
                        dir.extend_from_slice(other.as_bytes());
                    }
                }
            } else if let Some(after) = line.strip_prefix("$ ls") {
                len = dir.len();
                if !dir.last().unwrap() == b'/' {
                    dir.push(b'/');
                }
                dir.extend_from_slice(after.trim_start().as_bytes());
            }
        } else if (b'0'..=b'9').contains(&line.as_bytes()[0]) {
            size += unsafe { line.split_once(' ').unwrap().0.as_bytes().as_num::<usize>() };
        }
    }
    sizes.values().filter(|v| **v <= 100000).sum::<usize>()
}

pub fn part2(input: &str) -> impl Display {
    let mut sizes: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut dir = Vec::new();
    let mut len: usize = 0;
    let mut size: usize = 0;

    for (i, line) in input.lines().enumerate() {
        if line.starts_with('$') {
            if len != 0 {
                sizes
                    .iter_mut()
                    .filter(|(k, _)| dir.starts_with(k))
                    .for_each(|(k, v)| *v += size);
                sizes.insert(dir.clone(), size);
                dir.splice(len.., []);
                len = 0;
                size = 0;
            }

            if let Some(after) = line.strip_prefix("$ cd ") {
                match after {
                    "/" => {
                        dir.clear();
                        dir.push(b'/');
                    }
                    ".." => {
                        dir.splice(dir.rposition(|c| *c == b'/').unwrap().., []);
                        if dir.is_empty() {
                            dir.push(b'/');
                        }
                    }
                    other => {
                        if dir.last().map(|l| *l != b'/').unwrap_or_default() {
                            dir.push(b'/');
                        }
                        dir.extend_from_slice(other.as_bytes());
                    }
                }
            } else if let Some(after) = line.strip_prefix("$ ls") {
                len = dir.len();
                if !dir.last().unwrap() == b'/' {
                    dir.push(b'/');
                }
                dir.extend_from_slice(after.trim_start().as_bytes());
            }
        } else if (b'0'..=b'9').contains(&line.as_bytes()[0]) {
            size += unsafe { line.split_once(' ').unwrap().0.as_bytes().as_num::<usize>() };
        }
    }

    let sum = sizes.get([b'/'].as_slice()).unwrap();
    let free = 70000000 - *sum;
    *sizes
        .values()
        .filter(|v| free + *v > 30000000)
        .min()
        .unwrap()
}
