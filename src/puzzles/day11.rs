#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Galaxy {
    row: usize,
    col: usize,
}

struct Universe {
    galaxies: Vec<Galaxy>,
}

/// `dist_mul` controls how much space dialation we want to account for.
/// for part 1, each empty row or col has 2x
/// for part 2, each empty row or col has 1_000_000x for the test case and 10 or 100 in the examples
fn parse_input(input: &str, dist_mul: usize) -> Universe {
    let mut galaxies = Vec::new();
    let mut i = 0;

    for row in input.lines() {
        let mut has_galaxy = false;
        for (j, col) in row.chars().enumerate() {
            match col {
                '.' => {}
                '#' => {
                    has_galaxy = true;
                    galaxies.push(Galaxy { row: i, col: j })
                }
                _ => panic!("Invalid input"),
            }
        }
        if !has_galaxy {
            println!("row {} has no galaxies", i);
            // if this row had no galaxies, add twice to the counter
            // to account for expansion
            i += dist_mul - 1;
        }
        i += 1;
    }
    // now we account for expansion in the columns
    // traverse sorted by col
    galaxies.sort_by_key(|g| g.col);

    let mut col = 0; // last col with a galaxy
    let mut prev_col = 0; // prev examined galax's col
    let mut cuml_cols_to_add = 0;

    galaxies = galaxies
        .into_iter()
        .map(|g| {
            let mut g = g.clone();
            if g.col > prev_col + 1 {
                // prev galaxy is on a preceding col
                cuml_cols_to_add += (g.col - (prev_col + 1)) * (dist_mul - 1);
                println!(
                    "updating row: {} col: {} prev_col: {} by {}",
                    g.row, g.col, prev_col, cuml_cols_to_add
                );
                col = g.col;
            }
            prev_col = g.col;
            g.col += cuml_cols_to_add;
            g
        })
        .collect();

    Universe { galaxies }
}

fn get_distance(g1: Galaxy, g2: Galaxy) -> usize {
    let row_diff = if g1.row > g2.row {
        g1.row - g2.row
    } else {
        g2.row - g1.row
    };
    let col_diff = if g1.col > g2.col {
        g1.col - g2.col
    } else {
        g2.col - g1.col
    };
    row_diff + col_diff
}

fn sum_pairwise_distances(universe: Universe) -> i64 {
    let mut total_dist = 0;

    for i in 0..(universe.galaxies.len()) {
        let g1 = universe.galaxies[i];
        for j in i + 1..universe.galaxies.len() {
            let g2 = universe.galaxies[j];
            let dist = get_distance(g1, g2);
            total_dist += dist;
        }
    }
    total_dist as i64
}

pub fn part1(input: &str) -> i64 {
    let universe = parse_input(input, 2);
    sum_pairwise_distances(universe)
}

pub fn part2(input: &str, dist_mul: usize) -> i64 {
    let universe = parse_input(input, dist_mul);
    sum_pairwise_distances(universe)
}

#[cfg(test)]
mod test_day11 {
    use super::{part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1;

    const EXAMPLE_OUTPUT_PART_1: i64 = 374;
    const EXAMPLE_OUTPUT_PART_2_1: i64 = 1030;
    const EXAMPLE_OUTPUT_PART_2_2: i64 = 8410;

    #[test]
    fn day11_p1_example() {
        let res = part1(EXAMPLE_INPUT_PART_1);
        k9::snapshot!(res, "374");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1);
    }

    #[test]
    fn day11_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(11, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "9795148");
        k9::assert_equal!(res, 9795148);
    }

    #[test]
    fn day11_p2_example_1() {
        let res = part2(EXAMPLE_INPUT_PART_2, 10);
        k9::snapshot!(res, "1030");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2_1);
    }

    #[test]
    fn day11_p2_example_2() {
        let res = part2(EXAMPLE_INPUT_PART_2, 100);
        k9::snapshot!(res, "8410");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2_2);
    }

    #[test]
    fn day11_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(11, 1);
        let res = part2(&input2, 1_000_000);
        k9::snapshot!(res, "650672493820");
        k9::assert_equal!(res, 650672493820);
    }
}
