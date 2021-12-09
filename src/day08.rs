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
        let mut split = line.split('|');
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
    let mut all_readings: Vec<Reading> = vec![];

    for line in puzzle_input.lines() {
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
        all_readings.push(Reading {
            patterns: one_pattern,
            display: one_output,
        });
    }

    let puzzle_answer: i64 = all_readings.iter().map(calculate_output).sum();
    println!("Puzzle answer: {}", puzzle_answer);
    Ok(puzzle_answer)
}

fn calculate_output(reading: &Reading) -> i64 {
    let mut these_chars: Vec<String> = vec!["".to_string(); 10];
    for pattern in &reading.patterns {
        if pattern.len() == 2 {
            these_chars[1] = pattern.to_string();
        } else if pattern.len() == 3 {
            these_chars[7] = pattern.to_string();
        } else if pattern.len() == 4 {
            these_chars[4] = pattern.to_string();
        } else if pattern.len() == 7 {
            these_chars[8] = pattern.to_string();
        }
    }
    // We know the patterns of 1, 4, 7, and 8

    for pattern in &reading.patterns {
        if pattern.len() == 5 {
            if contains_all_chars(pattern, &these_chars[7]) {
                these_chars[3] = pattern.to_string();
            } else {
                let in_common_with_four = number_of_common_segments(pattern, &these_chars[4]);
                if in_common_with_four == 2 {
                    these_chars[2] = pattern.to_string();
                } else if in_common_with_four == 3 {
                    these_chars[5] = pattern.to_string();
                }
            }
        } else if pattern.len() == 6 {
            if contains_all_chars(pattern, &these_chars[7])
                && contains_all_chars(pattern, &these_chars[4])
            {
                these_chars[9] = pattern.to_string();
            } else {
                let in_common_with_seven = number_of_common_segments(pattern, &these_chars[7]);
                if in_common_with_seven == 2 {
                    these_chars[6] = pattern.to_string();
                } else if in_common_with_seven == 3 {
                    these_chars[0] = pattern.to_string();
                }
            }
        }
    }
    // We know all the patterns

    let mut output: String = "".to_string();
    for digit in &reading.display {
        'inner: for (i, mapping) in these_chars.iter().enumerate() {
            if digit == mapping {
                output.push(i.to_string().chars().next().unwrap());
                break 'inner;
            }
        }
    }
    output.parse::<i64>().unwrap()
}

fn contains_all_chars(a: &str, b: &str) -> bool {
    let chars: Vec<char> = b.chars().collect();
    chars.iter().all(|c| a.contains(*c))
}

fn number_of_common_segments(pattern: &str, segments: &str) -> usize {
    let chars: Vec<char> = segments.chars().collect();

    let in_common = chars.iter().filter(|c| pattern.contains(**c)).count();
    println!("in common: {}", in_common);
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
