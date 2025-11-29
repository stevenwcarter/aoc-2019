use anyhow::{ensure, Context};
use aoc_mine::{Coord, Grid, LinearGrid};

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Target {
    angle: f32,
    dist: usize,
    coord: Coord<usize>,
    // We will calculate this
    lap: usize,
}

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

#[inline]
fn calculate_angle(c1: Coord<usize>, c2: Coord<usize>) -> f32 {
    // 1. Calculate delta (still need isize for direction)
    let dy = c2.1 as isize - c1.1 as isize;
    let dx = c2.0 as isize - c1.0 as isize;

    // 2. Cast to f32 and call atan2
    // Result is in Radians: -3.14 to +3.14
    (dy as f32).atan2(dx as f32).to_degrees()
}

fn calculate_slope(coord1: Coord<usize>, coord2: Coord<usize>) -> (isize, isize) {
    let dy = coord2.1 as isize - coord1.1 as isize;
    let dx = coord2.0 as isize - coord1.0 as isize;

    let gcd = gcd(dx.unsigned_abs(), dy.unsigned_abs());

    if gcd == 0 {
        return (0, 0);
    }

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
            let mut slopes: Vec<f32> = Vec::new();
            grid.iter()
                .filter(|(_, is_asteroid)| *is_asteroid)
                .filter(|(other_coord, _)| other_coord != &coord)
                .for_each(|(other_coord, _)| {
                    let slope = calculate_angle(coord, other_coord);
                    if !slopes.contains(&slope) {
                        slopes.push(slope);
                    }
                });
            slopes.len()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    #[cfg(test)]
    let start_asteroid = Coord(11, 13);
    #[cfg(not(test))]
    let start_asteroid = Coord(28, 29);

    let mut angles_and_distances: Vec<(f32, usize, Coord<usize>)> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, c)| c == '#')
            .for_each(|(x, _)| {
                if Coord(x, y) == start_asteroid {
                    return;
                }
                let mut angle_to_asteroid = calculate_angle(start_asteroid, Coord(x, y));
                angle_to_asteroid -= 270.0;
                while angle_to_asteroid < 0.0 {
                    angle_to_asteroid += 360.0;
                }
                let distance_to_target = ((x as isize - start_asteroid.0 as isize).pow(2)
                    + (y as isize - start_asteroid.1 as isize).pow(2))
                    as usize;
                angles_and_distances.push((angle_to_asteroid, distance_to_target, Coord(x, y)));
            });
    });

    let coord = get_200th_target(angles_and_distances).expect("could not find target");
    Some((coord.2 .0 as u64) * 100 + (coord.2 .1 as u64))
}

fn get_200th_target(
    raw_data: Vec<(f32, usize, Coord<usize>)>,
) -> Option<(f32, usize, Coord<usize>)> {
    assert!(raw_data.len() >= 200,);
    // 1. Convert to struct for easier handling
    let mut targets: Vec<Target> = raw_data
        .into_iter()
        .map(|(angle, dist, coord)| Target {
            angle,
            dist,
            coord,
            lap: 0,
        })
        .collect();

    // 2. Primary Sort: Group by Angle, then by Distance
    // We use total_cmp because f32 is not Ord by default
    targets.sort_unstable_by(|a, b| {
        a.angle
            .total_cmp(&b.angle)
            .then_with(|| a.dist.cmp(&b.dist))
    });

    // 3. Assign "Lap" indices
    // We iterate through. If the angle is the same as the previous,
    // we increment the lap count. If it's a new angle, reset to 0.
    let mut current_lap = 0;

    // We can't compare the first element to a 'previous' one easily in a loop,
    // so we handle the logic by peeking or tracking state.
    for i in 1..targets.len() {
        // Check if current angle is "identical" to previous
        // Note: With f32, exact equality (==) is risky, but your prompt
        // implies discrete "identical" angles. If these are calculated values,
        // consider using an epsilon tolerance here.
        if targets[i].angle == targets[i - 1].angle {
            current_lap += 1;
        } else {
            current_lap = 0;
        }
        targets[i].lap = current_lap;
    }

    // 4. Find the 200th element
    // We want the order: Lap (Ascending), then Angle (Ascending)
    // select_nth_unstable is O(N) on average, faster than full sort.
    let (_, target, _) = targets.select_nth_unstable_by(199, |a, b| {
        a.lap.cmp(&b.lap).then_with(|| a.angle.total_cmp(&b.angle))
    });

    Some((target.angle, target.dist, target.coord))
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
