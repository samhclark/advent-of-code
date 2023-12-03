use std::collections::{HashSet, HashMap};
use std::hash::Hash;

static INPUT: &str = include_str!("../../inputs/2023/day03.in");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PartNumber {
    value: u32,
    locations: Vec<Point>,
}

#[allow(dead_code)]
pub fn part01() {
    let answer = sum_of_part_numbers(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = sum_of_gear_ratios(INPUT);
    println!("Puzzle answer: {answer}");
}

fn sum_of_part_numbers(schematic: &str) -> u64 {
    let adjacency_matrix: HashSet<Point> = build_adjacency_matrix_to_symbols(schematic);
    let parts: Vec<PartNumber> = get_list_of_parts(schematic);

    parts
        .iter()
        .filter(|part| part.locations.iter().any(|p| adjacency_matrix.contains(p)))
        .map(|part| u64::from(part.value))
        .sum()
}

fn build_adjacency_matrix_to_symbols(schematic: &str) -> HashSet<Point> {
    let mut adjacencies = HashSet::new();
    for (row_num, row) in schematic.lines().enumerate() {
        for (col_num, c) in row.chars().enumerate() {
            if c.is_ascii_digit() || c == '.' {
                continue;
            }

            let points = adjacent_points(&Point {
                row: row_num,
                col: col_num,
            });
            for p in points {
                adjacencies.insert(p);
            }
        }
    }

    adjacencies
}

fn adjacent_points(point: &Point) -> Vec<Point> {
    // O O O
    // O * *
    // O * *
    let mut adjacent_points = vec![
        Point {
            row: point.row,
            col: point.col,
        },
        Point {
            row: point.row + 1,
            col: point.col,
        },
        Point {
            row: point.row,
            col: point.col + 1,
        },
        Point {
            row: point.row + 1,
            col: point.col + 1,
        },
    ];

    // * O O
    // O * *
    // O * *
    if point.row > 0 && point.col > 0 {
        adjacent_points.push(Point {
            row: point.row - 1,
            col: point.col - 1,
        });
    }

    // * * *
    // O * *
    // O * *
    if point.row > 0 {
        adjacent_points.push(Point {
            row: point.row - 1,
            col: point.col,
        });
        adjacent_points.push(Point {
            row: point.row - 1,
            col: point.col + 1,
        });
    }

    // * * *
    // * * *
    // * * *
    if point.col > 0 {
        adjacent_points.push(Point {
            row: point.row,
            col: point.col - 1,
        });
        adjacent_points.push(Point {
            row: point.row + 1,
            col: point.col - 1,
        });
    }

    adjacent_points
}

fn get_list_of_parts(schematic: &str) -> Vec<PartNumber> {
    let mut parts = vec![];
    for (row, row_str) in schematic.lines().enumerate() {
        let mut part_locations: Vec<Point> = vec![];
        let mut current = String::new();
        for (col, c) in row_str.chars().enumerate() {
            if c.is_ascii_digit() {
                current.push(c);
                part_locations.push(Point { row, col });
            } else if !current.is_empty() {
                parts.push(PartNumber {
                    value: current.parse().expect("always parses"),
                    locations: part_locations.clone(),
                });
                part_locations = vec![];
                current = String::new();
            }
        }
        if !current.is_empty() {
            parts.push(PartNumber {
                value: current.parse().expect("always parses"),
                locations: part_locations.clone(),
            });
        }
    }

    parts
}

fn sum_of_gear_ratios(schematic: &str) -> u64 {
    let mut total_gear_ratios: u64 = 0;
    let parts = get_list_of_parts(schematic);
    let point_to_adjacent_parts = get_part_adjacency_map(parts);
    for (row, row_str) in schematic.lines().enumerate() {
        for (col, c) in row_str.chars().enumerate() {
            if c == '*' {
                if let Some(adjacent_parts) = point_to_adjacent_parts.get(&Point { row, col }) {
                    if adjacent_parts.len() == 2 {
                    total_gear_ratios += adjacent_parts.iter().map(|part| u64::from(part.value)).reduce(|acc, e| acc * e).unwrap();
                    }
                }
            }
        }
    }
    total_gear_ratios
}

fn get_part_adjacency_map(parts: Vec<PartNumber>) -> HashMap<Point, HashSet<PartNumber>> {
    let mut point_to_parts: HashMap<Point, HashSet<PartNumber>> = HashMap::new();

    for part in parts {
        let mut all_adj_points: HashSet<Point> = HashSet::new();
        for loc in part.locations.clone() {
            all_adj_points.extend(adjacent_points(&loc));
        }
        for adj_point in all_adj_points {
            if !point_to_parts.contains_key(&adj_point) {
                let mut set = HashSet::new();
                set.insert(part.clone());
                point_to_parts.insert(adj_point, set);
            } else {
                let current_parts = point_to_parts.get_mut(&adj_point).unwrap();
                current_parts.insert(part.clone());
                // point_to_parts.insert(adj_point, current_parts.clone());
            }
        }
    }

    point_to_parts
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(sum_of_part_numbers(input), 4361);
    }

    #[test]
    fn test_case_part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(sum_of_gear_ratios(input), 467835);
    }
}
