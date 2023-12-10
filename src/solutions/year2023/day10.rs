use rustc_hash::FxHashSet;
use std::fmt::Display;

use crate::helper::matrix::Matrix;
use bstr::ByteSlice;
use pathfinding::prelude::*;

fn to_board(input: &[u8]) -> Matrix<u8> {
    Matrix::from_iter(input.lines())
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    #[inline]
    fn from_positions(from: (usize, usize), to: (usize, usize)) -> Self {
        use Direction::*;
        let (fr, fc) = from;
        let (tr, tc) = to;

        if tr > fr {
            return South;
        }

        if tr < fr {
            return North;
        }

        if tc > fc {
            return East;
        }

        West
    }

    #[inline]
    fn from_shape(shape: u8) -> Option<[Self; 2]> {
        use Direction::*;
        let out = match shape {
            b'|' => [North, South],
            b'-' => [West, East],
            b'L' => [North, East],
            b'J' => [North, West],
            b'7' => [South, West],
            b'F' => [South, East],
            _ => return None,
        };

        Some(out)
    }

    #[inline]
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn reachable(mut from: ((usize, usize), u8), mut to: ((usize, usize), u8)) -> bool {
    use Direction::*;

    if from.1 == b'.' || to.1 == b'.' || from.1 == b' ' || to.1 == b' ' {
        return false;
    }

    if to.1 == b'S' {
        (to, from) = (from, to);
    }

    let d = Direction::from_positions(from.0, to.0);

    let Some(td) = Direction::from_shape(to.1) else {
        return false;
    };
    let td = td.map(|d| d.opposite());

    if from.1 == b'S' {
        return td.contains(&d);
    }

    let Some(fd) = Direction::from_shape(from.1) else {
        unreachable!();
    };

    fd.contains(&d) && td.contains(&d)
}

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut board = to_board(input);

    let start = board.iter_elements().find(|(_, &v)| v == b'S').unwrap().0;

    let mut prev = start;

    let mut curr = if board[(start.0.saturating_sub(1), start.1)] == b'|' {
        (start.0.saturating_sub(1), start.1)
    } else if board.get(start.0 + 1, start.1).copied() == Some(b'|') {
        (start.0 + 1, start.1)
    } else if board[(start.0, start.1.saturating_sub(1))] == b'-' {
        (start.0, start.1.saturating_sub(1))
    } else if board.get(start.0, start.1 + 1).copied() == Some(b'-') {
        (start.0, start.1 + 1)
    } else {
        unreachable!()
    };

    let mut len = 1;
    while curr != start {
        let curr_shape = Direction::from_shape(board[curr]).unwrap();
        let prev_dir = Direction::from_positions(curr, prev);
        let curr_dir = curr_shape.into_iter().find(|&d| d != prev_dir).unwrap();

        prev = curr;
        match curr_dir {
            Direction::North => curr.0 -= 1,
            Direction::South => curr.0 += 1,
            Direction::East => curr.1 += 1,
            Direction::West => curr.1 -= 1,
        }

        len += 1;
    }

    len / 2
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut board = to_board(input);

    let mut board = {
        let mut new = Matrix::with_width_and_capacity(
            board.columns() * 2,
            board.columns() * 2 * board.rows() * 2,
        );
        let mut new_row = Vec::with_capacity(new.columns());
        for row in board.iter_rows() {
            for &p in row {
                new_row.push(p);
                if matches!(p, b'-' | b'L' | b'F') {
                    new_row.push(b'-');
                } else {
                    new_row.push(b' ');
                }
            }
            new.push(&new_row);
            new_row.clear();

            for &p in row {
                if matches!(p, b'|' | b'7' | b'F') {
                    new_row.push(b'|');
                } else {
                    new_row.push(b' ');
                }
                new_row.push(b' ');
            }
            new.push(&new_row);
            new_row.clear();
        }
        new
    };

    let rows = board.rows();
    let cols = board.columns();

    let start = board.iter_elements().find(|(_, &v)| v == b'S').unwrap();
    let start = (start.0, *start.1);

    let spos = start.0;
    if board[(spos.0.saturating_sub(1), spos.1)] == b' ' {
        board[(spos.0.saturating_sub(1), spos.1)] = b'|';
    }
    if board[((spos.0 + 1) % rows, spos.1)] == b' ' {
        board[((spos.0 + 1) % rows, spos.1)] = b'|';
    }
    if board[(spos.0, spos.1.saturating_sub(1))] == b' ' {
        board[(spos.0, spos.1.saturating_sub(1))] = b'-';
    }
    if board[(spos.0, (spos.1 + 1) % cols)] == b' ' {
        board[(spos.0, (spos.1 + 1) % cols)] = b'-';
    }

    let mut lp = bfs_reach(start.0, |&(x, y)| {
        let curr = (x, y);
        let neighbours = board.neighbours_with_indices(curr.0, curr.1);
        let curr = (curr, board[curr]);

        let mut out = Vec::new();

        for &nb in neighbours.iter().flatten() {
            let nb = (nb.0, *nb.1);
            if reachable(curr, nb) {
                out.push(nb.0)
            }
        }

        out
    })
    .collect::<FxHashSet<_>>();

    let mut tiles = board
        .iter_elements()
        .filter(|i| !lp.contains(&i.0) && i.0 .0 & 1 == 0 && i.0 .1 & 1 == 0)
        .map(|i| i.0)
        .collect::<FxHashSet<_>>();

    for r in 0..rows {
        if board[(r, 0)] == b'.' || board[(r, 0)] == b' ' {
            let reachable = bfs_reach((r, 0), |&(x, y)| {
                let curr = (x, y);
                let neighbours = board.neighbours_with_indices(curr.0, curr.1);
                let curr = (curr, board[curr]);

                let mut out = Vec::new();

                for &(nb, _) in neighbours.iter().flatten() {
                    if !lp.contains(&nb) {
                        lp.insert(nb);
                        out.push(nb);
                    }
                }

                out
            });

            for rch in reachable {
                tiles.remove(&rch);
            }
        }

        if board[(r, cols - 1)] == b'.' || board[(r, cols - 1)] == b' ' {
            let reachable = bfs_reach((r, cols - 1), |&(x, y)| {
                let curr = (x, y);
                let neighbours = board.neighbours_with_indices(curr.0, curr.1);
                let curr = (curr, board[curr]);

                let mut out = Vec::new();

                for &(nb, v) in neighbours.iter().flatten() {
                    if !lp.contains(&nb) {
                        lp.insert(nb);
                        out.push(nb);
                    }
                }

                out
            });

            for rch in reachable {
                tiles.remove(&rch);
            }
        }
    }

    for c in 0..cols {
        if board[(0, c)] == b'.' || board[(0, c)] == b' ' {
            let reachable = bfs_reach((0, c), |&(x, y)| {
                let curr = (x, y);
                let neighbours = board.neighbours_with_indices(curr.0, curr.1);
                let curr = (curr, board[curr]);

                let mut out = Vec::new();

                for &(nb, _) in neighbours.iter().flatten() {
                    if !lp.contains(&nb) {
                        lp.insert(nb);
                        out.push(nb);
                    }
                }

                out
            });

            for rch in reachable {
                tiles.remove(&rch);
            }
        }

        if board[(rows - 1, c)] == b'.' || board[(rows - 1, c)] == b' ' {
            let reachable = bfs_reach((rows - 1, c), |&(x, y)| {
                let curr = (x, y);
                let neighbours = board.neighbours_with_indices(curr.0, curr.1);
                let curr = (curr, board[curr]);

                let mut out = Vec::new();

                for &(nb, _) in neighbours.iter().flatten() {
                    if !lp.contains(&nb) {
                        lp.insert(nb);
                        out.push(nb);
                    }
                }

                out
            });

            for rch in reachable {
                tiles.remove(&rch);
            }
        }
    }

    for tile in tiles.iter() {
        if board[*tile] == b' ' {
            continue;
        }
        board[*tile] = b'#';
    }

    tiles.len()
}
