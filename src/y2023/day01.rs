use itertools::Itertools;

static INPUT: &str = include_str!("../../inputs/2023/day01.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = sum_calibration_values(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let normal_doc = normalize_calibration_document(INPUT);
    let answer = sum_calibration_values(&normal_doc);
    println!("Puzzle answer: {answer}");
}

fn sum_calibration_values(calibration_document: &str) -> u64 {
    calibration_document
        .lines()
        .map(|mangled_value| {
            println!("line: {mangled_value}");
            let first_digit: char = mangled_value
                .to_owned()
                .chars()
                .find(char::is_ascii_digit)
                .expect("(first) all lines contain at least one digit");
            let last_digit: char = mangled_value
                .to_owned()
                .chars()
                .filter(char::is_ascii_digit)
                .last()
                .expect("(last) all lines contain at least one digit");
            let calibration_value = format!("{first_digit}{last_digit}");
            calibration_value
                .parse::<u64>()
                .expect("calibration value always fits in 8 bits")
        })
        .sum()
}

#[allow(clippy::many_single_char_names)]
fn normalize_calibration_document(calibration_document: &str) -> String {
    let mut normalized_document = String::new();
    for mangled_line in calibration_document.lines() {
        let mut normal_line = String::new();
        if mangled_line.len() >= 5 {
            for (c0, c1, c2, c3, c4) in mangled_line.chars().tuple_windows() {
                if c0.is_ascii_digit() {
                    normal_line.push(c0);
                    continue;
                }
                if let Some(c) = english_digit_from_start(c0, c1, c2, c3, c4) {
                    normal_line.push(c);
                }
            }
        }
        if mangled_line.len() >= 4 {
            if let Some((d, c, b, a)) = mangled_line.chars().rev().take(4).collect_tuple() {
                if a.is_ascii_digit() {
                    normal_line.push(a);
                }
                if let Some(other_three_letter) = three_letter_number(a, b, c) {
                    normal_line.push(other_three_letter);
                }
                if let Some(three_letter) = three_letter_number(b, c, d) {
                    normal_line.push(three_letter);
                }
                if let Some(four_letter) = four_letter_number(a, b, c, d) {
                    normal_line.push(four_letter);
                }
                if b.is_ascii_digit() {
                    normal_line.push(b);
                }
                if c.is_ascii_digit() {
                    normal_line.push(c);
                }
                if d.is_ascii_digit() {
                    normal_line.push(d);
                }
            }
        }

        if mangled_line.len() == 3 {
            if let Some((a, b, c)) = mangled_line.chars().take(3).collect_tuple() {
                if let Some(other_three_letter) = three_letter_number(a, b, c) {
                    normal_line.push(other_three_letter);
                }
                if a.is_ascii_digit() {
                    normal_line.push(a);
                }
                if b.is_ascii_digit() {
                    normal_line.push(b);
                }
                if c.is_ascii_digit() {
                    normal_line.push(c);
                }
            }
        }

        if mangled_line.len() < 3 {
            normal_line.push_str(mangled_line);
        }

        // println!("{normal_line}");
        normalized_document.push_str(&normal_line);
        normalized_document.push('\n');
    }
    normalized_document
}

#[allow(clippy::many_single_char_names)]
const fn english_digit_from_start(a: char, b: char, c: char, d: char, e: char) -> Option<char> {
    let five_letter = five_letter_number(a, b, c, d, e);
    if five_letter.is_some() {
        return five_letter;
    }

    let four_letter = four_letter_number(a, b, c, d);
    if four_letter.is_some() {
        return four_letter;
    }

    three_letter_number(a, b, c)
}

const fn three_letter_number(a: char, b: char, c: char) -> Option<char> {
    match (a, b, c) {
        ('o', 'n', 'e') => Some('1'),
        ('t', 'w', 'o') => Some('2'),
        ('s', 'i', 'x') => Some('6'),
        _ => None,
    }
}

const fn four_letter_number(a: char, b: char, c: char, d: char) -> Option<char> {
    match (a, b, c, d) {
        ('f', 'o', 'u', 'r') => Some('4'),
        ('f', 'i', 'v', 'e') => Some('5'),
        ('n', 'i', 'n', 'e') => Some('9'),
        _ => None,
    }
}

#[allow(clippy::many_single_char_names)]
const fn five_letter_number(a: char, b: char, c: char, d: char, e: char) -> Option<char> {
    match (a, b, c, d, e) {
        ('t', 'h', 'r', 'e', 'e') => Some('3'),
        ('s', 'e', 'v', 'e', 'n') => Some('7'),
        ('e', 'i', 'g', 'h', 't') => Some('8'),
        _ => None,
    }
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let normal = normalize_calibration_document(input);
        assert_eq!(sum_calibration_values(&normal), 281);
    }

    #[test]
    fn eight_three() {
        assert_eq!(normalize_calibration_document("eighthree"), "83\n");
    }

    #[test]
    fn seven_nine() {
        assert_eq!(normalize_calibration_document("sevenine"), "79\n");
    }

    #[test]
    fn edge_1() {
        let normal = normalize_calibration_document("three2fiveonexrllxsvfive");
        assert_eq!(normal, "32515\n");
        assert_eq!(sum_calibration_values(&normal), 35);
    }

    #[test]
    fn edge_2() {
        let normal = normalize_calibration_document("two1two");
        assert_eq!(normal, "212\n");
        assert_eq!(sum_calibration_values(&normal), 22);
    }
}
