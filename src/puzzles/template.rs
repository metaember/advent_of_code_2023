use anyhow::{Error, Result};
use colored::*;

pub fn part1(input: &str) -> i32 {
    todo!()
}

pub fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod test_day_4 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    /// Here 114 and 58 are not adjacent to anything
    const EXAMPLE_INPUT_PART_1: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: i32 = 4361;
    const EXAMPLE_OUTPUT_PART_2: i32 = 467835;

    #[test]
    fn day4_p1_example() {
        k9::snapshot!(part1(EXAMPLE_INPUT_PART_1));
        k9::assert_equal!(part1(EXAMPLE_INPUT_PART_1), EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day4_p1() {
        let input1 = puzzle_inputs::get_puzzle_input(4, 1);
        let res = part1(&input1);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day4_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day4_p2_example() {
        let input2 = puzzle_inputs::get_puzzle_input(4, 1);
        let res = part2(&input2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }
}
