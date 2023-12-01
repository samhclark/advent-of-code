// Day 11: Dumbo Octopus

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> i64 {
    let mut octopus_grid: [[u32; 10]; 10] = make_octopus_grid(puzzle_input);

    let puzzle_answer: i64 = (0..100).map(|_| step(&mut octopus_grid)).sum();

    println!("Puzzle answer: {puzzle_answer}");
    puzzle_answer
}

fn make_octopus_grid(input: &str) -> [[u32; 10]; 10] {
    let mut grid = [[0u32; 10]; 10];

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c.to_digit(10).unwrap();
        }
    }

    grid
}

fn step(grid: &mut [[u32; 10]; 10]) -> i64 {
    let mut flashes: Vec<(usize, usize)> = Vec::new();

    // Step 1: increment all the octopuses
    for (i, row) in grid.iter_mut().enumerate().take(10) {
        for (j, octopus) in row.iter_mut().enumerate().take(10) {
            *octopus += 1;
            if *octopus > 9 {
                flashes.push((i, j));
            }
        }
    }

    // Step 2: Check for flashes
    let mut num_flashes = 0;
    while !flashes.is_empty() {
        let (i, j) = flashes.pop().unwrap();
        if grid[i][j] == 0 {
            continue;
        }

        grid[i][j] = 0;
        num_flashes += 1;

        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if i > 0 {
            // top
            neighbors.push((i - 1, j));
        }
        if i > 0 && j > 0 {
            // top left
            neighbors.push((i - 1, j - 1));
        }
        if j > 0 {
            // left
            neighbors.push((i, j - 1));
        }
        if i < 9 && j > 0 {
            // botton left
            neighbors.push((i + 1, j - 1));
        }
        if i < 9 {
            // bottom
            neighbors.push((i + 1, j));
        }
        if i < 9 && j < 9 {
            // botton right
            neighbors.push((i + 1, j + 1));
        }
        if j < 9 {
            // right
            neighbors.push((i, j + 1));
        }
        if i > 0 && j < 9 {
            // top right
            neighbors.push((i - 1, j + 1));
        }

        for (i, j) in neighbors {
            if grid[i][j] != 0 {
                // already flashed
                grid[i][j] += 1;
            }
            if grid[i][j] > 9 {
                flashes.push((i, j));
            }
        }
    }

    num_flashes
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> i64 {
    let mut octopus_grid: [[u32; 10]; 10] = make_octopus_grid(puzzle_input);

    let mut step_number = 1;
    while step(&mut octopus_grid) < 100 {
        step_number += 1;
    }
    let puzzle_answer: i64 = step_number;
    println!("Puzzle answer: {puzzle_answer}");
    puzzle_answer
}

#[cfg(test)]
mod day11_tests {

    use super::*;

    #[test]
    fn test_case_part_1() {
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

        assert_eq!(1656, part1(input))
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

        assert_eq!(195, part2(input))
    }
}
