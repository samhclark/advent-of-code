use std::collections::HashSet;

static INPUT: &str = include_str!("../../inputs/2023/day03.in");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

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
    let answer = "";
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
        let input = "";
        assert_eq!(1, 1);
    }
}
