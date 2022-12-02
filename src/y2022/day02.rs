// Day 2: Rock Paper Scissors

static INPUT: &str = include_str!("../../inputs/2022/day02.in");

#[allow(dead_code)]
pub fn part01() {
    println!("Answer is: {}", do_part1(INPUT));
}

#[allow(dead_code)]
pub fn part02() {
    println!("Answer is: {}", do_part2(INPUT));
}

fn do_part1(strategy: &str) -> u64 {
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

fn do_part2(strategy: &str) -> u64 {
    let mut total_score: u64 = 0;
    for round in strategy.lines() {
        let (theirs, result) = round.split_once(' ').unwrap();
        println!("New round: theirs: {}, result: {}", theirs, result);
        let theirs = theirs.chars().nth(0).unwrap() as u8;
        let result = result.chars().nth(0).unwrap() as u8;
        let theirs = theirs - b'A' + 1;
        let result = result - b'X';
        println!("as bytes: theirs: {}, result: {}", theirs, result);

        if result == 1 {
            total_score += theirs as u64;
        } else if result == 0 { // have to lose
            if theirs == 1 {
                total_score += 3
            } else if (theirs == 2) {
                total_score += 1;
            } else {
                total_score += 2;
            }
        } else { // have to win
            if theirs == 1 {
                total_score += 2
            } else if (theirs == 2) {
                total_score += 3;
            } else {
                total_score += 1;
            }
        }
        total_score += (result * 3) as u64;
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
        assert_eq!(do_part1(input), 15);
    }

    #[test]
    fn test_case_part_2() {
        let input = "A Y
B X
C Z\n";
        assert_eq!(do_part2(input), 12);
    }
}
