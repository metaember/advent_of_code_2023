pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

pub fn part1(input: &str) -> i32 {
    let input = parse_input(input);

    let mut total = 0;
    for input in input {
        let mut differences = vec![input];
        while !is_constant(differences.last().unwrap()) {
            differences.push(
                differences
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<i32>>(),
            );
        }
        total += differences.iter().map(|d| d.last().unwrap()).sum::<i32>();
    }
    total
}

fn is_constant(seq: &[i32]) -> bool {
    seq.is_empty() || seq.iter().all(|&n| n == seq[0])
}

pub fn part2(input: &str) -> i32 {
    let input = parse_input(input);
    let mut total = 0;
    for input in input {
        let mut differences = vec![input];
        while !is_constant(differences.last().unwrap()) {
            differences.push(
                differences
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<i32>>(),
            );
        }
        total += differences
            .iter()
            .rev()
            .skip(1)
            .fold(*differences.last().unwrap().first().unwrap(), |acc, d| {
                d.first().unwrap() - acc
            })
    }
    total
}

#[cfg(test)]
mod test_day9 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: i32 = 114;
    const EXAMPLE_OUTPUT_PART_2: i32 = 2;

    #[test]
    fn day9_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res, "114");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day9_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(9, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "1995001648");
        k9::assert_equal!(res, 1995001648);
    }

    #[test]
    fn day9_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res, "2");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day9_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(9, 1);
        let res = part2(&input2);
        k9::snapshot!(res, "988");
        k9::assert_equal!(res, 988);
    }
}
