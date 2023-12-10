use rustc_hash::FxHashSet;
use std::fmt::Display;

use crate::helper::matrix::Matrix;
use bstr::ByteSlice;
use pathfinding::prelude::*;

fn to_board(input: &[u8]) -> Matrix<u8> {
    Matrix::from_iter(input.lines())
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum D {
    N,
    S,
    E,
    W,
}

impl D {
    fn from_positions(from: (usize, usize), to: (usize, usize)) -> Self {
        use D::*;
        let (fr, fc) = from;
        let (tr, tc) = to;

        if tr > fr {
            return S;
        }

        if tr < fr {
            return N;
        }

        if tc > fc {
            return E;
        }

        W
    }

    fn from_shape(shape: u8) -> Option<[Self; 2]> {
        use D::*;
        let out = match shape {
            b'|' => [N, S],
            b'-' => [W, E],
            b'L' => [N, E],
            b'J' => [N, W],
            b'7' => [S, W],
            b'F' => [S, E],
            _ => return None,
        };

        Some(out)
    }

    fn opp(&self) -> Self {
        match self {
            D::N => D::S,
            D::S => D::N,
            D::E => D::W,
            D::W => D::E,
        }
    }
}

fn reachable(mut from: ((usize, usize), u8), mut to: ((usize, usize), u8)) -> bool {
    use D::*;

    if from.1 == b'.' || to.1 == b'.' || from.1 == b' ' || to.1 == b' ' {
        return false;
    }

    if to.1 == b'S' {
        (to, from) = (from, to);
    }

    let d = D::from_positions(from.0, to.0);

    let Some(td) = D::from_shape(to.1) else {
        return false;
    };
    let td = td.map(|d| d.opp());

    if from.1 == b'S' {
        return td.contains(&d);
    }

    let Some(fd) = D::from_shape(from.1) else {
        unreachable!();
    };

    fd.contains(&d) && td.contains(&d)
}

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut board = to_board(input);

    let start = board.iter_elements().find(|(_, &v)| v == b'S').unwrap();
    let start = (start.0, *start.1);

    let res = dijkstra_all(&start.0, |&(x, y)| {
        let curr = (x, y);
        let neighbours = board.neighbours_with_indices(curr.0, curr.1);
        let curr = (curr, board[curr]);

        let mut out = Vec::new();

        for &nb in neighbours.iter().flatten() {
            let nb = (nb.0, *nb.1);
            if reachable(curr, nb) {
                out.push((nb.0, 1u64))
            }
        }

        out
    });

    res.values().max_by_key(|v| v.1).unwrap().1
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
