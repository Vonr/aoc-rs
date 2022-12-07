use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
};

pub fn part1(input: &str) -> impl Display {
    let mut sizes: BTreeMap<String, usize> = BTreeMap::new();
    let mut dir = String::new();
    let lines = input.lines();
    for (i, line) in lines.clone().enumerate() {
        match line {
            line if line.starts_with("$ cd") => match &line[5..] {
                "/" => dir = "/".to_owned(),
                ".." => {
                    dir = dir[..dir
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '/')
                        .last()
                        .unwrap()
                        .0]
                        .to_owned()
                }
                other => {
                    if !dir.ends_with('/') {
                        dir.push('/');
                    }
                    dir.push_str(&line[5..]);
                }
            },
            line if line.starts_with("$ ls") => {
                let old_dir = dir.clone();
                if !dir.ends_with('/') {
                    dir.push('/');
                }
                dir.push_str(&line[4..]);
                let dir_size = lines
                    .clone()
                    .skip(i + 1)
                    .take_while(|l| !l.starts_with('$'))
                    .filter_map(|l| {
                        if (b'0'..=b'9').contains(&l.as_bytes()[0]) {
                            return Some(l.split_once(' ').unwrap().0.parse::<usize>().unwrap());
                        }
                        None
                    })
                    .sum::<usize>();
                let iter = sizes.clone().into_iter();
                for (k, v) in iter.filter(|(k, _)| dir.starts_with(k.as_str())) {
                    sizes.insert(k.to_owned(), v + dir_size);
                }
                sizes.insert(dir, dir_size);
                dir = old_dir;
            }
            _ => (),
        }
    }
    sizes.values().filter(|v| **v <= 100000).sum::<usize>()
}

pub fn part2(input: &str) -> impl Display {
    let mut sizes: BTreeMap<String, usize> = BTreeMap::new();
    let mut dir = String::new();
    let lines = input.lines();
    for (i, line) in lines.clone().enumerate() {
        match line {
            line if line.starts_with("$ cd") => match &line[5..] {
                "/" => dir = "/".to_owned(),
                ".." => {
                    dir = dir[..dir
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '/')
                        .last()
                        .unwrap()
                        .0]
                        .to_owned()
                }
                other => {
                    if !dir.ends_with('/') {
                        dir.push('/');
                    }
                    dir.push_str(&line[5..]);
                }
            },
            line if line.starts_with("$ ls") => {
                let old_dir = dir.clone();
                if !dir.ends_with('/') {
                    dir.push('/');
                }
                dir.push_str(&line[4..]);
                let dir_size = lines
                    .clone()
                    .skip(i + 1)
                    .take_while(|l| !l.starts_with('$'))
                    .filter_map(|l| {
                        if (b'0'..=b'9').contains(&l.as_bytes()[0]) {
                            return Some(l.split_once(' ').unwrap().0.parse::<usize>().unwrap());
                        }
                        None
                    })
                    .sum::<usize>();
                let iter = sizes.clone().into_iter();
                for (k, v) in iter.filter(|(k, _)| dir.starts_with(k.as_str())) {
                    sizes.insert(k.to_owned(), v + dir_size);
                }
                sizes.insert(dir, dir_size);
                dir = old_dir;
            }
            _ => (),
        }
    }
    let sum = sizes.get("/").unwrap();
    let mut values = sizes
        .values()
        .filter(|v| 70000000 - *sum + *v > 30000000)
        .collect::<Vec<_>>();
    values.sort_unstable();
    *values[0]
}
