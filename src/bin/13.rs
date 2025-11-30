use std::{str::FromStr, thread::sleep, time::Duration};

use advent_of_code::intcode::IntCodeBuilder;
use aoc_mine::{Coord, Grid, HashGrid};

advent_of_code::solution!(13);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum JoystickPositions {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TileType {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}
impl TileType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TileType::Empty),
            1 => Some(TileType::Wall),
            2 => Some(TileType::Block),
            3 => Some(TileType::Paddle),
            4 => Some(TileType::Ball),
            _ => None,
        }
    }
}
impl FromStr for TileType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<u8>().map_err(|_| ())?;
        TileType::from_u8(value).ok_or(())
    }
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    pub x: i64,
    pub y: i64,
    pub tile_type: TileType,
}

struct Game {
    paddle_position: Option<Coord<i32>>,
    ball_position: Option<Coord<i32>>,
    tiles: HashGrid<i32, TileType>,
}
impl Game {
    pub fn new() -> Self {
        Self {
            paddle_position: None,
            ball_position: None,
            tiles: HashGrid::new(),
        }
    }

    pub fn draw(&self) {
        print!("{}[2J", 27 as char);
        let min_x = self.tiles.iter().map(|(c, _)| c.0).min().unwrap_or(0);
        let max_x = self.tiles.iter().map(|(c, _)| c.0).max().unwrap_or(0);
        let min_y = self.tiles.iter().map(|(c, _)| c.1).min().unwrap_or(0);
        let max_y = self.tiles.iter().map(|(c, _)| c.1).max().unwrap_or(0);

        for y in min_y..=max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                let display_char = match self.tiles.get(&Coord(x, y)) {
                    Some(TileType::Empty) | None => " ",
                    Some(TileType::Wall) => "",
                    Some(TileType::Block) => "",
                    Some(TileType::Paddle) => "",
                    Some(TileType::Ball) => "󰔇",
                };
                line.push_str(display_char);
            }
            println!("{line}");
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        if tile.tile_type == TileType::Paddle {
            if let Some(paddle_pos) = self.paddle_position {
                // Remove old paddle
                *self.tiles.entry(paddle_pos).or_insert(TileType::Empty) = TileType::Empty;
            }
            self.paddle_position = Some(Coord(tile.x as i32, tile.y as i32));
        } else if tile.tile_type == TileType::Ball {
            if let Some(ball_pos) = self.ball_position {
                // Remove old ball
                *self.tiles.entry(ball_pos).or_insert(TileType::Empty) = TileType::Empty;
            }
            self.ball_position = Some(Coord(tile.x as i32, tile.y as i32));
        }
        *self
            .tiles
            .entry(Coord(tile.x as i32, tile.y as i32))
            .or_insert(TileType::Empty) = tile.tile_type;
        if tile.tile_type == TileType::Ball || tile.tile_type == TileType::Paddle {
            self.draw();
            sleep(Duration::from_millis(7));
        }
    }

    pub fn count_blocks(&self) -> usize {
        self.tiles
            .iter()
            .filter(|(_, tile_type)| **tile_type == TileType::Block)
            .count()
    }

    pub fn paddle_position(&self) -> Option<Coord<i32>> {
        self.paddle_position
    }

    pub fn joystick_input(&mut self) -> i64 {
        if let Some(ball_pos) = self.ball_position {
            if let Some(paddle_pos) = self.paddle_position() {
                if ball_pos.0 < paddle_pos.0 {
                    JoystickPositions::Left as i64
                } else if ball_pos.0 > paddle_pos.0 {
                    JoystickPositions::Right as i64
                } else {
                    JoystickPositions::Neutral as i64
                }
            } else {
                JoystickPositions::Neutral as i64
            }
        } else {
            JoystickPositions::Neutral as i64
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut ic = IntCodeBuilder::default().build(input);

    let mut game = Game::new();

    while !ic.is_quit() {
        ic.process(true);
        ic.process(true);
        ic.process(true);

        if let [x, y, type_id] = ic.output[ic.output.len() - 3..] {
            let tile_type = TileType::from_u8(type_id as u8).unwrap();
            let tile = Tile { x, y, tile_type };
            game.add_tile(tile);
        } else {
            panic!("Unexpected output length");
        }
    }

    Some(game.count_blocks())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut ic = IntCodeBuilder::default().build(input);

    ic.data.insert(0, 2);

    ic.add_input(JoystickPositions::Neutral as i64);

    let mut game = Game::new();

    let mut score = 0;

    while !ic.is_quit() {
        let mut outputs = Vec::new();
        while outputs.len() < 3 {
            if ic.waiting_for_input.is_some() {
                ic.add_input(game.joystick_input());
            }
            ic.process(true);
            if ic.waiting_for_input.is_none() {
                outputs.push(ic.output[ic.output.len() - 1]);
            }
        }

        if let [x, y, type_id] = ic.output[ic.output.len() - 3..] {
            if x == -1 && y == 0 {
                // score update, ignore for now
                score = type_id;
            } else {
                let tile_type = TileType::from_u8(type_id as u8).unwrap();
                let tile = Tile { x, y, tile_type };
                game.add_tile(tile);
            }
        } else {
            panic!("Unexpected output length");
        }
    }

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

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
