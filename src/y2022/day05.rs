use itertools::Itertools;

static CRATES: &str = include_str!("../../inputs/2022/day05-0.in");
static MOVES: &str = include_str!("../../inputs/2022/day05-1.in");

#[allow(dead_code)]
pub fn part01() {
    println!(
        "[AOC 2022, Day 5, Part 1] Answer is: {}",
        do_part1(CRATES, MOVES)
    );
}

#[allow(dead_code)]
pub fn part02() {
    println!(
        "[AOC 2022, Day 5, Part 2] Answer is: {}",
        do_part2(CRATES, MOVES)
    );
}

fn do_part1(crates: &str, moves: &str) -> String {
    let mut stacks: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    for line in crates.lines().rev() {
        for i in 0..9_usize {
            let item = line.chars().nth(1 + (4 * i)).unwrap();
            if item.is_alphabetic() {
                stacks[i].push(item);
            }
        }
    }

    for line in moves.lines() {
        let line = line.split_ascii_whitespace().collect_vec();
        let qty: usize = line[1].parse().unwrap();
        let src: usize = line[3].parse().unwrap();
        let dest: usize = line[5].parse().unwrap();

        for _ in 0..qty {
            let target = stacks[src - 1].pop().unwrap();
            stacks[dest - 1].push(target);
        }
    }

    let mut result: Vec<char> = vec![];
    for i in 0..9_usize {
        result.push(stacks[i].pop().unwrap());
    }

    result.iter().collect()
}

fn do_part2(crates: &str, moves: &str) -> String {
    let mut stacks: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    for line in crates.lines().rev() {
        for (i, stack) in stacks.iter_mut().enumerate().take(9_usize) {
            let item = line.chars().nth(1 + (4 * i)).unwrap();
            if item.is_alphabetic() {
                stack.push(item);
            }
        }
    }

    for line in moves.lines() {
        let line = line.split_ascii_whitespace().collect_vec();
        let qty: usize = line[1].parse().unwrap();
        let src: usize = line[3].parse().unwrap();
        let dest: usize = line[5].parse().unwrap();

        let mut crates = vec![];
        for _ in 0..qty {
            let target = stacks[src - 1].pop().unwrap();
            crates.push(target);
        }
        for item in crates.iter().rev() {
            stacks[dest - 1].push(*item);
        }
    }

    let mut result: Vec<char> = vec![];
    for stack in stacks.iter_mut().take(9_usize) {
        result.push(stack.pop().unwrap());
    }

    result.iter().collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let creates = "    [D]     
[N] [C]    
[Z] [M] [P]\n";

        let moves = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2\n";
        assert_eq!(do_part1(creates, moves), "CMZ");
    }

    #[test]
    fn test_case_part_2() {
        let creates = "    [D]    
[N] [C]    
[Z] [M] [P]\n";

        let moves = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2\n";
        assert_eq!(do_part2(creates, moves), "");
    }
}
