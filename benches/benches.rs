use aoc_rs::solutions::*;
use iai::runner;

fn main() {
    let joined = get_solution_tuples()
        .into_values()
        .flatten()
        .flatten()
        .map(|t| &t)
        .collect::<Vec<_>>();
    let joined = joined.as_slice();

    runner(joined);
}
