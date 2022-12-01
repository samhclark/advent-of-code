// Day 9: Smoke Basin

use std::{collections::HashSet, error::Error};

type Point = (usize, usize);
type Heightmap = Vec<Vec<u32>>;

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> Result<u32, Box<dyn Error>> {
    let heightmap: Heightmap = build_heightmap_from_input(puzzle_input);

    let lowpoints: Vec<Point> = get_low_points_from_heightmap(&heightmap);

    let puzzle_answer: u32 = determine_risk_level(&heightmap, &lowpoints);
    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

fn build_heightmap_from_input(input: &str) -> Heightmap {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut heightmap: Heightmap = vec![vec![0; width]; height];

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            heightmap[i][j] = c.to_digit(10).unwrap();
        }
    }

    heightmap
}

fn get_low_points_from_heightmap(heightmap: &[Vec<u32>]) -> Vec<Point> {
    let mut lowpoints = Vec::<Point>::new();

    for (i, row) in heightmap.iter().enumerate() {
        for (j, height) in (*row).iter().enumerate() {
            if i > 0 {
                let up = heightmap.get(i - 1).map(|r| r.get(j)).flatten();
                if let Some(up) = up {
                    if up <= height {
                        continue;
                    }
                }
            }

            if j > 0 {
                let left = heightmap.get(i).map(|r| r.get(j - 1)).flatten();
                if let Some(left) = left {
                    if left <= height {
                        continue;
                    }
                }
            }

            let down = heightmap.get(i + 1).map(|r| r.get(j)).flatten();
            if let Some(down) = down {
                if down <= height {
                    continue;
                }
            }

            let right = heightmap.get(i).map(|r| r.get(j + 1)).flatten();
            if let Some(right) = right {
                if right <= height {
                    continue;
                }
            }

            lowpoints.push((i, j));
        }
    }
    lowpoints
}

fn determine_risk_level(heightmap: &[Vec<u32>], lowpoints: &[Point]) -> u32 {
    lowpoints.iter().map(|(i, j)| heightmap[*i][*j] + 1).sum()
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> Result<u128, Box<dyn Error>> {
    let heightmap: Heightmap = build_heightmap_from_input(puzzle_input);

    let lowpoints: Vec<Point> = get_low_points_from_heightmap(&heightmap);

    let largest_basins: (u32, u32, u32) = find_largest_basins(&heightmap, &lowpoints);

    let puzzle_answer: u128 =
        largest_basins.0 as u128 * largest_basins.1 as u128 * largest_basins.2 as u128;
    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

fn find_largest_basins(heightmap: &[Vec<u32>], lowpoints: &[Point]) -> (u32, u32, u32) {
    let mut basin_sizes = vec![];

    let mut visited: HashSet<Point> = HashSet::new();
    for point in lowpoints {
        let mut this_basin_size = 0;
        let mut to_visit: Vec<Point> = vec![*point];

        while !to_visit.is_empty() {
            let current = to_visit.pop().expect("to_visit was empty");
            // Skip if already visited
            if visited.contains(&current) {
                continue;
            }
            // Skip if out of bounds (I really want to unwrap_or_continue here)
            let maybe_height = heightmap.get(current.0).map(|r| r.get(current.1)).flatten();
            let current_height: u32 = match maybe_height {
                Some(x) => *x,
                None => continue,
            };

            // Skip if height is 9
            if current_height >= 9 {
                continue;
            }

            // Add if not visited
            if current.0 > 0 {
                let above = (current.0 - 1, current.1);
                if !visited.contains(&above) {
                    to_visit.push(above);
                }
            }
            if current.1 > 0 {
                let left = (current.0, current.1 - 1);
                if !visited.contains(&left) {
                    to_visit.push(left);
                }
            }
            let down = (current.0 + 1, current.1);
            if !visited.contains(&down) {
                to_visit.push(down);
            }
            let right = (current.0, current.1 + 1);
            if !visited.contains(&right) {
                to_visit.push(right);
            }

            // Visit this space
            this_basin_size += 1;
            visited.insert(current);
        }
        // println!("Basin centered at point {},{} is size {}", point.0, point.1, this_basin_size);
        basin_sizes.push(this_basin_size);
    }

    basin_sizes.sort_unstable();

    // largest_basins
    (
        basin_sizes.pop().unwrap(),
        basin_sizes.pop().unwrap(),
        basin_sizes.pop().unwrap(),
    )
}

#[cfg(test)]
mod day09_tests {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678\n";

        assert_eq!(15, part1(input).unwrap())
    }

    #[test]
    fn test_case_part_2() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678\n";

        assert_eq!(1134, part2(input).unwrap())
    }
}
