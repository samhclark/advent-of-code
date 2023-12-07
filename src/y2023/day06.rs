// Day 6: Wait For It

static INPUT: &str = include_str!("../../inputs/2023/day06.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = ways_to_win_score(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = ways_to_win_single_race(INPUT);
    println!("Puzzle answer: {answer}");
}

fn ways_to_win_score(records: &str) -> u64 {
    let mut lines = records.lines();
    let race_durations: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut score: u64 = 1;
    for (race_id, duration) in race_durations.iter().enumerate() {
        score *= (1..*duration)
            .filter(|charge_time| {
                (duration - charge_time) * charge_time > *distances.get(race_id).unwrap()
            })
            .count() as u64;
    }

    score
}

fn ways_to_win_single_race(records: &str) -> u64 {
    let mut lines = records.lines();
    let race_duration: String = lines
        .next()
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect();
    let race_duration: u64 = race_duration.parse().unwrap();

    let distance: String = lines
        .next()
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect();
    let distance: u64 = distance.parse().unwrap();

    (1..race_duration)
            .filter(|charge_time| {
                (race_duration - charge_time) * charge_time > distance
            })
            .count() as u64

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(ways_to_win_score(input), 288);
    }

    #[test]
    fn test_case_part_2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(ways_to_win_single_race(input), 71503);
    }
}
