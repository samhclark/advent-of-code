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

    for _ in 1..=days {
        let previous_bucket = buckets_of_fish.clone();
        buckets_of_fish = [0; 9];
        for (day_in_cycle, fish_count) in previous_bucket.iter().enumerate() {
            if day_in_cycle == 0 {
                buckets_of_fish[8] += fish_count;
                buckets_of_fish[6] += fish_count;
            } else {
                buckets_of_fish[day_in_cycle - 1] += fish_count;
            }
        }
    }

    Ok(buckets_of_fish.iter().copied().sum())
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
