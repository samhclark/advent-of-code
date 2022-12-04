use std::collections::HashSet;

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
    let mut sum_of_priorities: u64 = 0;
    for rucksack in puzzle_input.lines() {
        let (a, b) = rucksack.split_at(rucksack.len() / 2);
        let a: HashSet<char> = a.chars().collect();
        'inner: for c in b.chars() {
            if a.contains(&c) {
                let priority = if c.is_ascii_lowercase() {
                    (c as u8) - b'`'
                } else {
                    (c as u8) - b'&'
                };
                sum_of_priorities += u64::from(priority);
                break 'inner;
            }
        }
    }
    sum_of_priorities
}

fn do_part2(puzzle_input: &str) -> u64 {
    let mut sum_of_priorities: u64 = 0;

    let groups = puzzle_input.lines().chunks(3);
    for mut group in &groups {
        let elf1_items: HashSet<char> = group.next().unwrap().chars().collect();
        let elf2_items: HashSet<char> = group.next().unwrap().chars().collect();
        'inner: for item in group.next().unwrap().chars() {
            if elf1_items.contains(&item) && elf2_items.contains(&item) {
                let priority = if item.is_ascii_lowercase() {
                    (item as u8) - b'`'
                } else {
                    (item as u8) - b'&'
                };
                sum_of_priorities += u64::from(priority);
                break 'inner;
            }
        }
    }
    sum_of_priorities
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw\n";
        assert_eq!(do_part1(input), 157);
    }

    #[test]
    fn test_case_part_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw\n";
        assert_eq!(do_part2(input), 70);
    }
}
