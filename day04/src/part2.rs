use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
    RegexError(regex::Error),
    ParseError,
}

fn main() -> Result<(), Error> {
    let rules = rules();
    let input = std::fs::read_to_string("input").map_err(|e| Error::IoError(e))?;
    let count = input
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split(|c| c == ' ' || c == '\n')
                .fold(HashMap::new(), |mut acc, v| {
                    let mut split = v.split(":");
                    acc.insert(
                        split.next().expect("Expected key"),
                        split.next().expect("Expected value"),
                    );
                    acc
                })
        })
        .filter(|passport| {
            vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|k| {
                    if !passport.contains_key(k) {
                        return false;
                    }

                    if let Some(rule) = rules.get(k) {
                        return match rule(passport[k]) {
                            Ok(validation_result) => validation_result,
                            Err(_) => false,
                        };
                    }

                    return false;
                })
        })
        .count();
    println!("{}", count);

    Ok(())
}

type RuleFunction = Box<dyn Fn(&str) -> Result<bool, Error>>;
fn rules() -> HashMap<&'static str, RuleFunction> {
    let mut rules: HashMap<&str, RuleFunction> = HashMap::new();
    rules.insert("byr", number_range_rule(1920, 2002));
    rules.insert("iyr", number_range_rule(2010, 2020));
    rules.insert("eyr", number_range_rule(2020, 2030));
    rules.insert(
        "hgt",
        Box::new(|value| {
            let regex = Regex::new(r"^([0-9]+)(cm|in)$").map_err(|e| Error::RegexError(e))?;
            let captures = regex.captures(value).ok_or(Error::ParseError)?;
            let height =
                u32::from_str_radix(captures.get(1).ok_or(Error::ParseError)?.as_str(), 10)
                    .map_err(|_| Error::ParseError)?;
            let unit = captures.get(2).ok_or(Error::ParseError)?.as_str();

            Ok((unit == "cm" && height >= 150 && height <= 193)
                || (unit == "in" && height >= 59 && height <= 76))
        }),
    );
    rules.insert(
        "hcl",
        Box::new(|value| {
            let regex = Regex::new(r"^#([0-9a-f]{6})$").map_err(|e| Error::RegexError(e))?;
            Ok(regex.is_match(value))
        }),
    );
    rules.insert(
        "ecl",
        Box::new(|value| {
            let valid_color_set: HashSet<&str> =
                vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                    .into_iter()
                    .collect();
            Ok(valid_color_set.contains(&value))
        }),
    );
    rules.insert(
        "pid",
        Box::new(|value| Ok(value.len() == 9 && value.chars().all(char::is_numeric))),
    );

    rules
}

fn number_range_rule(min: u32, max: u32) -> RuleFunction {
    Box::new(move |value| {
        let value = u32::from_str_radix(value, 10).map_err(|_| Error::ParseError)?;
        Ok(value >= min && value <= max)
    })
}
