use aoc_rs::solutions::*;
use iai::runner;

fn main() {
    let joined = get_solution_tuples();
    runner(joined.into_values().collect::<Vec<_>>().as_slice());
}
