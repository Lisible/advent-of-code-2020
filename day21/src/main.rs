use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input_file = File::open("example_input").map_err(|e| Error::InputFileOpenError(e))?;
    let reader = BufReader::new(input_file);
    let foods: Vec<Food> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse().ok())
        .collect();

    dbg!(foods);

    Ok(())
}

type Ingredient = String;
type Allergen = String;
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
