use std::{collections::VecDeque, fmt::Display};

use bstr::ByteSlice;

use crate::helper::{matrix::Matrix, parsing::BytesAsNumber};

fn to_board(input: &[u8]) -> Matrix<u8> {
    let mut out = Matrix::new();

    let width = input.find_byte(b'\n').unwrap() + 2;
    let border = vec![b'.'; width];
    out.push(&border);

    let mut buf = Vec::with_capacity(width);
    for line in input.lines() {
        buf.push(b'.');
        buf.extend_from_slice(line);
        buf.push(b'.');
        out.push(&buf);
        buf.clear()
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

    let mut sum: u32 = 0;

    for (idx, mut line) in board.iter_rows().enumerate().skip(1).take(board.rows() - 2) {
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

            if sym(line[beg - 1])
                || sym(line[end + 1])
                || board[idx - 1][beg - 1..=end + 1].iter().any(|&b| sym(b))
                || board[idx + 1][beg - 1..=end + 1].iter().any(|&b| sym(b))
            {
                sum += line[beg..=end].as_num::<u32>();
            }

            skipped += end + 1 - beg;
        }
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let board = to_board(input);

    let mut sum: u32 = 0;

    for (idx, mut line) in board.iter_rows().enumerate().skip(1).take(board.rows() - 2) {
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

                        let num = line[beg..=end].as_num::<u32>();
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
