#![allow(unused)]
use std::{collections::BTreeMap, env::args, fmt::Display, process::ExitCode, time::Instant};

use aoc_driver::*;
use aoc_rs::solutions::*;

#[allow(clippy::complexity)]
pub fn main() -> ExitCode {
    let solutions = get_solutions();
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
