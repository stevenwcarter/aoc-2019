advent_of_code::solution!(8);

use anyhow::{anyhow, Result};
use aoc_mine::Coord;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

struct ImageLayers {
    layers: Vec<ImageLayer>,
}

impl ImageLayers {
    fn new() -> Self {
        Self { layers: Vec::new() }
    }

    fn add_pixel(&mut self, pixel: u8) -> Result<()> {
        if self.layers.is_empty() || self.layers.last().unwrap().pixels.len() >= WIDTH * HEIGHT {
            self.layers.push(ImageLayer { pixels: Vec::new() });
        }
        self.layers.last_mut().unwrap().add_pixel(pixel)
    }

    fn find_part_one(&self) -> Option<u64> {
        let mut min_zero_count = usize::MAX;
        let mut result = 0;

        for layer in &self.layers {
            let zero_count = layer.pixels.iter().filter(|&&p| p == 0).count();
            if zero_count < min_zero_count {
                min_zero_count = zero_count;
                let one_count = layer.pixels.iter().filter(|&&p| p == 1).count();
                let two_count = layer.pixels.iter().filter(|&&p| p == 2).count();
                result = (one_count * two_count) as u64;
            }
        }

        Some(result)
    }

    fn find_part_two(&self) -> Option<u64> {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = self.find_color_at_coord(Coord::new(x, y));
                let display_char = match color {
                    1 => "\x1b[47m\x1b[30m \x1b[0m", // White
                    _ => " ",                        // Transparent
                };
                print!("{}", display_char);
            }
            println!();
        }

        Some(0)
    }

    fn find_color_at_coord(&self, coord: Coord<usize>) -> u8 {
        for layer in &self.layers {
            let pixel = layer.pixels[coord.y() * WIDTH + coord.x()];
            if pixel != 2 {
                return pixel;
            }
        }
        2 // Default to transparent if all layers are transparent
    }
}

struct ImageLayer {
    pixels: Vec<u8>,
}
impl ImageLayer {
    fn add_pixel(&mut self, pixel: u8) -> Result<()> {
        if self.pixels.len() >= WIDTH * HEIGHT {
            return Err(anyhow!("Layer is full"));
        }
        self.pixels.push(pixel);
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut image_layers = ImageLayers::new();

    for ch in input.trim().chars() {
        let pixel = ch.to_digit(10)? as u8;
        image_layers.add_pixel(pixel).ok()?;
    }

    image_layers.find_part_one()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut image_layers = ImageLayers::new();

    for ch in input.trim().chars() {
        let pixel = ch.to_digit(10)? as u8;
        image_layers.add_pixel(pixel).ok()?;
    }

    image_layers.find_part_two()
}

#[cfg(test)]
mod tests {

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
