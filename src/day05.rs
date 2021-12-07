// Day 5: Hydrothermal Venture

use itertools::izip;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::error::Error;
use std::iter;
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

        let x_iter: Box<dyn Iterator<Item = usize>> = match self.start.x.cmp(&self.end.x) {
            Ordering::Equal => {
                let delta_y = max(self.start.y, self.end.y) - min(self.start.y, self.end.y) + 1;
                Box::new(iter::repeat(self.start.x).take(delta_y))
            }
            Ordering::Greater => Box::new((self.end.x..=self.start.x).rev()),
            Ordering::Less => Box::new(self.start.x..=self.end.x),
        };

        let y_iter: Box<dyn Iterator<Item = usize>> = match self.start.y.cmp(&self.end.y) {
            Ordering::Equal => {
                let delta_x = max(self.start.x, self.end.x) - min(self.start.x, self.end.x) + 1;
                Box::new(iter::repeat(self.start.y).take(delta_x))
            }
            Ordering::Greater => Box::new((self.end.y..=self.start.y).rev()),
            Ordering::Less => Box::new(self.start.y..=self.end.y),
        };

        for (x, y) in izip!(x_iter, y_iter) {
            points.push(Point { x, y })
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

    let answer = grid
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|&n| n > 1)
        .count();

    println!("Puzzle answer: {}", answer);
    Ok(answer as u64)
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

    let answer = grid
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|&n| n > 1)
        .count();

    println!("Puzzle answer: {}", answer);
    Ok(answer as u64)
}

mod day05_tests {

    // This isn't unused, idk why the compiler thinks it is
    #[allow(unused_imports)] 
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
