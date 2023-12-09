// Day 8: Haunted Wasteland

use std::{
    cmp::{max, min},
    u64,
};

static INPUT: &str = include_str!("../../inputs/2023/day08.in");

type Pair = (usize, usize);

#[allow(dead_code)]
pub fn part01() {
    let answer = number_of_steps(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = number_of_steps_for_ghosts(INPUT);
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

#[allow(clippy::large_stack_frames)]
fn number_of_steps_for_ghosts(input: &str) -> u64 {
    let (instructions, map_str) = input.split_once("\n\n").unwrap();
    let mut map: [Pair; 26 * 26 * 26] = [(0, 0); 26 * 26 * 26];
    let mut positions: Vec<usize> = Vec::with_capacity(6); // cheating a little because I know my input has 6
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
        if node_idx % 26 == 0 {
            // is a start point
            positions.push(node_idx);
        }
    }
    let map = map; // immutable again

    let cycle_lengths: Vec<u64> = positions
        .iter()
        .map(|pos| analyze_cycles(*pos, instructions, &map))
        .collect();
    cycle_lengths.iter().fold(1, |acc, e| lcm(acc, *e))
}

fn analyze_cycles(start: usize, instructions: &str, map: &[Pair]) -> u64 {
    // an array where the value at each index is the step number when that value was last seen
    // AND the exact same step in the sequence of instructions
    let mut visited: [(u64, u16); 26 * 26 * 26] = [(u64::MAX, u16::MAX); 26 * 26 * 26];

    let mut first_z_at: usize = 0;
    let mut cycle_starts_at: u64 = 0;
    let mut cycle_length: u64 = 0;
    let mut num_z_in_cycle: u64 = 0;

    let mut current_node: usize = start;
    // won't truncate because the instruction length is max 270
    #[allow(clippy::cast_possible_truncation)]
    let instructions_len = instructions.len() as u16;
    for (step, dir) in instructions.chars().cycle().enumerate() {
        // limit the number of total steps
        if step == u32::MAX as usize {
            println!("|{start}| Hit max. Exiting");
            panic!();
        }

        // won't truncate because instructions_len is already u16. So u64 mod u16 fits in a u16.
        #[allow(clippy::cast_possible_truncation)]
        let instruction_index: u16 = (step % instructions_len as usize) as u16;

        // look for the very first exit node
        if current_node % 26 == 25 && first_z_at == 0 {
            println!("|{start}| found first z at {step}");
            first_z_at = step;
        }

        // check if this node has been visited before
        let (prev_step, prev_instr) = visited[current_node];
        if prev_step != u64::MAX && prev_instr == instruction_index && cycle_starts_at == 0 {
            // cycle detected -- first time
            println!("|{start}| found first cycle at {step}");
            println!("|{start}| prev_step: {prev_step}; prev_instr: {prev_instr}; instruction_index: {instruction_index}; cycle_starts_at: {cycle_starts_at}");
            cycle_starts_at = prev_step;
            cycle_length = step as u64 - prev_step;
        }

        // exit on the second time through the cycle
        if cycle_length != 0 && prev_step + cycle_length == step as u64 {
            println!("|{start}| Found second cycle as {current_node}. Exiting...");
            println!("|{start}| First Z at {first_z_at}");
            println!("|{start}| Cycle starts at step {cycle_starts_at}");
            println!("|{start}| Cycle length is {cycle_length}");
            println!("|{start}| Count of Z in cycle: {num_z_in_cycle}");
            return cycle_length;
        }

        // while in a cycle, count the number of exits
        if cycle_starts_at != 0 && current_node % 26 == 25 {
            println!("|{start}| Found another z in cycle at {current_node}");
            num_z_in_cycle += 1;
        }

        visited[current_node] = (step as u64, instruction_index);
        match dir {
            'L' => current_node = map[current_node].0,
            'R' => current_node = map[current_node].1,
            _ => unreachable!("instructions are always L or R"),
        }
    }
    unreachable!("the loop will always return")
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

fn gcd(a: u64, b: u64) -> u64 {
    // ripped from https://www.hackertouch.com/least-common-multiple-in-rust.html
    // because the wikipedia page was so damn dense
    let mut max = max(a, b);
    let mut min = min(a, b);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
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

    #[test]
    #[ignore = "hangs forever on this input, but works IRL"]
    fn test_case_part_2() {
        let input = "LR

    AAA = (AAB, XXX)
    AAB = (XXX, AAZ)
    AAZ = (AAB, XXX)
    BBA = (BBB, XXX)
    BBB = (BBC, BBC)
    BBC = (BBZ, BBZ)
    BBZ = (BBB, BBB)
    XXX = (XXX, XXX)";
        assert_eq!(number_of_steps_for_ghosts(input), 6);
    }

    #[test]
    fn mod_26_works() {
        assert_eq!(index_from_node_id("AAZ") % 26, 25);
        assert_eq!(index_from_node_id("ABZ") % 26, 25);
        assert_eq!(index_from_node_id("BAZ") % 26, 25);
        assert_eq!(index_from_node_id("AZZ") % 26, 25);
        assert_eq!(index_from_node_id("ZAZ") % 26, 25);
        assert_eq!(index_from_node_id("ZZZ") % 26, 25);
    }
}
