use anyhow::{Error, Result};
use itertools::Itertools;
use smallvec::SmallVec;
use std::collections::HashMap;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardValueP2 {
    Jack, // jack is now joker, lowest rank individually
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl std::str::FromStr for CardValueP2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<CardValueP2> {
        match s {
            "2" => Ok(CardValueP2::Two),
            "3" => Ok(CardValueP2::Three),
            "4" => Ok(CardValueP2::Four),
            "5" => Ok(CardValueP2::Five),
            "6" => Ok(CardValueP2::Six),
            "7" => Ok(CardValueP2::Seven),
            "8" => Ok(CardValueP2::Eight),
            "9" => Ok(CardValueP2::Nine),
            "T" => Ok(CardValueP2::Ten),
            "J" => Ok(CardValueP2::Jack),
            "Q" => Ok(CardValueP2::Queen),
            "K" => Ok(CardValueP2::King),
            "A" => Ok(CardValueP2::Ace),
            _ => Err(anyhow::anyhow!("Invalid card value: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<T> {
    strength: HandStrength,
    counts: SmallVec<[(i32, T); 5]>, // not actually used but nice for logging and debugging
    cards: SmallVec<[T; 5]>,
}

impl<T: Ord> Hand<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let strength_ord = self.strength.cmp(&other.strength);
        if strength_ord != std::cmp::Ordering::Equal {
            return strength_ord;
        }
        // this would be for poker rules
        //self.counts.cmp(&other.counts)
        self.cards.cmp(&other.cards)
    }
}

impl std::str::FromStr for Hand<CardValue> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Hand<CardValue>> {
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

        parse_hand_from_card_counts(counts, cards)
    }
}

impl std::str::FromStr for Hand<CardValueP2> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Hand<CardValueP2>> {
        if !s.len() == 5 {
            return Err(anyhow::anyhow!("Hand should have exactly 5 cards: {}", s));
        }

        let cards = s
            .chars()
            .map(|s| s.to_string().parse::<CardValueP2>().unwrap())
            .collect::<SmallVec<[CardValueP2; 5]>>();

        let mut counts = HashMap::<CardValueP2, i32>::new();
        let joker_count = cards.iter().filter(|c| **c == CardValueP2::Jack).count();
        for card in cards.iter().filter(|c| **c != CardValueP2::Jack) {
            counts.insert(card.clone(), counts.get(card).unwrap_or(&0) + 1);
        }
        // sort by count then tiebreak by card value
        let mut counts: SmallVec<[(i32, CardValueP2); 5]> = counts
            .into_iter()
            .sorted_by_key(|(k, v)| (*v, *k))
            .rev()
            .map(|(card, count)| (count, card))
            .collect();

        if counts.len() == 0 {
            return Ok({
                Hand::<CardValueP2> {
                    strength: HandStrength::FiveOfAKind,
                    counts,
                    cards,
                }
            });
        }

        // not all jacks, so counts is not empty
        counts[0].0 += joker_count as i32;
        parse_hand_from_card_counts(counts, cards)
    }
}

fn parse_hand_from_card_counts<T: std::fmt::Debug>(
    counts: SmallVec<[(i32, T); 5]>,
    cards: SmallVec<[T; 5]>,
) -> Result<Hand<T>> {
    if counts.len() == 1 {
        return Ok({
            Hand {
                strength: HandStrength::FiveOfAKind,
                counts,
                cards,
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
                        cards,
                    }
                })
            }
            3 => {
                return Ok({
                    Hand {
                        strength: HandStrength::FullHouse,
                        counts,
                        cards,
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
                            cards,
                        }
                    });
                } else {
                    return Ok({
                        Hand {
                            strength: HandStrength::ThreeOfAKind,
                            counts,
                            cards,
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
                            cards,
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
                        cards,
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
                cards,
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

#[derive(Debug)]
struct PuzzleInput {
    hands: Vec<(Hand<CardValue>, i32)>,
}

impl std::str::FromStr for PuzzleInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<PuzzleInput> {
        let lines = s.trim().lines();
        let hands = lines
            .map(|line| {
                let (cards, bet) = line.split_ascii_whitespace().collect_tuple().unwrap();
                (
                    cards.parse::<Hand<CardValue>>().unwrap(),
                    bet.parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        Ok(PuzzleInput { hands })
    }
}

#[derive(Debug)]
struct PuzzleInputP2 {
    hands: Vec<(Hand<CardValueP2>, i32)>,
}

impl std::str::FromStr for PuzzleInputP2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<PuzzleInputP2> {
        let lines = s.trim().lines();
        let hands = lines
            .map(|line| {
                let (cards, bet) = line.split_ascii_whitespace().collect_tuple().unwrap();
                (
                    cards.parse::<Hand<CardValueP2>>().unwrap(),
                    bet.parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        Ok(PuzzleInputP2 { hands })
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
    let puzzle = input.parse::<PuzzleInputP2>().unwrap();
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
    const EXAMPLE_OUTPUT_PART_2: i32 = 5905;

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
        k9::snapshot!(res, "249390788");
        k9::assert_equal!(res, 249390788);
    }

    #[test]
    fn day7_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res, "5905");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day7_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(7, 1);
        let res = part2(&input2);
        k9::snapshot!(res, "248750248");
        k9::assert_equal!(res, 248750248);
    }
}
