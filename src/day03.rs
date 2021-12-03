// Day 3: Binary Diagnostic

use std::error::Error;

//0000 0100 0101

#[allow(dead_code)]
pub fn part1() -> Result<(), Box<dyn Error>> {
    let mut sums: [i64; 12] = [0; 12];

    include_str!("../inputs/day03.in").lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                sums[i] += 1
            }
        })
    });

    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    for num_occurances in sums {
        if num_occurances > 500 {
            gamma = format!("{}1", gamma);
            epsilon = format!("{}0", epsilon);
        } else {
            gamma = format!("{}0", gamma);
            epsilon = format!("{}1", epsilon);
        }
    }

    let gamma_number = u64::from_str_radix(&gamma, 2)?;
    let epsilon_number = u64::from_str_radix(&epsilon, 2)?;

    println!("Gamma number: {}", gamma);
    println!("Epsilon number: {}", epsilon);

    let result = gamma_number * epsilon_number;
    println!("Result: {}", result);
    Ok(())
}

#[allow(dead_code)]
pub fn part2() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inputs/day03.in");

    let oxygen_generator_rating: u64 = calculate_oxygen_generator_rating(&input)?;
    let co2_scrubber_rating: u64 = calculate_co2_scrubber_rating(&input)?;

    let puzzle_result = oxygen_generator_rating * co2_scrubber_rating;
    println!("Puzzle result: {}", puzzle_result);
    Ok(())
}

fn calculate_oxygen_generator_rating(input: &str) -> Result<u64, Box<dyn Error>> {
    println!("{:?}", input);
    let input_vec: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let mut result: u64 = 1;
    let mut filtered = input_vec.clone();
    for i in 0..12 {
        println!("Bit {}; Current list: {:?}", i, filtered);
        filtered = check_and_filter_oxygen_readings(&filtered, i)?;
        if filtered.len() == 1 {
            let result_string = filtered.get(0).unwrap();
            result = u64::from_str_radix(result_string, 2)?;
            break;
        }
    }

    Ok(result)
}

fn check_and_filter_oxygen_readings(
    input_vec: &[String],
    bit_to_check: usize,
) -> Result<Vec<String>, Box<dyn Error>> {
    let total_readings = input_vec.len();
    let filter_threshold = total_readings - (total_readings / 2);
    println!("Filter threshold: {}", filter_threshold);

    let mut number_of_ones = 0;
    input_vec.iter().for_each(|line| {
        if line.chars().nth(bit_to_check).unwrap() == '1' {
            number_of_ones += 1;
        }
    });

    let most_common = if number_of_ones < filter_threshold {
        '0'
    } else {
        '1'
    };
    println!("Bit {}; Most common: {}", bit_to_check, most_common);

    let output: Vec<String> = input_vec
        .iter()
        .filter(|line| line.chars().nth(bit_to_check).unwrap() == most_common)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Ok(output)
}

fn calculate_co2_scrubber_rating(input: &str) -> Result<u64, Box<dyn Error>> {
    println!("{:?}", input);
    let input_vec: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let mut result: u64 = 1;
    let mut filtered = input_vec.clone();
    for i in 0..12 {
        println!("Bit {}; Current list: {:?}", i, filtered);
        filtered = check_and_filter_c02_readings(&filtered, i)?;
        if filtered.len() == 1 {
            let result_string = filtered.get(0).unwrap();
            result = u64::from_str_radix(result_string, 2)?;
            break;
        }
    }

    Ok(result)
}

fn check_and_filter_c02_readings(
    input_vec: &[String],
    bit_to_check: usize,
) -> Result<Vec<String>, Box<dyn Error>> {
    let total_readings = input_vec.len();
    let filter_threshold = total_readings - (total_readings / 2);
    println!("Filter threshold: {}", filter_threshold);

    let mut number_of_ones = 0;
    let mut number_of_zeros = 0;
    input_vec.iter().for_each(|line| {
        if line.chars().nth(bit_to_check).unwrap() == '0' {
            number_of_zeros += 1;
        } else {
            number_of_ones += 1;
        }
    });

    let least_common = if number_of_zeros <= number_of_ones {
        '0'
    } else {
        '1'
    };
    println!("Bit {}; Least common: {}", bit_to_check, least_common);

    let output: Vec<String> = input_vec
        .iter()
        .filter(|line| line.chars().nth(bit_to_check).unwrap() == least_common)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Ok(output)
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn test_case_calculate_oxygen_generator_rating() {
        let input: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(23, calculate_oxygen_generator_rating(input).unwrap());
    }

    #[test]
    fn test_case_calculate_co2_scrubber_rating() {
        let input: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(10, calculate_co2_scrubber_rating(input).unwrap());
    }
}
