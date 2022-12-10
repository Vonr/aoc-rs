use std::fmt::Display;

use bstr::{io::BufReadExt, ByteSlice, B};

use crate::helper::parsing::BytesAsNumber;

pub fn part1(input: &str) -> impl Display {
    let lines = input.lines().collect::<Vec<_>>();
    let mut cycle = 1;
    let mut reg = 1;
    let mut interest = 20;
    let mut sum = 0;

    for line in lines {
        if interest == cycle {
            sum += cycle * reg;
            interest += 40;
        }
        if let Some(x) = line.strip_prefix("addx ") {
            if interest == cycle + 1 {
                sum += (cycle + 1) * reg;
                interest += 40;
            }
            reg += x.parse::<i32>().unwrap();
            cycle += 2;
            continue;
        }
        cycle += 1;
    }
    sum
}

pub fn part2(input: &str) -> impl Display {
    let lines = input.lines().collect::<Vec<_>>();
    let mut cycle: i32 = 0;
    let mut reg: i32 = 1;
    let mut drawing = String::new();

    for line in lines {
        if reg.abs_diff(cycle) <= 1 {
            drawing.push('#');
        } else {
            drawing.push('.');
        }
        if let Some(x) = line.strip_prefix("addx ") {
            cycle += 1;
            if cycle == 40 {
                cycle = 0;
                drawing.push('\n');
            }
            if reg.abs_diff(cycle) <= 1 {
                drawing.push('#');
            } else {
                drawing.push('.');
            }
            reg += x.parse::<i32>().unwrap();
        }
        cycle += 1;
        if cycle == 40 {
            cycle = 0;
            drawing.push('\n');
        }
    }
    println!("{drawing}");

    panic!("This answer should be provided by the user.");
    0
}
