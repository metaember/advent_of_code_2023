use anyhow::{Error, Result};
use colored::*;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use toml::map;

pub fn part1(input: &str) -> i32 {
    todo!()
}

pub fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod test_day12 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1: &str = "\
";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: i32 = 4361;
    const EXAMPLE_OUTPUT_PART_2: i32 = 467835;

    #[test]
    fn day12_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day12_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(12, 1);
        let res = part1(&input1);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day12_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day12_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(12, 1);
        let res = part2(&input2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }
}
