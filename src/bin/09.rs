use advent_of_code::intcode::IntCodeBuilder;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut ic = IntCodeBuilder::default().input(1).build(input);
    ic.process(false);
    Some(ic.get_last_output())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut ic = IntCodeBuilder::default().input(2).build(input);
    ic.process(false);
    Some(ic.get_last_output())
}

#[cfg(test)]
mod tests {
    use advent_of_code::intcode::IntCode;

    #[test]
    fn test_one_1() {
        let input = "104,1125899906842624,99";
        let mut ic = IntCode::new(input);
        ic.process(false);
        assert_eq!(ic.get_last_output(), 1125899906842624);
    }

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
