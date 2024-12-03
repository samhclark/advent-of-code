use std::{cmp::max, num::ParseIntError, str::FromStr};

static INPUT: &str = include_str!("../../inputs/2023/day02.in");

#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Copy)]
struct Handful {
    red_qty: u64,
    green_qty: u64,
    blue_qty: u64,
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

impl FromStr for Handful {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        let mut blue: u64 = 0;
        for count_of_one_color in s.trim().split(',') {
            let (qty, color) = count_of_one_color
                .trim()
                .split_once(' ')
                .expect("format is 'X color'");
            let quantity = qty.parse::<u64>().expect("digits always fit in 64 bits");
            match color {
                "red" => red = quantity,
                "green" => green = quantity,
                "blue" => blue = quantity,
                _ => unreachable!("colors are always red, green, or blue"),
            };
        }
        Ok(Self {
            red_qty: red,
            green_qty: green,
            blue_qty: blue,
        })
    }
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, rounds) = s.split_once(':').expect("all games have a header");
        let id: u32 = header.strip_prefix("Game ").unwrap().parse()?;
        let handfuls: Vec<Handful> = rounds
            .trim()
            .split(';')
            .map(|s| Handful::from_str(s).unwrap())
            .collect();
        Ok(Self { id, handfuls })
    }
}

impl Handful {
    const fn exceeds_max(&self) -> bool {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        self.red_qty > max_red || self.green_qty > max_green || self.blue_qty > max_blue
    }
}

impl Game {
    fn power(&self) -> u64 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for handful in &self.handfuls {
            min_red = max(handful.red_qty, min_red);
            min_green = max(handful.green_qty, min_green);
            min_blue = max(handful.blue_qty, min_blue);
        }

        min_red * min_green * min_blue
    }

    fn is_impossible(&self) -> bool {
        return self.handfuls.iter().any(|&h| h.exceeds_max());
    }
}

#[allow(dead_code)]
pub fn part01() {
    let answer = sum_ids_of_possible_games(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = sum_of_powers_of_min_sets(INPUT);
    println!("Puzzle answer: {answer}");
}

fn sum_ids_of_possible_games(records_of_games: &str) -> u64 {
    records_of_games
        .lines()
        .map(|line| {
            let game = Game::from_str(line).expect("always parses");
            if game.is_impossible() {
                0
            } else {
                u64::from(game.id)
            }
        })
        .sum()
}

fn sum_of_powers_of_min_sets(records_of_games: &str) -> u64 {
    records_of_games
        .lines()
        .map(|game_str| {
            let game = Game::from_str(game_str).expect("always parses");
            game.power()
        })
        .sum()
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(sum_ids_of_possible_games(input), 8);
    }

    #[test]
    fn test_case_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(sum_of_powers_of_min_sets(input), 2286);
    }
}
