use std::fmt::Display;

fn rotate(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut out = Vec::new();

    for i in 0..v[0].len() {
        let mut row = Vec::new();

        for e in v.iter().map(|v2| v2[i]) {
            if e != ' ' {
                row.push(e)
            }
        }
        out.push(row);
    }

    out
}

pub fn part1(input: &str) -> impl Display {
    let mut blocks = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut vec = l
                .chars()
                .array_chunks::<4>()
                .map(|b| b[1])
                .collect::<Vec<char>>();
            vec.push(l.chars().nth(l.len() - 2).unwrap());
            vec
        })
        .collect::<Vec<_>>();
    blocks.pop();
    let mut blocks = rotate(blocks);
    for insn in input.lines().skip(blocks.len() + 1) {
        let split = insn.split(' ').collect::<Vec<_>>();
        let num: usize = split[1].parse().unwrap();
        let fr: usize = split[3].parse::<usize>().unwrap() - 1;
        let to: usize = split[5].parse::<usize>().unwrap() - 1;
        for _ in 0..num {
            let popped = blocks[fr].remove(0);
            blocks[to].insert(0, popped);
        }
    }
    blocks
        .into_iter()
        .map(|v| v.first().cloned().unwrap_or(' '))
        .collect::<String>()
}

pub fn part2(input: &str) -> impl Display {
    let mut blocks = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut vec = l
                .chars()
                .array_chunks::<4>()
                .map(|b| b[1])
                .collect::<Vec<char>>();
            vec.push(l.chars().nth(l.len() - 2).unwrap());
            vec
        })
        .collect::<Vec<_>>();
    blocks.pop();
    let mut blocks = rotate(blocks);
    for insn in input.lines().skip(blocks.len() + 1) {
        let split = insn.split(' ').collect::<Vec<_>>();
        let num: usize = split[1].parse().unwrap();
        let fr: usize = split[3].parse::<usize>().unwrap() - 1;
        let to: usize = split[5].parse::<usize>().unwrap() - 1;
        let mut popped = Vec::new();
        for _ in 0..num {
            popped.insert(0, blocks[fr].remove(0));
        }
        for pop in popped {
            blocks[to].insert(0, pop);
        }
    }
    blocks
        .into_iter()
        .map(|v| v.first().cloned().unwrap_or(' '))
        .collect::<String>()
}
