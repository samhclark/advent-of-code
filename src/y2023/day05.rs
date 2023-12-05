// Day 5: If You Give A Seed A Fertilizer

use std::{collections::HashSet, borrow::BorrowMut};

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
    let seeds: Vec<u64> = starting_seeds
        .split_ascii_whitespace()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap().parse::<u64>().unwrap();
            let size = chunk.next().unwrap().parse::<u64>().unwrap();
            (start..(start+size)).collect::<Vec<u64>>()
        })
        .flatten()
        .collect();
    // println!("Ranges are: {:?}", seeds);

    // I think this is gonna balloon my memory...

    let soils = map_ids_by(&seeds, seeds_to_soil);
    // println!("soils: {:?}", soils);

    let fertilizers = map_ids_by(&soils, soil_to_fert);
    // println!("fertilizers: {:?}", fertilizers);

    let water = map_ids_by(&fertilizers, fert_to_water);
    // println!("water: {:?}", water);

    let light = map_ids_by(&water, water_to_light);
    // println!("light: {:?}", light);

    let temps = map_ids_by(&light, light_to_temp);
    // println!("temps: {:?}", temps);

    let humidities = map_ids_by(&temps, temp_to_humid);
    // println!("humidities: {:?}", humidities);

    let locations = map_ids_by(&humidities, humid_to_loc);
    // println!("locations: {:?}", locations);

    *locations.iter().min().unwrap()
}

fn map_ids_by(ids: &[u64], map: &str) -> Vec<u64> {
    let mut remaining_ids: HashSet<u64> = HashSet::new();
    remaining_ids.reserve(ids.len());
    for id in ids {
        remaining_ids.insert(*id);
    }

    let mut output_ids: Vec<u64> = vec![];
    output_ids.reserve(ids.len());
    for line in map.lines() {
        // println!("parsing {line}...");
        let (dest, src, size) = line
            .splitn(3, ' ')
            .map(|value| value.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        // println!("dest: {dest}, src: {src}, size: {size}");
        for input_id in remaining_ids.iter() {
            let range = src..(src + size);
            // println!("looking if range {:?} contains {input_id}", range);
            if range.contains(&input_id) {
                output_ids.push(input_id - src + dest);
                remaining_ids.remove(input_id);
            }
        }
    }
    for id in remaining_ids {
        output_ids.push(id);
    }
    output_ids
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_case_part_1() {
        let input = "";
        assert_eq!(13, 13);
    }

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

        assert_eq!(least_location_range_of_seeds(
            seeds,
            a,b,c,d,e,f,g

        ), 46);
    }
}
