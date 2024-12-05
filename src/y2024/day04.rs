// Day 4: Ceres Search

use crate::aoc::util::PuzzleAnswer;
use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/2024/day04.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = solve_part1(INPUT);
    println!("{answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = solve_part2(INPUT);
    println!("{answer}");
}

fn solve_part1(input: &str) -> PuzzleAnswer {
    PuzzleAnswer::from(count_xmas(input))
}

fn solve_part2(input: &str) -> PuzzleAnswer {
    PuzzleAnswer::from(count_x_mas(input))
}

fn count_xmas(input: &str) -> usize { 
    let xmas = "XMAS";
    let xmas_rev = "SAMX";

    let num_lines = input.lines().count();
    let mut rotated_90: Vec<String> = vec!(String::from(""); num_lines);
    rotated_90.reserve(input.lines().next().unwrap().len());
    
    let mut count = 0;
    for line in input.lines() {
        count += line.matches(xmas).count();
        count += line.matches(xmas_rev).count();

        for  (i, ch) in line.chars().enumerate() {
            rotated_90[num_lines - 1 - i].push(ch);
        }
    }

    // println!("counted: {count}");

    // again with rotated input
    for line in rotated_90.iter() {
        count += line.matches(xmas).count();
        count += line.matches(xmas_rev).count();
    }

    // println!("Rotated 90:");
    // println!("{}", rotated_90.join("\n"));
    // println!("counted: {count}");

    // rotate +/- 45 degrees
    let mut rotated_left_45: Vec<String> = vec!(String::from(""); num_lines * 2);
    let mut rotated_right_45: Vec<String> = vec!(String::from(""); num_lines * 2);
    
    for (i, line) in input.lines().enumerate() {
        let input_line_len = line.len();
        for (j, ch) in line.chars().enumerate() {
            rotated_left_45[input_line_len - 1 - j + i].push(ch)
        }
    }

    for (i, line) in rotated_90.iter().enumerate() {
        let input_line_len = line.len();
        for (j, ch) in line.chars().enumerate() {
            rotated_right_45[input_line_len - 1 - j + i].push(ch)
        }
    }

    for line in rotated_left_45.iter() {
        count += line.matches(xmas).count();
        count += line.matches(xmas_rev).count();
    }
    
    // println!("Rotated left 45:");
    // println!("{}", rotated_left_45.join("\n"));
    // println!("counted: {count}");

    for line in rotated_right_45.iter() {
        count += line.matches(xmas).count();
        count += line.matches(xmas_rev).count();
    }
    
    // println!("Rotated right 45:");
    // println!("{}", rotated_right_45.join("\n"));
    // println!("counted: {count}");

    count
}

fn count_x_mas(input: &str) -> usize {
    let mut count = 0;

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l|
            l.chars().collect::<Vec<char>>()
        ).collect();

    for (i, line) in grid.iter().enumerate() {
        if i == 0 || i == grid.len() - 1 {
            continue;
        }
        for (j, &ch) in line.iter().enumerate() {
            if j == 0 || j == line.len() - 1 {
                continue;
            }

            if ch == 'A' {
                if grid.get(i-1).unwrap().get(j-1).unwrap() == &'M'
                    && grid.get(i-1).unwrap().get(j+1).unwrap() == &'M'
                    && grid.get(i+1).unwrap().get(j-1).unwrap() == &'S'
                    && grid.get(i+1).unwrap().get(j+1).unwrap() == &'S' 
                {
                    count += 1
                } else if grid.get(i-1).unwrap().get(j-1).unwrap() == &'S'
                    && grid.get(i-1).unwrap().get(j+1).unwrap() == &'M'
                    && grid.get(i+1).unwrap().get(j-1).unwrap() == &'S'
                    && grid.get(i+1).unwrap().get(j+1).unwrap() == &'M' 
                {
                    count += 1
                } else if grid.get(i-1).unwrap().get(j-1).unwrap() == &'M'
                    && grid.get(i-1).unwrap().get(j+1).unwrap() == &'S'
                    && grid.get(i+1).unwrap().get(j-1).unwrap() == &'M'
                    && grid.get(i+1).unwrap().get(j+1).unwrap() == &'S' 
                {
                    count += 1
                } else if grid.get(i-1).unwrap().get(j-1).unwrap() == &'S'
                    && grid.get(i-1).unwrap().get(j+1).unwrap() == &'S'
                    && grid.get(i+1).unwrap().get(j-1).unwrap() == &'M'
                    && grid.get(i+1).unwrap().get(j+1).unwrap() == &'M' 
                {
                    count += 1
                }
            }
        }
    }
    
    count
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_0() {
        let input = "ABC
DEF
GHI";

        assert_eq!(count_xmas(input), 0);
    }

    #[test]
    fn test_case_part_1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(count_xmas(input), 18);
    }

    #[test]
    fn test_case_part_2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(count_x_mas(input), 9);
    }
}