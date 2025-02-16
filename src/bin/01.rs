advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|m| m.parse::<u32>().ok())
            .map(|m| m / 3 - 2)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|m| m.parse::<u32>().ok())
            .map(|m| calculate_total_cost_with_fuel(m / 3 - 2, true))
            .sum(),
    )
}

fn calculate_total_cost_with_fuel(mass: u32, first: bool) -> u32 {
    let cost: i32 = mass as i32 / 3 - 2;

    // janky, but for now it works
    if cost > 0 {
        if first {
            mass + cost as u32 + calculate_total_cost_with_fuel(cost as u32, false)
        } else {
            cost as u32 + calculate_total_cost_with_fuel(cost as u32, false)
        }
    } else if first {
        mass
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_fuel_cost_calculation() {
        assert_eq!(calculate_total_cost_with_fuel(14 / 3 - 2, true), 2);
        assert_eq!(calculate_total_cost_with_fuel(1969 / 3 - 2, true), 966);
        assert_eq!(calculate_total_cost_with_fuel(100756 / 3 - 2, true), 50346)
    }
}
