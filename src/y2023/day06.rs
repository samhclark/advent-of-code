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

    find_longest_charge(race_duration, distance) - find_shortest_charge(race_duration, distance) + 1
}

const fn find_shortest_charge(total_time: u64, distance_to_beat: u64) -> u64 {
    // always in the bottom half of possible times
    let center = total_time / 2;

    let mut greatest_loser: u64 = 1;
    let mut smallest_winner = center;
    while greatest_loser != smallest_winner - 1 {
        // binary search for the smallest value where distance_traveled() > distance_to_beat
        let charge_time = (greatest_loser + smallest_winner) / 2;
        if distance_traveled(charge_time, total_time) > distance_to_beat {
            smallest_winner = charge_time;
        } else {
            greatest_loser = charge_time;
        }
    }

    smallest_winner
}

const fn find_longest_charge(total_time: u64, distance_to_beat: u64) -> u64 {
    // always in the top half of possible times
    let center = total_time / 2;

    let mut greatest_winner: u64 = center;
    let mut smallest_loser = total_time;
    while greatest_winner != smallest_loser - 1 {
        // binary search for the greatest value where distance_traveled() > distance_to_beat
        let charge_time = (greatest_winner + smallest_loser) / 2;
        if distance_traveled(charge_time, total_time) > distance_to_beat {
            greatest_winner = charge_time;
        } else {
            smallest_loser = charge_time;
        }
    }

    greatest_winner
}

const fn distance_traveled(charge_time: u64, total_time: u64) -> u64 {
    (total_time - charge_time) * charge_time
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
