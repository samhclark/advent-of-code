// use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/2023/day01.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = sum_calibration_values(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {}

fn sum_calibration_values(calibration_document: &str) -> u64 {
    calibration_document
        .lines()
        .map(|mangled_value| {
            let first_digit: char = mangled_value
                .to_owned()
                .chars()
                .filter(|it| it.is_ascii_digit())
                .next()
                .expect("all lines container at least one digit");
            let last_digit: char = mangled_value
                .to_owned()
                .chars()
                .filter(|it| it.is_ascii_digit())
                .last()
                .expect("all lines container at least one digit");
            let calibration_value = format!("{first_digit}{last_digit}");
            u64::from_str_radix(&calibration_value, 10)
                .expect("calibration value always fits in 8 bits")
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(sum_calibration_values(input), 142);
    }

    #[test]
    fn test_case_part_2() {
        assert_eq!(2, 2);
    }
}
