use std::{fmt::Display, hint::unreachable_unchecked};

use bstr::ByteSlice;

use crate::helper::parsing::{BytesAsNumber, PartialConsume};

pub fn part1(input: &str) -> impl Display {
    const QUOTA: [u8; 3] = [12, 13, 14];

    let mut sum = 0;
    let input = input.as_bytes();
    'outer: for (game, line) in input.lines().enumerate() {
        let line = &mut &line[6..];
        line.skip_to_unit(b' ');

        while !line.is_empty() {
            let mut sets = line.skip_to_unit(b';');
            line.skip_to_unit(b' ');
            let mut sums: [u8; 3] = [0; 3];

            while !sets.is_empty() {
                let mut set = sets.skip_to_unit(b',');
                sets.skip_to_unit(b' ');

                let num = set.skip_to_unit(b' ');
                let ty = match set.next().unwrap() {
                    b'r' => {
                        set.skip_n(3);
                        0
                    }
                    b'g' => {
                        set.skip_n(5);
                        1
                    }
                    b'b' => {
                        set.skip_n(4);
                        2
                    }
                    _ => unsafe { unreachable_unchecked() },
                };

                sums[ty] += num.as_num::<u8>();

                if sums[ty] > QUOTA[ty] {
                    continue 'outer;
                }
            }
        }

        sum += game + 1;
    }

    sum
}

pub fn part2(input: &str) -> impl Display {
    let mut sum: u32 = 0;
    let input = input.as_bytes();

    'outer: for mut line in input.lines() {
        let mut cut_off: &[u8] = &[];

        let line = &mut &line[6..];
        line.skip_to_unit(b' ');

        let mut mins: [u32; 3] = [0; 3];
        while !line.is_empty() {
            let num = line.skip_to_unit(b' ');

            let ty = match line.next().unwrap() {
                b'r' => {
                    line.skip_n(2);
                    0
                }
                b'g' => {
                    line.skip_n(4);
                    1
                }
                b'b' => {
                    line.skip_n(3);
                    2
                }
                _ => unsafe { unreachable_unchecked() },
            };

            mins[ty] = mins[ty].max(num.as_num());
        }
        sum += mins[0] * mins[1] * mins[2];
    }

    sum
}
