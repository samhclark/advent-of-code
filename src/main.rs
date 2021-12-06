use std::error::Error;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input_numbers = include_str!("../inputs/day04-numbers.in");
    let puzzle_input_boards = include_str!("../inputs/day04-boards.in");
    day04::part2(puzzle_input_numbers, puzzle_input_boards)?;
    Ok(())
}
