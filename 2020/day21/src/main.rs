use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    // mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    // trh fvjkl sbzzf mxmxvkd (contains dairy)
    static ref RE_LINE: Regex = Regex::new(r"([\w ]+)\(contains ([\w, ]+)\)").unwrap();
}

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: Vec<String>,
}

fn parse_input(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|l| {
            let comps = RE_LINE.captures(l).unwrap();
            let ingredients = comps
                .get(1)
                .unwrap()
                .as_str()
                .split(" ")
                .map(|s| s.trim())
                .map(|s| s.to_string())
                .filter(|s| s.len() > 0)
                .collect();
            let allergens = comps
                .get(2)
                .unwrap()
                .as_str()
                .split(",")
                .map(|s| s.trim())
                .map(|s| s.to_string())
                .filter(|s| s.len() > 0)
                .collect();
            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

fn part_1(input: &str) {
    let foods = parse_input(input);
    let mut known_allergens: HashMap<&String, HashSet<String>> = HashMap::new();

    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            known_allergens.insert(
                allergen,
                match known_allergens.get(allergen) {
                    Some(ingredients) => ingredients
                        .intersection(&food.ingredients)
                        .cloned()
                        .collect(),
                    None => food.ingredients.iter().cloned().collect(),
                },
            );
        }
    }

    let allergenic_ingredients: HashSet<&String> = known_allergens
        .values()
        .flat_map(|ingredients| ingredients)
        .collect();

    println!(
        "{}",
        foods
            .iter()
            .map(|f| {
                f.ingredients
                    .iter()
                    .filter(|&ingredient| !allergenic_ingredients.contains(ingredient))
                    .count()
            })
            .sum::<usize>()
    );
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
}
