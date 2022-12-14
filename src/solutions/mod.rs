use std::collections::BTreeMap;
use std::fmt::Display;

pub mod year2022;

macro_rules! solutions {
    {$($years:expr => [$([$($parts:expr),*]),*$(,)?]),*$(,)?} => {
        pub fn get_solutions() -> BTreeMap<i32, Vec<Vec<Box<dyn Fn(&str) -> String>>>> {
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
        }

        pub fn get_solution_names() -> BTreeMap<i32, Vec<Vec<&'static str>>> {
            let mut sols: BTreeMap<i32, Vec<Vec<&'static str>>> = BTreeMap::new();
            $(
                sols.insert($years, vec![$(
                    vec![$(
                        stringify!($parts)
                    ),*]
                ),*]);
            )*
            sols
        }

        pub fn get_solution_tuples() -> BTreeMap<i32, Vec<Vec<(&'static str, fn())>>> {
            let session = std::fs::read_to_string(".session").unwrap();
            let session = session.trim_end();

            let mut sols: BTreeMap<i32, Vec<Vec<(&'static str, fn())>>> = BTreeMap::new();
            $(
                sols.insert($years, vec![$(
                    vec![$(
                        (stringify!($parts), || { $parts("").to_string(); })
                    ),*]
                ),*]);
            )*
            sols
        }
    };
}

solutions! {
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
}
