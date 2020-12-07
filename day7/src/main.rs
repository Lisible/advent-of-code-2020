use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::{FromStr, Split};

fn main() -> Result<(), Error> {
    let file = File::open("input2").map_err(|_| Error::InputFileNotFound)?;
    let rules = BufReader::new(file)
        .lines()
        .try_fold(HashMap::new(), |mut acc, v| {
            let rule = parse_rule(&v.map_err(|_| Error::InputReadError)?)?;
            acc.insert(rule.color.clone(), rule);
            Ok(acc)
        })?;
    let bags_containing_shiny_gold = rules
        .iter()
        .filter(|r| can_contain_bag("shiny gold", r.1, &rules))
        .count();

    let bag_count_in_shiny_gold = required_bag_count("shiny gold", &rules);

    println!("{}", bags_containing_shiny_gold);
    println!("{}", bag_count_in_shiny_gold);
    Ok(())
}

fn required_bag_count(color: &str, rules: &HashMap<String, Rule>) -> usize {
    count(color, rules) - 1
}

fn count(color: &str, rules: &HashMap<String, Rule>) -> usize {
    let bag = rules.get(color).unwrap();
    let mut bags = 1;
    for (c, color) in bag.valid_content.iter() {
        bags += *c as usize * count(color, rules);
    }

    bags
}

fn can_contain_bag(color: &str, rule: &Rule, rules: &HashMap<String, Rule>) -> bool {
    let result = rule.valid_content.iter().any(|c| c.1 == color);
    result
        || rule
            .valid_content
            .iter()
            .any(|c| can_contain_bag(color, rules.get(&c.1).unwrap(), rules))
}

fn parse_rule(rule_string: &str) -> Result<Rule, Error> {
    let mut split_rule_string = rule_string.split(' ').peekable();
    let color = parse_color(&mut split_rule_string)?;
    eat_word("bags", &mut split_rule_string)?;
    eat_word("contain", &mut split_rule_string)?;

    let mut valid_content = vec![];
    loop {
        let quantity = parse_quantity(&mut split_rule_string)?;
        if quantity == 0 && split_rule_string.peek() == Some(&"other") {
            break;
        }

        let color = parse_color(&mut split_rule_string)?;
        valid_content.push((quantity, color));
        match split_rule_string.peek() {
            Some(&"bag.") | Some(&"bags.") => break,
            _ => eat_any(&["bag,", "bags,"], &mut split_rule_string)?,
        }
    }

    Ok(Rule {
        color,
        valid_content,
    })
}

fn parse_color(split_rule_string: &mut Peekable<Split<char>>) -> Result<String, Error> {
    let adjective = split_rule_string
        .next()
        .ok_or(Error::ColorAdjectiveNotFound)?;
    let color = split_rule_string.next().ok_or(Error::ColorNotFound)?;

    Ok(adjective.to_string() + " " + color)
}

fn parse_quantity(split_rule_string: &mut Peekable<Split<char>>) -> Result<u32, Error> {
    let quantity = split_rule_string.next().ok_or(Error::QuantityNotFound)?;
    if quantity == "no" {
        Ok(0)
    } else {
        u32::from_str(quantity).map_err(|e| Error::QuantityParseError(e))
    }
}

fn eat_word(
    expected_word: &str,
    split_rule_string: &mut Peekable<Split<char>>,
) -> Result<(), Error> {
    let word = split_rule_string
        .next()
        .ok_or(Error::UnexpectedEndOfString)?;
    if word == expected_word {
        Ok(())
    } else {
        Err(Error::UnexpectedWord(word.into()))
    }
}

fn eat_any(
    expected_words: &[&str],
    split_rule_string: &mut Peekable<Split<char>>,
) -> Result<(), Error> {
    let word = split_rule_string
        .next()
        .ok_or(Error::UnexpectedEndOfString)?;
    if expected_words.contains(&word) {
        Ok(())
    } else {
        Err(Error::UnexpectedWord(word.into()))
    }
}

#[derive(Debug)]
struct Rule {
    color: String,
    valid_content: Vec<(u32, String)>,
}

#[derive(Debug)]
enum Error {
    InputFileNotFound,
    InputReadError,
    ColorAdjectiveNotFound,
    ColorNotFound,
    QuantityNotFound,
    QuantityParseError(ParseIntError),
    UnexpectedEndOfString,
    UnexpectedWord(String),
}
