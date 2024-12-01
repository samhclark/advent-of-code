// Day 1: Historian Hysteria

// use itertools::Itertools;
use std::iter::zip;

static INPUT: &str = include_str!("../../inputs/2024/day01.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = total_distance(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = similarity_score(INPUT);
    println!("Puzzle answer: {answer}");
}

fn total_distance(input: &str) -> i64 {
    let mut left_ids: Vec<i32> = vec!();
    let mut right_ids: Vec<i32> = vec!();

    for line in input.lines() {
        let (l, r) = line.split_once("   ").expect("Each line contains 3 spaces");
        left_ids.push(l.parse::<i32>().expect("Each location id is a digit"));
        right_ids.push(r.parse::<i32>().expect("Each location id is a digit"));
    }

    left_ids.sort_unstable();
    right_ids.sort_unstable();

    let mut distance: i64 = 0;
    for (l,r) in zip(left_ids, right_ids) {
        distance += (l - r).abs() as i64
    }

    distance
}

fn similarity_score(input: &str) -> i64 {
    let mut count_map = [0u16; 100_000];
    
    let mut left_ids: Vec<u32> = vec!();

    for line in input.lines() {
        let (l, r) = line.split_once("   ").expect("Each line contains 3 spaces");
        left_ids.push(l.parse::<u32>().expect("Each location id is a digit"));
        
        let right = r.parse::<usize>().expect("Each location id is a digit");
        assert!(right < 100_000);
        count_map[right] += 1; 
    }

    let mut score: i64 = 0;
    for location_id in left_ids {
        score += i64::from(location_id * u32::from(count_map[usize::try_from(location_id).unwrap()]))
    }

    score
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(total_distance(input), 11);
    }

    #[test]
    fn test_case_part_2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(similarity_score(input), 31);
    }
}
