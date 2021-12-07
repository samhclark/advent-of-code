// Day 5: Hydrothermal Venture

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

        let mut x = self.start.x as i64;
        let mut y = self.start.y as i64;

        let x_step = (self.end.x as i64 - self.start.x as i64).signum();
        let y_step = (self.end.y as i64 - self.start.y as i64).signum();

        while (x, y) != (self.end.x as i64, self.end.y as i64) {
            points.push(Point { x: x as usize, y: y as usize});
            x += x_step;
            y += y_step;
        }
        points.push(Point { x: x as usize, y: y as usize});

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
