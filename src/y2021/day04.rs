// Day 4: Giant Squid

use std::num::ParseIntError;
use std::str::FromStr;

const CARD_SIZE: usize = 5;
const CARD_DELIMITER: &str = "\n\n";

#[derive(Debug, Clone, Copy)]
struct Square {
    number: u32,
    is_called: bool,
}

#[derive(Debug, Clone, Copy)]
struct Card {
    squares: [[Square; CARD_SIZE]; CARD_SIZE],
    has_bingo: bool,
}

impl Card {
    /// Marks a number that has been called.
    ///
    /// Has the side effect of calculating `has_bingo` for the Card.
    fn mark(&mut self, number: u32) {
        'outer: for (i, row) in self.squares.iter_mut().enumerate() {
            for (j, square) in row.iter_mut().enumerate() {
                if square.number == number {
                    square.is_called = true;
                    if (self.squares[i][0].is_called
                        && self.squares[i][1].is_called
                        && self.squares[i][2].is_called
                        && self.squares[i][3].is_called
                        && self.squares[i][4].is_called)
                        || (self.squares[0][j].is_called
                            && self.squares[1][j].is_called
                            && self.squares[2][j].is_called
                            && self.squares[3][j].is_called
                            && self.squares[4][j].is_called)
                    {
                        self.has_bingo = true;
                    }
                    break 'outer;
                }
            }
        }
    }
}

impl FromStr for Card {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed_card: Self = Self {
            squares: [[Square {
                number: 0,
                is_called: false,
            }; CARD_SIZE]; CARD_SIZE],
            has_bingo: false,
        };
        for (i, row) in s.lines().enumerate() {
            for (j, num) in row.split_ascii_whitespace().enumerate() {
                parsed_card.squares[i][j] = num.parse::<Square>().unwrap();
            }
        }
        Ok(parsed_card)
    }
}

impl FromStr for Square {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            number: s.parse::<u32>().unwrap(),
            is_called: false,
        })
    }
}

#[allow(dead_code)]
pub fn part1(puzzle_numbers: &str, puzzle_boards: &str) -> u64 {
    let numbers: Vec<u32> = puzzle_numbers
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut cards: Vec<Card> = puzzle_boards
        .split(CARD_DELIMITER)
        .map(|five_lines| five_lines.parse::<Card>().unwrap())
        .collect();

    let mut winning_card: Option<Card> = None;
    let mut winning_number: Option<u32> = None;
    'outer: for number in numbers {
        for card in &mut cards {
            card.mark(number);
            if card.has_bingo {
                winning_card = Some(*card);
                winning_number = Some(number);
                break 'outer;
            }
        }
    }

    let puzzle_answer = calculate_puzzle_answer(winning_card.unwrap(), winning_number.unwrap());

    println!("Puzzle answer: {puzzle_answer}");
    puzzle_answer
}

#[allow(dead_code)]
pub fn part2(puzzle_numbers: &str, puzzle_boards: &str) -> u64 {
    let numbers: Vec<u32> = puzzle_numbers
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut cards: Vec<Card> = puzzle_boards
        .split(CARD_DELIMITER)
        .map(|five_lines| five_lines.parse::<Card>().unwrap())
        .collect();

    let mut last_card_to_win: Option<Card> = None;
    let mut last_number_to_produce_win: Option<u32> = None;
    for number in numbers {
        for card in &mut cards {
            if !card.has_bingo {
                card.mark(number);
                if card.has_bingo {
                    last_card_to_win = Some(*card);
                    last_number_to_produce_win = Some(number);
                }
            }
        }
    }

    println!("Last winning card was: {last_card_to_win:?}", );
    println!("Last number called to produce win was: {last_number_to_produce_win:?}");

    let puzzle_answer = calculate_puzzle_answer(
        last_card_to_win.unwrap(),
        last_number_to_produce_win.unwrap(),
    );

    println!("Puzzle answer: {puzzle_answer}");
    puzzle_answer
}

fn calculate_puzzle_answer(card: Card, number: u32) -> u64 {
    let sum_all_unmarked_numbers: u64 = card
        .squares
        .into_iter()
        .flat_map(std::iter::IntoIterator::into_iter)
        .filter(|square| !square.is_called)
        .map(|square| u64::from(square.number))
        .sum::<u64>();

    sum_all_unmarked_numbers * u64::from(number)
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn test_case_part_1() {
        let numbers = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
        let boards = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(4512, part1(numbers, boards))
    }
}
