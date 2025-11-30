use std::str::FromStr;

use atoi_simd::parse;

advent_of_code::solution!(4);

struct Password {
    number: usize,
    digits: [u8; 6],
}

impl FromStr for Password {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len != 6 {
            return Err(());
        }
        let num: usize = parse(s.as_bytes()).map_err(|_| ())?;
        if !(100000..=999999).contains(&num) {
            return Err(());
        }
        Ok(Password {
            number: num,
            digits: {
                let bytes = s.as_bytes();
                [
                    bytes[0] - b'0',
                    bytes[1] - b'0',
                    bytes[2] - b'0',
                    bytes[3] - b'0',
                    bytes[4] - b'0',
                    bytes[5] - b'0',
                ]
            },
        })
    }
}

fn extract_digits(value: usize) -> [u8; 6] {
    let mut value = value;
    let mut container = [0u8; 6];

    (0..6).rev().for_each(|i| {
        container[i] = (value % 10) as u8;
        value /= 10;
    });

    container
}

impl TryFrom<usize> for Password {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if !(100000..1000000).contains(&value) {
            return Err(());
        }

        let digits = extract_digits(value);

        Ok(Password {
            number: value,
            digits,
        })
    }
}

impl AsRef<usize> for Password {
    fn as_ref(&self) -> &usize {
        &self.number
    }
}

impl PartialEq<usize> for Password {
    fn eq(&self, other: &usize) -> bool {
        *self.as_ref() == *other
    }
}

impl PartialOrd<usize> for Password {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other)
    }
}

impl Password {
    fn digits(&self) -> [u8; 6] {
        self.digits
    }
    fn has_adjacent(&self) -> bool {
        self.digits().windows(2).any(|w| w[0] == w[1])
    }
    fn has_adjacent_strict(&self) -> bool {
        let digits = self.digits();
        let mut counts = [0; 10];
        let mut current_digit = digits[0];
        let mut current_count = 1;

        for &digit in &digits[1..] {
            if digit == current_digit {
                current_count += 1;
            } else {
                counts[current_digit as usize] = counts[current_digit as usize].max(current_count);
                current_digit = digit;
                current_count = 1;
            }
        }
        counts[current_digit as usize] = counts[current_digit as usize].max(current_count);

        counts.contains(&2)
    }
    fn never_decreases(&self) -> bool {
        self.digits().windows(2).all(|w| w[0] <= w[1])
    }
    fn within_range(&self, start: usize, end: usize) -> bool {
        *self >= start && *self <= end
    }
    fn is_valid(&self, start: usize, end: usize) -> bool {
        self.within_range(start, end) && self.has_adjacent() && self.never_decreases()
    }
    fn is_valid_strict(&self, start: usize, end: usize) -> bool {
        self.within_range(start, end) && self.has_adjacent_strict() && self.never_decreases()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let range = input.trim().split_once('-')?;
    let start: usize = parse(range.0.as_bytes()).ok()?;
    let end: usize = parse(range.1.as_bytes()).ok()?;
    Some(
        (start..=end)
            .filter_map(|n| Password::try_from(n).ok())
            .filter(|p| p.is_valid(start, end))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let range = input.trim().split_once('-')?;
    let start: usize = parse(range.0.as_bytes()).ok()?;
    let end: usize = parse(range.1.as_bytes()).ok()?;
    Some(
        (start..=end)
            .filter_map(|n| Password::try_from(n).ok())
            .filter(|p| p.is_valid_strict(start, end))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_digits() {
        assert_eq!(extract_digits(123456), [1, 2, 3, 4, 5, 6]);
    }
}
