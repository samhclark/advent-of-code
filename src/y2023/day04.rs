// Day 4: Scratchcards

use std::collections::HashSet;

static INPUT: &str = include_str!("../../inputs/2023/day04.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = total_points_of_scratchers(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = how_many_after_playing(INPUT);
    println!("Puzzle answer: {answer}");
}

fn total_points_of_scratchers(scratchcards: &str) -> u64 {
    scratchcards
        .lines()
        .map(|card| {
            let (_header, numbers) = card.split_once(':').expect("all cards have a header");
            let (winning_numbers_str, your_numbers_str) = numbers
                .split_once('|')
                .expect("all cards have two sections of numbers");
            let winning_numbers: HashSet<u32> = winning_numbers_str
                .split_ascii_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            let points = your_numbers_str
                .split_ascii_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .filter(|num| winning_numbers.contains(num))
                .fold(0, |acc, _e| if acc == 0 { 1 } else { acc * 2 });
            points
        })
        .sum()
}

fn how_many_after_playing(scratchcards: &str) -> u64 {
    let mut copies = [0u32; 194]; // there are max 193 cards by id. Add 1 to fix offsets

    for card in scratchcards.lines() {
        let (header, numbers) = card.split_once(':').expect("all cards have a header");
        let this_card = header
            .strip_prefix("Card")
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        copies[this_card] += 1;

        let (winning_numbers_str, your_numbers_str) = numbers
            .split_once('|')
            .expect("all cards have two sections of numbers");
        let winning_numbers: HashSet<u32> = winning_numbers_str
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let num_winners = your_numbers_str
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .filter(|num| winning_numbers.contains(num))
            .count();
        if num_winners > 0 {
            let copies_of_this_card = copies[this_card];
            for offset in 1..=num_winners {
                copies[this_card + offset] += copies_of_this_card;
            }
        }
    }

    copies.iter().map(|i| u64::from(*i)).sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(total_points_of_scratchers(input), 13);
    }

    #[test]
    fn test_case_part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(how_many_after_playing(input), 30);
    }
}
