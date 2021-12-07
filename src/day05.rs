// Day 5: Hydrothermal Venture

use itertools::izip;
use std::cmp::max;
use std::cmp::min;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

const GRID_SIZE: usize = 1_000;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<u64> = s.split(',').map(|n| n.trim().parse().unwrap()).collect();
        Ok(Point {
            x: coords[0] as usize,
            y: coords[1] as usize,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = s
            .split("->")
            .map(|p| p.trim().parse::<Point>().unwrap())
            .collect();
        Ok(Line {
            start: points[0],
            end: points[1],
        })
    }
}

impl Line {
    fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        if self.start.x == self.end.x {
            let starting_y = min(self.start.y, self.end.y);
            let ending_y = max(self.start.y, self.end.y);
            for y in starting_y..=ending_y {
                points.push(Point {
                    x: self.start.x,
                    y: y,
                })
            }
        } else if self.start.y == self.end.y {
            let starting_x = min(self.start.x, self.end.x);
            let ending_x = max(self.start.x, self.end.x);
            for x in starting_x..=ending_x {
                points.push(Point {
                    x: x,
                    y: self.start.y,
                })
            }
        } else {
            let x_iter: Box<dyn Iterator<Item = usize>> = if self.end.x > self.start.x {
                Box::new(self.start.x..=self.end.x)
            } else {
                Box::new((self.end.x..=self.start.x).rev())
            };
            let y_iter: Box<dyn Iterator<Item = usize>> = if self.end.y > self.start.y {
                Box::new(self.start.y..=self.end.y)
            } else {
                Box::new((self.end.y..=self.start.y).rev())
            };
            for (x, y) in izip!(x_iter, y_iter) {
                points.push(Point { x, y })
            }
        }

        points
    }
}

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> Result<u64, Box<dyn Error>> {
    let mut vents: Vec<Line> = puzzle_input
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .collect();

    vents.retain(|vent| vent.start.x == vent.end.x || vent.start.y == vent.end.y);

    let mut grid: Vec<Vec<usize>> = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for vent in vents {
        let points_to_plot = vent.points();
        for point in points_to_plot {
            grid[point.x][point.y] += 1;
        }
    }

    let mut num_overlaps = 0;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if grid[i][j] > 1 {
                num_overlaps += 1;
            }
        }
    }

    println!("Puzzle answer: {}", num_overlaps);
    Ok(num_overlaps)
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> Result<u64, Box<dyn Error>> {
    let vents: Vec<Line> = puzzle_input
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .collect();

    let mut grid: Vec<Vec<usize>> = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for vent in vents {
        let points_to_plot = vent.points();
        for point in points_to_plot {
            grid[point.x][point.y] += 1;
        }
    }

    let mut num_overlaps = 0;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if grid[i][j] > 1 {
                num_overlaps += 1;
            }
        }
    }

    println!("Puzzle answer: {}", num_overlaps);
    Ok(num_overlaps)
}

mod day05_tests {
    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(5, part1(input).unwrap())
    }

    #[test]
    fn test_case_part_2() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(12, part2(input).unwrap())
    }
}
