use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let foods = parse_input("input.txt");
    let allergen_ingredient_map =
        reduce_allergen_ingredient_map(generate_allergen_ingredient_map(&foods));
    let ingredients_with_allergens: HashSet<&String> = allergen_ingredient_map
        .values()
        .flatten()
        .cloned()
        .collect();
    let result_part1 = foods
        .iter()
        .map(|(ingredients, _)| ingredients)
        .flatten()
        .filter(|ingredient| !ingredients_with_allergens.contains(ingredient))
        .count();
    println!("part 1: {}", result_part1);
    let mut allergen_ingredient_pairs: Vec<(&String, &String)> = allergen_ingredient_map
        .iter()
        .map(|(allergen, ingredients)| (*allergen, *ingredients.iter().next().unwrap()))
        .collect();
    allergen_ingredient_pairs.sort();
    let result_part2 = allergen_ingredient_pairs
        .iter()
        .map(|(_, ingredient)| ingredient)
        .fold(String::new(), |acc, ingredient| {
            if acc.len() > 1 {
                acc + "," + ingredient
            } else {
                acc + ingredient
            }
        });
    println!("part 2: {}", result_part2);
}

fn parse_input(path: &str) -> Vec<(Vec<String>, Vec<String>)> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" (contains ").collect();
            (
                parts[0].split(" ").map(|s| s.to_string()).collect(),
                parts[1][0..parts[1].len() - 1]
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect(),
            )
        })
        .collect()
}

fn generate_allergen_ingredient_map(
    foods: &Vec<(Vec<String>, Vec<String>)>,
) -> HashMap<&String, HashSet<&String>> {
    let mut map: HashMap<&String, HashSet<&String>> = HashMap::new();
    for food in foods {
        for allergen in &food.1 {
            match map.entry(allergen) {
                Entry::Occupied(mut o) => {
                    *o.get_mut() = o
                        .get()
                        .intersection(&food.0.iter().collect())
                        .cloned()
                        .collect()
                }
                Entry::Vacant(v) => {
                    v.insert(food.0.iter().collect());
                }
            }
        }
    }
    map
}

fn reduce_allergen_ingredient_map<'a>(
    mut map: HashMap<&'a String, HashSet<&'a String>>,
) -> HashMap<&'a String, HashSet<&'a String>> {
    let mut assigned_ingredients = HashSet::new();
    while map.values().map(|v| v.len()).any(|l| l > 1) {
        for (_, ingredients) in &mut map {
            if ingredients.len() == 1 {
                assigned_ingredients.insert(*ingredients.iter().next().unwrap());
            } else {
                for ingredient in &assigned_ingredients {
                    ingredients.remove(ingredient);
                }
            }
        }
    }
    map
}
