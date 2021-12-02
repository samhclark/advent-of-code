// Day 01, part 1: Sonar Sweep

use std::error::Error;

#[allow(dead_code)]
pub fn part1() -> Result<(), Box<dyn Error>> {
    let input: Vec<i64> = include_str!("../inputs/day01.in")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let increases = input
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count();

    println!("Total number of depth increases: {}", increases);

    Ok(())
}

#[allow(dead_code)]
pub fn part2() -> Result<(), Box<dyn Error>> {
    let input: Vec<i64> = include_str!("../inputs/day01.in")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let increases = input
        .windows(4)
        .filter(|window| window[3] > window[0])
        .count();

    println!("Total number of depth increases: {}", increases);

    Ok(())
}
