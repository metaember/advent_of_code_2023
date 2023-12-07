use anyhow::{Error, Result};
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;

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
            .progress()
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

/// Takes less than a minute with parallelization and release compilation mode
pub fn part2(input: &str) -> i64 {
    let puzzle_input = input.parse::<PuzzleInput>().unwrap();
    puzzle_input.get_closest_pairwise_location()
}

/// basically instant
pub fn part2_take2(input: &str) -> usize {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds = seeds
        .split_ascii_whitespace()
        .filter_map(|id| id.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let maps = maps
        .split("\n\n")
        .map(|m| {
            m.lines()
                .skip(1)
                .map(|l| {
                    l.split_ascii_whitespace()
                        .filter_map(|num| num.parse::<usize>().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // translate each seed interval into a range
    // This array will hold the range of seeds at the start, and we'll
    // repeatedly translate it until we get to the end of the maps
    let mut arr = seeds
        .chunks_exact(2)
        .map(|ele| ele[0]..(ele[0] + ele[1]))
        .collect::<Vec<_>>();

    // iterate over each layer of the mappings
    // seed -> soil then soil -> fertilizer -> ...-> water -> light -> temperature -> humidity -> location
    for mut map in maps {
        // sort by source
        map.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        // 1. First, we check if the current range is solved. If it is, we move to the next range.
        // 2. We iterate over the list of mappings.
        //     For each mapping, we check if the current range is fully contained in the mapping.
        //     - If it is, we translate the current range and mark it as solved.
        //     If it is not, we check if the current range is partially contained in the mapping.
        //     - If it is, we split the current range into two ranges.
        //     - We translate the first range and mark it as solved.
        //     - We add the second range to the list of ranges to be solved.
        // 3. Once we have translated all the ranges in the list of ranges, we return the minimum value of the start of the ranges.

        // While our array of inputs is not fully translated
        let mut idx = 0;
        while arr.get(idx).is_some() {
            // This is the current range we want to translate.
            let input_range = arr[idx].clone();

            // iterate over each range in the current layer of mappings
            // until we find a mapping that contains the current range
            // even if partially. Then we plit the current range into two ranges
            // if needed, apply that mapping to the overlapping part
            // and add the remaining part if any to the array of ranges to be translated
            // we break because we have fully translated the current range
            for m in map.iter() {
                let destination = m[0];
                let source = m[1];
                let length = m[2];
                let range = source..(source + length);

                let input_start = input_range.start;
                let input_end = input_range.end - 1;

                let start_distance = input_start.saturating_sub(source);
                let end_distance = input_end.saturating_sub(source);

                if range.contains(&input_start) && range.contains(&input_end) {
                    // range fully contains current range
                    // we replace it in the array with the translated range
                    arr[idx] = (destination + start_distance)..(destination + end_distance);
                } else if range.contains(&input_start) && !range.contains(&input_end) {
                    // range contains start of current range, but does not contain the end
                    // we replace the overlapping part
                    arr[idx] = (destination + start_distance)..(destination + length);

                    // create a new range with the rest
                    let next_range = (source + length)..input_end + 1;

                    // println!(
                    //     "Start Included, End Excluded -> Split Range: {:?} -> {:?}",
                    //     current_start..source + length,
                    //     next_range
                    // );
                    // println!("Output Range -> {:?}", arr[idx]);

                    // push the new range to the array
                    arr.insert(idx + 1, next_range);
                } else if !range.contains(&input_start) && range.contains(&input_end) {
                    // translate the current overlap
                    arr[idx] = (destination)..(destination + end_distance);
                    // create a new range with the rest
                    let next_range = (input_start)..(source);

                    // println!(
                    //     "Start Excluded, End Included -> Split Range: {:?} -> {:?}",
                    //     source..source + end_distance,
                    //     next_range
                    // );
                    // println!("Output Range -> {:?}", arr[idx]);

                    arr.insert(idx + 1, next_range);
                }
                // There is no default else case here, becase in that case the translation is 1:1 so
                // we don't need to do anything to the range, just keep it and move on
            }
            idx += 1;
        }
    }

    arr.iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod test_day_5 {
    use super::{part1, part2_take2};
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
        let res = part2_take2(EXAMPLE_INPUT_PART_2) as i64;
        k9::snapshot!(res, "46");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day5_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(5, 1);
        let res = part2_take2(&input2) as i64;
        k9::snapshot!(res, "1493866");
        k9::assert_equal!(res, 1493866);
    }
}
