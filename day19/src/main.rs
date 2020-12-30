use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("input2").map_err(|e| Error::InputReadError(e))?;
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    let mut rules = Rules::new();
    while let Some(Ok(rule)) = lines.next() {
        if rule.is_empty() {
            break;
        }
        let rule: Rule = rule.parse()?;
        rules.insert(rule.identifier, rule);
    }

    let regex_string = generate_regex_for_rule(0, 0, &rules);
    println!("{}", generate_regex_for_rule(0, 11, &rules));
    let regex = Regex::new(&*format!("{}{}{}", "^", &regex_string, "$")).unwrap();
    let p1 = lines
        .filter(|str| {
            let str = str.as_ref().unwrap();
            regex.is_match(&str)
        })
        .count();

    println!("{}", p1);
    Ok(())
}

type Rules = HashMap<usize, Rule>;

fn generate_regex_for_rule(n: usize, rule_index: usize, rules: &Rules) -> String {
    let rule = rules.get(&rule_index).unwrap();
    let mut regex_string = String::new();
    for (i, definition) in rule.definitions.iter().enumerate() {
        regex_string += generate_regex_for_definition(n + 1, definition, rules).as_str();
        if i < rule.definitions.len() - 1 {
            regex_string += "|"
        }
    }

    regex_string
}

fn generate_regex_for_definition(n: usize, definition: &RuleDefinition, rules: &Rules) -> String {
    match definition {
        RuleDefinition::TerminalRule(c) => String::from(*c),
        RuleDefinition::RuleSequence(s) => s.iter().fold(String::new(), |acc, i| {
            let mut result = String::new();

            if n < 15 {
                result = format!("{}{}{}", "(", &*generate_regex_for_rule(n, *i, &rules), ")")
            }

            acc + &*result
        }),
    }
}

#[derive(Debug)]
struct Rule {
    identifier: usize,
    definitions: Vec<RuleDefinition>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_string = s.split(":");
        let identifier = usize::from_str(split_string.next().ok_or(Error::RuleIdentifierNotFound)?)
            .map_err(|e| Error::RuleIdentifierParseError(e))?;

        let rule_definitions = split_string
            .next()
            .ok_or(Error::RuleDefinitionNotFound)?
            .trim();
        let split_rule_definitions = rule_definitions.split("|");

        let definitions: Vec<RuleDefinition> = split_rule_definitions
            .filter_map(|definition_str| definition_str.parse().ok())
            .collect();

        Ok(Self {
            identifier,
            definitions,
        })
    }
}

#[derive(Debug)]
enum RuleDefinition {
    RuleSequence(Vec<usize>),
    TerminalRule(char),
}

impl FromStr for RuleDefinition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_rule_definition = s.trim().split(" ");
        let first_fragment = split_rule_definition
            .next()
            .ok_or(Error::RuleDefinitionFragmentNotFound)?;
        if first_fragment.starts_with("\"") {
            Ok(RuleDefinition::TerminalRule(
                first_fragment
                    .chars()
                    .nth(1)
                    .ok_or(Error::TerminalCharNotFound)?,
            ))
        } else {
            let mut fragments = vec![usize::from_str(first_fragment)
                .map_err(|e| Error::RuleDefinitionFragmentParseError(e))?];
            for next_fragment in split_rule_definition {
                fragments.push(
                    usize::from_str(next_fragment)
                        .map_err(|e| Error::RuleDefinitionFragmentParseError(e))?,
                );
            }

            Ok(RuleDefinition::RuleSequence(fragments))
        }
    }
}

#[derive(Debug)]
enum Error {
    InputReadError(std::io::Error),
    RuleIdentifierNotFound,
    RuleIdentifierParseError(std::num::ParseIntError),
    RuleDefinitionNotFound,
    RuleDefinitionFragmentNotFound,
    RuleDefinitionFragmentParseError(std::num::ParseIntError),
    TerminalCharNotFound,
}
