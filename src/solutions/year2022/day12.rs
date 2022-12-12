use std::{collections::HashMap, fmt::Display};

use bstr::ByteSlice;
use pathfinding::num_traits::SaturatingSub;

const AROUND: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn successors(matrix: &HashMap<(usize, usize), u8>, x: usize, y: usize) -> Vec<(usize, usize)> {
    AROUND
        .iter()
        .filter_map(|(dx, dy)| {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || ny < 0 {
                return None;
            }
            if let Some(v) = matrix.get(&(nx.try_into().unwrap(), ny.try_into().unwrap())) {
                if v.saturating_sub(matrix.get(&(x, y)).unwrap()) <= 1 {
                    return Some((nx as usize, ny as usize));
                }
            }
            None
        })
        .collect()
}

pub fn part1(input: &str) -> impl Display {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let matrix = input
        .as_bytes()
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, byte)| {
                    let byte = *byte;

                    match byte {
                        b'S' => {
                            start = (x, y);
                            ((x, y), 0)
                        }
                        b'E' => {
                            end = (x, y);
                            ((x, y), 25)
                        }
                        _ => ((x, y), byte - b'a'),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(usize, usize), u8>>();

    pathfinding::directed::bfs::bfs(
        &start,
        |(x, y)| successors(&matrix, *x, *y),
        |(x, y)| (*x, *y) == end,
    )
    .unwrap()
    .len()
        - 1
}

pub fn part2(input: &str) -> impl Display {
    let mut end: (usize, usize) = (0, 0);
    let matrix = input
        .as_bytes()
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, byte)| {
                    let byte = *byte;

                    match byte {
                        b'S' => ((x, y), 0),
                        b'E' => {
                            end = (x, y);
                            ((x, y), 25)
                        }
                        _ => ((x, y), byte - b'a'),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(usize, usize), u8>>();

    matrix
        .iter()
        .filter(|(_, v)| **v == 0u8)
        .filter_map(|(k, _)| {
            pathfinding::directed::bfs::bfs(
                k,
                |(x, y)| successors(&matrix, *x, *y),
                |(x, y)| (*x, *y) == end,
            )
            .map(|inner| inner.len() - 1)
        })
        .min()
        .unwrap()
}
