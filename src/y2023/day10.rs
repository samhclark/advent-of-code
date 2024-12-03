// Day 10: Pipe Maze

// use itertools::Itertools;
// use rayon::prelude::*;

static INPUT: &str = include_str!("../../inputs/2023/day10.in");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Runner {
    previous: Point,
    current: Point,
}

impl Runner {
    const fn move_to(self, point: Point) -> Self {
        Self {
            previous: self.current,
            current: point,
        }
    }
}

#[allow(dead_code)]
pub fn part01() {
    let answer = find_farthest_from_start(INPUT);
    println!("Puzzle answer: {answer}");
}

fn find_farthest_from_start(pipe_sketch: &str) -> u64 {
    let mut starting_point: Point = Point { x: 0, y: 0 };
    'outer: for (row, line) in pipe_sketch.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if Tile::from(c) == Tile::Start {
                starting_point = Point { x: column, y: row };
                break 'outer;
            }
        }
    }
    let starting_point = starting_point;

    let grid = pipe_sketch
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();

    // from the starting point, walk the grid in both directions.
    // to walk the grid, there are two "runners". Set them up and let them run
    let (a, b) = adjacent_pipes(&starting_point, &grid);
    let mut runner_1 = Runner {
        previous: starting_point,
        current: a,
    };
    let mut runner_2 = Runner {
        previous: starting_point,
        current: b,
    };
    let mut steps: u64 = 1;

    while runner_1.current != runner_2.current {
        // println!("Step {steps}: r1 = {runner_1:?} and r2 = {runner_2:?}");
        let (option_1, option_2) = adjacent_pipes(&runner_1.current, &grid);
        if option_1 == runner_1.previous {
            runner_1 = runner_1.move_to(option_2);
        } else {
            runner_1 = runner_1.move_to(option_1);
        }

        let (option_1, option_2) = adjacent_pipes(&runner_2.current, &grid);
        if option_1 == runner_2.previous {
            runner_2 = runner_2.move_to(option_2);
        } else {
            runner_2 = runner_2.move_to(option_1);
        }

        // if steps >= u8::MAX as u64 {
        //     panic!("hit step limit");
        // }
        steps += 1;
    }

    steps
}

fn adjacent_pipes(point: &Point, tiles: &[Vec<Tile>]) -> (Point, Point) {
    let tile = tiles.get(point.y).unwrap().get(point.x).unwrap();
    match tile {
        Tile::NS => (
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
        ),
        Tile::EW => (
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
        ),
        Tile::NE => (
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
        ),
        Tile::NW => (
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
        ),
        Tile::SW => (
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
        ),
        Tile::SE => (
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
        ),
        Tile::Ground => panic!("Tile at point {point:?} is ground. No adjacent pipes possible"),
        Tile::Start => {
            let mut adjacents: Vec<Point> = Vec::with_capacity(2);
            // in my cases, the start is never on the edge
            let north: Tile = *tiles.get(point.y - 1).unwrap().get(point.x).unwrap();
            let east: Tile = *tiles.get(point.y).unwrap().get(point.x + 1).unwrap();
            let south: Tile = *tiles.get(point.y + 1).unwrap().get(point.x).unwrap();
            let west: Tile = *tiles.get(point.y).unwrap().get(point.x - 1).unwrap();

            match north {
                Tile::NS | Tile::SE | Tile::SW => adjacents.push(Point {
                    x: point.x,
                    y: point.y - 1,
                }),
                _ => (),
            };

            match east {
                Tile::NW | Tile::EW | Tile::SW => adjacents.push(Point {
                    x: point.x + 1,
                    y: point.y,
                }),
                _ => (),
            };

            match south {
                Tile::NS | Tile::NE | Tile::NW => adjacents.push(Point {
                    x: point.x,
                    y: point.y + 1,
                }),
                _ => (),
            };

            match west {
                Tile::NE | Tile::EW | Tile::SE => adjacents.push(Point {
                    x: point.x - 1,
                    y: point.y,
                }),
                _ => (),
            };

            (adjacents.pop().unwrap(), adjacents.pop().unwrap())
        }
    }
}

#[allow(dead_code)]
pub fn part02() {
    let answer = "";
    println!("Puzzle answer: {answer}");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1_small() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(find_farthest_from_start(input), 4);
    }

    #[test]
    fn test_case_part_1_med() {
        let input = "...F7.
..FJ|.
.SJ.L7
.|F--J
.LJ...";
        assert_eq!(find_farthest_from_start(input), 8);
    }
}
