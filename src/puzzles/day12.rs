use itertools::Itertools;
use memoize::memoize;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            let (springs, runs) = parse_line(l);
            get_candidate_count(springs, runs)
        })
        .collect()
}

fn parse_input_p2(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            let (springs, runs) = parse_line(l);
            let springs = [
                springs.clone(),
                springs.clone(),
                springs.clone(),
                springs.clone(),
                springs,
            ]
            .join("?");
            let runs = (0..5).flat_map(|_| runs.clone()).collect();
            get_candidate_count(springs, runs)
        })
        .collect()
}

fn parse_line(line: &str) -> (String, Vec<usize>) {
    let (springs, broken) = line.splitn(2, " ").collect_tuple().unwrap();
    let runs: Vec<usize> = broken.split(',').map(|b| b.parse().unwrap()).collect();
    // let runs: Vec<&str> = springs.split(".").filter(|seq| !seq.is_empty()).collect();
    (springs.to_string(), runs)
}

#[memoize]
fn get_candidate_count(springs: String, runs: Vec<usize>) -> usize {
    if springs.len() == 0 {
        if runs.len() == 0 {
            return 1;
        } else {
            return 0;
        };
    }

    if runs.len() == 0 {
        if springs.chars().any(|c| c == '#') {
            return 0;
        }
        return 1;
    }

    if springs.len() < runs.iter().sum::<usize>() + runs.len() - 1 {
        // not enough remaining springs, impossible for this to be valid
        return 0;
    }

    let next_char = springs.chars().next().unwrap();
    if next_char == '.' {
        return get_candidate_count(springs.chars().skip(1).collect(), runs);
    } else if next_char == '#' {
        let this_run = runs[0];
        let other_runs = runs.into_iter().skip(1);

        for i in 0..this_run {
            if springs.chars().nth(i).unwrap() == '.' {
                return 0;
            }
        }
        if springs.chars().nth(this_run).unwrap_or('X') == '#' {
            return 0;
        }
        return get_candidate_count(
            springs.chars().skip(this_run + 1).collect(),
            other_runs.collect(),
        );
    } else if next_char != '?' {
        panic!("Invalid char {next_char}");
    }
    // we have a '?'
    let remaining_springs = springs.chars().skip(1).collect::<String>();
    // DP on the question mark possibilities
    return get_candidate_count("#".to_string() + &remaining_springs, runs.clone())
        + get_candidate_count(".".to_string() + &remaining_springs, runs);
}

pub fn part1(input: &str) -> usize {
    // parse_input(input.lines().next().unwrap()).iter().sum()
    parse_input(input).iter().sum()
}

pub fn part2(input: &str) -> usize {
    parse_input_p2(input).iter().sum()
}

#[cfg(test)]
mod test_day12 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: usize = 21;
    const EXAMPLE_OUTPUT_PART_2: usize = 525152;

    #[test]
    fn day12_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res, "21");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day12_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(12, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "7260");
        k9::assert_equal!(res, 7260);
    }

    #[test]
    fn day12_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res, "525152");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day12_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(12, 1);
        let res = part2(&input2);
        k9::snapshot!(res, "1909291258644");
        k9::assert_equal!(res, 1909291258644);
    }
}
