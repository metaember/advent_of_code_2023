use once_cell::sync::Lazy;
use std::collections::HashMap;

pub fn part_1(input: &str) -> i32 {
    let calibration = input
        .trim()
        .lines()
        .map(|l| {
            let digits = l.chars().filter(|c| c.is_numeric()).collect::<String>();
            format!(
                "{}{}",
                digits.chars().next().expect("first digit not found"),
                digits.chars().last().expect("last digit not found")
            )
            .parse::<i32>()
            .expect("Error parsing input data.")
        })
        .sum();

    calibration
}

/// Mangled such that
///  (a) the actual digit is in the replaced value
///  (b) the result is not a match for the initial replacement
///  and critically (c) overlapping matches are not destroyed
static DIGIT_NAME_TO_DIGIT_MANGLED: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    HashMap::from([
        ("one", "on1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ])
});

/// First solution to part 2. This is a bit yuky because `twone` is expected to give `21` ...
pub fn part_2(input: &str) -> i32 {
    let mut converted_str = input.to_string();
    DIGIT_NAME_TO_DIGIT_MANGLED
        .iter()
        .for_each(|(name, mangled)| converted_str = converted_str.replace(name, &mangled));
    part_1(&converted_str)
}


#[cfg(test)]
mod test_day_1 {
    use super::*;
    use crate::puzzle_inputs;

    #[test]
    fn test_part_1_example() {
        let example_string = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let res = part_1(example_string);
        k9::assert_equal!(res, 142);
    }

    #[test]
    fn test_part_1() {
        let input = puzzle_inputs::get_puzzle_input(1, 1);
        let result = part_1(&input);
        k9::assert_equal!(result, 54304);
    }

    #[test]
    fn test_part_2_example() {
        let example_string = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let res = part_2(example_string);
        k9::assert_equal!(res, 281);
    }

    #[test]
    fn test_part_2() {
        let input = puzzle_inputs::get_puzzle_input(1, 1);
        let result = part_2(&input);
        k9::assert_equal!(result, 54418);
    }
}
