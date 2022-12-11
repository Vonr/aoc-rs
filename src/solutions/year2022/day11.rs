use std::fmt::Display;

use bstr::ByteSlice;

use crate::helper::parsing::BytesAsNumber;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum Op {
    Add(usize),
    Mul(usize),
    #[default]
    Sqr,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Monkey {
    pub items: Vec<usize>,
    pub op: Op,
    pub test: usize,
    pub t: usize,
    pub f: usize,
}

fn parse(input: &[u8]) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut monkey = Monkey::default();
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
                for item in line[18..].split_str(", ") {
                    monkey.items.push(unsafe { item.as_num() });
                }
            }
            2 => {
                let (op, mag) = line[23..].split_at(1);
                let mag = &mag[1..];
                match op[0] {
                    b'+' => monkey.op = Op::Add(unsafe { mag.as_num() }),
                    b'*' => {
                        if mag[0] == b'o' {
                            monkey.op = Op::Sqr
                        } else {
                            monkey.op = Op::Mul(unsafe { mag.as_num() })
                        }
                    }
                    _ => (),
                };
            }
            3 => {
                monkey.test = unsafe { line[20..].as_num() };
            }
            4 => monkey.t = unsafe { line[29..].as_num::<usize>() },
            5 => monkey.f = unsafe { line[30..].as_num::<usize>() },
            _ => (),
        });
    monkeys.push(monkey);
    monkeys
}

pub fn part1(input: &str) -> impl Display {
    let mut monkeys = parse(input.as_bytes());
    let mut inspects;

    #[allow(clippy::correctness)]
    /// SAFETY: This is safe as the Vec is filled right after set_len
    unsafe {
        inspects = Vec::with_capacity(monkeys.len());
        inspects.set_len(monkeys.len());
        inspects.fill(0);
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            monkey.items.iter_mut().for_each(|it| {
                *it = match monkey.op {
                    Op::Add(mag) => *it + mag,
                    Op::Mul(mag) => *it * mag,
                    Op::Sqr => *it * *it,
                } / 3
            });
            let clone = monkey.clone();
            let len = monkey.items.len();
            inspects[i] += len;
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

    inspects.sort_unstable();
    let n = inspects.len();
    inspects[n - 1] * inspects[n - 2]
}

pub fn part2(input: &str) -> impl Display {
    let mut monkeys = parse(input.as_bytes());
    let mut inspects;

    #[allow(clippy::correctness)]
    /// SAFETY: This is safe as the Vec is filled right after set_len
    unsafe {
        inspects = Vec::with_capacity(monkeys.len());
        inspects.set_len(monkeys.len());
        inspects.fill(0);
    }

    let lcm: usize = monkeys.iter().map(|m| m.test).product();

    for step in 0..10000 {
        for i in 0..monkeys.len() {
            let (drained, test, t, f);
            {
                let mut monkey = &mut monkeys[i];
                monkey.items.iter_mut().for_each(|it| {
                    *it = match monkey.op {
                        Op::Add(mag) => *it + mag,
                        Op::Mul(mag) => *it * mag,
                        Op::Sqr => *it * *it,
                    } % lcm
                });
                let len = monkey.items.len();
                inspects[i] += len;

                t = monkey.t;
                f = monkey.f;
                test = monkey.test;
                drained = monkey.items.drain(..).collect::<Vec<_>>();
            }
            drained.into_iter().for_each(|it| {
                if it % test == 0 {
                    monkeys[t].items.push(it);
                } else {
                    monkeys[f].items.push(it);
                }
            });
        }
    }

    inspects.sort_unstable();
    let n = inspects.len();
    inspects[n - 1] * inspects[n - 2]
}
