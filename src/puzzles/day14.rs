use colored::*;
use memoize::memoize;
use std::collections::HashMap;

use crate::utils::{flip, transpose};

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

fn tilt(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let cols = transpose(input);
    let mut new_cols: Vec<Vec<char>> = vec![]; // for vis only

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
    }
    transpose(new_cols)
}

#[memoize]
fn cycle(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // println!("INPUT");
    // print_input(input.clone());

    // println!("NORTH");
    let res = tilt(input);
    // print_input(res.clone());

    // println!("WEST");
    let res = transpose(tilt(transpose(res.clone())));
    // print_input(res.clone());

    // println!("SOUTH");
    let res = reverse(tilt(reverse(res.clone())));
    // print_input(res.clone());

    // println!("EAST");
    let res = flip(transpose(tilt(transpose(flip(res.clone())))));
    // print_input(res.clone());
    res
}

fn reverse(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    v.into_iter().rev().collect()
}

fn get_north_load(input: Vec<Vec<char>>) -> usize {
    transpose(input)
        .iter()
        .flat_map(|col| {
            col.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'O')
                .map(|(i, _)| col.len() - i)
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    let input = parse_inputs(input);
    print_input(input.clone());
    get_north_load(tilt(input))
}

pub fn part2(input: &str) -> usize {
    let mut input = parse_inputs(input);

    let mut visited = HashMap::<Vec<Vec<char>>, usize>::new();
    visited.insert(input.clone(), 0);
    let mut i = 0;

    while i < N_CYCLES {
        input = cycle(input);
        i += 1;
        if let Some(last_index) = visited.get(&input) {
            let cycle_length = i - last_index;
            // println!("found cycle at {last_index} of length: {}", cycle_length);
            i += ((N_CYCLES - i) / cycle_length) * cycle_length;
        }
        visited.insert(input.clone(), i);
    }
    get_north_load(input)
}

#[cfg(test)]
mod test_day14 {
    use super::{cycle, parse_inputs, part1, part2, print_input};
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
        k9::snapshot!(res, "64");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day14_p2_1_cycle() {
        let res = cycle(parse_inputs(EXAMPLE_INPUT_PART_2));
        let expected = "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        print_input(res.clone());
        println!("{}", expected);
        k9::assert_equal!(res, parse_inputs(expected));
    }

    #[test]
    fn day14_p2_2_cycle() {
        let res = cycle(cycle(parse_inputs(EXAMPLE_INPUT_PART_2)));
        let expected = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
        print_input(res.clone());
        println!("{}", expected);
        k9::assert_equal!(res, parse_inputs(expected));
    }

    #[test]
    fn day14_p2_3_cycle() {
        let res = cycle(cycle(cycle(parse_inputs(EXAMPLE_INPUT_PART_2))));
        print_input(res.clone());
        let expected = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        println!("{}", expected);
        k9::assert_equal!(res, parse_inputs(expected));
    }

    #[test]
    fn day14_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(14, 1);
        let res = part2(&input2);
        k9::snapshot!(res, "102509");
        k9::assert_equal!(res, 102509);
    }
}
