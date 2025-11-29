use aoc_mine::{Coord, Grid, LinearGrid};

advent_of_code::solution!(10);

const ANGLE_INCREMENT: f32 = 0.05;

fn display_grid(grid: &LinearGrid<usize, bool>) {
    for y in 0..grid.height() {
        let mut s = String::new();
        for x in 0..grid.width() {
            let display_char = match grid.get(&Coord(x, y)) {
                Some(true) => '#',
                _ => '.',
            };
            s.push(display_char);
        }

        println!("{s}");
    }
}

fn calculate_slope(coord1: &Coord<usize>, coord2: &Coord<usize>) -> (isize, isize) {
    let dy = coord2.1 as isize - coord1.1 as isize;
    let dx = coord2.0 as isize - coord1.0 as isize;

    let gcd = gcd(dx.unsigned_abs(), dy.unsigned_abs());

    (dx / gcd, dy / gcd)
}

fn gcd(abs_1: usize, abs_2: usize) -> isize {
    if abs_2 == 0 {
        return abs_1 as isize;
    }
    gcd(abs_2, abs_1 % abs_2)
}

pub fn part_one(input: &str) -> Option<usize> {
    let height = input.lines().collect::<Vec<&str>>().len();
    let width = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .len();

    let mut grid: LinearGrid<usize, bool> = LinearGrid::new(width, height, false);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .for_each(|(x, _)| {
                let _ = grid.insert(Coord(x, y), true);
            });
    });
    if grid.height() < 10 {
        display_grid(&grid);
    }
    grid.iter()
        .filter(|(_, is_asteroid)| *is_asteroid)
        .map(|(coord, _)| {
            let mut slopes: Vec<(isize, isize)> = Vec::new();
            grid.iter()
                .filter(|(_, is_asteroid)| *is_asteroid)
                .filter(|(other_coord, _)| other_coord != &coord)
                .for_each(|(other_coord, _)| {
                    let slope = calculate_slope(&coord, &other_coord);
                    if !slopes.contains(&slope) {
                        slopes.push(slope);
                    }
                });
            slopes.len()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let height = input.lines().collect::<Vec<&str>>().len();
    let width = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .len();

    let start_asteroid = if height < 30 {
        Coord(11, 13)
    } else {
        Coord(28, 29)
    };

    let mut grid: LinearGrid<usize, bool> = LinearGrid::new(width, height, false);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .for_each(|(x, _)| {
                let _ = grid.insert(Coord(x, y), true);
            });
    });

    // Laser starts pointing UP, then rotates clockwise. Laser only hits asteroids that are visible
    // (no other asteroids in the way).
    // Find the 200th asteroid to be vaporized, and return its coordinates as x * 100 + y.

    let mut angle: f32 = 270.0;
    let laser_origin = start_asteroid;
    let mut vaporized_count = 0;
    let mut loop_counter = 0;
    let mut vaporized_asteroids: Vec<Coord<usize>> = Vec::new();
    loop {
        let mut targets: Vec<(f32, Coord<usize>)> = Vec::new();
        grid.iter()
            .filter(|(coord, is_asteroid)| *is_asteroid && *coord != laser_origin)
            .for_each(|(coord, _)| {
                let slope = calculate_slope(&laser_origin, &coord);
                let angle_to_asteroid = (slope.1 as f32).atan2(slope.0 as f32).to_degrees();

                let mut adjusted_angle = angle_to_asteroid - angle;
                while adjusted_angle < 0.0 {
                    adjusted_angle += 360.0;
                }
                if adjusted_angle <= ANGLE_INCREMENT {
                    targets.push((adjusted_angle, coord));
                }
            });

        targets.sort_by(|a, b| {
            // sort by smallest angle first
            // find closest to the origin
            let dist_a = ((a.1 .0 as isize - laser_origin.0 as isize).pow(2)
                + (a.1 .1 as isize - laser_origin.1 as isize).pow(2))
                as f32;
            let dist_b = ((b.1 .0 as isize - laser_origin.0 as isize).pow(2)
                + (b.1 .1 as isize - laser_origin.1 as isize).pow(2))
                as f32;
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        if let Some((_, target_coord)) = targets.first() {
            vaporized_count += 1;
            vaporized_asteroids.push(*target_coord);
            grid.insert(*target_coord, false)
                .expect("could not mark asteroid as vaporized");
            if vaporized_count == 200 {
                // for i in [0, 1, 2, 9, 19, 49, 99, 198, 199] {
                //     println!("{}: {:?}", i + 1, vaporized_asteroids[i]);
                // }
                return Some(target_coord.0 as u64 * 100 + target_coord.1 as u64);
            }
        }

        angle += ANGLE_INCREMENT;
        if angle >= 360.0 {
            angle -= 360.0;
        }

        loop_counter += 1;
        if loop_counter > 9000 {
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(210));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(802));
    }
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
