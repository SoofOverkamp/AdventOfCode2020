use std::collections::{HashMap, HashSet};

use aoc2020_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day21";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_ingredients: HashMap<&str, u32> = HashMap::new();

    for line in input.iter() {
        let mut split = line.split(" (");
        let ingredients: HashSet<&str> = split.next().ok_or(format!("Expected <ingredient [more]> (contains <allergens, [more]>), got {}", line))?
            .split(" ")
            .collect();
        let allergens: Vec<&str> = split.next()
            .and_then(|s| s.strip_prefix("contains "))
            .and_then(|s| s.strip_suffix(")"))
            .ok_or(format!("Expected <ingredient [more]> (contains <allergens, [more]>), got {}", line))?
            .split(", ")
            .collect();

        for allergen in allergens {
            let ingredients = map.get(allergen)
                .map(|ing| ing.intersection(&ingredients)
                    .map(|s| *s)
                    .collect::<HashSet<&str>>())
                .unwrap_or(ingredients.clone());

            map.insert(allergen, ingredients);
        };

        for ingredient in ingredients {
            all_ingredients.insert(ingredient, all_ingredients.get(ingredient).unwrap_or(&0) + 1);
        }
    }

    flatten(&mut map, None);

    let mut dangerous_ingredients = Vec::new();

    for (allergen, ingredients) in map.iter() {
        for ingredient in ingredients {
            all_ingredients.remove(ingredient);
        }

        if ingredients.len() == 1 {
            dangerous_ingredients.push((*allergen, *ingredients.iter().next().unwrap()));
        }
    }
    dangerous_ingredients.sort();
    let mut dangerous_ingredients: String = dangerous_ingredients.into_iter().map(|e| format!("{},", e.1))
        .collect();
    dangerous_ingredients.pop();

    Ok([Some(all_ingredients.values().sum::<u32>().to_string()), Some(dangerous_ingredients)])
}

fn flatten(map: &mut HashMap<&str, HashSet<&str>>, to_remove: Option<&str>) {
    let mut new_singles = Vec::new();

    for (allergen, ingredients) in map.iter_mut() {
        if to_remove.map(|e| ingredients.remove(e)).unwrap_or(true) {
            if ingredients.len() == 1 {
                new_singles.push(*allergen);
            }
        }
    }

    for single in new_singles {
        let ingredients = map.remove(single).unwrap();
        flatten(map, Some(*ingredients.iter().next().unwrap()));
        map.insert(single, ingredients);
    };
}

#[test]
pub fn test_day21() {
    assert!(common::run_test(DAY, &run))
}