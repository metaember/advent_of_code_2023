use anyhow::{Error, Result};
use colored::*;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};
use toml::map;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,  // 11
    Queen, // 12
    King,  // 13
    Ace,   // 14
}

impl std::str::FromStr for CardValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<CardValue> {
        match s {
            "2" => Ok(CardValue::Two),
            "3" => Ok(CardValue::Three),
            "4" => Ok(CardValue::Four),
            "5" => Ok(CardValue::Five),
            "6" => Ok(CardValue::Six),
            "7" => Ok(CardValue::Seven),
            "8" => Ok(CardValue::Eight),
            "9" => Ok(CardValue::Nine),
            "T" => Ok(CardValue::Ten),
            "J" => Ok(CardValue::Jack),
            "Q" => Ok(CardValue::Queen),
            "K" => Ok(CardValue::King),
            "A" => Ok(CardValue::Ace),
            _ => Err(anyhow::anyhow!("Invalid card value: {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]

enum HandStrength {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    strength: HandStrength,
    counts: SmallVec<[(i32, CardValue); 5]>,
}

impl std::str::FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Hand> {
        if !s.len() == 5 {
            return Err(anyhow::anyhow!("Hand should have exactly 5 cards: {}", s));
        }

        let cards = s
            .chars()
            .map(|s| s.to_string().parse::<CardValue>().unwrap())
            .collect::<SmallVec<[CardValue; 5]>>();

        let mut counts = HashMap::<CardValue, i32>::new();
        for card in cards.iter() {
            counts.insert(card.clone(), counts.get(card).unwrap_or(&0) + 1);
        }
        // sort by count then tiebreak by card value
        let counts: SmallVec<[(i32, CardValue); 5]> = counts
            .into_iter()
            .sorted_by_key(|(k, v)| (*v, *k))
            .rev()
            .map(|(card, count)| (count, card))
            .collect();

        if counts.len() == 1 {
            return Ok({
                Hand {
                    strength: HandStrength::FiveOfAKind,
                    counts,
                }
            });
        } else if counts.len() == 2 {
            let first = counts.get(0).unwrap();
            match first.0 {
                4 => {
                    return Ok({
                        Hand {
                            strength: HandStrength::FourOfAKind,
                            counts,
                        }
                    })
                }
                3 => {
                    return Ok({
                        Hand {
                            strength: HandStrength::FullHouse,
                            counts,
                        }
                    })
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Invalid hand with 2 different cards: {:?}",
                        counts
                    ))
                }
            }
        } else if counts.len() == 3 {
            let first = counts.get(0).unwrap();
            let second = counts.get(1).unwrap();
            match first.0 {
                3 => {
                    if second.0 == 2 {
                        return Ok({
                            Hand {
                                strength: HandStrength::FullHouse,
                                counts,
                            }
                        });
                    } else {
                        return Ok({
                            Hand {
                                strength: HandStrength::ThreeOfAKind,
                                counts,
                            }
                        });
                    }
                }
                2 => {
                    if second.0 == 2 {
                        return Ok({
                            Hand {
                                strength: HandStrength::TwoPairs,
                                counts,
                            }
                        });
                    } else {
                        return Err(anyhow::anyhow!(
                            "Invalid hand with 3 different cards: {:?}",
                            counts
                        ));
                    }
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Invalid hand with 3 different cards: {:?}",
                        counts
                    ))
                }
            }
        } else if counts.len() == 4 {
            let first = counts.get(0).unwrap();
            match first.0 {
                2 => {
                    return Ok({
                        Hand {
                            strength: HandStrength::Pair,
                            counts,
                        }
                    })
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Invalid hand with 4 different cards: {:?}",
                        counts
                    ))
                }
            }
        } else if counts.len() == 5 {
            return Ok({
                Hand {
                    strength: HandStrength::HighCard,
                    counts,
                }
            });
        } else {
            return Err(anyhow::anyhow!(
                "Invalid hand with {} different cards: {:?}",
                counts.len(),
                counts
            ));
        };
    }
}

#[derive(Debug)]
struct PuzzleInput {
    hands: Vec<(Hand, i32)>,
}

impl std::str::FromStr for PuzzleInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<PuzzleInput> {
        let lines = s.trim().lines();
        let hands = lines
            .map(|line| {
                let (cards, bet) = line.split_ascii_whitespace().collect_tuple().unwrap();
                (cards.parse::<Hand>().unwrap(), bet.parse::<i32>().unwrap())
            })
            .collect::<Vec<_>>();
        Ok(PuzzleInput { hands })
    }
}

pub fn part1(input: &str) -> i32 {
    let puzzle = input.parse::<PuzzleInput>().unwrap();
    puzzle
        .hands
        .iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .enumerate()
        .map(|f| {
            println!("{:?}", f);
            f
        })
        .fold(0, |acc, (i, (_, bet))| acc + bet * (i + 1) as i32)
}

pub fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod test_day_7 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    /// Here 114 and 58 are not adjacent to anything
    const EXAMPLE_INPUT_PART_1: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: i32 = 6440;
    const EXAMPLE_OUTPUT_PART_2: i32 = 467835;

    #[test]
    fn day7_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res, "6440");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day7_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(7, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "249161408"); // not 249161408
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day7_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day7_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(7, 1);
        let res = part2(&input2);
        k9::snapshot!(res);
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }
}
