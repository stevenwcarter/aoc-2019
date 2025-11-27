use std::str::FromStr;

advent_of_code::solution!(4);

struct Password(usize);

impl FromStr for Password {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: usize = s.parse().map_err(|_| ())?;
        if !(100000..=999999).contains(&num) {
            return Err(());
        }
        Ok(Password(num))
    }
}

impl AsRef<usize> for Password {
    fn as_ref(&self) -> &usize {
        &self.0
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
    fn digits(&self) -> Vec<u8> {
        self.0
            .to_string()
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect()
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
    let start: usize = range.0.parse().ok()?;
    let end: usize = range.1.parse().ok()?;
    Some(
        (start..=end)
            .filter_map(|n| n.to_string().parse::<Password>().ok())
            .filter(|p| p.is_valid(start, end))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let range = input.trim().split_once('-')?;
    let start: usize = range.0.parse().ok()?;
    let end: usize = range.1.parse().ok()?;
    Some(
        (start..=end)
            .filter_map(|n| n.to_string().parse::<Password>().ok())
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
}
