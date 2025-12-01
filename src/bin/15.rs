use std::{env, thread::sleep, time::Duration};

use advent_of_code::intcode::{IntCode, IntCodeBuilder};
use aoc_mine::{Coord, Grid, HashGrid};
use hashbrown::HashSet;
use pathfinding::prelude::bfs;

advent_of_code::solution!(15);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}
impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Wall = 0,
    Empty = 1,
    OxygenSystem = 2,
}

pub struct Room {
    current_position: Coord<i32>,
    grid: HashGrid<i32, TileType>,
    ic: IntCode,
    visited: HashSet<Coord<i32>>,
}

impl Room {
    pub fn new(ic: IntCode) -> Self {
        let mut grid: HashGrid<i32, TileType> = HashGrid::new();
        let _ = grid.insert(Coord::new(0, 0), TileType::Empty);

        Self {
            current_position: Coord::new(0, 0),
            grid,
            ic,
            visited: HashSet::new(),
        }
    }

    pub fn oxygen_position(&self) -> Option<Coord<i32>> {
        for (pos, tile_type) in self.grid.iter() {
            if *tile_type == TileType::OxygenSystem {
                return Some(*pos);
            }
        }
        None
    }

    pub fn try_move(&mut self, direction: Direction) -> TileType {
        let new_position = match direction {
            Direction::North => self.current_position.up(None).unwrap(),
            Direction::South => self.current_position.down(None).unwrap(),
            Direction::West => self.current_position.left(None).unwrap(),
            Direction::East => self.current_position.right(None).unwrap(),
        };
        if let Some(tile_type) = self.grid.get(&new_position) {
            // Already explored
            if tile_type == &TileType::Wall {
                return TileType::Wall;
            }
        }
        self.ic.add_input(direction as i64);
        self.ic.process(true);

        let status = self.ic.output.last().copied().unwrap() as u8;

        let tile_type = match status {
            0 => TileType::Wall,
            1 => TileType::Empty,
            2 => TileType::OxygenSystem,
            _ => panic!("Unknown status code from IntCode output: {}", status),
        };

        let _ = self.grid.insert(new_position, tile_type);

        if tile_type != TileType::Wall {
            self.current_position = new_position;
        }

        self.display();
        // let mut buffer = [0u8; 1];
        // std::io::stdin().read_exact(&mut buffer).unwrap();
        tile_type
    }

    fn explore(&mut self) {
        self.visited.insert(self.current_position);

        for dir in [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            let next_pos = match dir {
                Direction::North => self.current_position.up(None).unwrap(),
                Direction::South => self.current_position.down(None).unwrap(),
                Direction::West => self.current_position.left(None).unwrap(),
                Direction::East => self.current_position.right(None).unwrap(),
            };

            // Optimization: Don't move if we already know what's there
            if self.grid.contains_key(&next_pos) {
                continue;
            }

            // 1. Try to move physically
            let tile_type = self.try_move(dir);

            let _ = self.grid.insert(next_pos, tile_type);

            // 3. If we moved, we must recurse, then backtrack
            if tile_type != TileType::Wall {
                self.current_position = next_pos;
                self.explore();

                // CRITICAL: Move back to where we were to restore state
                // for the next loop iteration
                self.try_move(dir.opposite());
            }
        }
    }

    pub fn display(&self) {
        if env::var("AOC_ANIMATE").is_err() {
            return;
        }
        sleep(Duration::from_millis(6));
        print!("{}[2J", 27 as char);
        let min_x = self.grid.iter().map(|(c, _)| c.0).min().unwrap_or(0);
        let max_x = self.grid.iter().map(|(c, _)| c.0).max().unwrap_or(0);
        let min_y = self.grid.iter().map(|(c, _)| c.1).min().unwrap_or(0);
        let max_y = self.grid.iter().map(|(c, _)| c.1).max().unwrap_or(0);

        for y in min_y..=max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                let is_current_pos = Coord(x, y) == self.current_position;
                let display_char = match (self.grid.get(&Coord(x, y)), is_current_pos) {
                    (Some(TileType::Wall), _) => '󰟾',
                    (Some(TileType::Empty), false) => '.',
                    (Some(TileType::OxygenSystem), _) => '󰙇',
                    (_, true) => '',
                    (None, _) => ' ',
                };
                line.push(display_char);
            }
            println!("{line}");
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ic: IntCode = IntCodeBuilder::default().build(input);

    let mut room = Room::new(ic);

    room.explore();
    let oxygen_pos = room.oxygen_position().expect("Oxygen not found!");
    let start = Coord::new(0, 0);

    let path = bfs(
        &start,
        |&pos| {
            // Generate neighbors based on the known map data
            let mut successors = Vec::new();
            for dir in [
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ] {
                let next = match dir {
                    Direction::North => pos.up(None).unwrap(),
                    Direction::South => pos.down(None).unwrap(),
                    Direction::West => pos.left(None).unwrap(),
                    Direction::East => pos.right(None).unwrap(),
                };
                if let Some(tile) = room.grid.get(&next) {
                    if *tile != TileType::Wall {
                        successors.push(next);
                    }
                }
            }
            successors
        },
        |&pos| pos == oxygen_pos,
    );

    path.expect("No path found")
        .len()
        .checked_sub(1)
        .map(|v| v as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    // use super::*;

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
