use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/2022/day03.in");

#[allow(dead_code)]
pub fn part01() {
    println!("[AOC 2022, Day 3, Part 1] Answer is: {}", do_part1(INPUT));
}

#[allow(dead_code)]
pub fn part02() {
    println!("[AOC 2022, Day 3, Part 2] Answer is: {}", do_part2(INPUT));
}

fn do_part1(puzzle_input: &str) -> u64 {
    0
}

fn do_part2(puzzle_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "";
        assert_eq!(do_part1(input), 15);
    }

    #[test]
    fn test_case_part_2() {
        let input = "";
        assert_eq!(do_part2(input), 12);
    }
}