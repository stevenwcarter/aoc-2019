advent_of_code::solution!(14);

#[derive(Debug)]
pub struct Recipes<'a> {
    pub recipes: Vec<Recipe<'a>>,
}

impl<'a> Recipes<'a> {
    pub fn parse(input: &'a str) -> Option<Recipes<'a>> {
        let recipes: Vec<Recipe> = input
            .lines()
            .map(|line| Recipe::parse(line).expect("could not parse recipe"))
            .collect();
        Some(Recipes { recipes })
    }
    pub fn find_ingredients_for_output(&'a self, output_name: &str) -> Option<&'a Recipe<'a>> {
        self.recipes.iter().find(|r| r.output == output_name)
    }
    pub fn find_ore_needed_for_output(
        &'a self,
        output_name: &'a str,
        quantity: usize,
        remainders: &mut Vec<(&'a str, usize)>,
    ) -> Option<usize> {
        if output_name == "ORE" {
            return Some(quantity);
        }
        let recipe = self.find_ingredients_for_output(output_name)?;
        let times = quantity.div_ceil(recipe.output_quantity);
        if recipe.output_quantity * times > quantity {
            let remainder = recipe.output_quantity * times - quantity;
            if let Some((_, rem_quantity)) =
                remainders.iter_mut().find(|(name, _)| *name == output_name)
            {
                *rem_quantity += remainder;
            } else {
                remainders.push((output_name, remainder));
            }
        }
        let mut total_ore = 0;
        for (ingredient_name, ingredient_quantity) in &recipe.ingredients {
            if let Some((_, rem_quantity)) = remainders
                .iter_mut()
                .find(|(name, _)| name == ingredient_name)
            {
                if *rem_quantity >= ingredient_quantity * times {
                    *rem_quantity -= ingredient_quantity * times;
                    continue;
                } else {
                    let needed = ingredient_quantity * times - *rem_quantity;
                    *rem_quantity = 0;
                    let ore_needed =
                        self.find_ore_needed_for_output(ingredient_name, needed, remainders)?;
                    total_ore += ore_needed;
                    continue;
                }
            } else {
                let ore_needed = self.find_ore_needed_for_output(
                    ingredient_name,
                    ingredient_quantity * times,
                    remainders,
                )?;
                total_ore += ore_needed;
            }
        }
        Some(total_ore)
    }
}

#[derive(Debug)]
pub struct Recipe<'a> {
    output: &'a str,
    output_quantity: usize,
    ingredients: Vec<(&'a str, usize)>,
}

impl<'a> Recipe<'a> {
    pub fn new(
        output: &'a str,
        output_quantity: usize,
        ingredients: Vec<(&'a str, usize)>,
    ) -> Self {
        Self {
            output,
            output_quantity,
            ingredients,
        }
    }
    pub fn parse(line: &'a str) -> Option<Self> {
        let (ingredients_part, output_part) = line.split_once("=>")?;
        let output_part = output_part.trim();
        let output_parts: Vec<&str> = output_part.split_whitespace().collect();
        if output_parts.len() != 2 {
            return None;
        }
        let output_quantity = output_parts[0].parse().ok()?;
        let output_name = output_parts[1];

        let ingredients: Vec<(&'a str, usize)> = ingredients_part
            .split(',')
            .map(|part| {
                let part = part.trim();
                let parts: Vec<&str> = part.split_whitespace().collect();
                if parts.len() != 2 {
                    return None;
                }
                let quantity: usize = parts[0].parse().ok()?;
                let name = parts[1];
                Some((name, quantity))
            })
            .collect::<Option<Vec<(&'a str, usize)>>>()
            .expect("could not parse ingredients");

        Some(Recipe::new(output_name, output_quantity, ingredients))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let recipes = Recipes::parse(input).expect("could not parse recipes");

    recipes.find_ore_needed_for_output("FUEL", 1, &mut Vec::new())
}

pub fn part_two(input: &str) -> Option<usize> {
    let recipes = Recipes::parse(input).expect("could not parse recipes");
    let mut lower_bound: usize = 1;
    let mut upper_bound: usize = 1_000_000_000_000;

    // binary search
    while lower_bound < upper_bound {
        let mid = (lower_bound + upper_bound).div_ceil(2);
        let ore_needed = recipes
            .find_ore_needed_for_output("FUEL", mid, &mut Vec::new())
            .unwrap();
        if ore_needed > 1_000_000_000_000 {
            upper_bound = mid - 1;
        } else {
            lower_bound = mid;
        }
    }

    Some(lower_bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(165));
    }
    #[test]
    fn test_part_one_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(13312));
    }
    #[test]
    fn test_part_one_4() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(180697));
    }
    #[test]
    fn test_part_one_5() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(2210736));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(82892753));
    }
    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(5586022));
    }
    #[test]
    fn test_part_two_5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(460664));
    }
}
