// Day 8: Seven Segment Search

use std::error::Error;

struct Reading {
    patterns: Vec<String>,
    display: Vec<String>,
}

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> Result<i64, Box<dyn Error>> {
    let mut all_patterns: Vec<Vec<&str>> = vec![vec![]];
    let mut all_outputs: Vec<Vec<&str>> = vec![vec![]];

    for line in puzzle_input.lines() {
        let mut split = line.trim().split('|');
        let one_pattern: Vec<&str> = split
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .collect();
        let one_output: Vec<&str> = split
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .collect();
        all_patterns.push(one_pattern);
        all_outputs.push(one_output);
    }

    let mut puzzle_answer: i64 = 0;
    for output in all_outputs {
        for digit in output {
            if digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7 {
                puzzle_answer += 1;
            }
        }
    }

    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

#[allow(dead_code)]
pub fn part2(puzzle_input: &str) -> Result<i64, Box<dyn Error>> {
    let all_readings: Vec<Reading> = puzzle_input
        .lines()
        .map(|line| {
            let mut split = line.split('|');
            let one_pattern: Vec<String> = split
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| {
                    let mut chars = s.chars().collect::<Vec<char>>();
                    chars.sort_unstable();
                    String::from_iter(chars)
                })
                .collect();
            let one_output: Vec<String> = split
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| {
                    let mut chars = s.chars().collect::<Vec<char>>();
                    chars.sort_unstable();
                    String::from_iter(chars)
                })
                .collect();
            Reading {
                patterns: one_pattern,
                display: one_output,
            }
        })
        .collect();

    let puzzle_answer: i64 = all_readings.iter().map(calculate_output).sum();
    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

fn calculate_output(reading: &Reading) -> i64 {
    // We need the patterns of four and seven to figure out all the others
    let segments_of_four = &reading.patterns.iter().find(|p| p.len() == 4).unwrap();
    let segments_of_seven = &reading.patterns.iter().find(|p| p.len() == 3).unwrap();

    let mut pattern_for_digit: Vec<String> = vec!["".to_string(); 10];
    for this_pattern in &reading.patterns {
        match (
            this_pattern.len(),
            number_of_common_segments(this_pattern, segments_of_four),
            number_of_common_segments(this_pattern, segments_of_seven),
        ) {
            (6, 3, 3) => pattern_for_digit[0] = this_pattern.to_string(),
            (2, 2, 2) => pattern_for_digit[1] = this_pattern.to_string(),
            (5, 2, 2) => pattern_for_digit[2] = this_pattern.to_string(),
            (5, 3, 3) => pattern_for_digit[3] = this_pattern.to_string(),
            (4, 4, 2) => pattern_for_digit[4] = this_pattern.to_string(),
            (5, 3, 2) => pattern_for_digit[5] = this_pattern.to_string(),
            (6, 3, 2) => pattern_for_digit[6] = this_pattern.to_string(),
            (3, 2, 3) => pattern_for_digit[7] = this_pattern.to_string(),
            (7, 4, 3) => pattern_for_digit[8] = this_pattern.to_string(),
            (6, 4, 3) => pattern_for_digit[9] = this_pattern.to_string(),
            (_, _, _) => unreachable!(),
        };
    }

    let mut output: String = "".to_string();
    for digit in &reading.display {
        'inner: for (i, mapping) in pattern_for_digit.iter().enumerate() {
            if digit == mapping { // This comparison only works because we sorted all the characters
                // We know they are all single digits
                output.push(i.to_string().chars().next().unwrap());
                break 'inner;
            }
        }
    }
    output.parse::<i64>().unwrap()
}

fn number_of_common_segments(pattern: &str, segments: &str) -> usize {
    let chars: Vec<char> = segments.chars().collect();
    let in_common = chars.iter().filter(|c| pattern.contains(**c)).count();
    in_common
}

#[cfg(test)]
mod day08_tests {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n";

        assert_eq!(26, part1(input).unwrap())
    }

    #[test]
    fn test_case_part_2_easy() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n";

        assert_eq!(5353, part2(input).unwrap())
    }

    #[test]
    fn test_case_part_2_hard() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n";

        assert_eq!(61229, part2(input).unwrap())
    }
}
