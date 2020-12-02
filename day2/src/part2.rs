use std::fs::File;
use std::io::{BufRead, BufReader, Error};

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
    if let Ok((first_pos, second_pos, character, password)) = parse_password_entry(password_entry) {
        let first_char = password
            .chars()
            .nth(first_pos)
            .expect("Should have first pos");
        let second_char = password
            .chars()
            .nth(second_pos)
            .expect("Should have second pos");

        return (first_char == character) ^ (second_char == character);
    }
    false
}

fn parse_password_entry(password_entry: &str) -> Result<(usize, usize, char, &str), ParseError> {
    let mut split_password_entry = password_entry.split(" ");

    let (first_pos, second_pos) = extract_pos(
        split_password_entry
            .next()
            .ok_or(ParseError::MissingPositionsString)?,
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

    Ok((first_pos, second_pos, character, password))
}

fn extract_pos(min_max_str: &str) -> Result<(usize, usize), ParseError> {
    let mut split_min_max_str = min_max_str.split("-");

    let first_position = usize::from_str_radix(
        split_min_max_str
            .next()
            .ok_or(ParseError::MissingFirstPosition)?,
        10,
    )
    .map_err(|_| ParseError::ParseNumberError)?;

    let second_position = usize::from_str_radix(
        split_min_max_str
            .next()
            .ok_or(ParseError::MissingSecondPosition)?,
        10,
    )
    .map_err(|_| ParseError::ParseNumberError)?;

    Ok((first_position - 1, second_position - 1))
}

enum ParseError {
    ParseNumberError,
    MissingCharacter,
    MissingFirstPosition,
    MissingSecondPosition,
    MissingPositionsString,
    MissingCharacterString,
    MissingPasswordString,
}
