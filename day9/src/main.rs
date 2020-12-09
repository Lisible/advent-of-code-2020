use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|_| Error::InputFileNotFound)?;
    let reader = BufReader::new(&file);
    let numbers: Vec<u64> = reader
        .lines()
        .map(|v| u64::from_str(&v.expect("Expected row")).expect("Cannot parse number"))
        .collect();

    let not_the_sum_number = not_the_sum_of_last_numbers(&numbers);
    let encryption_weakness = compute_encryption_weakness(not_the_sum_number, &numbers);

    println!("part1: {}", not_the_sum_number);
    println!("part2: {}", encryption_weakness);

    Ok(())
}

fn compute_encryption_weakness(target_number: u64, numbers: &Vec<u64>) -> u64 {
    let mut sum = 0;
    let mut result = 0;
    let mut max_range_value = 0;
    let mut min_range_value = 0;
    for i in 0..numbers.len() {
        let mut range = 0usize;
        while sum < target_number {
            sum += numbers[i + range];
            if numbers[i + range] > max_range_value || range == 0 {
                max_range_value = numbers[i + range];
            }

            if numbers[i + range] < min_range_value || range == 0 {
                min_range_value = numbers[i + range];
            }

            range += 1;
        }

        if sum == target_number && range > 1 {
            result = min_range_value + max_range_value;
            break;
        } else {
            sum = 0;
        }
    }

    result
}

fn not_the_sum_of_last_numbers(numbers: &Vec<u64>) -> u64 {
    const PREAMBLE_LENGTH: usize = 25;
    let mut result = 0;
    for i in PREAMBLE_LENGTH..numbers.len() {
        let number = numbers[i];
        let mut low_index = 0;
        let mut high_index = PREAMBLE_LENGTH - 1;
        let mut preamble = numbers[i - PREAMBLE_LENGTH..=i - 1].to_vec();
        preamble.sort();
        let mut sum = preamble[low_index] + preamble[high_index];
        while number != sum {
            if low_index == high_index {
                result = number;
                break;
            }

            if sum < number {
                low_index += 1;
            } else if sum > number {
                high_index -= 1;
            } else {
                break;
            }
            sum = preamble[low_index] + preamble[high_index];
        }
    }

    result
}

#[derive(Debug)]
enum Error {
    InputFileNotFound,
}
