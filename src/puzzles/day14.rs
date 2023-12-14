use anyhow::{Error, Result};
use colored::*;
use core::panic;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use toml::map;

use crate::utils::{self, transpose};

const N_CYCLES: usize = 1_000_000_000;

fn parse_inputs(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn print_input(input: Vec<Vec<char>>) {
    input.iter().for_each(|r| {
        r.iter().for_each(|c| {
            print!(
                "{}",
                match c {
                    'O' => "O".green(),
                    '#' => "#".red(),
                    '.' => ".".into(),
                    _ => panic!(),
                }
            )
        });
        println!();
    });
    println!();
}

fn tilt_up_load(input: Vec<Vec<char>>) -> usize {
    let cols = utils::transpose(input);
    let mut new_cols: Vec<Vec<char>> = vec![]; // for vis only

    let mut cumul_load = 0;

    for col in cols.iter() {
        let mut positions: Vec<usize> = Vec::with_capacity(cols[0].len());
        let mut new_col = Vec::<char>::with_capacity(cols[0].len());

        // smallest row where a rock can roll to
        let mut last_free_row = 0;

        for (i, c) in col.iter().enumerate() {
            match c {
                'O' => {
                    positions.push(last_free_row);
                    new_col.push(*c);
                    last_free_row += 1;
                }
                '#' => {
                    positions.push(i);
                    for _ in new_col.len()..i {
                        new_col.push('.');
                    }
                    new_col.push('#');
                    last_free_row = i + 1;
                }
                '.' => {}
                _ => panic!("Unrecognized char {c}"),
            }
        }
        for _ in new_col.len()..col.len() {
            new_col.push('.');
        }
        new_cols.push(new_col);
        cumul_load += positions.iter().map(|x| col.len() - x).sum::<usize>();
    }

    print_input(transpose(new_cols.clone()));

    // cumul_load
    new_cols
        .iter()
        .flat_map(|col| {
            col.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'O')
                .map(|(i, _)| col.len() - i)
        })
        .sum()
}

enum Direction {
    North,
    West,
    South,
    East,
}

fn tilt(input: Vec<Vec<char>>, direction: Direction) -> Vec<Vec<char>> {
    todo!()
}

pub fn part1(input: &str) -> usize {
    let input = parse_inputs(input);
    print_input(input.clone());
    tilt_up_load(input)
}

pub fn part2(input: &str) -> usize {
    let input = parse_inputs(input);
    todo!()
}

#[cfg(test)]
mod test_day14 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: usize = 136;
    const EXAMPLE_OUTPUT_PART_2: usize = 64;

    #[test]
    fn day14_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res, "136");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day14_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(14, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "109424");
        k9::assert_equal!(res, 109424);
    }

    #[test]
    fn day14_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day14_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(14, 1);
        let res = part2(&input2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }
}
