use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(6);

fn count_orbits(map: &HashMap<&str, &str>, target: &str) -> usize {
    if let Some(end) = map.get(target) {
        1 + count_orbits(map, end)
    } else {
        0
    }
}

fn build_orbit_path<'a>(
    map: &'a HashMap<&'a str, &'a str>,
    target: &'a str,
    end: &'a str,
    path: &mut Vec<&'a str>,
) {
    if target == end {
        return;
    }
    if let Some(new_target) = map.get(target) {
        path.push(new_target);
        build_orbit_path(map, new_target, end, path);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: HashMap<&str, &str> = input
        .lines()
        .filter_map(|l| l.split(')').rev().collect_tuple())
        .collect();

    let ends: HashSet<&&str> = map.keys().collect();

    Some(
        ends.iter()
            // .inspect(|e| println!("\n!!!{e}!!!"))
            .map(|e| count_orbits(&map, e))
            // .inspect(|c| println!("Orbit count: {c}"))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: HashMap<&str, &str> = input
        .lines()
        .filter_map(|l| l.split(')').rev().collect_tuple())
        .collect();

    let mut you_path: Vec<&str> = Vec::new();
    build_orbit_path(&map, "YOU", "COM", &mut you_path);

    let mut san_path: Vec<&str> = Vec::new();
    build_orbit_path(&map, "SAN", "COM", &mut san_path);

    for you_planet in you_path.clone() {
        if san_path.contains(&you_planet) {
            return Some(
                you_path
                    .iter()
                    .position(|&e| e == you_planet)
                    .expect("just found it")
                    + san_path
                        .iter()
                        .position(|&e| e == you_planet)
                        .expect("just found the other one"),
            );
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(input);
        assert_eq!(result, Some(4));
    }
}
