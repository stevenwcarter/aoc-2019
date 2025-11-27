advent_of_code::solution!(2);

use advent_of_code::intcode::IntCode;

fn attempt(ic: &IntCode, a: usize, b: usize) -> usize {
    let mut ic = ic.clone();

    *ic.data.entry(1).or_default() = a as i64;
    *ic.data.entry(2).or_default() = b as i64;

    ic.process(false);
    *ic.data.entry(0).or_default() as usize
}

pub fn part_one(input: &str) -> Option<usize> {
    let ic = IntCode::new(input);

    Some(attempt(&ic, 12, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let ic = IntCode::new(input);

    let mut noun_result = 0;
    let mut verb_result = 0;

    (0..100).for_each(|noun| {
        (0..100).for_each(|verb| {
            if noun_result == 0 {
                let result = attempt(&ic, noun, verb);
                if result == 19690720 {
                    noun_result = noun;
                    verb_result = verb;
                }
            }
        });
    });

    Some(100 * noun_result + verb_result)
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
