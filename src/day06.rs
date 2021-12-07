// Day 6: Lanternfish

use std::error::Error;

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> Result<u128, Box<dyn Error>> {
    const DAYS: usize = 80;

    let input_fish: Vec<usize> = puzzle_input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let puzzle_answer = simulate_lanternfish_lifecycle(DAYS, input_fish).unwrap();

    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> Result<u128, Box<dyn Error>> {
    const DAYS: usize = 256;

    let input_fish: Vec<usize> = puzzle_input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let puzzle_answer = simulate_lanternfish_lifecycle(DAYS, input_fish).unwrap();

    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

fn simulate_lanternfish_lifecycle(
    days: usize,
    starting_fish: Vec<usize>,
) -> Result<u128, Box<dyn Error>> {
    let mut buckets_of_fish: [u128; 9] = [0; 9];

    for fish in starting_fish {
        buckets_of_fish[fish] += 1;
    }

    let mut b = buckets_of_fish; // rename to something smaller
    for _ in 1..=days {
        let new = [b[1], b[2], b[3], b[4], b[5], b[6], b[7] + b[0], b[8], b[0]];
        b = new;
    }

    Ok(b.iter().copied().sum())
}

#[cfg(test)]
mod day06_tests {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "3,4,3,1,2";

        assert_eq!(5934, part1(input).unwrap())
    }

    #[test]
    fn test_case_part_2() {
        let input = "3,4,3,1,2";

        assert_eq!(26984457539, part2(input).unwrap())
    }
}
