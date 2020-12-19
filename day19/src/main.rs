use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("example_input").map_err(|e| Error::InputReadError(e))?;
    let buf_reader = BufReader::new(file);
    let rules: Rules = buf_reader.lines().try_fold(Rules::new(), |mut acc, v| {
        let rule = v.map_err(|e| Error::InputReadError(e))?;
        let rule: Rule = rule.parse()?;
        acc.insert(rule.identifier, rule);
        Ok(acc)
    })?;

    dbg!(rules);
    Ok(())
}

type Rules = HashMap<usize, Rule>;

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
        let mut split_rule_definitions = rule_definitions.split("|");

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
