use std::error::Error;

mod day01;
mod day02;
mod day03;

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = include_str!("../inputs/day03.in");
    day03::part2(puzzle_input)?;
    Ok(())
}
