use std::{collections::VecDeque, fmt::Display};

use bstr::ByteSlice;
use nom::{bytes::streaming::take_while1, Parser};
use nom_supreme::ParserExt;

use crate::helper::parsing::BytesAsNumber;

fn to_board(input: &[u8]) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<_>> = Vec::new();

    let border = vec![b'.'; input.find_byte(b'\n').unwrap() - 1];
    out.push(border.clone());

    for line in input.lines() {
        let mut new = Vec::with_capacity(line.len() + 2);
        new.push(b'.');
        new.extend_from_slice(line);
        new.push(b'.');
        out.push(new);
    }

    out.push(border);

    out
}

fn sym(b: u8) -> bool {
    b != b'.' && !b.is_ascii_digit()
}

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let board = to_board(input);

    let mut sum: u64 = 0;

    for (idx, mut line) in board.iter().enumerate().skip(1).rev().skip(1).rev() {
        let mut skipped: usize = 0;
        while line.len() > skipped {
            if !line[skipped].is_ascii_digit() {
                skipped += 1;
                continue;
            }

            let beg = skipped;
            let mut end = beg;

            while line[end + 1].is_ascii_digit() {
                end += 1;
            }

            let mut num = &line[beg..=end];

            if sym(line[skipped - 1])
                || sym(line[skipped + num.len()])
                || board[idx - 1][skipped - 1..=skipped + num.len()]
                    .iter()
                    .any(|&b| sym(b))
                || board[idx + 1][skipped - 1..=skipped + num.len()]
                    .iter()
                    .any(|&b| sym(b))
            {
                let num = num.as_num::<u64>();
                sum += num;
            }

            skipped += num.len();
        }
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let board = to_board(input);

    let mut sum: u64 = 0;

    for (idx, mut line) in board.iter().enumerate().skip(1).rev().skip(1).rev() {
        let mut skipped: usize = 1;
        'outer: while let Some(gear_idx) = line[skipped..].find_byte(b'*') {
            let mut found = 0;
            skipped += gear_idx + 1;

            let mut seen = [(0, 0); 2];
            let mut ans = 1;

            for row in 0..3 {
                for col in 0..3 {
                    let y = idx + row - 1;
                    let x = skipped + col - 2;

                    if board[y][x].is_ascii_digit() {
                        let line = &board[y];
                        let mut beg = x;
                        let mut end = x;
                        while line[beg - 1].is_ascii_digit() {
                            beg -= 1;
                        }

                        if seen[..found].contains(&(y, beg)) {
                            continue;
                        }

                        if found > 2 {
                            continue 'outer;
                        }

                        while line[end + 1].is_ascii_digit() {
                            end += 1;
                        }

                        seen[found] = (y, beg);
                        found += 1;

                        let num = line[beg..=end].as_num::<u64>();
                        ans *= num;
                    }
                }
            }

            if found == 2 {
                sum += ans;
            }
        }
    }

    sum
}
