use anyhow::{Error, Result};
use chrono_tz::Chile::Continental;
use colored::*;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use toml::{de, map};

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

pub fn next_direction(dx: i32, dy: i32, current_char: char) -> (i32, i32) {
    // println!("getting next_direction: {} {} {}", dx, dy, current_char);
    if current_char == '|' && dx == 0 {
        return (0, dy);
    } else if current_char == '-' && dy == 0 {
        return (dx, 0);
    } else if current_char == 'L' && (dx == -1 || dy == 1) {
        if dx == -1 {
            return (0, -1);
        } else {
            return (1, 0);
        }
    } else if current_char == 'J' && (dx == 1 || dy == 1) {
        if dx == 1 {
            return (0, -1);
        } else {
            return (-1, 0);
        }
    } else if current_char == 'F' && (dx == -1 || dy == -1) {
        if dx == -1 {
            return (0, 1);
        } else {
            return (1, 0);
        }
    } else if current_char == '7' && (dx == 1 || dy == -1) {
        if dx == 1 {
            return (0, 1);
        } else {
            return (-1, 0);
        }
    } else {
        panic!("Invalid char")
    }
}

pub fn part1(input: &str) -> i32 {
    let input = parse_input(input);
    let mut start_line = 0;
    let mut start_col = 0;

    let height = input.len();
    let width = input[0].len();

    // Find start
    'outer: for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_line = i;
                start_col = j;
                break 'outer;
            }
        }
    }

    // go around the loop assuming it's a single loop
    let mut current_char = 'X'; // anything except for `S`
    let mut curr_distance = 0;

    let mut cur_x = start_col as i32;
    let mut cur_y = start_line as i32;

    let mut prev_dx = 0;
    let mut prev_dy = 0;

    // println!("Start at {cur_x}, {cur_y}");

    // Find the first step
    for &(dx, dy) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
        let candidate_x = cur_x + dx;
        let candidate_y = cur_y + dy;

        // continue if this is outside the bounds of the map
        if candidate_x < 0
            || candidate_y < 0
            || candidate_x >= width as i32
            || candidate_y >= height as i32
        {
            continue;
        }

        let next_char = input[candidate_y as usize][candidate_x as usize];

        // println!("{cur_x}, {cur_y} considering direction {dx}, {dy}: got char {next_char} at {candidate_x}, {candidate_y}");
        if next_char == '|' && dx == 0 {
        } else if next_char == '-' && dy == 0 {
        } else if next_char == 'L' && (dx == -1 || dy == 1) {
        } else if next_char == 'J' && (dx == 1 || dy == 1) {
        } else if next_char == 'F' && (dx == -1 || dy == -1) {
        } else if next_char == '7' && (dx == 1 || dy == -1) {
        } else {
            continue;
        }
        prev_dx = dx;
        prev_dy = dy;

        cur_x = candidate_x;
        cur_y = candidate_y;
        current_char = next_char;
        curr_distance += 1;
        break;
    }

    println!("Selected direction {} {}", prev_dx, prev_dy);

    while current_char != 'S' {
        let (dx, dy) = next_direction(prev_dx, prev_dy, current_char);
        cur_x = cur_x + dx;
        cur_y = cur_y + dy;
        // println!(
        //     "current_char: {current_char}, prev: {prev_dx}, {prev_dy}, dx: {dx}, dy: {dy}, cur_x: {cur_x}, cur_y: {cur_y}"
        // );
        current_char = input[cur_y as usize][cur_x as usize];
        prev_dx = dx;
        prev_dy = dy;
        // println!("next_char {current_char}");
        curr_distance += 1;
    }

    curr_distance / 2
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum State {
    Outside,
    Inside,
    Edge,
}

pub fn part2(input: &str) -> i32 {
    let input = parse_input(input);
    let mut start_line = 0;
    let mut start_col = 0;

    let height = input.len();
    let width = input[0].len();

    // Find start
    'outer: for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                start_line = i;
                start_col = j;
                break 'outer;
            }
        }
    }

    // go around the loop assuming it's a single loop
    let mut current_char = 'X'; // anything except for `S`
    let mut curr_distance = 0;

    let mut cur_x = start_col as i32;
    let mut cur_y = start_line as i32;

    let mut prev_dx = 0;
    let mut prev_dy = 0;

    // println!("Start at {cur_x}, {cur_y}");

    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert((cur_x, cur_y));

    // Find the first step
    for &(dx, dy) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
        let candidate_x = cur_x + dx;
        let candidate_y = cur_y + dy;

        // continue if this is outside the bounds of the map
        if candidate_x < 0
            || candidate_y < 0
            || candidate_x >= width as i32
            || candidate_y >= height as i32
        {
            continue;
        }

        let next_char = input[candidate_y as usize][candidate_x as usize];

        // println!("{cur_x}, {cur_y} considering direction {dx}, {dy}: got char {next_char} at {candidate_x}, {candidate_y}");
        if next_char == '|' && dx == 0 {
        } else if next_char == '-' && dy == 0 {
        } else if next_char == 'L' && (dx == -1 || dy == 1) {
        } else if next_char == 'J' && (dx == 1 || dy == 1) {
        } else if next_char == 'F' && (dx == -1 || dy == -1) {
        } else if next_char == '7' && (dx == 1 || dy == -1) {
        } else {
            continue;
        }
        prev_dx = dx;
        prev_dy = dy;

        cur_x = candidate_x;
        cur_y = candidate_y;
        current_char = next_char;
        curr_distance += 1;
        break;
    }

    visited.insert((cur_x, cur_y));
    let after_start_pos = (cur_x, cur_y);
    let mut before_start_pos = (0, 0);

    println!("Selected direction {} {}", prev_dx, prev_dy);

    while current_char != 'S' {
        let (dx, dy) = next_direction(prev_dx, prev_dy, current_char);
        before_start_pos = (cur_x, cur_y);
        cur_x = cur_x + dx;
        cur_y = cur_y + dy;
        current_char = input[cur_y as usize][cur_x as usize];
        prev_dx = dx;
        prev_dy = dy;
        curr_distance += 1;
        visited.insert((cur_x, cur_y));
    }

    let mut area = 0;

    // determine what shape the S was
    println!(
        "before / after start {},{} -> {},{}",
        before_start_pos.0, before_start_pos.1, after_start_pos.0, after_start_pos.1
    );
    let dx = after_start_pos.0 - before_start_pos.0;
    let dy = after_start_pos.1 - before_start_pos.1;

    println!("dx, dy: {}, {}", dx, dy);
    let start_char = match (dx, dy) {
        (0, -2) => '|',
        (0, 2) => '|',
        (2, 0) => '-',
        (-2, 0) => '-',
        (1, -1) => 'F',
        (-1, -1) => '7',
        (1, 1) => 'J',
        (-1, 1) => 'L',
        _ => panic!("Invalid start char"),
    };
    println!("start_char: {}", start_char);

    // replace it
    let mut input = input.clone();
    input[start_line][start_col] = start_char;

    for (i, line) in input.iter().enumerate() {
        let mut intersections = 0;
        let mut edge_enter_char = 'X';
        for (j, &c) in line.iter().enumerate() {
            if visited.contains(&(j as i32, i as i32)) {
                if c == '|' {
                    intersections += 1;
                    print!("{}", c.to_string().red());
                } else if c == 'L' || c == 'F' {
                    edge_enter_char = c;
                    print!("{}", c.to_string().yellow());
                } else if c == 'J' || c == '7' {
                    if (c == 'J' && edge_enter_char == 'F') || (c == '7' && edge_enter_char == 'L')
                    {
                        intersections += 1;
                        print!("{}", c.to_string().red());
                    } else {
                        print!("{}", c.to_string().yellow());
                    }
                } else {
                    print!("{}", c.to_string().blue());
                }
            } else {
                if intersections % 2 == 1 {
                    area += 1;
                    print!("{}", c.to_string().green());
                } else {
                    print!("{}", c.to_string());
                }
            }
        }
        println!("");
    }

    area
}

#[cfg(test)]
mod test_day10 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const EXAMPLE_INPUT_PART_2: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EXAMPLE_INPUT_PART_2_2: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const EXAMPLE_INPUT_PART_2_3: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    const EXAMPLE_OUTPUT_PART_1: i32 = 4;
    const EXAMPLE_OUTPUT_PART_2: i32 = 4;
    const EXAMPLE_OUTPUT_PART_2_2: i32 = 8;
    const EXAMPLE_OUTPUT_PART_2_3: i32 = 10;

    #[test]
    fn day10_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res, "4");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day10_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(10, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "6738");
        k9::assert_equal!(res, 6738);
    }

    #[test]
    fn day10_p2_example_1() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res, "4");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day10_p2_example_2() {
        let res = part2(EXAMPLE_INPUT_PART_2_2);
        k9::snapshot!(res, "8");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2_2);
    }

    #[test]
    fn day10_p2_example_3() {
        let res = part2(EXAMPLE_INPUT_PART_2_3);
        k9::snapshot!(res, "10");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2_3);
    }

    #[test]
    fn day10_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(10, 1);
        let res = part2(&input2);
        k9::snapshot!(res, "579");
        k9::assert_equal!(res, 579);
    }
}
