use anyhow::{Error, Result};
use chrono_tz::Australia::{North, South};
use colored::*;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use memoize::memoize;
use num::PrimInt;
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
}

impl Puzzle {
    /// starting at position (x, y) and going in the direction or Ray,
    /// find the next intersecting mirror, and record the squares we traversed
    fn propagate(
        &self,
        input: &str,
        x: usize,
        y: usize,
        direction: Direction,
        mut visited: &mut HashSet<(usize, usize, Direction)>,
    ) {
        display(input, visited);
        // println!("Going {direction:?} from ({x}, {y})");
        if visited.contains(&(x, y, direction)) {
            // we already visited, no need to double count
            // println!("Already visited {x}, {y} going {direction:?}");
            return;
        }

        visited.insert((x, y, direction));

        match direction {
            Direction::North | Direction::South => {
                let next_mirror = match direction {
                    Direction::North => self.bycol[x].iter().filter(|&m| m.y < y).last(),
                    Direction::South => self.bycol[x].iter().filter(|&m| m.y > y).next(),
                    _ => panic!("Already filtered out other directions"),
                };

                println!("next: {next_mirror:?}");
                match next_mirror {
                    Some(mirror) => {
                        // mark the squares as visited
                        for j in range(y, mirror.y) {
                            // println!("i {x}, {j}, {direction:?}");
                            visited.insert((x, j, direction));
                            // early return if we already visited
                        }
                        // return this as the next mirror
                        let next_directions = mirror.reflect(direction);
                        // recurse
                        for next_direction in next_directions {
                            self.propagate(input, mirror.x, mirror.y, next_direction, &mut visited);
                        }
                    }
                    None => {
                        // mark the squares as visited, return None
                        for j in range_inc(y, self.edge(direction)) {
                            // println!("{x}, {j}, {direction:?}");
                            visited.insert((x, j, direction));
                        }
                    }
                }
            }
            Direction::East | Direction::West => {
                let next_mirror = match direction {
                    Direction::East => {
                        if x == 0 && y == 0 {
                            self.byrow[y].iter().filter(|&m| m.x >= x).next()
                        } else {
                            self.byrow[y].iter().filter(|&m| m.x > x).next()
                        }
                    }
                    Direction::West => self.byrow[y].iter().filter(|&m| m.x < x).last(),
                    _ => panic!("Already filtered out other directions"),
                };
                println!("next: {next_mirror:?}");
                match next_mirror {
                    Some(mirror) => {
                        // mark the squares as visited
                        for i in range(x, mirror.x) {
                            println!("{i}, {x}, {direction:?}");
                            visited.insert((i, y, direction));
                            // early return if we already visited
                        }
                        // return this as the next mirror
                        let next_directions = mirror.reflect(direction);
                        // recurse
                        for next_direction in next_directions {
                            self.propagate(input, mirror.x, mirror.y, next_direction, &mut visited);
                        }
                    }
                    None => {
                        // mark the squares as visited, return None
                        for i in range_inc(x, self.edge(direction)) {
                            println!("{i}, {x}, {direction:?}");
                            visited.insert((i, y, direction));
                            // early return if we already visited
                        }
                    }
                }
            }
        }
    }

    fn edge(&self, direction: Direction) -> usize {
        match direction {
            Direction::East => self.ncols - 1,
            Direction::West => 0,
            Direction::North => 0,
            Direction::South => self.nrows - 1,
        }
    }
}

fn range_inc(start: usize, end: usize) -> Vec<usize> {
    if start <= end {
        (start..=end).collect()
    } else {
        (end..=start).rev().collect()
    }
}

fn range(start: usize, end: usize) -> Vec<usize> {
    if start <= end {
        (start..end).collect()
    } else {
        ((end + 1)..=start).rev().collect()
    }
}

// fn range_to(x: usize, y: usize, maybe_mirror: Option<Mirror>) -> Vec<(usize, usize)> {

// }

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

#[derive(Debug, Clone, Copy)]
struct Mirror {
    x: usize,
    y: usize,
    c: char,
}

impl Mirror {
    /// get the Rays after encountering a reflector
    fn reflect(self, ray: Direction) -> SmallVec<[Direction; 2]> {
        let res = match ray {
            Direction::West => match self.c {
                '/' => smallvec![Direction::South],
                '\\' => smallvec![Direction::North],
                '|' => smallvec![Direction::North, Direction::South],
                '-' => smallvec![ray],
                _ => panic!("Unrecognized mirror type {}", self.c),
            },
            Direction::East => match self.c {
                '/' => smallvec![Direction::North],
                '\\' => smallvec![Direction::South],
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
        };
        // println!("[{}, {}] {:?} => {:?}", self.x, self.y, ray, res);
        res
    }
}

fn display(input: &str, visited: &HashSet<(usize, usize, Direction)>) {
    println!();
    println!("  0123456789");
    for (y, line) in input.lines().enumerate() {
        print!("{y} ");
        for (x, c) in line.chars().enumerate() {
            if visited.contains(&(x, y, Direction::East))
                || visited.contains(&(x, y, Direction::West))
                || visited.contains(&(x, y, Direction::North))
                || visited.contains(&(x, y, Direction::South))
            {
                print!("{}", c.to_string().red());
            } else {
                print!("{}", c)
            }
        }
        println!();
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

    max_line_len += 1;
    let mut bycol: Vec<Vec<Mirror>> = vec![Vec::<Mirror>::new(); max_line_len];

    byrow
        .clone()
        .into_iter()
        .flatten()
        .sorted_by_key(|m| (m.x, m.y))
        .group_by(|m| m.x)
        .into_iter()
        .for_each(|(col, g)| bycol[col] = g.into_iter().collect::<Vec<_>>());

    Puzzle {
        nrows: n_lines,
        ncols: max_line_len,
        bycol,
        byrow,
    }
}
pub fn part1(input: &str) -> usize {
    let input = input.trim();
    let puzzle = parse_input(input);
    let mut visited = HashSet::<(usize, usize, Direction)>::new();
    puzzle.propagate(input, 0, 0, Direction::East, &mut visited);

    display(input, &visited);
    // println!("{:#?}", puzzle.bycol);

    println!();
    println!("visited unique dirs: {}", visited.len());

    println!("dims rows: {} cols:{}", puzzle.nrows, puzzle.ncols);

    const EXAMPLE_INPUT_PART_1_ENERGIZED: &str = "
######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..";

    println!(
        "{:?}",
        visited
            .iter()
            .map(|(x, y, _)| (x, y))
            .sorted()
            .dedup()
            .collect::<Vec<_>>()
    );

    visited
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

pub fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test_day16 {
    use itertools::Itertools;

    use super::{parse_input, part1, part2, Direction};
    use crate::puzzle_inputs;
    use std::collections::HashSet;

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

    const EXAMPLE_INPUT_PART_1_ENERGIZED: &str = "
######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: usize = 46;
    const EXAMPLE_OUTPUT_PART_2: usize = 467835;

    #[test]
    fn day16_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res, "46");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day16_p1_example_energized() {
        let input = EXAMPLE_INPUT_PART_1.trim();
        let puzzle = parse_input(input);
        let mut visited = HashSet::<(usize, usize, Direction)>::new();
        puzzle.propagate(input, 0, 0, Direction::East, &mut visited);

        let energized_cells = EXAMPLE_INPUT_PART_1_ENERGIZED
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(j, _c)| (j, i))
            })
            .collect::<HashSet<(usize, usize)>>();

        k9::assert_equal!(
            energized_cells,
            visited
                .into_iter()
                .map(|(x, y, _)| (x, y))
                .collect::<HashSet<(usize, usize)>>()
        );
    }

    #[test]
    fn day16_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(16, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "8034");
        k9::assert_equal!(res, 8034);
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
