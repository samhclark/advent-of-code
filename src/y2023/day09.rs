// Day 9: Mirage Maintenance

use itertools::Itertools;
use rayon::prelude::*;

static INPUT: &str = include_str!("../../inputs/2023/day09.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = sum_of_predictions(INPUT);
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = sum_of_extrapolations(INPUT);
    println!("Puzzle answer: {answer}");
}

fn sum_of_predictions(report: &str) -> i64 {
    report
        .par_lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().expect("all input values will parse"))
                .collect::<Vec<i64>>()
        })
        .map(|values| predict_next_value(&values))
        .sum()
}

fn sum_of_extrapolations(report: &str) -> i64 {
    report
        .par_lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().expect("all input values will parse"))
                .collect::<Vec<i64>>()
        })
        .map(|values| extrapolate_previous_value(&values))
        .sum()
}

fn predict_next_value(values: &[i64]) -> i64 {
    // i feel like there's a recursive solution, but I can't really see the shape of it after working it out on paper
    // gonna try doing it the way the example shows and seeing if that makes sense
    let max_layers = values.len() - 1;
    let mut work: Vec<Vec<i64>> = Vec::with_capacity(max_layers);

    // insert first layer
    work.push(values.to_vec());

    // build the rest of the layers
    loop {
        let next_layer = next_layer(work.last().unwrap());
        work.push(next_layer.clone());
        if layer_is_all_zeros(&next_layer) {
            break;
        }
    }

    // working from the bottom up, add the predicted value to each layer
    let num_layers = work.len();
    let mut last_val_in_prev_layer = 0;
    for i in 1..=num_layers {
        let layer = work.get_mut(num_layers - i).unwrap();
        let last_val_in_this_layer = *layer.last().unwrap();
        let prediction_for_layer = last_val_in_this_layer + last_val_in_prev_layer;
        layer.push(prediction_for_layer);
        last_val_in_prev_layer = prediction_for_layer;
    }

    *work.first().unwrap().last().unwrap()
}

fn extrapolate_previous_value(values: &[i64]) -> i64 {
    let max_layers = values.len() - 1;
    let mut work: Vec<Vec<i64>> = Vec::with_capacity(max_layers);

    // insert first layer
    work.push(values.to_vec());

    // build the rest of the layers
    loop {
        let next_layer = next_layer(work.last().unwrap());
        work.push(next_layer.clone());
        if layer_is_all_zeros(&next_layer) {
            break;
        }
    }

    // working from the bottom up, add the extrapolated value to each layer
    let num_layers = work.len();
    let mut first_val_in_prev_layer = 0;
    for i in 1..=num_layers {
        let layer = work.get_mut(num_layers - i).unwrap();
        let first_val_in_this_layer = *layer.first().unwrap();
        let extrapolation_for_layer = first_val_in_this_layer - first_val_in_prev_layer;
        layer.insert(0, extrapolation_for_layer);
        first_val_in_prev_layer = extrapolation_for_layer;
    }

    *work.first().unwrap().first().unwrap()
}

fn next_layer(values: &[i64]) -> Vec<i64> {
    values.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn layer_is_all_zeros(values: &[i64]) -> bool {
    values.iter().all(|it| *it == 0)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(sum_of_predictions(input), 114);
    }

    #[test]
    fn test_case_part_2_small() {
        let input = "10 13 16 21 30 45";
        assert_eq!(sum_of_extrapolations(input), 5);
    }

    #[test]
    fn test_case_part_2_full() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(sum_of_extrapolations(input), 2);
    }
}
