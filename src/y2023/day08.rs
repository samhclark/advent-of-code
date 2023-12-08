// Day 8: Haunted Wasteland

static INPUT: &str = include_str!("../../inputs/2023/day08.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = number_of_steps(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = "";
    println!("Puzzle answer: {answer}");
}

fn number_of_steps(map: &str) -> u64 {

    0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_1_part_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, 2);
    }

    #[test]
    fn test_case_2_part_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, 6);
    }
}