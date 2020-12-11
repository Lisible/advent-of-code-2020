use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let file = File::open("input").expect("Input file not found");
    let reader = BufReader::new(&file);
    let numbers: Vec<u64> = reader
        .lines()
        .map(|v| u64::from_str(&v.expect("Expected row")).expect("Cannot parse number"))
        .collect();

    let not_the_sum_number = not_the_sum_of_last_numbers(&numbers);
    let encryption_weakness = compute_encryption_weakness(not_the_sum_number, &numbers);

    println!("part1: {}", not_the_sum_number);
    println!("part2: {}", encryption_weakness);
}

fn compute_encryption_weakness(target_number: u64, numbers: &Vec<u64>) -> u64 {
    let mut result = 0;
    let mut max_range_value = 0;
    let mut min_range_value = 0;
    for i in 0..numbers.len() {
        let mut sum = 0;
        let mut range_length = 0usize;
        while sum < target_number {
            sum += numbers[i + range_length];

            if range_length == 0 {
                min_range_value = numbers[i + range_length];
                max_range_value = numbers[i + range_length];
            } else if numbers[i + range_length] > max_range_value {
                max_range_value = numbers[i + range_length];
            } else if numbers[i + range_length] < min_range_value {
                min_range_value = numbers[i + range_length];
            }

            range_length += 1;
        }

        if sum == target_number && range_length > 1 {
            result = min_range_value + max_range_value;
            break;
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

            sum = preamble[low_index] + preamble[high_index];

            if sum < number {
                low_index += 1;
            } else if sum > number {
                high_index -= 1;
            }
        }
    }

    result
}
