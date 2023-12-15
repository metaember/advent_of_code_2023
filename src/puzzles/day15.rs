use itertools::Itertools;

fn hash(prev: i32, c: char) -> i32 {
    ((prev + (c as u8) as i32) * 17) % 256
}

fn hash_string(s: String) -> i32 {
    s.chars().fold(0, hash)
}
fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .flat_map(|l| l.chars())
        .collect::<String>()
        .split(',')
        .map(|s| s.to_string())
        .collect()
}
pub fn part1(input: &str) -> i32 {
    parse_input(input).into_iter().map(hash_string).sum()
}

pub fn part2(input: &str) -> usize {
    let mut res: Vec<Vec<(String, i32)>> = vec![Vec::with_capacity(10); 256];

    for step in parse_input(input) {
        if step.ends_with('-') {
            let label = step.trim_end_matches('-');
            let label_hash = hash_string(label.to_string());
            let box_nb = label_hash as usize;
            res[box_nb].retain(|(lab, _)| *lab != label);
            continue;
        } else {
            let (label, focal) = step.split('=').collect_tuple().unwrap();
            let focal = focal.parse::<i32>().unwrap();
            let label_hash = hash_string(label.to_string()) as usize;
            let mut replaced = false;
            res[label_hash] = res[label_hash]
                .clone()
                .into_iter()
                .map(|(lab, foc)| {
                    if lab == label {
                        replaced = true;
                        (lab, focal)
                    } else {
                        (lab, foc)
                    }
                })
                .collect();
            if !replaced {
                res[label_hash].push((label.to_string(), focal));
            }
        }
    }

    res.iter()
        .enumerate()
        .flat_map(|(i, lens)| {
            lens.iter()
                .enumerate()
                .map(move |(j, (_, focal))| (i + 1) * (j + 1) * (*focal as usize))
        })
        .sum()
}

#[cfg(test)]
mod test_day15 {
    use rstest::rstest;

    use super::{hash_string, part1, part2};
    use crate::puzzle_inputs;

    const EXAMPLE_INPUT_PART_1A: &str = "\
HASH";
    const EXAMPLE_INPUT_PART_1B: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    const EXAMPLE_INPUT_PART_2: &str = EXAMPLE_INPUT_PART_1B;

    const EXAMPLE_OUTPUT_PART_1A: i32 = 52;
    const EXAMPLE_OUTPUT_PART_1B: i32 = 1320;
    const EXAMPLE_OUTPUT_PART_2: usize = 145;

    #[rstest]
    #[case("rn", 0)]
    #[case("cm", 0)]
    #[case("qp", 1)]
    #[case("pc", 3)]
    fn test_hash_labels(#[case] label: &str, #[case] expected: i32) {
        assert_eq!(hash_string(label.to_string()), expected)
    }
    #[test]
    fn day15_p1_example_a() {
        let res = part1(EXAMPLE_INPUT_PART_1A);
        k9::snapshot!(res, "52");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1A);
    }

    #[test]
    fn day15_p1_example_b() {
        let res = part1(EXAMPLE_INPUT_PART_1B);
        k9::snapshot!(res, "1320");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_1B);
    }

    #[test]
    fn day15_p1_real() {
        let input1 = puzzle_inputs::get_puzzle_input(15, 1);
        let res = part1(&input1);
        k9::snapshot!(res, "517551");
        k9::assert_equal!(res, 517551);
    }

    #[test]
    fn day15_p2_example() {
        let res = part2(EXAMPLE_INPUT_PART_2);
        k9::snapshot!(res, "145");
        k9::assert_equal!(res, EXAMPLE_OUTPUT_PART_2);
    }

    #[test]
    fn day15_p2_real() {
        let input2 = puzzle_inputs::get_puzzle_input(15, 1);
        let res = part2(&input2);
        k9::snapshot!(res, "286097");
        k9::assert_equal!(res, 286097);
    }
}
