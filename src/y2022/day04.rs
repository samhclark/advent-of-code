static INPUT: &str = include_str!("../../inputs/2022/day04.in");

#[allow(dead_code)]
pub fn part01() {
    println!("[AOC 2022, Day 4, Part 1] Answer is: {}", do_part1(INPUT));
}

#[allow(dead_code)]
pub fn part02() {
    println!("[AOC 2022, Day 4, Part 2] Answer is: {}", do_part2(INPUT));
}

fn do_part1(puzzle_input: &str) -> u64 {
    let mut num_contains: u64 = 0;
    for pair in puzzle_input.lines() {
        let (elf1, elf2) = pair.split_once(',').unwrap();
        let range1 = elf1.split_once('-').unwrap();
        let range2 = elf2.split_once('-').unwrap();
        if contains(range1, range2) || contains(range2, range1) {
            num_contains += 1;
        }
    }
    num_contains
}

fn contains(outer: (&str, &str), inner: (&str, &str)) -> bool {
    if (outer.0.parse::<i64>().unwrap() <= inner.0.parse::<i64>().unwrap())
        && (outer.1.parse::<i64>().unwrap() >= inner.1.parse::<i64>().unwrap())
    {
        return true;
    }
    false
}

fn do_part2(puzzle_input: &str) -> u64 {
    let mut num_contains: u64 = 0;
    for pair in puzzle_input.lines() {
        let (elf1, elf2) = pair.split_once(',').unwrap();
        let range1 = elf1.split_once('-').unwrap();
        let range2 = elf2.split_once('-').unwrap();
        if overlaps(range1, range2) || contains(range2, range1) {
            num_contains += 1;
        }
    }
    num_contains
}

fn overlaps(a: (&str, &str), b: (&str, &str)) -> bool {
    let start_a = a.0.parse::<i64>().unwrap();
    let start_b = b.0.parse::<i64>().unwrap();
    let end_a = a.1.parse::<i64>().unwrap();
    let end_b = b.1.parse::<i64>().unwrap();

    if (start_b >= start_a && start_b <= end_a) || (end_b >= start_a && end_b <= end_a) {
        return true;
    }
    false
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8\n";
        assert_eq!(do_part1(input), 2);
    }

    #[test]
    fn test_case_part_2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8\n";
        assert_eq!(do_part2(input), 4);
    }
}
