use std::fmt::Display;

use bstr::ByteSlice;

use crate::helper::{parsing::BytesAsNumber, util::CollectToArray};

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut sum = 0;

    let mut seq = [0; 21];

    for mut line in input.lines() {
        line.as_signed_nums().collect_into_array(&mut seq);

        let mut layers = 0;

        while !seq[..seq.len() - 1 - layers].iter().all(|&d| d == 0) {
            sum += seq[seq.len() - 1 - layers];

            let mut i = 0;
            while i < seq.len() - 1 - layers {
                seq[i] = seq[i + 1] - seq[i];
                i += 1;
            }

            layers += 1;
        }
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut sum = 0;

    let mut seq = [0; 21];
    let mut lmosts = [0; 21];

    for mut line in input.lines() {
        line.as_signed_nums().collect_into_array(&mut seq);

        let mut layers = 0;

        while !seq[..seq.len() - 1 - layers].iter().all(|&d| d == 0) {
            let lmost = seq[0];
            lmosts[layers] = lmost;

            let mut i = 0;
            while i < seq.len() - 1 - layers {
                seq[i] = seq[i + 1] - seq[i];
                i += 1;
            }

            layers += 1;
        }

        seq[layers] = 0;

        let mut extra = 0;
        while layers != 0 {
            layers -= 1;
            extra = lmosts[layers] - extra;
        }

        sum += extra;
    }

    sum
}
