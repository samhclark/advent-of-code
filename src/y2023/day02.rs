use std::{cmp::max, num::ParseIntError, str::FromStr};

static INPUT: &str = include_str!("../../inputs/2023/day02.in");

#[derive(Debug)]
struct Handful {
    pub red_qty: u64,
    pub green_qty: u64,
    pub blue_qty: u64,
}

impl FromStr for Handful {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        let mut blue: u64 = 0;
        let trimmed = s.trim();
        let quantities = trimmed.split(',');
        for count_of_one_color in quantities {
            let quantity_str: String = count_of_one_color
                .chars()
                .filter(char::is_ascii_digit)
                .collect::<String>();
            let quantity = quantity_str
                .parse::<u64>()
                .expect("digits always fit in 64 bits");
            let color_str: String = count_of_one_color
                .chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<String>();
            match color_str.as_str() {
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

#[allow(dead_code)]
pub fn part01() {
    let answer = sum_ids_of_possible_games(INPUT, 100);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = sum_of_powers_of_min_sets(INPUT);
    println!("Puzzle answer: {answer}");
}

fn sum_ids_of_possible_games(records_of_games: &str, num_games: u64) -> u64 {
    // To let the loop exit early, we'll assume all games are possible, and subtract the IDs of
    // games that we determine are impossible.
    let mut sum_of_ids: u64 = (1..=num_games).sum();
    'thisgame: for (id, game) in records_of_games.lines().enumerate() {
        // strip game header
        let (_header, rounds) = game.split_once(':').expect("all games have a header");

        // there's probably an idiomatic way to split this into "handfuls.any { exceeds_max }..."
        for round in rounds.split(';') {
            let handful = Handful::from_str(round).expect("always parseable");
            if exceeds_max_allowed(&handful) {
                sum_of_ids -= 1 + id as u64;
                continue 'thisgame;
            }
        }
    }

    sum_of_ids
}

const fn exceeds_max_allowed(handful: &Handful) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    handful.red_qty > max_red || handful.green_qty > max_green || handful.blue_qty > max_blue
}

fn sum_of_powers_of_min_sets(records_of_games: &str) -> u64 {
    records_of_games
        .lines()
        .map(|game| {
            // strip game header
            let (_header, rounds) = game.split_once(':').expect("all games have a header");

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            for round in rounds.split(';') {
                let handful = Handful::from_str(round).expect("always parseable");
                min_red = max(handful.red_qty, min_red);
                min_green = max(handful.green_qty, min_green);
                min_blue = max(handful.blue_qty, min_blue);
            }

            min_red * min_green * min_blue
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
        assert_eq!(sum_ids_of_possible_games(input, 5), 8);
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
