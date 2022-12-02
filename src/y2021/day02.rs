// Day 2: Dive!

use std::num::ParseIntError;
use std::str::FromStr;

enum Command {
    Forward,
    Down,
    Up,
}

struct Instruction {
    command: Command,
    magnitude: i64,
}

struct Location {
    x: i64,
    y: i64,
}

struct LocationV2 {
    x: i64,
    y: i64,
    aim: i64,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, data) = s.split_once(' ').unwrap();
        let command = match cmd.to_ascii_lowercase().as_str() {
            "forward" => Command::Forward,
            "down" => Command::Down,
            "up" => Command::Up,
            unknown => panic!("Unknown command {}", unknown),
        };
        let magnitude = data.parse::<i64>()?;

        Ok(Self { command, magnitude })
    }
}

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> i64 {
    let input: Vec<Instruction> = puzzle_input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

    let mut current_location = Location { x: 0, y: 0 };
    for instruction in input {
        match instruction.command {
            Command::Forward => current_location.x += instruction.magnitude,
            Command::Down => current_location.y += instruction.magnitude,
            Command::Up => current_location.y -= instruction.magnitude,
        }
    }

    let puzzle_answer = current_location.x * current_location.y;
    println!(
        "Current location: ({}, {}) = {}",
        current_location.x, current_location.y, puzzle_answer,
    );

    puzzle_answer
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> i64 {
    let input: Vec<Instruction> = puzzle_input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();

    let mut current_location = LocationV2 { x: 0, y: 0, aim: 0 };
    for instruction in input {
        match instruction.command {
            Command::Forward => {
                current_location.x += instruction.magnitude;
                current_location.y += instruction.magnitude * current_location.aim;
            }
            Command::Down => current_location.aim += instruction.magnitude,
            Command::Up => current_location.aim -= instruction.magnitude,
        }
    }

    let puzzle_answer = current_location.x * current_location.y;
    println!(
        "Current location: ({}, {}) = {}",
        current_location.x, current_location.y, puzzle_answer,
    );

    puzzle_answer
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(150, part1(input));
    }

    #[test]
    fn test_case_part_2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(900, part2(input));
    }
}
