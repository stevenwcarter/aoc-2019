use advent_of_code::intcode::IntCodeBuilder;
use aoc_mine::{Coord, Grid, HashGrid};

advent_of_code::solution!(11);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }
}

use Direction::*;

pub fn part_one(input: &str) -> Option<usize> {
    let mut ic = IntCodeBuilder::default().build(input);

    let mut grid: HashGrid<isize, u8> = HashGrid::new();
    let mut position: Coord<isize> = Coord(0, 0);
    let mut direction = Up;

    let mut painted_grid: HashGrid<isize, bool> = HashGrid::new();

    loop {
        if ic.is_quit() {
            break;
        }
        let color_code = grid.get(&position).unwrap_or(&0);
        ic.add_input(*color_code as i64);

        ic.process(true);
        if ic.is_quit() {
            break;
        }
        let new_color = ic.get_last_output();
        grid.insert(position, new_color as u8)
            .expect("could not insert");
        let _ = painted_grid.insert(position, true);
        ic.process(true);
        if ic.is_quit() {
            break;
        }
        match ic.get_last_output() {
            0 => direction = direction.turn_left(),
            _ => direction = direction.turn_right(),
        };
        match direction {
            Up => position = position.up(None).unwrap(),
            Down => position = position.down(None).unwrap(),
            Right => position = position.right(None).unwrap(),
            Left => position = position.left(None).unwrap(),
        };
    }

    Some(painted_grid.iter().filter(|(_, v)| **v).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut ic = IntCodeBuilder::default().build(input);

    let mut grid: HashGrid<isize, u8> = HashGrid::new();
    let mut position: Coord<isize> = Coord(0, 0);
    let mut direction = Up;

    let _ = grid.insert(position, 1);

    loop {
        if ic.is_quit() {
            break;
        }
        let color_code = grid.get(&position).unwrap_or(&0);
        ic.add_input(*color_code as i64);

        ic.process(true);
        if ic.is_quit() {
            break;
        }
        let new_color = ic.get_last_output();
        grid.insert(position, new_color as u8)
            .expect("could not insert");
        ic.process(true);
        if ic.is_quit() {
            break;
        }
        match ic.get_last_output() {
            0 => direction = direction.turn_left(),
            _ => direction = direction.turn_right(),
        };
        match direction {
            Up => position = position.up(None).unwrap(),
            Down => position = position.down(None).unwrap(),
            Right => position = position.right(None).unwrap(),
            Left => position = position.left(None).unwrap(),
        };
    }

    display_grid(&grid);
    Some(0)
}

fn display_grid(grid: &HashGrid<isize, u8>) {
    let left_bound = grid
        .iter()
        .filter_map(|(c, v)| if *v == 1 { Some(c.x()) } else { None })
        .min()
        .expect("Could not find left bound");
    let right_bound = grid
        .iter()
        .filter_map(|(c, v)| if *v == 1 { Some(c.x()) } else { None })
        .max()
        .expect("Could not find right bound");
    let top_bound = grid
        .iter()
        .filter_map(|(c, v)| if *v == 1 { Some(c.y()) } else { None })
        .min()
        .expect("Could not find top bound");
    let bottom_bound = grid
        .iter()
        .filter_map(|(c, v)| if *v == 1 { Some(c.y()) } else { None })
        .max()
        .expect("Could not find bottom bound");

    for y in top_bound..bottom_bound + 1 {
        for x in left_bound..right_bound + 1 {
            match grid.get(&Coord(x, y)) {
                Some(1) => print!("\x1b[47m\x1b[30m \x1b[0m"),
                _ => print!(" "),
            };
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
