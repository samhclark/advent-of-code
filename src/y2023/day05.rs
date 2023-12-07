// Day 5: If You Give A Seed A Fertilizer

use crate::aoc::range::Range;
use std::collections::HashSet;

use itertools::Itertools;

static START_SEEDS: &str = include_str!("../../inputs/2023/day05/1-starting-seeds.in");
static SEED_TO_SOIL: &str = include_str!("../../inputs/2023/day05/2-seed-to-soil.in");
static SOIL_TO_FERT: &str = include_str!("../../inputs/2023/day05/3-soil-to-fert.in");
static FERT_TO_WATER: &str = include_str!("../../inputs/2023/day05/4-fert-to-water.in");
static WATER_TO_LIGHT: &str = include_str!("../../inputs/2023/day05/5-water-to-light.in");
static LIGHT_TO_TEMP: &str = include_str!("../../inputs/2023/day05/6-light-to-temp.in");
static TEMP_TO_HUMID: &str = include_str!("../../inputs/2023/day05/7-temp-to-humid.in");
static HUMID_TO_LOC: &str = include_str!("../../inputs/2023/day05/8-humid-to-loc.in");

#[allow(dead_code)]
pub fn part01() {
    let answer = least_location_given_seeds(
        START_SEEDS,
        SEED_TO_SOIL,
        SOIL_TO_FERT,
        FERT_TO_WATER,
        WATER_TO_LIGHT,
        LIGHT_TO_TEMP,
        TEMP_TO_HUMID,
        HUMID_TO_LOC,
    );
    println!("Puzzle answer: {answer}");
}

#[allow(dead_code)]
pub fn part02() {
    let answer = least_location_range_of_seeds(
        START_SEEDS,
        SEED_TO_SOIL,
        SOIL_TO_FERT,
        FERT_TO_WATER,
        WATER_TO_LIGHT,
        LIGHT_TO_TEMP,
        TEMP_TO_HUMID,
        HUMID_TO_LOC,
    );
    println!("Puzzle answer: {answer}");
}

#[allow(clippy::too_many_arguments)]
fn least_location_given_seeds(
    starting_seeds: &str,
    seeds_to_soil: &str,
    soil_to_fert: &str,
    fert_to_water: &str,
    water_to_light: &str,
    light_to_temp: &str,
    temp_to_humid: &str,
    humid_to_loc: &str,
) -> u64 {
    let seeds: Vec<u64> = starting_seeds
        .split_ascii_whitespace()
        .map(|id| id.parse::<u64>().unwrap())
        .collect();

    let soils = map_ids_by(&seeds, seeds_to_soil);
    let fertilizers = map_ids_by(&soils, soil_to_fert);
    let water = map_ids_by(&fertilizers, fert_to_water);
    let light = map_ids_by(&water, water_to_light);
    let temps = map_ids_by(&light, light_to_temp);
    let humidities = map_ids_by(&temps, temp_to_humid);
    let locations = map_ids_by(&humidities, humid_to_loc);

    *locations.iter().min().unwrap()
}

#[allow(clippy::too_many_arguments)]
fn least_location_range_of_seeds(
    starting_seeds: &str,
    seeds_to_soil: &str,
    soil_to_fert: &str,
    fert_to_water: &str,
    water_to_light: &str,
    light_to_temp: &str,
    temp_to_humid: &str,
    humid_to_loc: &str,
) -> u64 {
    let seeds: Vec<Range> = starting_seeds
        .split_ascii_whitespace()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap().parse::<u64>().unwrap();
            let size = chunk.next().unwrap().parse::<u64>().unwrap();
            Range::new(start, start + size).unwrap()
        })
        .collect();
    let soils = map_ranges_by(&seeds, seeds_to_soil);
    let fertilizers = map_ranges_by(&soils, soil_to_fert);
    let water = map_ranges_by(&fertilizers, fert_to_water);
    let light = map_ranges_by(&water, water_to_light);
    let temps = map_ranges_by(&light, light_to_temp);
    let humidities = map_ranges_by(&temps, temp_to_humid);
    let locations = map_ranges_by(&humidities, humid_to_loc);

    locations.iter().map(|r| r.start).min().unwrap()
}

fn map_ids_by(ids: &[u64], map: &str) -> Vec<u64> {
    let mut remaining_ids: HashSet<u64> = HashSet::new();
    remaining_ids.reserve(ids.len());
    for id in ids {
        remaining_ids.insert(*id);
    }

    let mut output_ids: Vec<u64> = Vec::with_capacity(ids.len());
    for line in map.lines() {
        let (dest, src, size) = line
            .splitn(3, ' ')
            .map(|value| value.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        let mut to_remove: Vec<u64> = vec![];
        for input_id in &remaining_ids {
            let range = src..(src + size);
            if range.contains(input_id) {
                output_ids.push(input_id - src + dest);
                to_remove.push(*input_id);
            }
        }
        for id in &to_remove {
            remaining_ids.remove(id);
        }
    }
    for id in remaining_ids {
        output_ids.push(id);
    }
    output_ids
}

fn map_ranges_by(inputs: &[Range], map: &str) -> Vec<Range> {
    inputs
        .iter()
        .flat_map(|r| map_range_by(*r, map))
        .collect_vec()
}

fn map_range_by(input: Range, map: &str) -> Vec<Range> {
    let mut output: Vec<Range> = vec![];

    let mut remaining_inputs: Vec<Range> = vec![input];
    for line in map.lines() {
        let (dest, src, size) = line
            .splitn(3, ' ')
            .map(|value| value.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        let src_range = Range::new(src, src + size).unwrap();

        if let Some(overlap) = input.intersection(src_range) {
            output.push(Range::new(overlap.start - src + dest, overlap.end - src + dest).unwrap());
            remaining_inputs = remaining_inputs
                .iter()
                .flat_map(|range| range.difference(overlap))
                .collect();
        }
    }

    output.extend(remaining_inputs);
    output
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_2() {
        let seeds = "79 14 55 13";
        let a = "50 98 2
52 50 48";

        let b = "0 15 37
37 52 2
39 0 15";

        let c = "49 53 8
0 11 42
42 0 7
57 7 4";

        let d = "88 18 7
18 25 70";

        let e = "45 77 23
81 45 19
68 64 13";

        let f = "0 69 1
1 0 69";

        let g = "60 56 37
56 93 4";

        assert_eq!(
            least_location_range_of_seeds(seeds, a, b, c, d, e, f, g),
            46
        );
    }
}
