advent_of_code::solution!(2);

use advent_of_code::intcode::IntCode;

fn attempt(ic: &IntCode, a: usize, b: usize) -> usize {
    let mut ic = ic.clone();

    ic.data.insert(1, a as i64);
    ic.data.insert(2, b as i64);

    ic.process(false);
    *ic.data.get(&0).unwrap() as usize
}

pub fn part_one(input: &str) -> Option<usize> {
    let ic = IntCode::new(input);

    Some(attempt(&ic, 12, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let ic = IntCode::new(input);

    for noun in (0..100).rev() {
        for verb in (0..100).rev() {
            let result = attempt(&ic, noun, verb);
            if result == 19690720 {
                return Some(100 * noun + verb);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(part_one("1,1,1,4,99,5,6,0,99"), Some(30));
        assert_eq!(part_one("2,4,4,5,99,0"), Some(2));
        assert_eq!(part_one("1,0,0,0,99"), Some(2));
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
