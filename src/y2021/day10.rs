// Day 10: Syntax Scoring

#[allow(dead_code, clippy::redundant_else)]
pub fn part1(puzzle_input: &str) -> i64 {
    let mut scores: Vec<i64> = vec![];

    for chunk in puzzle_input.lines() {
        let mut stack: Vec<char> = vec![];
        'inner: for c in chunk.chars() {
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                ')' => {
                    if let Some(opening) = stack.pop() {
                        if opening == '(' {
                            continue;
                        } else {
                            scores.push(3);
                            break 'inner;
                        }
                    } else {
                        break 'inner;
                    }
                }
                '}' => {
                    if let Some(opening) = stack.pop() {
                        if opening == '{' {
                            continue;
                        } else {
                            scores.push(1197);
                            break 'inner;
                        }
                    } else {
                        break 'inner;
                    }
                }
                ']' => {
                    if let Some(opening) = stack.pop() {
                        if opening == '[' {
                            continue;
                        } else {
                            scores.push(57);
                            break 'inner;
                        }
                    } else {
                        break 'inner;
                    }
                }
                '>' => {
                    if let Some(opening) = stack.pop() {
                        if opening == '<' {
                            continue;
                        } else {
                            scores.push(25137);
                            break 'inner;
                        }
                    } else {
                        break 'inner;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let puzzle_answer: i64 = scores.iter().sum();
    println!("Puzzle answer: {puzzle_answer}");
    puzzle_answer
}

#[allow(dead_code, clippy::redundant_else)]
pub fn part2(puzzle_input: &str) -> i64 {
    let mut scores: Vec<i64> = vec![];

    for chunk in puzzle_input.lines() {
        let mut stack: Vec<char> = vec![];
        'inner: for c in chunk.chars() {
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                ')' => {
                    if let Some(opening) = stack.pop() {
                        if opening == '(' {
                            continue;
                        } else {
                            stack.clear();
                            break 'inner;
                        }
                    } else {
                        stack.clear();
                        break 'inner;
                    }
                }
                '}' => {
                    if let Some(opening) = stack.pop() {
                        if opening == '{' {
                            continue;
                        } else {
                            stack.clear();
                            break 'inner;
                        }
                    } else {
                        stack.clear();
                        break 'inner;
                    }
                }
                ']' => {
                    if let Some(opening) = stack.pop() {
                        if opening == '[' {
                            continue;
                        } else {
                            stack.clear();
                            break 'inner;
                        }
                    } else {
                        break 'inner;
                    }
                }
                '>' => {
                    if let Some(opening) = stack.pop() {
                        if opening == '<' {
                            continue;
                        } else {
                            stack.clear();
                            break 'inner;
                        }
                    } else {
                        stack.clear();
                        break 'inner;
                    }
                }
                _ => unreachable!(),
            }
        }
        //line is incomplete
        if !stack.is_empty() {
            let mut this_score: i64 = 0;
            while let Some(c) = stack.pop() {
                match c {
                    '(' => {
                        this_score *= 5;
                        this_score += 1;
                    }
                    '[' => {
                        this_score *= 5;
                        this_score += 2;
                    }
                    '{' => {
                        this_score *= 5;
                        this_score += 3;
                    }
                    '<' => {
                        this_score *= 5;
                        this_score += 4;
                    }
                    _ => unreachable!(),
                }
            }
            scores.push(this_score);
        }
    }

    scores.sort_unstable();

    let puzzle_answer: i64 = scores.get(scores.len() / 2).unwrap().to_owned();
    println!("Puzzle answer: {puzzle_answer}");
    puzzle_answer
}

#[cfg(test)]
mod day10_tests {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]\n";

        assert_eq!(26397, part1(input))
    }

    #[test]
    fn test_case_part_2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]\n";

        assert_eq!(288957, part2(input))
    }
}
