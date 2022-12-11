use std::{collections::HashMap, fmt::Display};

use bstr::ByteSlice;

use crate::helper::parsing::BytesAsNumber;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    pub items: Vec<usize>,
    pub op: u8,
    pub mag: usize,
    pub test: usize,
    pub t: usize,
    pub f: usize,
}

pub fn part1(input: &str) -> impl Display {
    let mut monkeys = Vec::new();
    let mut inspects: HashMap<usize, usize> = HashMap::new();
    let mut monkey = Monkey {
        items: vec![],
        op: b'+',
        mag: 0,
        test: 0,
        t: 0,
        f: 0,
    };
    input
        .as_bytes()
        .lines()
        .enumerate()
        .skip(1)
        .for_each(|(i, line)| match i % 7 {
            0 => {
                monkeys.push(monkey.clone());
                monkey.items.clear();
            }
            1 => {
                for item in line
                    .strip_prefix(b"  Starting items: ")
                    .unwrap()
                    .split_str(", ")
                {
                    monkey.items.push(unsafe { item.as_num() });
                }
            }
            2 => {
                let (op, mag) = line
                    .strip_prefix(b"  Operation: new = old ")
                    .unwrap()
                    .split_at(1);
                let mag = &mag[1..];
                match op {
                    b"+" => {
                        monkey.op = b'+';
                        unsafe {
                            monkey.mag = mag.as_num();
                        }
                    }
                    b"*" => {
                        if mag == b"old" {
                            monkey.op = b'^'
                        } else {
                            monkey.op = b'*';
                            unsafe {
                                monkey.mag = mag.as_num();
                            }
                        }
                    }
                    _ => (),
                };
            }
            3 => {
                monkey.test = unsafe {
                    line.strip_prefix(b"  Test: divisible by ")
                        .unwrap()
                        .as_num()
                };
            }
            4 => {
                monkey.t = unsafe {
                    line.strip_prefix(b"    If true: throw to monkey ")
                        .unwrap()
                        .as_num::<usize>()
                }
            }
            5 => {
                monkey.f = unsafe {
                    line.strip_prefix(b"    If false: throw to monkey ")
                        .unwrap()
                        .as_num::<usize>()
                }
            }
            _ => (),
        });
    monkeys.push(monkey);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            monkey.items.iter_mut().for_each(|it| {
                *it = match monkey.op {
                    b'+' => *it + monkey.mag,
                    b'*' => *it * monkey.mag,
                    b'^' => *it * *it,
                    _ => 0,
                } / 3
            });
            let clone = monkey.clone();
            let len = monkey.items.len();
            inspects.insert(i, inspects.get(&i).unwrap_or(&0usize) + len);
            monkey.items.clear();
            clone.items.iter().for_each(|it| {
                if it % clone.test == 0 {
                    monkeys[clone.t].items.push(*it);
                } else {
                    monkeys[clone.f].items.push(*it);
                }
            });
        }
    }

    let mut inspects = inspects
        .values()
        .into_iter()
        .copied()
        .collect::<Vec<usize>>();
    inspects.sort_unstable();
    let n = inspects.len();
    inspects[n - 1] * inspects[n - 2]
}

pub fn part2(input: &str) -> impl Display {
    let mut monkeys = Vec::new();
    let mut inspects: HashMap<usize, usize> = HashMap::new();
    let mut monkey = Monkey {
        items: vec![],
        op: b'+',
        mag: 0,
        test: 0,
        t: 0,
        f: 0,
    };
    input
        .as_bytes()
        .lines()
        .enumerate()
        .skip(1)
        .for_each(|(i, line)| match i % 7 {
            0 => {
                monkeys.push(monkey.clone());
                monkey.items.clear();
            }
            1 => {
                for item in line
                    .strip_prefix(b"  Starting items: ")
                    .unwrap()
                    .split_str(", ")
                {
                    monkey.items.push(unsafe { item.as_num() });
                }
            }
            2 => {
                let (op, mag) = line
                    .strip_prefix(b"  Operation: new = old ")
                    .unwrap()
                    .split_at(1);
                let mag = &mag[1..];
                match op {
                    b"+" => {
                        monkey.op = b'+';
                        unsafe {
                            monkey.mag = mag.as_num();
                        }
                    }
                    b"*" => {
                        if mag == b"old" {
                            monkey.op = b'^'
                        } else {
                            monkey.op = b'*';
                            unsafe {
                                monkey.mag = mag.as_num();
                            }
                        }
                    }
                    _ => (),
                };
            }
            3 => {
                monkey.test = unsafe {
                    line.strip_prefix(b"  Test: divisible by ")
                        .unwrap()
                        .as_num()
                };
            }
            4 => {
                monkey.t = unsafe {
                    line.strip_prefix(b"    If true: throw to monkey ")
                        .unwrap()
                        .as_num::<usize>()
                }
            }
            5 => {
                monkey.f = unsafe {
                    line.strip_prefix(b"    If false: throw to monkey ")
                        .unwrap()
                        .as_num::<usize>()
                }
            }
            _ => (),
        });
    monkeys.push(monkey);
    let lcm: usize = monkeys.iter().map(|m| m.test).fold(1, lcm);

    for step in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            monkey.items.iter_mut().for_each(|it| {
                *it = match monkey.op {
                    b'+' => *it + monkey.mag,
                    b'*' => *it * monkey.mag,
                    b'^' => *it * *it,
                    _ => 0,
                } % lcm
            });
            let clone = monkey.clone();
            let len = monkey.items.len();
            inspects.insert(i, inspects.get(&i).unwrap_or(&0usize) + len);
            monkey.items.clear();
            clone.items.iter().for_each(|it| {
                if *it % clone.test == 0 {
                    monkeys[clone.t].items.push(*it);
                } else {
                    monkeys[clone.f].items.push(*it);
                }
            });
        }
    }

    let mut inspects = inspects
        .values()
        .into_iter()
        .copied()
        .collect::<Vec<usize>>();
    inspects.sort_unstable();
    let n = inspects.len();
    inspects[n - 1] * inspects[n - 2]
}

const fn gcd(x: usize, y: usize) -> usize {
    let (mut a, mut b) = (x, y);

    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

const fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}
