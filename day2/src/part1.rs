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
    let (valid_character_count_range, character, password) = parse_password_entry(password_entry);
    valid_character_count_range
        .contains(&(password.chars().filter(|&c| c == character).count() as u32))
}

fn parse_password_entry(password_entry: &str) -> (RangeInclusive<u32>, char, &str) {
    let mut split_password_entry = password_entry.split(" ");

    let valid_character_count_range =
        extract_char_min_max_count(split_password_entry.next().expect("Range not found"));

    let character = split_password_entry
        .next()
        .expect("Character string not found")
        .chars()
        .nth(0)
        .expect("Character not found");

    let password = split_password_entry
        .next()
        .expect("Password string not found");

    (valid_character_count_range, character, password)
}

fn extract_char_min_max_count(min_max_str: &str) -> RangeInclusive<u32> {
    const PARSE_ERROR: &'static str = "Couldn't parse range bound";
    let mut split_min_max_str = min_max_str.split("-");
    if let Some(min_value) = split_min_max_str.next() {
        if let Some(max_value) = split_min_max_str.next() {
            return u32::from_str_radix(min_value, 10).expect(PARSE_ERROR)
                ..=u32::from_str_radix(max_value, 10).expect(PARSE_ERROR);
        }
    }

    panic!("Couldn't parse range");
}
