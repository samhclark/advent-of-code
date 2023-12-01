// Day 1: Sonar Sweep

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> usize {
    let input: Vec<i64> = puzzle_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let increases = input
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count();

    println!("Total number of depth increases: {increases}");
    increases
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> usize {
    let input: Vec<i64> = puzzle_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let increases = input
        .windows(4)
        .filter(|window| window[3] > window[0])
        .count();

    println!("Total number of depth increases: {increases}");
    increases
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(7, part1(input));
    }

    #[test]
    fn test_case_part_2() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(5, part2(input));
    }
}
