use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use bstr::io::BufReadExt;

use crate::helper::parsing::BytesAsNumber;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
struct End {
    pub x: isize,
    pub y: isize,
}

impl End {
    pub fn towards(&mut self, other: &Self) {
        if (self.x - other.x).abs().max((self.y - other.y).abs()) < 2 {
            return;
        }

        if self.y == other.y {
            self.x += (self.x < other.x) as isize;
            self.x -= (self.x > other.x) as isize;
            return;
        }
        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Less => {
                self.x += 1;
                self.y += (self.y < other.y) as isize;
                self.y -= (self.y > other.y) as isize;
            }
            std::cmp::Ordering::Equal => {
                self.y += (self.y < other.y) as isize;
                self.y -= (self.y > other.y) as isize;
            }
            std::cmp::Ordering::Greater => {
                self.x -= 1;
                self.y += (self.y < other.y) as isize;
                self.y -= (self.y > other.y) as isize;
            }
        }
    }

    pub const fn coords(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn up(&mut self) {
        self.y += 1;
    }

    pub fn down(&mut self) {
        self.y -= 1;
    }

    pub fn left(&mut self) {
        self.x -= 1;
    }

    pub fn right(&mut self) {
        self.x += 1;
    }

    pub fn go(&mut self, dir: u8) {
        match dir {
            b'U' => {
                self.up();
            }
            b'D' => {
                self.down();
            }
            b'L' => {
                self.left();
            }
            b'R' => {
                self.right();
            }
            _ => (),
        }
    }
}

pub fn part1(input: &str) -> impl Display {
    let mut head = End { x: 0, y: 0 };
    let mut tail = End { x: 0, y: 0 };

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(tail.coords());

    input.as_bytes().for_byte_line(|line| {
        let dir = line[0];
        let steps: isize = line[2..].as_num();

        for _ in 0..steps {
            head.go(dir);
            tail.towards(&head);
            visited.insert(tail.coords());
        }

        Ok(true)
    });

    visited.len()
}

pub fn part2(input: &str) -> impl Display {
    let mut head = End { x: 0, y: 0 };
    let mut tails: [End; 9] = Default::default();

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert((0, 0));

    input.as_bytes().for_byte_line(|line| {
        let dir = line[0];
        let steps: isize = line[2..].as_num();

        for _ in 0..steps {
            head.go(dir);
            tails[0].towards(&head);
            for j in 0..tails.len() - 1 {
                let fst = tails[j];
                tails[j + 1].towards(&fst);
            }
            visited.insert(tails[tails.len() - 1].coords());
        }

        Ok(true)
    });

    visited.len()
}
