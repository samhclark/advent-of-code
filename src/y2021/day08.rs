// Day 8: Seven Segment Search

use std::error::Error;

struct Reading {
    patterns: Vec<String>,
    display: Vec<String>,
}

#[allow(dead_code)]
pub fn part1(puzzle_input: &str) -> Result<usize, Box<dyn Error>> {
    let puzzle_answer: usize = puzzle_input
        .lines()
        .flat_map(|line| -> Vec<&str> {
            let mut split = line.trim().split('|');
            split
                .nth(1)
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .collect()
        })
        .filter(|digit| {
            digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7
        })
        .count();

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
                .map(|s| s.to_owned())
                .collect();
            let one_output: Vec<String> = split
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.to_owned())
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

    let output: &str = &reading
        .display
        .iter()
        .map(|digit| -> char {
            match (
                digit.len(),
                number_of_common_segments(digit, segments_of_four),
                number_of_common_segments(digit, segments_of_seven),
            ) {
                (6, 3, 3) => '0',
                (2, 2, 2) => '1',
                (5, 2, 2) => '2',
                (5, 3, 3) => '3',
                (4, 4, 2) => '4',
                (5, 3, 2) => '5',
                (6, 3, 2) => '6',
                (3, 2, 3) => '7',
                (7, 4, 3) => '8',
                (6, 4, 3) => '9',
                (_, _, _) => unreachable!(),
            }
        })
        .collect::<String>();

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
