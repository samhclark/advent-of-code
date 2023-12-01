// Day 6: Lanternfish

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> u128 {
    const DAYS: usize = 80;

    let input_fish: Vec<usize> = puzzle_input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let puzzle_answer = simulate_lanternfish_lifecycle(DAYS, input_fish);

    println!("Puzzle answer: {puzzle_answer}");
    puzzle_answer
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> u128 {
    const DAYS: usize = 256;

    let input_fish: Vec<usize> = puzzle_input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let puzzle_answer = simulate_lanternfish_lifecycle(DAYS, input_fish);

    println!("Puzzle answer: {puzzle_answer}");
    puzzle_answer
}

fn simulate_lanternfish_lifecycle(days: usize, starting_fish: Vec<usize>) -> u128 {
    let mut buckets_of_fish: [u128; 9] = [0; 9];

    for fish in starting_fish {
        buckets_of_fish[fish] += 1;
    }

    for _ in 1..=days {
        buckets_of_fish.rotate_left(1);
        buckets_of_fish[6] += buckets_of_fish[8];
    }

    buckets_of_fish.iter().copied().sum()
}

#[cfg(test)]
mod day06_tests {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "3,4,3,1,2";

        assert_eq!(5934, part1(input))
    }

    #[test]
    fn test_case_part_2() {
        let input = "3,4,3,1,2";

        assert_eq!(26984457539, part2(input))
    }
}
