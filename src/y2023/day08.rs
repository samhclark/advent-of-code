// Day 8: Haunted Wasteland

use std::u64;

static INPUT: &str = include_str!("../../inputs/2023/day08.in");

type Pair = (usize, usize);

#[allow(dead_code)]
pub fn part01() {
    let answer = number_of_steps(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = "";
    println!("Puzzle answer: {answer}");
}

fn number_of_steps(input: &str) -> u64 {
    let (instructions, map_str) = input.split_once("\n\n").unwrap();
    let mut map: [Pair; 26 * 26 * 26] = [(0, 0); 26 * 26 * 26];
    for line in map_str.lines() {
        let (node, leaves) = line.split_once(" = ").unwrap();
        let node_idx = index_from_node_id(node);
        let (left, right) = leaves
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        map[node_idx] = (index_from_node_id(left), index_from_node_id(right));
    }

    let mut steps: u64 = 0;
    let mut pos: usize = 0;
    for dir in instructions.chars().cycle() {
        if pos == index_from_node_id("ZZZ") {
            break;
        }
        if steps == u64::MAX {
            break;
        }
        match dir {
            'L' => pos = map[pos].0,
            'R' => pos = map[pos].1,
            _ => unreachable!(),
        }
        steps += 1;
    }
    steps
}

/// AAA -> 0
/// AAB -> 1
/// ...
/// ZZZ -> 17575
fn index_from_node_id(node: &str) -> usize {
    let mut char_iter = node.chars();
    (26 * 26 * (char_iter.next().unwrap() as u32 - 65)) as usize
        + (26 * (char_iter.next().unwrap() as u32 - 65)) as usize
        + (char_iter.next().unwrap() as u32 - 65) as usize
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_1_part_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(number_of_steps(input), 2);
    }

    #[test]
    fn test_case_2_part_1() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(number_of_steps(input), 6);
    }
}
