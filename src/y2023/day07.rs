// Day 7: Camel Cards

use std::{num::ParseIntError, str::FromStr};

static INPUT: &str = include_str!("../../inputs/2023/day07.in");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct HandAndBid {
    hand: Vec<u8>,
    kind: HandKind,
    bid: u32,
}

impl HandAndBid {
    fn kind_from_values(cards: &[u8]) -> HandKind {
        let mut counts = [0u8; 15];
        for card in cards {
            counts[usize::from(*card)] += 1;
        }
        let mut has_trips: bool = false;
        let mut num_pairs = 0;
        for count in counts {
            match count {
                5 => return HandKind::FiveOfAKind,
                4 => return HandKind::FourOfAKind,
                3 => has_trips = true,
                2 => num_pairs += 1,
                _ => continue,
            }
        }
        if has_trips && num_pairs == 1 {
            return HandKind::FullHouse;
        }
        if has_trips {
            return HandKind::ThreeOfAKind;
        }
        if num_pairs == 2 {
            return HandKind::TwoPair;
        }
        if num_pairs == 1 {
            return HandKind::OnePair;
        }

        HandKind::HighCard
    }
}

impl FromStr for HandAndBid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_hand, raw_bid) = s.split_once(' ').unwrap();
        let hand: Vec<u8> = raw_hand.chars().map(card_to_value).collect();
        let kind: HandKind = Self::kind_from_values(&hand);
        let bid: u32 = raw_bid.parse().unwrap();
        Ok(Self { hand, kind, bid })
    }
}

impl PartialEq for HandAndBid {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    match self.hand[i].cmp(&other.hand[i]) {
                        std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                        std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
                        std::cmp::Ordering::Equal => continue,
                    }
                }
                unreachable!();
            }
        }
    }
}

#[allow(dead_code)]
pub fn part01() {
    let answer = total_winnings(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = "";
    println!("Puzzle answer: {answer}");
}

fn card_to_value(card: char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!("all cards are defined"),
    }
}

fn total_winnings(input: &str) -> u64 {
    let mut inputs: Vec<HandAndBid> = input
        .lines()
        .map(|line| HandAndBid::from_str(line).unwrap())
        .collect();
    let num_hands = inputs.len();

    inputs.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    let winnings: usize = inputs
        .iter()
        .enumerate()
        .map(|(i, hand)| (num_hands - i) * hand.bid as usize)
        .sum();
    winnings as u64
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(total_winnings(input), 6440);
    }

    #[test]
    fn test_case_part_2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(1, 1);
    }
}
