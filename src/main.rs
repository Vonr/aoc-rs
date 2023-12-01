#![allow(unused)]
use std::{
    collections::BTreeMap,
    env::args,
    fmt::Display,
    process::ExitCode,
    time::{Duration, Instant},
};

use aoc_driver::*;
use aoc_rs::solutions::get_solutions;

#[allow(clippy::complexity)]
pub fn main() -> ExitCode {
    let solutions = get_solutions();
    let mut args = args();
    let binary_name = args.next().unwrap();

    let exit = move |exit_code| {
        eprintln!("Usage: {binary_name} <year> <day> <part> [-p] [-b]");
        std::process::exit(exit_code);
    };

    let year: i32 = args
        .next()
        .unwrap_or_else(|| return exit(1))
        .parse()
        .unwrap();
    let day: i32 = args
        .next()
        .unwrap_or_else(|| return exit(1))
        .parse()
        .unwrap();
    let part: i32 = args
        .next()
        .unwrap_or_else(|| return exit(1))
        .parse()
        .unwrap();

    let args = args.collect::<Vec<String>>();
    if args.iter().any(|s| s == "-h") {
        exit(0);
    }

    let session = std::fs::read_to_string(".session").unwrap();
    let session = session.trim_end();

    let path = format!("inputs/{}/{}.txt", year, day);
    let input = get_input_or_file(session, year, day, path).unwrap();

    let solution = &solutions.get(&year).unwrap()[day as usize - 1][part as usize - 1];
    if args.iter().any(|s| s == "-b") {
        let mut total = Duration::ZERO;
        let duration = Duration::from_secs(3);
        let true_start = Instant::now();
        let mut n = 0;
        while true_start.elapsed() < duration {
            let start = Instant::now();
            let _ = solution(&input);
            total += start.elapsed();
            n += 1;
        }
        eprintln!("Average time of {} runs: {:?}", n, total / n);
        return ExitCode::SUCCESS;
    }
    let start = Instant::now();
    let answer = solution(&input);
    eprintln!("Calculated in: {:?}", start.elapsed());
    if answer.is_empty() {
        return ExitCode::SUCCESS;
    }
    eprintln!("Answer: {answer}");

    if args.iter().any(|s| s == "-p") {
        if let Err(e) = post_answer(session, year, day, part, None::<String>, answer) {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
