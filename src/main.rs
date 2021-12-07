use std::error::Error;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = include_str!("../inputs/day05.in");
    day05::part2(puzzle_input)?;
    Ok(())
}
