use std::fmt::Display;

use bstr::{io::BufReadExt, ByteSlice};

use crate::helper::parsing::IntoColumns;

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let nrows = input.byte_lines().count();
    let ncols = input.find_byte(b'\n').unwrap();

    let mut visible = 0;

    let input = input.as_bytes().replace(b"\n", b"");
    for (i, t) in input.iter().copied().enumerate() {
        let ri = i / ncols;
        let ci = i % ncols;

        let mut vis = true;
        for j in (0..ci).rev() {
            if input[ri * nrows + j] >= t {
                vis = false;
                break;
            }
        }
        if vis {
            visible += 1;
            continue;
        }

        vis = true;
        for j in ci + 1..ncols {
            if input[ri * nrows + j] >= t {
                vis = false;
                break;
            }
        }
        if vis {
            visible += 1;
            continue;
        }

        vis = true;
        for j in (0..ri).rev() {
            if input[j * ncols + ci] >= t {
                vis = false;
                break;
            }
        }
        if vis {
            visible += 1;
            continue;
        }

        vis = true;
        for j in ri + 1..nrows {
            if input[j * ncols + ci] >= t {
                vis = false;
                break;
            }
        }
        if vis {
            visible += 1;
            continue;
        }
    }

    visible
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let nrows = input.byte_lines().count();
    let ncols = input.find_byte(b'\n').unwrap();

    let mut max = 0;

    let input = input.as_bytes().replace(b"\n", b"");
    for (i, t) in input.iter().copied().enumerate() {
        let ri = i / ncols;
        let ci = i % ncols;

        let mut score = 1;

        let mut line_score = 0;
        for j in (0..ci).rev() {
            line_score += 1;
            if input[ri * nrows + j] >= t {
                break;
            }
        }
        score *= line_score;
        line_score = 0;
        for j in ci + 1..ncols {
            line_score += 1;
            if input[ri * nrows + j] >= t {
                break;
            }
        }
        score *= line_score;
        line_score = 0;
        for j in (0..ri).rev() {
            line_score += 1;
            if input[j * ncols + ci] >= t {
                break;
            }
        }
        score *= line_score;
        line_score = 0;
        for j in ri + 1..nrows {
            line_score += 1;
            if input[j * ncols + ci] >= t {
                break;
            }
        }
        score *= line_score;

        max = if score > max { score } else { max };
    }

    max
}
