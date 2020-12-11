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

    let valid_character_count_range = extract_char_min_max_count(
        split_password_entry
            .next()
            .ok_or(ParseError::MissingRangeString)?,
    )?;

    let character = split_password_entry
        .next()
        .ok_or(ParseError::MissingCharacterString)?
        .chars()
        .nth(0)
        .ok_or(ParseError::MissingCharacter)?;

    let password = split_password_entry
        .next()
        .ok_or(ParseError::MissingPasswordString)?;

    Ok((valid_character_count_range, character, password))
}

fn extract_char_min_max_count(min_max_str: &str) -> Result<RangeInclusive<u32>, ParseError> {
    let mut split_min_max_str = min_max_str.split("-");
    let min_value = parse_range_bound(
        split_min_max_str
            .next()
            .ok_or(ParseError::MissingMinBound)?,
    )?;
    let max_value = parse_range_bound(
        split_min_max_str
            .next()
            .ok_or(ParseError::MissingMaxBound)?,
    )?;
    Ok(min_value..=max_value)
}

fn parse_range_bound(bound_str: &str) -> Result<u32, ParseError> {
    u32::from_str_radix(bound_str, 10).map_err(|_| ParseError::ParseRangeBoundError)
}

enum ParseError {
    ParseRangeBoundError,
    MissingRangeString,
    MissingMinBound,
    MissingMaxBound,
    MissingCharacter,
    MissingCharacterString,
    MissingPasswordString,
}
