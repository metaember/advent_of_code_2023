use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn get_line_wins(line: &str) -> usize {
    let (winning, ours) = line.trim().splitn(2, " | ").collect_tuple().unwrap();
    let winning = winning
        .trim()
        .splitn(2, ": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    let ours = ours
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    ours.into_iter().filter(|o| winning.contains(&o)).count()
}

pub fn part1(input: &str) -> i32 {
    let mut total_points = 0;
    for (i, line) in input.trim().lines().enumerate() {
        let winning_count = get_line_wins(line);
        let cur_total = if winning_count >= 1 {
            2_i32.pow(winning_count as u32 - 1)
        } else {
            0
        };
        total_points += cur_total;
        // println!("{i}: {winning_count}, {cur_total}, {total_points}");
    }
    total_points
}

pub fn part2(input: &str) -> i32 {
    let mut card_counts = HashMap::<usize, i32>::new();
    let nb_cards = input.trim().lines().count();

    for (i, line) in input.trim().lines().enumerate() {
        if !card_counts.contains_key(&i) {
            card_counts.insert(i, 1);
        }

        let winning_count = get_line_wins(line);

        // tbe min is not required in my test cases but it seems like it could be possible
        // to have to clip based on the problem definition
        for next_card in (i + 1)..=(i + winning_count).min(nb_cards) {
            card_counts.insert(
                next_card,
                card_counts.get(&next_card).unwrap_or(&1) + card_counts.get(&i).unwrap(),
            );
        }
    }
    // println!();
    // card_counts
    //     .clone()
    //     .into_iter()
    //     .sorted_by_key(|(k, _)| *k)
    //     .for_each(|(k, v)| println!("{k}: {v}"));
    card_counts.into_values().sum()
}

#[cfg(test)]
mod test_day_4 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    /// Here 114 and 58 are not adjacent to anything
    const EXAMPLE_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const EXAMPLE_OUTPUT_PART_1: i32 = 13;
    const EXAMPLE_OUTPUT_PART_2: i32 = 30;

    #[test]
    fn test_part_1_example() {
        let res = part1(EXAMPLE_INPUT);
        k9::snapshot!(res, "13");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn test_part_1() {
        let input1 = puzzle_inputs::get_puzzle_input(4, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "23235");
        k9::assert_equal!(res, 23235);
    }

    #[test]
    fn test_part_2_example() {
        let res = part2(EXAMPLE_INPUT);
        k9::snapshot!(res, "30");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn test_part_2() {
        let input2 = puzzle_inputs::get_puzzle_input(4, 1);
        let res = part2(&input2);
        k9::snapshot!(res, "5920640");
        k9::assert_equal!(res, 5920640);
    }
}
