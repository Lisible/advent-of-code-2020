use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::ops::RangeInclusive;

fn main() -> Result<(), Error> {
    let input_file = File::open("input")?;
    let reader = BufReader::new(input_file);
    println!(
        "{}",
        reader
            .lines()
            .filter(|s| is_password_valid(s.as_ref().expect("Invalid input")))
            .count()
    );
    Ok(())
}

fn is_password_valid(password_entry: &str) -> bool {
    if let Ok((valid_character_count_range, character, password)) =
        parse_password_entry(password_entry)
    {
        return valid_character_count_range
            .contains(&(password.chars().filter(|&c| c == character).count() as u32));
    }
    false
}

fn parse_password_entry(
    password_entry: &str,
) -> Result<(RangeInclusive<u32>, char, &str), ParseError> {
    let mut split_password_entry = password_entry.split(" ");

    let valid_character_count_range =
        extract_char_min_max_count(split_password_entry.next().expect("Range not found"))?;

    let character = match split_password_entry.next() {
        Some(string) => match string.chars().nth(0) {
            Some(character) => character,
            None => return Err(ParseError::MissingCharacter),
        },
        None => return Err(ParseError::MissingCharacterString),
    };

    let password = match split_password_entry.next() {
        Some(password) => password,
        None => return Err(ParseError::MissingPasswordString),
    };

    Ok((valid_character_count_range, character, password))
}

fn extract_char_min_max_count(min_max_str: &str) -> Result<RangeInclusive<u32>, ParseError> {
    let mut split_min_max_str = min_max_str.split("-");
    if let Some(min_value) = split_min_max_str.next() {
        if let Some(max_value) = split_min_max_str.next() {
            return Ok(parse_range_bound(min_value)?..=parse_range_bound(max_value)?);
        }
    }

    panic!("Couldn't parse range");
}

fn parse_range_bound(bound_str: &str) -> Result<u32, ParseError> {
    match u32::from_str_radix(bound_str, 10) {
        Ok(value) => Ok(value),
        Err(_) => Err(ParseError::ParseRangeBoundError),
    }
}

enum ParseError {
    ParseRangeBoundError,
    MissingCharacter,
    MissingCharacterString,
    MissingPasswordString,
}
