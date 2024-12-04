// Day 3: Mull It Over

use regex::Regex;

use crate::aoc::util::PuzzleAnswer;

static INPUT: &str = include_str!("../../inputs/2024/day03.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = solve_part1(INPUT);
    println!("{answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = solve_part2(INPUT);
    println!("{answer}");
}

fn solve_part1(input: &str) -> PuzzleAnswer {
    PuzzleAnswer::from(fix_mul(input))
}

fn solve_part2(input: &str) -> PuzzleAnswer {
    PuzzleAnswer::from(dos_and_donts(input))
}

fn extract_mul_values(capture: &str) -> (u64, u64) {
    capture
        .strip_prefix("mul(")
        .and_then(|s| s.strip_suffix(")"))
        .and_then(|s| s.split_once(','))
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .unwrap()
}

fn fix_mul(input: &str) -> u64 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    input
        .lines()
        .flat_map(|line| re.captures_iter(line))
        .map(|cap| extract_mul_values(&cap[0]))
        .map(|(a, b)| a * b)
        .sum()
}

fn dos_and_donts(input: &str) -> u64 {
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();

    let mut total = 0;
    let mut mul_enabled = true;

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            match &cap[0] {
                "do()" => mul_enabled = true,
                "don't()" => mul_enabled = false,
                mul_cap if mul_enabled => {
                    let (a, b) = extract_mul_values(mul_cap);
                    total += a * b;
                }
                _ => {}
            }
        }
    }
    total
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(fix_mul(input), 161);
    }

    #[test]
    fn test_case_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(dos_and_donts(input), 48);
    }
}
