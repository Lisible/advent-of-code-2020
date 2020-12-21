use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input_file = File::open("input").map_err(|e| Error::InputFileOpenError(e))?;
    let reader = BufReader::new(input_file);
    let foods: Vec<Food> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse().ok())
        .collect();

    let all_ingredients = foods.iter().fold(Ingredients::new(), |acc, f| {
        acc.union(&f.ingredients).cloned().collect()
    });
    let all_allergens = foods.iter().fold(Allergens::new(), |acc, f| {
        acc.union(&f.allergens).cloned().collect()
    });

    let mut allergens_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for allergen in &all_allergens {
        let mut foods_containing_allergens =
            foods.iter().filter(|f| f.allergens.contains(allergen));
        let mut ingredients = foods_containing_allergens
            .next()
            .unwrap()
            .ingredients
            .clone();
        for f in foods_containing_allergens {
            ingredients = ingredients.intersection(&f.ingredients).cloned().collect();
        }

        allergens_ingredients.insert(allergen.clone(), ingredients);
    }

    while allergens_ingredients
        .iter()
        .any(|(_, ingredients)| ingredients.len() != 1)
    {
        let mut unique_ingredients: Vec<String> = allergens_ingredients
            .iter()
            .filter(|(_, ingredients)| ingredients.len() == 1)
            .map(|f| f.1.iter().next().unwrap())
            .cloned()
            .collect();

        for (_, is) in allergens_ingredients.iter_mut() {
            for unique_ingredient in &unique_ingredients {
                if is.len() > 1 && is.contains(&*unique_ingredient) {
                    is.remove(unique_ingredient);
                }
            }
        }
    }

    let non_allergen_ingredients: Ingredients = all_ingredients
        .difference(
            &allergens_ingredients
                .iter()
                .fold(HashSet::new(), |mut acc, v| {
                    acc.insert(v.1.iter().next().unwrap().clone());
                    acc
                }),
        )
        .cloned()
        .collect();
    let count = foods.iter().fold(0, |acc, f| {
        acc + non_allergen_ingredients
            .iter()
            .fold(0, |acc, i| acc + f.ingredients.contains(i) as i32)
    });
    println!("count: {}", count);

    let mut sorted_allergens_ingredient: Vec<_> = allergens_ingredients.into_iter().collect();
    sorted_allergens_ingredient.sort_by(|a, b| a.0.cmp(&b.0));

    println!(
        "sorted: {}",
        sorted_allergens_ingredient
            .iter()
            .map(|a| a.1.iter().next().unwrap().clone())
            .collect::<Vec<String>>()
            .join(",")
    );

    Ok(())
}

type Ingredients = HashSet<String>;
type Allergens = HashSet<String>;
#[derive(Debug)]
struct Food {
    ingredients: Ingredients,
    allergens: Allergens,
}

impl FromStr for Food {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_str = s.split(" ");
        let mut ingredients = Ingredients::new();
        let mut allergens: Allergens = Allergens::new();
        while let Some(str) = split_str.next() {
            if str.starts_with("(contains") {
                while let Some(str) = split_str.next() {
                    allergens.insert(str[0..str.len() - 1].to_owned());
                }
            } else {
                ingredients.insert(str.into());
            }
        }

        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

#[derive(Debug)]
enum Error {
    InputFileOpenError(std::io::Error),
}
