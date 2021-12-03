// Day 2: Dive!

use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

struct Command {
    direction: String,
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

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_ascii_whitespace().collect();
        let dir = split[0];
        let mag = split[1].parse::<i64>()?;

        Ok(Command {
            direction: dir.to_string(),
            magnitude: mag,
        })
    }
}

#[allow(dead_code)]
pub fn part1() -> Result<(), Box<dyn Error>> {
    let input: Vec<Command> = include_str!("../inputs/day02.in")
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .collect();

    let mut current_location = Location { x: 0, y: 0 };
    for command in input {
        if command.direction == "forward" {
            current_location.x += command.magnitude;
        } else if command.direction == "down" {
            current_location.y += command.magnitude;
        } else if command.direction == "up" {
            current_location.y -= command.magnitude;
        }
    }

    println!(
        "Current location: ({}, {}) = {}",
        current_location.x,
        current_location.y,
        (current_location.x * current_location.y)
    );

    Ok(())
}

#[allow(dead_code)]
pub fn part2() -> Result<(), Box<dyn Error>> {
    let input: Vec<Command> = include_str!("../inputs/day02.in")
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .collect();

    let mut current_location = LocationV2 { x: 0, y: 0, aim: 0 };
    for command in input {
        if command.direction == "forward" {
            current_location.x += command.magnitude;
            current_location.y += command.magnitude * current_location.aim;
        } else if command.direction == "down" {
            current_location.aim += command.magnitude;
        } else if command.direction == "up" {
            current_location.aim -= command.magnitude;
        }
    }

    println!(
        "Current location: ({}, {}) = {}",
        current_location.x,
        current_location.y,
        (current_location.x * current_location.y)
    );
    Ok(())
}
