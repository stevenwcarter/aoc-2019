use advent_of_code::intcode::IntCode;

use itertools::Itertools;

advent_of_code::solution!(7);

fn try_combination(phase_settings: &[&u8], ic: &IntCode) -> i64 {
    let mut input_signal = 0;

    for &phase in phase_settings {
        let mut ic = ic.clone();
        ic.add_input(*phase as i64);
        ic.add_input(input_signal);
        ic.process(true);
        input_signal = ic.get_last_output();
    }

    input_signal
}
fn try_combination_part2(phase_settings: &[&u8], ic: &IntCode) -> i64 {
    let mut input_signal = 0;

    let mut ics: Vec<IntCode> = phase_settings
        .iter()
        .map(|&&phase| {
            let mut ic = ic.clone();
            ic.add_input(phase as i64);
            ic
        })
        .collect();

    while !ics.last().unwrap().is_quit() {
        for ic in ics.iter_mut() {
            ic.add_input(input_signal);
            ic.process(true);
            input_signal = ic.get_last_output();
        }
    }

    input_signal
}

pub fn part_one(input: &str) -> Option<i64> {
    let phases = [0, 1, 2, 3, 4];
    let ic = IntCode::new(input);
    phases
        .iter()
        .permutations(phases.len())
        .map(|phases| try_combination(&phases, &ic))
        .max()
}

pub fn part_two(input: &str) -> Option<i64> {
    let phases = [5, 6, 7, 8, 9];
    let ic = IntCode::new(input);
    phases
        .iter()
        .permutations(phases.len())
        .map(|phases| try_combination_part2(&phases, &ic))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let result = part_one(input);
        assert_eq!(result, Some(65210));
    }

    #[test]
    fn test_part_two() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(input);
        assert_eq!(result, Some(139629729));
    }

    #[test]
    fn test_part_two_2() {
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let result = part_two(input);
        assert_eq!(result, Some(18216));
    }
}
