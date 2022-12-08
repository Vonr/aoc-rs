use std::fmt::Display;

use bstr::ByteSlice;

use crate::helper::parsing::IntoColumns;

pub fn part1(input: &str) -> impl Display {
    let rows = input.lines().collect::<Vec<_>>();
    let columns = input.lines().into_columns().collect::<Vec<_>>();
    let nrows = rows.len();
    let ncols = columns.len();

    let mut visible = 0;

    for (ri, r) in rows.iter().enumerate() {
        for (ci, c) in columns.iter().enumerate() {
            let t = r.chars().nth(ci).unwrap();

            let mut vis = true;
            for o in r.chars().take(ci) {
                if o >= t {
                    vis = false;
                    break;
                }
            }
            if vis {
                visible += 1;
                continue;
            }

            vis = true;
            for o in r.chars().rev().take(ncols - ci - 1) {
                if o >= t {
                    vis = false;
                    break;
                }
            }
            if vis {
                visible += 1;
                continue;
            }

            vis = true;
            for o in c.chars().take(ri) {
                if o >= t {
                    vis = false;
                    break;
                }
            }
            if vis {
                visible += 1;
                continue;
            }

            vis = true;
            for o in c.chars().rev().take(nrows - ri - 1) {
                if o >= t {
                    vis = false;
                    break;
                }
            }
            if vis {
                visible += 1;
                continue;
            }
        }
    }

    visible
}

pub fn part2(input: &str) -> impl Display {
    let rows = input.lines().collect::<Vec<_>>();
    let columns = input.lines().into_columns().collect::<Vec<_>>();
    let nrows = rows.len();
    let ncols = columns.len();

    let mut max = 0;

    for (ri, r) in rows.iter().enumerate() {
        for (ci, c) in columns.iter().enumerate() {
            let t = r.chars().nth(ci).unwrap();

            let mut score = 1;

            let mut trees = 0;
            for o in r.chars().skip(ci + 1) {
                trees += 1;
                if o >= t {
                    break;
                }
            }
            score *= trees;
            trees = 0;

            for o in r.chars().rev().skip(ncols - ci) {
                trees += 1;
                if o >= t {
                    break;
                }
            }
            score *= trees;
            trees = 0;

            for o in c.chars().skip(ri + 1) {
                trees += 1;
                if o >= t {
                    break;
                }
            }
            score *= trees;
            trees = 0;

            for o in c.chars().rev().skip(nrows - ri) {
                trees += 1;
                if o >= t {
                    break;
                }
            }
            score *= trees;
            trees = 0;

            max = if score > max { score } else { max };
        }
    }

    max
}
