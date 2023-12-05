use anyhow::{Error, Result};
use colored::*;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use toml::map;

struct PuzzleInput {
    seeds: Vec<i64>,
    seed_to_soil_map: SmartMap,
    soil_to_fertilizer_map: SmartMap,
    fertilizer_to_water_map: SmartMap,
    water_to_light_map: SmartMap,
    light_to_temperature_map: SmartMap,
    temperature_to_humidity_map: SmartMap,
    humidity_to_location_map: SmartMap,
}

impl std::str::FromStr for PuzzleInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<PuzzleInput> {
        let mut lines = s.trim().lines();
        let seeds = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let _ = lines.next();

        // println!("Getting seed to soil map");
        let seed_to_soil_map = get_map_for_current_lines(&mut lines);
        // println!("Getting soil to fertilizer map");
        let soil_to_fertilizer_map = get_map_for_current_lines(&mut lines);
        // println!("Getting fertilizer to water map");
        let fertilizer_to_water_map = get_map_for_current_lines(&mut lines);
        // println!("Getting water to light map");
        let water_to_light_map = get_map_for_current_lines(&mut lines);
        // println!("Getting light to temperature map");
        let light_to_temperature_map = get_map_for_current_lines(&mut lines);
        // println!("Getting temperature to humidity map");
        let temperature_to_humidity_map = get_map_for_current_lines(&mut lines);
        // println!("Getting humidity to location map");
        let humidity_to_location_map = get_map_for_current_lines(&mut lines);
        // println!("Done parsing input");

        // println!("seed_to_soil_map: {:?}", seed_to_soil_map);
        // assert_eq!(50, seed_to_soil_map.get(98));
        // assert_eq!(51, seed_to_soil_map.get(99));
        // assert_eq!(55, seed_to_soil_map.get(53));

        Ok(PuzzleInput {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        })
    }
}

impl PuzzleInput {
    pub fn get_location(&self, seed: i64) -> i64 {
        // println!("seed: {}", seed);
        let soil = self.seed_to_soil_map.get(seed);
        // println!("soil: {}", soil);
        let fertilizer = self.soil_to_fertilizer_map.get(soil);
        let water = self.fertilizer_to_water_map.get(fertilizer);
        let light = self.water_to_light_map.get(water);
        let temperature = self.light_to_temperature_map.get(light);
        let humidity = self.temperature_to_humidity_map.get(temperature);
        let location = self.humidity_to_location_map.get(humidity);

        // println!(
        //     "seed: {}, soil: {}, fertilizer: {}, water: {}, light: {}, temperature: {}, humidity: {}, location: {}",
        //     seed, soil, fertilizer, water, light, temperature, humidity, location
        // );
        location
    }

    pub fn get_closest_location(&self) -> i64 {
        self.seeds
            .iter()
            .map(|s| self.get_location(*s))
            .min()
            .unwrap()
    }

    pub fn get_closest_pairwise_location(&self) -> i64 {
        // iterate over seeds 2 by 2
        println!("{}", self.seeds.len() / 2);
        (0..(self.seeds.len() / 2))
            .into_par_iter()
            .map(|i| {
                let mut locations: Vec<i64> = vec![];
                let start = self.seeds[2 * i];
                let end = start + self.seeds[2 * i + 1];
                for j in start..end {
                    let loc = self.get_location(j);
                    locations.push(loc);
                }
                println!("{i}/{}", self.seeds.len() / 2);
                locations.iter().min().unwrap().clone()
            })
            .min()
            .unwrap()
    }
}

#[derive(Debug)]
struct SmartMap {
    src: Vec<i64>,
    dest: Vec<i64>,
    len: Vec<i64>,
}

impl SmartMap {
    pub fn default() -> Self {
        Self {
            src: vec![],
            dest: vec![],
            len: vec![],
        }
    }

    pub fn new(src: i64, dest: i64, len: i64) -> Self {
        Self {
            src: vec![src],
            dest: vec![dest],
            len: vec![len],
        }
    }

    pub fn extend(&mut self, other: SmartMap) {
        self.src.extend(other.src);
        self.dest.extend(other.dest);
        self.len.extend(other.len);
    }

    pub fn get(&self, src: i64) -> i64 {
        for i in 0..self.src.len() {
            if self.src[i] <= src && src < self.src[i] + self.len[i] {
                return self.dest[i] + (src - self.src[i]);
            }
        }
        src
    }
}
fn get_map_for_current_lines(lines: &mut std::str::Lines) -> SmartMap {
    let lines = lines.skip(1);
    let mut map = SmartMap::default();
    lines
        .take_while(|line| !line.is_empty())
        .map(|line| line_to_map(line))
        .for_each(|m| map.extend(m));
    map
}

fn line_to_map(line: &str) -> SmartMap {
    // println!("{}", line);
    let (dest, src, len) = line
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();
    SmartMap::new(src, dest, len)
}

pub fn part1(input: &str) -> i64 {
    let puzzle_input = input.parse::<PuzzleInput>().unwrap();
    puzzle_input.get_closest_location()
}

pub fn part2(input: &str) -> i64 {
    let puzzle_input = input.parse::<PuzzleInput>().unwrap();
    puzzle_input.get_closest_pairwise_location()
}

#[cfg(test)]
mod test_day_5 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    /// Here 114 and 58 are not adjacent to anything
    const EXAMPLE_INPUT_PART_1: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: i64 = 35;
    const EXAMPLE_OUTPUT_PART_2: i64 = 46;

    #[test]
    fn day5_p1_example() {
        k9::snapshot!(part1(EXAMPLE_INPUT_PART_1), "35");
        k9::assert_equal!(part1(EXAMPLE_INPUT_PART_1), EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day5_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(5, 1);
        println!("got input");
        let res = part1(&input1);
        k9::snapshot!(res, "174137457");
        k9::assert_equal!(res, 174137457);
    }

    #[test]
    fn day5_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res, "46");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day5_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(5, 1);
        let res = part2(&input2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }
}
