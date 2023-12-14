use anyhow::{Error, Result};

#[derive(Debug)]
struct PuzzleInput {
    times: Vec<i64>,
    distances: Vec<i64>,
}

impl std::str::FromStr for PuzzleInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<PuzzleInput> {
        let mut lines = s.trim().lines();
        let times = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let distances = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Ok(PuzzleInput { times, distances })
    }
}

#[derive(Debug)]
struct PuzzleInputPart2 {
    time: i64,
    distance: i64,
}

impl std::str::FromStr for PuzzleInputPart2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<PuzzleInputPart2> {
        let mut lines = s.trim().lines();
        let time = lines
            .next()
            .unwrap()
            .split(':')
            .skip(1)
            .collect::<String>()
            .replace(" ", "")
            .parse::<i64>()
            .unwrap();
        let distance = lines
            .next()
            .unwrap()
            .split(':')
            .skip(1)
            .collect::<String>()
            .replace(" ", "")
            .parse::<i64>()
            .unwrap();
        Ok(PuzzleInputPart2 { time, distance })
    }
}
pub fn distance_traveled(time: i64, charge: i64) -> i64 {
    let remaining = time - charge;
    remaining * charge
}

pub fn part1(input: &str) -> i64 {
    let puzzle_input = &input.parse::<PuzzleInput>().unwrap();
    // dbg!(puzzle_input);

    let mut score = 1;
    for i in 0..puzzle_input.times.len() {
        let time = puzzle_input.times[i];
        let distance = puzzle_input.distances[i];
        let mut winning_count = 0;
        for charge in 0..=time {
            let distance_traveled = distance_traveled(time, charge);
            if distance_traveled > distance {
                winning_count += 1;
            }
        }
        score *= winning_count;
    }
    score
}

pub fn part2(input: &str) -> i64 {
    let puzzle_input = &input.parse::<PuzzleInputPart2>().unwrap();
    dbg!(puzzle_input);
    solve_poly(puzzle_input.time, puzzle_input.distance)
}

pub fn solve_poly(time: i64, distance: i64) -> i64 {
    // let puzzle_input = input.parse::<PuzzleInput>().unwrap();

    // (time- charge) * charge >= distance
    // time * charge - charge^2 >= distance
    // -charge**2 + time * charge - distance >= 0

    let det = time.pow(2) - 4 * (-1) * (-distance);
    let det = det as f64;
    let det = dbg!(det);
    let upper = (-time as f64 + det.sqrt()) / 2.;
    let lower = (-time as f64 - det.sqrt()) / 2.;
    upper as i64 - (lower as i64)
}

#[cfg(test)]
mod test_day_6 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    /// Here 114 and 58 are not adjacent to anything
    const EXAMPLE_INPUT_PART_1: &str = "\
Time:      7  15   30
Distance:  9  40  200
";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: i64 = 288;
    const EXAMPLE_OUTPUT_PART_2: i64 = 71503;

    #[test]
    fn day6_p1_example() {
        k9::snapshot!(part1(EXAMPLE_INPUT_PART_1), "288");
        k9::assert_equal!(part1(EXAMPLE_INPUT_PART_1), EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day6_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(6, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "505494");
        k9::assert_equal!(res, 505494);
    }

    #[test]
    fn day6_p2_example() {
        // let res = solve_poly(71530, 940200);
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res, "71503");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day6_p2_real() {
        // let res = solve_poly(40829166, 277133813491063);
        let input2 = puzzle_inputs::get_puzzle_input(6, 1);
        let res = part2(&input2);

        k9::snapshot!(res, "23632299");
        k9::assert_equal!(res, 23632299);
    }
}
