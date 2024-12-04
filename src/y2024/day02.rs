// Day 2: Red-Nosed Reports

use itertools::Itertools;

use crate::aoc::util::PuzzleAnswer;

static INPUT: &str = include_str!("../../inputs/2024/day02.in");

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

// TODO: Trying out this method signature.
// I want to get a trait going so that I can generate all
// this scaffolding every time by `impl` a new day
fn private_part1(input: &str) -> PuzzleAnswer {
    PuzzleAnswer::from(how_many_reports_are_safe(input, is_report_safe))
}

fn private_part2(input: &str) -> PuzzleAnswer {
    PuzzleAnswer::from(how_many_reports_are_safe(input, is_report_safe_with_damper))
}

fn how_many_reports_are_safe(input: &str, is_safe: fn(&[u32]) -> bool) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|it| it.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|report| is_safe(report))
        .count()
}

fn is_report_safe_with_damper(report: &[u32]) -> bool {
    is_report_safe(report)
        || report.iter().enumerate().any(|(i, _)| {
            let mut report_copy = report.to_owned();
            report_copy.remove(i);
            is_report_safe(&report_copy)
        })
}

#[derive(PartialEq, Eq)]
enum Direction {
    Decreasing,
    Increasing,
    Invalid,
}

fn is_report_safe(report: &[u32]) -> bool {
    let directions = report
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let diff = a.abs_diff(*b);
            if (1..4).contains(&diff) {
                if b > a {
                    Direction::Increasing
                } else {
                    Direction::Decreasing
                }
            } else {
                Direction::Invalid
            }
        })
        .collect::<Vec<Direction>>();

    return directions.iter().all(|it| *it == Direction::Increasing)
        || directions.iter().all(|it| *it == Direction::Decreasing);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(how_many_reports_are_safe(input, is_report_safe), 2);
    }

    #[test]
    fn test_case_part_2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(
            how_many_reports_are_safe(input, is_report_safe_with_damper),
            4
        );
    }
}
