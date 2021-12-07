// Day 7: The Treachery of Whales

use std::cmp::min;
use std::error::Error;

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> Result<i64, Box<dyn Error>> {
    let mut positions: Vec<i64> = puzzle_input
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    positions.sort_unstable();
    let positions = positions; // make immutable
    let midpoint = positions.len() / 2;
    let median: i64 = positions[midpoint];

    let spent_fuel: i64 = positions.iter().map(|x| (x - median).abs()).sum();
    println!("Puzzle answer: {}", spent_fuel);
    Ok(spent_fuel)
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> Result<i64, Box<dyn Error>> {
    let positions: Vec<i64> = puzzle_input
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    
    // The minimum is one of the two numbers near the arithmetic mean. So, if the mean is 3.5, then
    // the most fuel efficient position is either 3 or 4, so we just check both.
    let discrete_mean = (positions.iter().sum::<i64>() as f64 / positions.len() as f64) as i64;
    let spent_fuel_1: i64 = positions
        .iter()
        .map(|x: &i64| {
            let distance = (x - discrete_mean).abs();
            distance * (distance + 1) / 2 // closed-form summation from 0..N
        })
        .sum();

    let discrete_mean = discrete_mean + 1;
    let spent_fuel_2: i64 = positions
        .iter()
        .map(|x: &i64| {
            let distance = (x - discrete_mean).abs();
            distance * (distance + 1) / 2 
        })
        .sum();
    
    // ...and choose whichever answer is smaller
    let puzzle_answer = min(spent_fuel_1, spent_fuel_2);
    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

#[cfg(test)]
mod day07_tests {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(37, part1(input).unwrap())
    }

    #[test]
    fn test_case_part_2() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(168, part2(input).unwrap())
    }
}
