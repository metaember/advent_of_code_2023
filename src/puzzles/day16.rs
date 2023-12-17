use anyhow::{Error, Result};
use chrono_tz::Australia::{North, South};
use colored::*;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;
use smallvec::{smallvec, SmallVec};
use std::collections::{HashMap, HashSet};
use toml::map;

#[derive(Debug)]
struct Puzzle {
    nrows: usize,
    ncols: usize,
    /// for each row, vector of mirrors on that row,
    /// sorted by increasing coord
    byrow: Vec<Vec<Mirror>>,
    bycol: Vec<Vec<Mirror>>,
    // set of visited locations
    visited: HashSet<(usize, usize, Direction)>,
}

impl Puzzle {
    /// starting at position (x, y) and going in the direction or Ray,
    /// find the next intersecting mirror, and record the squares we traversed
    fn propagate(mut self, x: usize, y: usize, direction: Direction) -> (usize, usize) {
        let mut newx = x as i32;
        let mut newy = y as i32;

        let (dx, dy) = direction.to_offsets();
        match direction {
            Direction::North | Direction::South => {
                newx = x as i32;
                let col = self.bycol[x];
                // todo: binary search to find the next mirror?

                let next_mirror = match direction {
                    Direction::North => col.iter().filter(|&&m| m.y > y).next(),
                    Direction::South => col.iter().filter(|&&m| m.y < y).last(),
                    _ => panic!("Already filtered out other directions"),
                };

                match next_mirror {
                    Some(mirror) => {
                        // mark the squares as visited
                        for j in y..=mirror.y {
                            self.visited.insert((x, j, direction));
                            // early return if we already visited
                        }
                        // return this as the next mirror
                        let next_directions = mirror.propagate(direction);
                        // recurse
                        for next_direction in next_directions {
                            self.propagate(mirror.x, mirror.y, next_direction);
                        }
                    }
                    None => {
                        // mark the squares as visited, return None
                    }
                }
            }
            Direction::East | Direction::West => {}
        }
        todo!();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_offsets(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Mirror {
    x: usize,
    y: usize,
    c: char,
}

impl Mirror {
    /// get the Rays after encountering a reflector
    fn propagate(self, ray: Direction) -> SmallVec<[Direction; 2]> {
        match ray {
            Direction::West => match self.c {
                '/' => smallvec![Direction::South],
                '\\' => smallvec![Direction::North],
                '|' => smallvec![Direction::North, Direction::South],
                '-' => smallvec![ray],
                _ => panic!("Unrecognized mirror type {}", self.c),
            },
            Direction::East => match self.c {
                '/' => smallvec![Direction::South],
                '\\' => smallvec![Direction::North],
                '|' => smallvec![Direction::North, Direction::South],
                '-' => smallvec![ray],
                _ => panic!("Unrecognized mirror type {}", self.c),
            },
            Direction::South => match self.c {
                '/' => smallvec![Direction::West],
                '\\' => smallvec![Direction::East],
                '|' => smallvec![ray],
                '-' => smallvec![Direction::West, Direction::East],
                _ => panic!("Unrecognized mirror type {}", self.c),
            },
            Direction::North => match self.c {
                '/' => smallvec![Direction::East],
                '\\' => smallvec![Direction::West],
                '|' => smallvec![ray],
                '-' => smallvec![Direction::West, Direction::East],
                _ => panic!("Unrecognized mirror type {}", self.c),
            },
        }
    }
}

fn parse_input(input: &str) -> Puzzle {
    let n_lines = input.trim().lines().count();

    let mut max_line_len = 0;
    let byrow = input
        .trim()
        .lines()
        .enumerate()
        .map(|(j, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(i, c)| {
                    max_line_len = max_line_len.max(i);
                    Mirror { x: i, y: j, c }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut bycol = Vec::<Vec<Mirror>>::new();

    byrow
        .clone()
        .into_iter()
        .flatten()
        .sorted_by_key(|m| (m.x, m.y))
        .group_by(|m| m.x)
        .into_iter()
        .for_each(|(col, g)| g.into_iter().collect::<Vec<_>>());

    Puzzle {
        nrows: n_lines,
        ncols: max_line_len,
        bycol,
        byrow,
        visited: HashSet::new(),
    }
}
pub fn part1(input: &str) -> i32 {
    let puzzle = parse_input(input);

    let mut x = 0;
    let mut y = 0;

    println!("{:#?}", puzzle.bycol);

    todo!()
}

pub fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod test_day16 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: i32 = 4361;
    const EXAMPLE_OUTPUT_PART_2: i32 = 467835;

    #[test]
    fn day16_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day16_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(16, 1);
        let res = part1(&input1);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day16_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day16_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(16, 1);
        let res = part2(&input2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }
}
