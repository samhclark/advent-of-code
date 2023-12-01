use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/2022/day07.in");

#[allow(dead_code)]
pub fn part01() {
    println!("[AOC 2022, Day 7, Part 1] Answer is: {}", do_part1(INPUT));
}

#[allow(dead_code)]
pub fn part02() {
    println!("[AOC 2022, Day 7, Part 2] Answer is: {}", do_part2(INPUT));
}

const fn do_part1(puzzle_input: &str) -> u64 {
    0
}

const fn do_part2(puzzle_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k\n";
        assert_eq!(do_part1(input), 0);
    }

    #[test]
    fn test_case_part_2() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k\n";
        assert_eq!(do_part2(input), 0);
    }
}
