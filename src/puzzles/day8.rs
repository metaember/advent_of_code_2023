use anyhow::{Error, Result};
use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
struct PuzzeInput {
    right: Vec<bool>,
    map: HashMap<String, (String, String)>,
}

impl std::str::FromStr for PuzzeInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<PuzzeInput> {
        let mut lines = s.lines();

        let right = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c == 'R')
            .collect::<Vec<bool>>();

        // let map = HashMap::<String, (String, String)>::new();
        let map = lines
            .skip(1)
            .into_iter()
            .map(|line| {
                let mut parts = line.split(" = (");
                let key = parts.next().unwrap();
                let (left, right) = parts
                    .next()
                    .unwrap()
                    .split(')')
                    .next()
                    .unwrap()
                    .split(", ")
                    .collect_tuple()
                    .unwrap();
                (key.to_string(), (left.to_string(), right.to_string()))
            })
            .collect::<HashMap<String, (String, String)>>();

        Ok(PuzzeInput { right, map })
    }
}

pub fn part1(input: &str) -> i32 {
    let input = input.parse::<PuzzeInput>().unwrap();

    // println!("{:?}", input);

    let mut current = "AAA".to_string();
    let mut steps = 0;

    for right in input.right.into_iter().cycle() {
        // println!("{steps}: {current}");
        current = if right {
            input.map.get(&current).unwrap().1.clone()
        } else {
            input.map.get(&current).unwrap().0.clone()
        };
        steps += 1;
        if current == "ZZZ" {
            break;
        }
    }
    steps
}

pub fn part2(input: &str) -> usize {
    let input = input.parse::<PuzzeInput>().unwrap();

    // println!("{:?}", input);

    let starts = input
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_string())
        .collect::<Vec<String>>();

    println!("{:?}", starts.len());

    let mut step_counts = Vec::<usize>::new();

    for mut current in starts {
        let mut steps = 0usize;
        for right in input.right.clone().into_iter().cycle() {
            // println!("{steps}: {}", current[i]);
            current = if right {
                input.map.get(&current).unwrap().1.clone()
            } else {
                input.map.get(&current).unwrap().0.clone()
            };
            steps += 1;
            if current.ends_with('Z') {
                break;
            }
        }
        step_counts.push(steps);
    }
    step_counts.iter().fold(1usize, |acc, &x| lcm(acc, x))
}

#[cfg(test)]
mod test_day8 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE_INPUT_PART_1_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_INPUT_PART_2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    const EXAMPLE_OUTPUT_PART_1: i32 = 2;
    const EXAMPLE_OUTPUT_PART_1_2: i32 = 6;

    const EXAMPLE_OUTPUT_PART_2: i32 = 6;

    #[test]
    fn day8_p1_example() {
        k9::snapshot!(part1(EXAMPLE_INPUT_PART_1), "2");
        k9::assert_equal!(part1(EXAMPLE_INPUT_PART_1), EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day8_p1_example_2() {
        k9::snapshot!(part1(EXAMPLE_INPUT_PART_1_2), "6");
        k9::assert_equal!(part1(EXAMPLE_INPUT_PART_1_2), EXAMPLE_OUTPUT_PART_1_2);
    }

    #[test]
    fn day8_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(8, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "14429");
        k9::assert_equal!(res, 14429);
    }

    #[test]
    fn day8_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res, "6");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2 as usize);
    }

    #[test]
    fn day8_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(8, 1);
        let res = part2(&input2);
        k9::snapshot!(res, "10921547990923");
        k9::assert_equal!(res, 10921547990923usize);
    }
}
