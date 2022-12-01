use std::error::Error;

mod y2021;
mod y2022;

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = include_str!("../inputs/day11.in");
    y2021::day11::part1(puzzle_input.trim())?;
    Ok(())
}
