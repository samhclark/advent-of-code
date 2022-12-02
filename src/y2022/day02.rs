// Day 2: Rock Paper Scissors

use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/2022/day02.in");

#[allow(dead_code)]
pub fn part01() {
    println!("Answer is: {}", calc_score_for_strategy(INPUT));
}

#[allow(dead_code)]
pub fn part02() {
    println!("Answer is: {}", todo!());
}

fn calc_score_for_strategy(strategy: &str) -> u64 {
    let mut total_score: u64 = 0;
    for round in strategy.lines() {
        let (theirs, mine) = round.split_once(' ').unwrap();
        println!("New round: theirs: {}, mine: {}", theirs, mine);
        let theirs = theirs.chars().nth(0).unwrap() as u8;
        let mine = mine.chars().nth(0).unwrap() as u8;
        let theirs = theirs - b'A' + 1;
        let mine = mine - b'X' + 1;
        println!("as bytes: theirs: {}, mine: {}", theirs, mine);

        if mine == theirs {
            total_score += 3;
            println!("draw")
        } else if (mine == 1 && theirs == 3) 
            || (mine == 2 && theirs == 1)
            || (mine == 3 && theirs == 2) {
            total_score += 6;
            println!("win")
        }
        total_score += mine as u64;
    }
    
    total_score
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "A Y
B X
C Z\n";
        assert_eq!(calc_score_for_strategy(input), 15);
    }

    #[test]
    fn test_case_part_2() {
        let input = "";
        assert_eq!(true, true);
    }
}
