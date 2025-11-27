advent_of_code::solution!(5);

use advent_of_code::intcode::IntCodeBuilder;

pub fn solve_with_input(input: &str, input_num: i64) -> Option<i64> {
    let mut ic = IntCodeBuilder::default().input(input_num).build(input);

    ic.process(false);

    ic.output.last().copied()
}

pub fn part_one(input: &str) -> Option<i64> {
    solve_with_input(input, 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve_with_input(input, 5)
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
