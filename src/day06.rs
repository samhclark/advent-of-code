// Day 6: Lanternfish

use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> Result<u128, Box<dyn Error>> {
    const DAYS: usize = 80;

    let input_fish: Vec<u8> = puzzle_input
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

    let input_fish: Vec<u8> = puzzle_input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let puzzle_answer = simulate_lanternfish_lifecycle(DAYS, input_fish).unwrap();

    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

fn simulate_lanternfish_lifecycle(
    days: usize,
    starting_fish: Vec<u8>,
) -> Result<u128, Box<dyn Error>> {
    let mut buckets_of_fish: HashMap<u8, u128> = HashMap::with_capacity(9);

    for fish in starting_fish {
        let fish_count = buckets_of_fish.entry(fish).or_insert(0);
        *fish_count += 1;
    }

    for _ in 1..=days {
        let previous_bucket = buckets_of_fish.clone();
        buckets_of_fish.clear();
        for (k, v) in previous_bucket {
            if k == 0u8 {
                buckets_of_fish.insert(8, v);
                let parent_fish = buckets_of_fish.entry(6).or_insert(0);
                *parent_fish += v;
            } else {
                let other_fish = buckets_of_fish.entry(k - 1).or_insert(0);
                *other_fish += v;
            }
        }
    }

    Ok(buckets_of_fish.values().sum())
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
