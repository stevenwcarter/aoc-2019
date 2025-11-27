advent_of_code::solution!(5);

use advent_of_code::intcode::IntCodeBuilder;

pub fn part_one(input: &str) -> Option<i64> {
    let mut ic = IntCodeBuilder::default().input(1).build(input);

    ic.process(false);

    ic.output.last().copied()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut ic = IntCodeBuilder::default().input(5).build(input);

    ic.process(false);

    ic.output.last().copied()
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
