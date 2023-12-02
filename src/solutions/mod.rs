use std::collections::BTreeMap;
use std::fmt::Display;

pub mod year2022;
pub mod year2023;

macro_rules! solutions {
    {$($years:expr => [$([$($parts:expr),*]),*$(,)?]),*$(,)?} => {
        pub fn solutions() -> BTreeMap<i32, Vec<Vec<Box<dyn Fn(&str) -> String>>>> {
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

        pub fn stubs() -> BTreeMap<i32, Vec<Vec<Box<dyn Fn(&str)>>>> {
            fn translate<D: Display + 'static>(f: fn(&str) -> D) -> Box<dyn Fn(&str)> {
                Box::new(move |x| { ::std::hint::black_box(f(x)); })
            }
            let mut sols: BTreeMap<i32, Vec<Vec<Box<dyn Fn(&str)>>>> = BTreeMap::new();
            $(
                sols.insert($years, vec![$(
                    vec![$(
                        translate($parts)
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
    2023 => [
        [year2023::day01::part1, year2023::day01::part2],
        [year2023::day02::part1, year2023::day02::part2],
        [year2023::day03::part1, year2023::day03::part2],
        [year2023::day04::part1, year2023::day04::part2],
        [year2023::day05::part1, year2023::day05::part2],
        [year2023::day06::part1, year2023::day06::part2],
        [year2023::day07::part1, year2023::day07::part2],
        [year2023::day08::part1, year2023::day08::part2],
        [year2023::day09::part1, year2023::day09::part2],
        [year2023::day10::part1, year2023::day10::part2],
        [year2023::day11::part1, year2023::day11::part2],
        [year2023::day12::part1, year2023::day12::part2],
        [year2023::day13::part1, year2023::day13::part2],
        [year2023::day14::part1, year2023::day14::part2],
        [year2023::day15::part1, year2023::day15::part2],
        [year2023::day16::part1, year2023::day16::part2],
        [year2023::day17::part1, year2023::day17::part2],
        [year2023::day18::part1, year2023::day18::part2],
        [year2023::day19::part1, year2023::day19::part2],
        [year2023::day20::part1, year2023::day20::part2],
        [year2023::day21::part1, year2023::day21::part2],
        [year2023::day22::part1, year2023::day22::part2],
        [year2023::day23::part1, year2023::day23::part2],
        [year2023::day24::part1, year2023::day24::part2],
        [year2023::day25::part1, year2023::day25::part2],
    ],
}
