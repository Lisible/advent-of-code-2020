use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::RangeInclusive;
use std::str::FromStr;

type Rules = HashMap<String, Rule>;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|e| Error::InputFileOpenError(e))?;
    let mut lines = BufReader::new(file).lines();

    let rules = parse_rules(&mut lines)?;
    let _ = parse_your_ticket(&mut lines)?;
    let nearby_tickets = parse_nearby_tickets(&mut lines)?;

    println!(
        "Error rate: {}",
        compute_ticket_scanning_error_rate(&rules, &nearby_tickets)
    );
    Ok(())
}

fn compute_ticket_scanning_error_rate(rules: &Rules, nearby_tickets: &Vec<Ticket>) -> u32 {
    nearby_tickets.iter().fold(0, |acc, ticket| {
        acc + compute_ticket_error_count(rules, ticket)
    })
}

fn compute_ticket_error_count(rules: &Rules, ticket: &Ticket) -> u32 {
    ticket.values.iter().fold(0, |acc, &value| {
        if rules.iter().any(|(_, rule)| validate(value, rule)) {
            acc
        } else {
            acc + value
        }
    })
}

fn validate(value: u32, rule: &Rule) -> bool {
    rule.valid_ranges.iter().any(|r| r.contains(&value))
}

fn parse_your_ticket(lines: &mut Lines<BufReader<File>>) -> Result<Ticket, Error> {
    let line = read_line(lines)?;
    if line != "your ticket:" {
        return Err(Error::ExpectedYourTicket);
    }

    let ticket = parse_ticket(lines);

    let line = read_line(lines)?;
    if line != "" {
        return Err(Error::ExpectedEmptyLine);
    }

    ticket
}

fn parse_nearby_tickets(lines: &mut Lines<BufReader<File>>) -> Result<Vec<Ticket>, Error> {
    let line = read_line(lines)?;
    if line != "nearby tickets:" {
        return Err(Error::ExpectedNearbyTickets);
    }

    lines.filter_map(|l| l.ok()).map(|l| l.parse()).collect()
}

fn parse_ticket(lines: &mut Lines<BufReader<File>>) -> Result<Ticket, Error> {
    Ok(read_line(lines)?.parse()?)
}

fn parse_rules(lines: &mut Lines<BufReader<File>>) -> Result<Rules, Error> {
    let mut rules: Rules = Rules::new();
    loop {
        let line = read_line(lines)?;

        if line.trim().is_empty() {
            break;
        }

        let rule: Rule = line.parse()?;
        rules.insert(rule.field.clone(), rule);
    }

    Ok(rules)
}

fn read_line(lines: &mut Lines<BufReader<File>>) -> Result<String, Error> {
    Ok(lines
        .next()
        .ok_or(Error::UnexpectedEof)?
        .map_err(|e| Error::InputFileReadError(e))?)
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u32>,
}

impl FromStr for Ticket {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s.split(",").filter_map(|v| u32::from_str(v).ok()).collect(),
        })
    }
}

#[derive(Debug)]
struct Rule {
    field: String,
    valid_ranges: Vec<RangeInclusive<u32>>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_str = s.split(":");
        let field = split_str.next().ok_or(Error::ExpectedField)?.to_owned();

        let ranges_str = split_str.next().ok_or(Error::ExpectedRanges)?.trim();
        let split_ranges_str = ranges_str.split(" or ");
        let mut valid_ranges = Vec::new();
        for range_str in split_ranges_str {
            let mut split_range_str = range_str.split("-");
            let lower_bound = u32::from_str(
                split_range_str
                    .next()
                    .ok_or(Error::ExpectedLowerRangeBound)?,
            )
            .map_err(|e| Error::ParseRangeBoundError(e))?;

            let higher_bound = u32::from_str(
                split_range_str
                    .next()
                    .ok_or(Error::ExpectedHigherRangeBound)?,
            )
            .map_err(|e| Error::ParseRangeBoundError(e))?;

            valid_ranges.push(lower_bound..=higher_bound);
        }

        Ok(Self {
            field,
            valid_ranges,
        })
    }
}

#[derive(Debug)]
enum Error {
    InputFileOpenError(std::io::Error),
    InputFileReadError(std::io::Error),
    ExpectedField,
    ExpectedRanges,
    ExpectedLowerRangeBound,
    ExpectedHigherRangeBound,
    ParseRangeBoundError(std::num::ParseIntError),
    ExpectedYourTicket,
    ExpectedEmptyLine,
    ExpectedNearbyTickets,
    UnexpectedEof,
}
