// Day 2: Rock Paper Scissors

use std::{str::FromStr, string::ParseError};

static INPUT: &str = include_str!("../../inputs/2022/day02.in");

#[allow(dead_code)]
pub fn part01() {
    println!("[AOC 2022, Day 2, Part 1] Answer is: {}", do_part1(INPUT));
}

#[allow(dead_code)]
pub fn part02() {
    println!("[AOC 2022, Day 2, Part 2] Answer is: {}", do_part2(INPUT));
}

fn do_part1(strategy: &str) -> u64 {
    let mut total_score: u64 = 0;
    for round in strategy.lines() {
        let (theirs, mine) = round.split_once(' ').unwrap();
        let theirs = theirs.chars().next().unwrap() as u8;
        let mine = mine.chars().next().unwrap() as u8;
        let theirs = theirs - b'A' + 1;
        let mine = mine - b'X' + 1;

        if mine == theirs {
            total_score += 3;
        } else if (mine == 1 && theirs == 3)
            || (mine == 2 && theirs == 1)
            || (mine == 3 && theirs == 2)
        {
            total_score += 6;
        }
        total_score += u64::from(mine);
    }

    total_score
}

fn do_part2(strategy: &str) -> u64 {
    let mut total_score: u64 = 0;
    for round in strategy.lines() {
        let (theirs, result) = round.split_once(' ').unwrap();
        let theirs = theirs.chars().next().unwrap() as u8;
        let result = result.chars().next().unwrap() as u8;
        let theirs = theirs - b'A' + 1;
        let result = result - b'X';

        if result == 1 {
            total_score += u64::from(theirs);
        } else if result == 0 {
            // have to lose
            if theirs == 1 {
                total_score += 3;
            } else if theirs == 2 {
                total_score += 1;
            } else {
                total_score += 2;
            }
        } else {
            // have to win
            if theirs == 1 {
                total_score += 2;
            } else if theirs == 2 {
                total_score += 3;
            } else {
                total_score += 1;
            }
        }
        total_score += u64::from(result * 3);
    }

    total_score
}

// After reading some threads online, I realized you can do this with a LUT.
// Let's try
struct Round {
    part1_score: u64,
    part2_score: u64,
}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A X" => Self {
                part1_score: 4,
                part2_score: 3,
            },
            "A Y" => Self {
                part1_score: 8,
                part2_score: 4,
            },
            "A Z" => Self {
                part1_score: 3,
                part2_score: 8,
            },
            "B X" => Self {
                part1_score: 1,
                part2_score: 1,
            },
            "B Y" => Self {
                part1_score: 5,
                part2_score: 5,
            },
            "B Z" => Self {
                part1_score: 9,
                part2_score: 9,
            },
            "C X" => Self {
                part1_score: 7,
                part2_score: 2,
            },
            "C Y" => Self {
                part1_score: 2,
                part2_score: 6,
            },
            "C Z" => Self {
                part1_score: 6,
                part2_score: 7,
            },
            _ => unreachable!(),
        })
    }
}

pub fn with_lut() {
    let rounds: Vec<Round> = INPUT.lines().flat_map(Round::from_str).collect();
    println!(
        "part 1 LUT: {}",
        rounds.iter().map(|r| r.part1_score).sum::<u64>()
    );
    println!(
        "part 2 LUT: {}",
        rounds.iter().map(|r| r.part2_score).sum::<u64>()
    );
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "A Y
B X
C Z\n";
        assert_eq!(do_part1(input), 15);
    }

    #[test]
    fn test_case_part_2() {
        let input = "A Y
B X
C Z\n";
        assert_eq!(do_part2(input), 12);
    }
}
