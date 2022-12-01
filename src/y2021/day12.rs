// Day 12: Passage Pathing

use std::{collections::HashMap, collections::HashSet, error::Error};

type CaveSystem = HashMap<String, HashSet<String>>;

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> Result<i64, Box<dyn Error>> {
    let cave_system: CaveSystem = make_cave_system(puzzle_input);
    let paths: Vec<Vec<&str>> = find_paths(cave_system);

    // println!("Parsed cave system like: {:?}", cave_system);

    let puzzle_answer: i64 = 1;
    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

fn make_cave_system(input: &str) -> CaveSystem {
    let mut cave_system: CaveSystem = HashMap::new();

    let mut node_id = HashSet::<String>::new();
    for line in input.lines() {
        let (start, end) = line.split_once('-').unwrap();
        let (start, end) = (start.to_string(), end.to_string());
        node_id.insert(start);
        node_id.insert(end);
    }

    // node_id.

    for line in input.lines() {
        let (start, end) = line.split_once('-').unwrap();
        let (start, end) = (start.to_string(), end.to_string());
        node_id.insert(start);
        node_id.insert(end);
        
        let start_node = cave_system.entry(start).or_insert(HashSet::<String>::new());
        start_node.insert(end);

        let end_node = cave_system.entry(end).or_insert(HashSet::<String>::new());
        end_node.insert(start);
    }

    cave_system
}

fn find_paths<'a>(cave_system: CaveSystem) -> () {
    let mut to_visit= Vec::<&str>::new();
    to_visit.push("start");

    let mut all_paths = Vec::<String>::new();
    let mut current_path = Vec::<&str>::new();
    while !to_visit.is_empty() {
        let mut current = to_visit.pop().unwrap();
        current_path.push(current);

        if current == "end" {
            all_paths.push(current_path.join(",").to_string());
            current_path.pop();
        }

        let mut connected_caves = cave_system.get(current).unwrap();

        for cave in connected_caves {
            to_visit.push(cave);
        }


    }
}

fn is_small_cave(s: &str) -> bool {
    s == s.to_ascii_lowercase()
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> Result<i64, Box<dyn Error>> {
    let puzzle_answer: i64 = 1;
    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

#[cfg(test)]
mod day12_tests {

    use super::*;

    #[test]
    fn test_case_part_1_easy() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end\n";

        assert_eq!(10, part1(input).unwrap())
    }

    #[test]
    fn test_case_part_1_med() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc\n";

        assert_eq!(19, part1(input).unwrap())
    }

    #[test]
    fn test_case_part_1_hard() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW\n";

        assert_eq!(226, part1(input).unwrap())
    }

    #[test]
    fn test_case_part_2() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526\n";

        assert_eq!(195, part2(input).unwrap())
    }
}
