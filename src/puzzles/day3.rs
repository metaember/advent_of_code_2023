use anyhow::{Error, Result};
use colored::*;

/// Struct representing a number in the schematic
#[derive(Debug)]
pub struct Number {
    /// the parsed number value
    number: i32,
    /// index of the column where the number is found
    start: usize,
    /// index of the last column
    end: usize,
}

impl Number {
    pub fn new(number: i32, start: usize, end: usize) -> Self {
        Number { number, start, end }
    }
}

#[derive(Debug)]
pub struct Symbol {
    symbol: char,
    index: usize,
}

impl Symbol {
    pub fn new(symbol: char, index: usize) -> Self {
        Self { symbol, index }
    }
}

#[derive(Debug)]
pub struct Schematic {
    line_length: usize,
    last_line: usize,
    numbers: Vec<Vec<Number>>,
    symbols: Vec<Vec<Symbol>>,
}

impl std::str::FromStr for Schematic {
    type Err = Error;

    fn from_str(s: &str) -> Result<Schematic> {
        let line_length = s.trim().lines().next().expect("Input has no lines").len();
        let mut last_line: usize = 0;
        let mut numbers: Vec<Vec<Number>> = vec![];
        let mut symbols: Vec<Vec<Symbol>> = vec![];

        let mut is_parsing_num = false;

        let mut number_start_idx: usize = 0;
        let mut number_end_idx: usize = 0;
        let mut prev_line = s.trim().lines().next().unwrap();

        for (row, line) in s.trim().lines().enumerate() {
            if is_parsing_num {
                numbers[row - 1].push(Number::new(
                    prev_line[number_start_idx..=number_end_idx]
                        .parse::<i32>()
                        .expect("This should be a number"),
                    number_start_idx,
                    number_end_idx,
                ));
                is_parsing_num = false;
            }

            number_start_idx = 0;
            number_end_idx = 0;
            let mut current_numbers: Vec<Number> = vec![];
            let mut current_symbols: Vec<Symbol> = vec![];

            for (col, cur_char) in line.chars().enumerate() {
                if cur_char.is_numeric() {
                    if !is_parsing_num {
                        number_start_idx = col;
                    }
                    is_parsing_num = true;
                    number_end_idx = col;
                    continue;
                } else {
                    if is_parsing_num {
                        current_numbers.push(Number::new(
                            line[number_start_idx..=number_end_idx]
                                .parse::<i32>()
                                .expect("This should be a number"),
                            number_start_idx,
                            number_end_idx,
                        ))
                    }
                    is_parsing_num = false;
                }
                if cur_char == '.' {
                    continue;
                }
                // At this point it's a symbol
                current_symbols.push(Symbol::new(cur_char, col));
            }
            numbers.push(current_numbers);
            symbols.push(current_symbols);
            last_line = row;
            prev_line = line;
        }
        let schematic = Schematic {
            line_length,
            last_line,
            numbers,
            symbols,
        };
        Ok(schematic)
    }
}

impl Schematic {
    pub fn has_adjacent_symbol(&self, row: usize, num: &Number) -> bool {
        let start_col = if num.start == 0 {
            num.start
        } else {
            num.start - 1
        };
        let end_col = if num.end == self.line_length {
            num.end
        } else {
            num.end + 1
        };
        let start_row = if row == 0 { row } else { row - 1 };
        let end_row = if row == self.last_line { row } else { row + 1 };

        for row_nb in start_row..=end_row {
            let symbols = &self.symbols[row_nb];
            let match_idx = symbols.binary_search_by_key(&start_col, |s| s.index);
            if match_idx.is_ok() {
                return true;
            }

            let next_idx = match_idx.unwrap_err();
            if let Some(next_val) = &symbols.get(next_idx) {
                if next_val.index <= end_col {
                    return true;
                }
            }
        }
        return false;
    }
    /// a symbol is a gear if it's a `*` (not checked here) and if it is adjacent to
    /// exactly two numbers. This returns an option, with the some value being the
    /// product of the two adjacent numbers if it's a gear, and None if it's not
    pub fn maybe_gear_value(&self, row: usize, symbol: &Symbol) -> Option<i32> {
        let start_col = if symbol.index == 0 {
            symbol.index
        } else {
            symbol.index - 1
        };
        let end_col = if symbol.index == self.line_length {
            symbol.index
        } else {
            symbol.index + 1
        };
        let start_row = if row == 0 { row } else { row - 1 };
        let end_row = if row == self.last_line { row } else { row + 1 };

        let mut matches = 0;
        let mut prod = 1;

        for row_nb in start_row..=end_row {
            let numbers = &self.numbers[row_nb];

            for num in numbers {
                if num.start <= end_col && num.end >= start_col {
                    // this is a match
                    matches += 1;
                    prod *= num.number;
                    if matches > 2 {
                        // cannot be a gear anymore, too many adjacent numbers
                        return None;
                    }
                }
            }
        }
        if matches == 2 {
            Some(prod)
        } else {
            None
        }
    }

    /// Print the input with the numbers and symbols colored
    /// numbers with an adjacent symbol are green, ones without red
    /// and symbols (non periods) as blue. Useful for debugging :)
    pub fn print_colored_adjacencies(s: &str) {
        let schematic: Schematic = s.trim().parse().unwrap();
        for (row, line) in s.trim().lines().enumerate() {
            let this_row_numbers = schematic.numbers.get(row).unwrap();
            for (col, char) in line.chars().enumerate() {
                if char.is_numeric() {
                    if this_row_numbers.iter().any(|n| {
                        schematic.has_adjacent_symbol(row, n) && n.start <= col && n.end >= col
                    }) {
                        // has adjacency
                        print!("{}", char.to_string().green());
                    } else {
                        print!("{}", char.to_string().red());
                    }
                } else if char == '.' {
                    print!("{}", char);
                } else {
                    print!("{}", char.to_string().blue())
                }
            }
            println!();
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let schematic: Schematic = input.parse().unwrap();
    let result: i32 = schematic
        .numbers
        .iter()
        .enumerate()
        .map(|(row, l)| {
            l.into_iter()
                // .map(|num| {
                //     println!("{} {:?}", schematic.has_adjacent_symbol(row, num), num);
                //     num
                // })
                .filter(|&num| schematic.has_adjacent_symbol(row, num))
                .map(|n| n.number)
                .sum::<i32>()
        })
        .sum();
    // dbg!(schematic);
    result
}

pub fn part2(input: &str) -> i32 {
    let schematic: Schematic = input.parse().unwrap();
    let result: i32 = schematic
        .symbols
        .iter()
        .enumerate()
        .flat_map(|(row, symbols)| {
            symbols
                .iter()
                .filter(|s| s.symbol == '*')
                .map(move |s| (row, s))
        })
        .map(|(row, symbol)| {
            println!(
                "{:?} {:?}",
                schematic.maybe_gear_value(row, &symbol),
                symbol
            );
            (row, symbol)
        })
        .filter_map(|(row, symbol)| schematic.maybe_gear_value(row, &symbol))
        .sum();
    // dbg!(schematic);
    result
}

#[cfg(test)]
mod test_day_2 {
    use super::{part1, part2, Schematic};
    use crate::puzzle_inputs;

    /// Here 114 and 58 are not adjacent to anything
    const EXAMPLE_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const EXAMPLE_OUTPUT_PART_1: i32 = 4361;
    const EXAMPLE_OUTPUT_PART_2: i32 = 467835;

    #[test]
    fn test_part_1_example() {
        Schematic::print_colored_adjacencies(EXAMPLE_INPUT);
        k9::assert_equal!(part1(EXAMPLE_INPUT), EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn test_custom_example() {
        let example = "\
..+.222$.......458.817....66..
..............+...........*...
...621+....................169
..............303.....554.....";
        Schematic::print_colored_adjacencies(example);
        let res = part1(example);
        let expected = 222 + 458 + 66 + 621 + 169;
        k9::assert_equal!(res, expected);
    }

    #[test]
    fn test_part_1() {
        let input1 = puzzle_inputs::get_puzzle_input(3, 1);
        Schematic::print_colored_adjacencies(input1.as_str());
        let res = part1(&input1);
        assert_ne!(res, 548403);
        k9::assert_equal!(res, 550064);
    }

    #[test]
    fn test_part_2_example() {
        let res = part2(EXAMPLE_INPUT);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn test_part_2() {
        let input1 = puzzle_inputs::get_puzzle_input(3, 1);
        let res = part2(&input1);
        k9::assert_equal!(res, 85010461);
    }
}
