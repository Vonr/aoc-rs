#![feature(iter_array_chunks)]
#![allow(unused)]
use std::{collections::BTreeMap, env::args, fmt::Display, process::ExitCode, time::Instant};

use aoc_driver::*;

mod year2022;

#[allow(clippy::complexity)]
pub fn main() -> ExitCode {
    macro_rules! solutions {
        {$($years:expr => [$([$($parts:expr),*]),*$(,)?]),*$(,)?} => {{
            fn translate<D: Display + 'static>(f: fn(&str) -> D) -> Box<dyn Fn(&str) -> String> {
                Box::new(move |x| f(x).to_string())
            }

            let mut sols: BTreeMap<i32, Vec<Vec<Box<dyn Fn(&str) -> String>>>> = BTreeMap::new();
            $(
                sols.insert($years, vec![$(
                    vec![$(
                        translate($parts)
                    ),*]
                ),*]);
            )*
            sols
        }};
    }
    let solutions = solutions! {
        2022 => [
            [year2022::day01::part1, year2022::day01::part2],
            [year2022::day02::part1, year2022::day02::part2],
            [year2022::day03::part1, year2022::day03::part2],
            [year2022::day04::part1, year2022::day04::part2],
            [year2022::day05::part1, year2022::day05::part2],
            [year2022::day06::part1, year2022::day06::part2],
            [year2022::day07::part1, year2022::day07::part2],
            [year2022::day08::part1, year2022::day08::part2],
            [year2022::day09::part1, year2022::day09::part2],
            [year2022::day10::part1, year2022::day10::part2],
            [year2022::day11::part1, year2022::day11::part2],
            [year2022::day12::part1, year2022::day12::part2],
            [year2022::day13::part1, year2022::day13::part2],
            [year2022::day14::part1, year2022::day14::part2],
            [year2022::day15::part1, year2022::day15::part2],
            [year2022::day16::part1, year2022::day16::part2],
            [year2022::day17::part1, year2022::day17::part2],
            [year2022::day18::part1, year2022::day18::part2],
            [year2022::day19::part1, year2022::day19::part2],
            [year2022::day20::part1, year2022::day20::part2],
            [year2022::day21::part1, year2022::day21::part2],
            [year2022::day22::part1, year2022::day22::part2],
            [year2022::day23::part1, year2022::day23::part2],
            [year2022::day24::part1, year2022::day24::part2],
            [year2022::day25::part1, year2022::day25::part2],
        ],
    };

    let args = args().collect::<Vec<String>>();
    if args.len() < 4 {
        eprintln!("Usage: {} <year> <day> <part> [-p]", args[0]);
        return ExitCode::FAILURE;
    }

    let year: i32 = args[1].parse().unwrap();
    let day: i32 = args[2].parse().unwrap();
    let part: i32 = args[3].parse().unwrap();

    let session = std::fs::read_to_string(".session").unwrap();
    let session = session.trim_end();

    let path = format!("inputs/{}/{}.txt", year, day);
    let input = get_input_or_file(session, year, day, path).unwrap();

    let solution = &solutions.get(&year).unwrap()[day as usize - 1][part as usize - 1];
    let start = Instant::now();
    let answer = solution(&input);
    println!("Calculated in: {:?}", Instant::now().duration_since(start));
    println!("Answer: {}", answer);

    if args.contains(&"-p".to_owned()) {
        post_answer(session, year, day, part, None::<String>, answer).unwrap();
    }
    ExitCode::SUCCESS
}
