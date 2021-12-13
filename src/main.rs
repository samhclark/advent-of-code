use std::error::Error;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() -> Result<(), Box<dyn Error>> {
    let puzzle_input = include_str!("../inputs/day11.in");
    day11::part2(puzzle_input.trim())?;
    Ok(())
}
