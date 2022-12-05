use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    input
        .lines()
        .map(|l| {
            let fst = &l[..l.len() / 2];
            let snd = &l[l.len() / 2..];
            let mut ret: isize = 0;
            for c in fst.chars() {
                if snd.contains(c) {
                    ret = match c {
                        'a'..='z' => (c as u8 - b'a' + 1).into(),
                        'A'..='Z' => (c as u8 - b'A' + 1 + 26).into(),
                        _ => 0,
                    };
                    break;
                }
            }
            ret
        })
        .sum::<isize>()
}

pub fn part2(input: &str) -> impl Display {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[aa, ba, ca]| {
            let mut ret: isize = 0;
            for c in aa.chars() {
                if ba.contains(c) && ca.contains(c) {
                    ret = match c {
                        'a'..='z' => (c as u8 - b'a' + 1).into(),
                        'A'..='Z' => (c as u8 - b'A' + 1 + 26).into(),
                        _ => 0,
                    };
                    break;
                }
            }
            ret
        })
        .sum::<isize>()
}
