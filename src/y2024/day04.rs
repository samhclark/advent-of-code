use crate::aoc::util::PuzzleAnswer;

static INPUT: &str = include_str!("../../inputs/2024/day04.in");

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
    PuzzleAnswer::from(0)
}

fn solve_part2(input: &str) -> PuzzleAnswer {
    PuzzleAnswer::from(0)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "";

        assert_eq!(1, 1);
    }

    #[test]
    fn test_case_part_2() {
        let input = "";

        assert_eq!(2, 2);
    }
}