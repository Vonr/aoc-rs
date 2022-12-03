use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut score = 0;
    input.lines().for_each(|l| {
        l.split_once(' ')
            .map(|(a, b)| match b {
                "X" => match a {
                    "A" => score += 1 + 3,
                    "B" => score += 1 + 0,
                    "C" => score += 1 + 6,
                    _ => (),
                },
                "Y" => match a {
                    "A" => score += 2 + 6,
                    "B" => score += 2 + 3,
                    "C" => score += 2 + 0,
                    _ => (),
                },
                "Z" => match a {
                    "A" => score += 3 + 0,
                    "B" => score += 3 + 6,
                    "C" => score += 3 + 3,
                    _ => (),
                },
                _ => (),
            })
            .unwrap();
    });
    score
}

pub fn part2(input: &str) -> impl Display {
    let mut score = 0;
    input.lines().for_each(|l| {
        l.split_once(' ')
            .map(|(a, b)| match b {
                "X" => match a {
                    "A" => score += 3, // scissors
                    "B" => score += 1, // rock
                    "C" => score += 2, // paper
                    _ => (),
                },
                "Y" => match a {
                    "A" => score += 1 + 3, // rock
                    "B" => score += 2 + 3, // paper
                    "C" => score += 3 + 3, // scissors
                    _ => (),
                },
                "Z" => match a {
                    "A" => score += 2 + 6, // paper
                    "B" => score += 3 + 6, // scissors
                    "C" => score += 1 + 6, // rock
                    _ => (),
                },
                _ => (),
            })
            .unwrap();
    });
    score
}
