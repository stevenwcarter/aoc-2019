advent_of_code::solution!(3);

use anyhow::Result;
use atoi_simd::parse;
use terminal_size::{terminal_size, Height, Width};

use std::str::FromStr;

use itertools::Itertools;

use aoc_mine::*;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    distance: u32,
}

impl Instruction {
    pub fn direction(&self) -> Direction {
        self.direction
    }
    pub fn distance(&self) -> u32 {
        self.distance
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_at(1);
        let direction = dir.parse()?;
        let distance = parse(dist.as_bytes()).unwrap();
        Ok(Instruction {
            direction,
            distance,
        })
    }
}

struct Wire<'a> {
    instructions: &'a [Instruction],
    coord: Coord<i32>,
    steps: usize,
}
impl<'a> Wire<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Self {
        Wire {
            instructions,
            coord: Coord::new(0, 0),
            steps: 0,
        }
    }

    pub fn trace(&mut self, grid: &mut HashGrid<i32, (usize, usize)>, first: bool) {
        for instruction in self.instructions {
            for _ in 0..instruction.distance() {
                self.steps += 1;
                self.coord = match instruction.direction() {
                    Up => self.coord.up(None).unwrap(),
                    Down => self.coord.down(None).unwrap(),
                    Left => self.coord.left(None).unwrap(),
                    Right => self.coord.right(None).unwrap(),
                };
                let entry = grid.entry(self.coord).or_default();
                if first && entry.0 == 0 {
                    entry.0 = self.steps;
                } else if !first && entry.1 == 0 {
                    entry.1 = self.steps;
                }
            }
        }
    }

    pub fn reduce_grid_to_term(
        grid: &HashGrid<i32, (usize, usize)>,
        term_width: usize,
        term_height: usize,
    ) -> Vec<Vec<usize>> {
        // If empty, return an empty reduced grid
        if grid.iter().count() == 0 {
            return vec![vec![0; term_width]; term_height];
        }

        // Find original grid bounds
        let main_min_x = grid.iter().map(|(c, _)| c.x()).min().unwrap();
        let main_max_x = grid.iter().map(|(c, _)| c.x()).max().unwrap();
        let main_min_y = grid.iter().map(|(c, _)| c.y()).min().unwrap();
        let main_max_y = grid.iter().map(|(c, _)| c.y()).max().unwrap();

        let spread_x = (main_max_x - main_min_x + 1) as f32;
        let spread_y = (main_max_y - main_min_y + 1) as f32;

        // How many original units map to a single reduced-cell?
        let scale_x = (spread_x / term_width as f32).ceil().max(1.0);
        let scale_y = (spread_y / term_height as f32).ceil().max(1.0);

        let mut reduced = vec![vec![0usize; term_width]; term_height];

        // Populate reduced grid
        for (coord, _) in grid.iter() {
            let original_x = coord.x() as f32 - main_min_x as f32;
            let original_y = coord.y() as f32 - main_min_y as f32;

            let rx = (original_x / scale_x) as usize;
            let ry = (original_y / scale_y) as usize;

            if rx < term_width && ry < term_height {
                reduced[ry][rx] += 1;
            }
        }

        reduced
    }

    pub fn display(&self, grid: &mut HashGrid<i32, (usize, usize)>) {
        if std::env::var("AOC_ANIMATE").is_err() {
            return;
        }
        let (term_width, term_height) = if let Some((Width(w), Height(h))) = terminal_size() {
            (w as usize, h as usize)
        } else {
            (80, 24)
        };
        let grid = Self::reduce_grid_to_term(grid, term_width - 1, term_height - 5);

        let max_value = grid
            .iter()
            .flat_map(|row| row.iter())
            .copied()
            .max()
            .unwrap_or(1);
        let modifier = max_value / 10 + 1;

        print!("{}[2J", 27 as char);
        for y in grid.iter() {
            let mut buffer = String::new();

            for x in y.iter() {
                let display_char = char::from_digit(
                    ((*x as f32 / modifier as f32).ceil() as u32).clamp(0, 10),
                    10,
                );
                match display_char {
                    Some('0') => {
                        buffer.push(' ');
                    }
                    Some('1'..='9') => {
                        buffer.push(display_char.unwrap());
                    }
                    _ => {
                        buffer.push('+');
                    }
                }
            }
            println!("{buffer}");
        }
    }
}

fn parse_wires(input: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    let (first, second) = input.trim().lines().collect_tuple().unwrap();

    let first: Vec<Instruction> = first
        .split(',')
        .map(|s| s.trim())
        .filter_map(|s| s.parse::<Instruction>().ok())
        .collect();
    let second: Vec<Instruction> = second
        .split(',')
        .map(|s| s.trim())
        .filter_map(|s| s.parse::<Instruction>().ok())
        .collect();

    (first, second)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (first, second) = parse_wires(input);
    let mut first = Wire::new(&first);
    let mut second = Wire::new(&second);

    let mut grid: HashGrid<i32, (usize, usize)> = HashGrid::new();

    first.trace(&mut grid, true);
    second.trace(&mut grid, false);
    second.display(&mut grid);

    grid.iter()
        .filter_map(|(coord, (a, b))| {
            if *a > 0 && *b > 0 {
                Some(coord.manhattan())
            } else {
                None
            }
        })
        .min()
        .map(|d| d as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (first, second) = parse_wires(input);
    let mut first = Wire::new(&first);
    let mut second = Wire::new(&second);

    let mut grid: HashGrid<i32, (usize, usize)> = HashGrid::new();

    first.trace(&mut grid, true);
    second.trace(&mut grid, false);

    grid.iter()
        .filter_map(
            |(_, (a, b))| {
                if *a > 0 && *b > 0 {
                    Some(a + b)
                } else {
                    None
                }
            },
        )
        .min()
        .map(|d| d as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(part_one(input), Some(159));
    }

    #[test]
    fn test_one_2() {
        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(part_one(input), Some(135));
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
