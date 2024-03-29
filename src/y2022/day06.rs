use std::collections::HashSet;

static INPUT: &str = include_str!("../../inputs/2022/day06.in");

#[allow(dead_code)]
pub fn part01() {
    println!("[AOC 2022, Day 6, Part 1] Answer is: {}", do_part1(INPUT));
}

#[allow(dead_code)]
pub fn part02() {
    println!("[AOC 2022, Day 6, Part 2] Answer is: {}", do_part2(INPUT));
}

fn do_part1(puzzle_input: &str) -> u64 {
    do_puzzle(puzzle_input, 4).expect("Should always pass")
}

fn do_part2(puzzle_input: &str) -> u64 {
    do_puzzle(puzzle_input, 14).expect("Should always pass")
}

fn do_puzzle(puzzle_input: &str, window_size: usize) -> Option<u64> {
    for (i, w) in puzzle_input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
    {
        let s: HashSet<char> = w.iter().copied().collect();

        if s.len() == window_size {
            return Some(window_size as u64 + i as u64);
        }
    }
    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_1_part_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n";
        assert_eq!(do_part1(input), 7);
    }

    #[test]
    fn test_2_part_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz\n";
        assert_eq!(do_part1(input), 5);
    }

    #[test]
    fn test_3_part_1() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg\n";
        assert_eq!(do_part1(input), 6);
    }

    #[test]
    fn test_4_part_1() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n";
        assert_eq!(do_part1(input), 10);
    }

    #[test]
    fn test_5_part_1() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n";
        assert_eq!(do_part1(input), 11);
    }

    #[test]
    fn test_1_part_2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n";
        assert_eq!(do_part2(input), 19);
    }

    #[test]
    fn test_2_part_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz\n";
        assert_eq!(do_part2(input), 23);
    }

    #[test]
    fn test_3_part_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg\n";
        assert_eq!(do_part2(input), 23);
    }

    #[test]
    fn test_4_part_2() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n";
        assert_eq!(do_part2(input), 29);
    }

    #[test]
    fn test_5_part_2() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n";
        assert_eq!(do_part2(input), 26);
    }
}
