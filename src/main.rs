#![allow(unused)]
use std::{
    collections::BTreeMap,
    env::args,
    fmt::Display,
    hint::black_box,
    process::ExitCode,
    time::{Duration, Instant},
};

use aoc_driver::*;
use aoc_rs::solutions::{solutions, stubs};

#[allow(clippy::complexity)]
pub fn main() -> ExitCode {
    let mut args = args();
    let binary_name = args.next().unwrap();

    let exit = move |exit_code| {
        eprintln!("Usage: {binary_name} <year> <day> <part> [-p] [-b]");
        std::process::exit(exit_code);
    };

    let year: i32 = args.next().unwrap_or_else(|| exit(1)).parse().unwrap();
    let day: i32 = args.next().unwrap_or_else(|| exit(1)).parse().unwrap();
    let part: i32 = args.next().unwrap_or_else(|| exit(1)).parse().unwrap();

    let args = args.collect::<Vec<String>>();
    if args.iter().any(|s| s == "-h") {
        exit(0);
    }

    let session = std::fs::read_to_string(".session").unwrap();
    let session = session.trim_end();

    let path = format!("inputs/{}/{}.txt", year, day);
    let input = get_input_or_file(session, year, day, path).unwrap();

    if args.iter().any(|s| s == "-b") {
        let stubs = stubs();
        let stub = &stubs.get(&year).unwrap()[day as usize - 1][part as usize - 1];

        eprintln!("Warming up...");
        const WARMUP: Duration = Duration::from_secs(2);
        let warmup_start = Instant::now();
        while warmup_start.elapsed() < WARMUP {
            black_box(stub(&input));
        }

        eprintln!("Starting benchmark");
        const DURATION: Duration = Duration::from_secs(3);
        let mut total = Duration::ZERO;
        let true_start = Instant::now();
        let mut n = 0;
        while true_start.elapsed() < DURATION {
            let start = Instant::now();
            black_box(stub(&input));
            total += start.elapsed();
            n += 1;
        }
        eprintln!("Average time of {} runs: {:?}", n, total / n);
        return ExitCode::SUCCESS;
    }

    let solutions = solutions();
    let solution = &solutions.get(&year).unwrap()[day as usize - 1][part as usize - 1];
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
