use std::{fmt::Display, ops::Range};

const unsafe fn unique<const N: usize>(arr: &[u8; N]) -> bool {
    let mut found = 0u32;
    let mut i = 0;

    while i < N {
        found |= 1u32.unchecked_shl((arr.get_unchecked(i).unchecked_sub(b'a')) as u32);
        i += 1;
    }
    if found.count_ones() as usize != N {
        return false;
    }
    true
}

fn solve<const N: usize>(input: &str) -> impl Display {
    input
        .as_bytes()
        .array_windows::<N>()
        .position(|w| unsafe { unique(w) })
        .unwrap()
        + N
}

pub fn part1(input: &str) -> impl Display {
    solve::<4>(input)
}

pub fn part2(input: &str) -> impl Display {
    solve::<14>(input)
}
