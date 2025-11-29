use std::str::FromStr;

use aoc_mine::{lcm, Coord};

advent_of_code::solution!(12);

#[derive(Debug, Copy, Clone, Default)]
struct Coord3(isize, isize, isize);
impl Coord3 {
    pub fn x(&self) -> isize {
        self.0
    }
    pub fn y(&self) -> isize {
        self.1
    }
    pub fn z(&self) -> isize {
        self.2
    }
}

#[derive(Debug, Copy, Clone)]
struct Moon {
    pub position: Coord3,
    pub velocity: Coord3,
}
impl Moon {
    pub fn new(position: Coord3) -> Self {
        Self {
            position,
            velocity: Coord3::default(),
        }
    }
}
impl FromStr for Moon {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<isize> = s
            .trim_matches(|c| c == '<' || c == '>')
            .split(',')
            .map(|part| {
                let (_, value) = part.trim().split_once('=').unwrap();
                value.trim().parse::<isize>().unwrap()
            })
            .collect();
        Ok(Moon::new(Coord3(coords[0], coords[1], coords[2])))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut moons: Vec<Moon> = input
        .lines()
        .map(|l| l.parse::<Moon>().expect("could not parse: {l}"))
        .collect();

    (0..1000).for_each(|_step| {
        apply_gravity(&mut moons);
        update_positions(&mut moons);
    });

    calculate_total_energy(&moons)
}

fn apply_gravity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in (i + 1)..moons.len() {
            let (moon_a, moon_b) = {
                let (left, right) = moons.split_at_mut(j);
                (&mut left[i], &mut right[0])
            };
            if moon_a.position.x() < moon_b.position.x() {
                moon_a.velocity.0 += 1;
                moon_b.velocity.0 -= 1;
            } else if moon_a.position.x() > moon_b.position.x() {
                moon_a.velocity.0 -= 1;
                moon_b.velocity.0 += 1;
            }
            if moon_a.position.y() < moon_b.position.y() {
                moon_a.velocity.1 += 1;
                moon_b.velocity.1 -= 1;
            } else if moon_a.position.y() > moon_b.position.y() {
                moon_a.velocity.1 -= 1;
                moon_b.velocity.1 += 1;
            }
            if moon_a.position.z() < moon_b.position.z() {
                moon_a.velocity.2 += 1;
                moon_b.velocity.2 -= 1;
            } else if moon_a.position.z() > moon_b.position.z() {
                moon_a.velocity.2 -= 1;
                moon_b.velocity.2 += 1;
            }
        }
    }
}
fn apply_gravity_1d(moons: &mut [Coord<isize>]) {
    for i in 0..moons.len() {
        for j in (i + 1)..moons.len() {
            let (moon_a, moon_b) = {
                let (left, right) = moons.split_at_mut(j);
                (&mut left[i], &mut right[0])
            };
            if moon_a.0 < moon_b.0 {
                moon_a.1 += 1;
                moon_b.1 -= 1;
            } else if moon_a.0 > moon_b.0 {
                moon_a.1 -= 1;
                moon_b.1 += 1;
            }
        }
    }
}

fn update_positions(moons: &mut [Moon]) {
    for moon in moons.iter_mut() {
        moon.position.0 += moon.velocity.0;
        moon.position.1 += moon.velocity.1;
        moon.position.2 += moon.velocity.2;
    }
}
fn update_positions_1d(moons: &mut [Coord<isize>]) {
    for moon in moons.iter_mut() {
        moon.0 += moon.1;
    }
}

fn calculate_total_energy(moons: &[Moon]) -> Option<u64> {
    Some(
        moons
            .iter()
            .map(|moon| {
                let potential_energy =
                    moon.position.x().abs() + moon.position.y().abs() + moon.position.z().abs();
                let kinetic_energy =
                    moon.velocity.x().abs() + moon.velocity.y().abs() + moon.velocity.z().abs();
                (potential_energy * kinetic_energy) as u64
            })
            .sum(),
    )
}

fn find_cycle_for_axis<FGet>(moons: &[Moon], get_axis: FGet) -> u64
where
    FGet: Fn(&Moon) -> isize,
{
    let initial_state: Vec<Coord<isize>> =
        moons.iter().map(|moon| Coord(get_axis(moon), 0)).collect();
    find_cycle(&initial_state)
}

pub fn part_two(input: &str) -> Option<u64> {
    let moons: Vec<Moon> = input
        .lines()
        .map(|l| l.parse::<Moon>().expect("could not parse: {l}"))
        .collect();

    let x = find_cycle_for_axis(&moons, |moon| moon.position.x());
    let y = find_cycle_for_axis(&moons, |moon| moon.position.y());
    let z = find_cycle_for_axis(&moons, |moon| moon.position.z());

    Some(lcm(lcm(x, y), z))
}

fn find_cycle(moons: &[Coord<isize>]) -> u64 {
    let initial_state = moons;

    let mut steps = 0u64;
    let moons = &mut moons.to_vec();
    loop {
        steps += 1;
        apply_gravity_1d(moons);
        update_positions_1d(moons);

        if moons == initial_state {
            break;
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    // use super::*;
    //
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
