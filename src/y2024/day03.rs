// Day 3: Mull It Over

use regex::Regex;

use crate::aoc::util::PuzzleAnswer;

static INPUT: &str = include_str!("../../inputs/2024/day03.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = private_part1(INPUT);
    println!("{answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = private_part2(INPUT);
    println!("{answer}");
}

fn private_part1(input: &str) -> PuzzleAnswer {
    PuzzleAnswer::from(fix_mul(input))
}

fn private_part2(input: &str) -> PuzzleAnswer {
    PuzzleAnswer::from(0)
}

fn fix_mul(input: &str) -> u64 {
    let mut total: u64 = 0;
    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    for line in input.lines() {
        for (_, [mul]) in re.captures_iter(line).map(|c| c.extract()) {
            let (a, b) = mul
                .strip_prefix("mul(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(',')
                .unwrap();
            total += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
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
        let input = "";

        assert_eq!(2, 2);
    }
}
