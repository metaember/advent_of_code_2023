use crate::utils;

#[derive(Debug, Clone)]
struct Mirrors {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}

fn parse_inputs(input: &str) -> Vec<Mirrors> {
    let mut res: Vec<Mirrors> = vec![];
    for cur_input in input.split("\n\n") {
        let rows = cur_input
            .lines()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let cols = utils::transpose(rows.clone());
        res.push(Mirrors { rows, cols })
    }
    res
}

enum Direction {
    Rows,
    Cols,
}

fn find_reflections(mirror: Mirrors, direction: Direction) -> Option<usize> {
    let vector = match direction {
        Direction::Rows => mirror.rows,
        Direction::Cols => mirror.cols,
    };

    // when considering the i'th row (or col) we check if the mirror line is
    // immediately **after** i but before i + 1
    for i in 0..vector.len() - 1 {
        let rows_to_left = i + 1;
        let rows_to_right = vector.len() - i - 1;

        // this is how many rows need to match
        let cur_max_width = rows_to_left.min(rows_to_right);

        let mut is_mirror = false;
        for offset in 0..cur_max_width {
            let left = i - offset;
            let right = i + offset + 1;
            if vector[left] != vector[right] {
                break;
            }
            is_mirror = offset == cur_max_width - 1;
        }
        if is_mirror {
            return Some(rows_to_left);
        }
    }
    return None;
}

fn find_reflections_with_smudge(mirror: Mirrors, direction: Direction) -> Option<usize> {
    let vector = match direction {
        Direction::Rows => mirror.rows,
        Direction::Cols => mirror.cols,
    };

    // when considering the i'th row (or col) we check if the mirror line is
    // immediately **after** i but before i + 1
    for i in 0..vector.len() - 1 {
        let rows_to_left = i + 1;
        let rows_to_right = vector.len() - i - 1;

        // this is how many rows need to match
        let cur_max_width = rows_to_left.min(rows_to_right);

        let mut diff_count = 0;
        for offset in 0..cur_max_width {
            let left = i - offset;
            let right = i + offset + 1;
            diff_count += vector[left]
                .iter()
                .zip(vector[right].iter())
                .map(|(l, r)| if l == r { 0 } else { 1 })
                .sum::<usize>();
        }
        if diff_count == 1 {
            return Some(rows_to_left);
        }
    }
    return None;
}

pub fn part1(input: &str) -> usize {
    let all_mirrors = parse_inputs(input);
    println!("rows");
    let row_reflections: Vec<usize> = all_mirrors
        .iter()
        .filter_map(|mirror| find_reflections(mirror.clone(), Direction::Rows))
        .collect();

    println!("cols");
    let col_reflections: Vec<usize> = all_mirrors
        .into_iter()
        .filter_map(|mirror| find_reflections(mirror, Direction::Cols))
        .collect();

    col_reflections.iter().sum::<usize>() + 100 * row_reflections.iter().sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    let all_mirrors = parse_inputs(input);
    println!("rows");
    let row_reflections: Vec<usize> = all_mirrors
        .iter()
        .filter_map(|mirror| find_reflections_with_smudge(mirror.clone(), Direction::Rows))
        .collect();

    println!("cols");
    let col_reflections: Vec<usize> = all_mirrors
        .into_iter()
        .filter_map(|mirror| find_reflections_with_smudge(mirror, Direction::Cols))
        .collect();

    col_reflections.iter().sum::<usize>() + 100 * row_reflections.iter().sum::<usize>()
}

#[cfg(test)]
mod test_day13 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: usize = 405;
    const EXAMPLE_OUTPUT_PART_2: usize = 400;

    #[test]
    fn day13_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res, "405");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day13_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(13, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "29130");
        k9::assert_equal!(res, 29130);
    }

    #[test]
    fn day13_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res, "400");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day13_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(13, 1);
        let res = part2(&input2);
        k9::snapshot!(res, "33438");
        k9::assert_equal!(res, 33438);
    }
}
