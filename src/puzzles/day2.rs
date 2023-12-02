use anyhow::{Error, Result};
use itertools::Itertools;

struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    pub fn id_if_counts_possible(&self) -> i32 {
        if self.red <= 12 && self.green <= 13 && self.blue <= 14 {
            return self.id;
        }
        return 0;
    }

    pub fn get_power(&self) -> i32 {
        self.red * self.blue * self.green
    }
}

impl std::str::FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Game> {
        let (id, rounds) = s.trim().splitn(2, ": ").collect_tuple().unwrap();
        let id = id.trim().split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        rounds.split("; ").for_each(|r| {
            r.split(", ").for_each(|c| {
                let (count, color) = c.splitn(2, ' ').collect_tuple().unwrap();
                let count = count.trim().parse::<i32>().unwrap();
                let color = color.trim();

                match color {
                    "red" => red = red.max(count),
                    "blue" => blue = blue.max(count),
                    "green" => green = green.max(count),
                    _ => panic!("Unknown color {}", color),
                }
            })
        });
        let game = Self {
            id,
            red,
            green,
            blue,
        };
        Ok(game)
    }
}

pub fn part1(inputs: &str) -> i32 {
    inputs
        .trim()
        .lines()
        .filter_map(|l| l.parse::<Game>().map(|g| g.id_if_counts_possible()).ok())
        .sum()
}

pub fn part2(inputs: &str) -> i32 {
    inputs
        .trim()
        .lines()
        .filter_map(|l| l.parse::<Game>().map(|g| g.get_power()).ok())
        .sum()
}


#[cfg(test)]
mod test_day_2 {
    use super::*;
    use crate::puzzle_inputs;

    const PART_1_EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1_example() {
        k9::assert_equal!(part1(PART_1_EXAMPLE), 8);
    }

    #[test]
    fn test_part_1() {
        let input1 = puzzle_inputs::get_puzzle_input(2, 1);
        k9::assert_equal!(part1(&input1), 2505);
    }

    #[test]
    fn test_part_2_example() {
        k9::assert_equal!(part2(PART_1_EXAMPLE), 2286);
    }

    #[test]
    fn test_part_2() {
        let input1 = puzzle_inputs::get_puzzle_input(2, 1);
        k9::assert_equal!(part2(&input1), 70265);
    }

}
