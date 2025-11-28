use std::{
    fmt::Display,
    hash::{BuildHasherDefault, DefaultHasher},
    sync::OnceLock,
};

use cached::proc_macro::cached;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::hash::{Hash, Hasher};

use nohash::NoHashHasher;

type BuildNoHash = BuildHasherDefault<NoHashHasher<u64>>;
type OptimizedMap = HashMap<u64, u64, BuildNoHash>;

advent_of_code::solution!(6);

// Creating a OnceLock so the `count_orbits` doesn't need the map as a parameter for caching
// purposes
static PART1_MAP: OnceLock<OptimizedMap> = OnceLock::new();
static PART2_MAP: OnceLock<OptimizedMap> = OnceLock::new();

#[cached]
fn count_orbits(target: u64) -> usize {
    if let Some(end) = PART1_MAP.get().unwrap().get(&target) {
        1 + count_orbits(*end)
    } else {
        0
    }
}

fn build_orbit_path(target: u64, end: u64, path: &mut Vec<u64>) {
    if target == end {
        return;
    }
    if let Some(new_target) = PART2_MAP.get().unwrap().get(&target) {
        path.push(*new_target);
        build_orbit_path(*new_target, end, path);
    }
}
pub fn calculate_hash<T: Hash + Display>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
pub fn part_one(input: &str) -> Option<usize> {
    let map: OptimizedMap = input
        .lines()
        .filter_map(|l| l.split(')').rev().collect_tuple())
        .map(|(a, b)| (calculate_hash(&a.trim()), calculate_hash(&b.trim())))
        .collect();

    let ends: HashSet<u64> = map.keys().copied().collect();

    PART1_MAP.get_or_init(|| map);

    Some(
        ends.iter()
            // .inspect(|e| println!("\n!!!{e}!!!"))
            .map(|e| count_orbits(*e))
            // .inspect(|c| println!("Orbit count: {c}"))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: OptimizedMap = input
        .lines()
        .filter_map(|l| l.split(')').rev().collect_tuple())
        .map(|(a, b)| (calculate_hash(&a), calculate_hash(&b)))
        .collect();

    let you = calculate_hash(&"YOU");
    let san = calculate_hash(&"SAN");
    let com = calculate_hash(&"COM");

    PART2_MAP.get_or_init(|| map);

    let mut you_path: Vec<u64> = Vec::new();
    build_orbit_path(you, com, &mut you_path);

    let mut san_path: Vec<u64> = Vec::new();
    build_orbit_path(san, com, &mut san_path);

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
