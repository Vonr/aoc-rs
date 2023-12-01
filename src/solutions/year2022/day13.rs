use std::fmt::Display;

use bstr::ByteSlice;

enum Value {
    Integer(usize),
    List(Vec<Value>),
}

impl From<&[u8]> for Value {
    fn from(bytes: &[u8]) -> Self {
        let mut out = Self::List(Vec::new());
        let mut nests = 0;

        for (i, byte) in bytes.as_bytes().iter().enumerate() {
            let byte = *byte;

            let ele = match byte {
                b'[' => Self::from(&bytes[i + 1..]),
                b']' => todo!(),
                b'0'..=b'9' => todo!(),
                b',' => todo!(),
                _ => unreachable!(),
            };

            match out {
                Self::Integer(int) => todo!(),
                Self::List(list) => todo!(),
            }
        }
        out
    }
}

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();

    let mut sum = 0;
    let mut left = Value::List(Vec::new());
    let mut right = Value::List(Vec::new());
    for (i, line) in input.lines().enumerate() {
        match i % 3 {
            0 => {}
            1 => {}
            2 => {}
            _ => (),
        }
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    ""
}
