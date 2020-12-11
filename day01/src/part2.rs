fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut values: Vec<u32> = std::fs::read_to_string("../input")?
        .trim()
        .split("\n")
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    values.sort();

    'loops: for i in 0..values.len() {
        let first_value = values.get(i).unwrap();
        for j in 0..values.len() {
            let second_value = values.get(j).unwrap();
            let expected_third_value = 2020 - first_value - second_value;
            if can_find_binary_search(&values, expected_third_value) {
                println!(
                    "{} * {} * {} = {}",
                    first_value,
                    second_value,
                    expected_third_value,
                    first_value * second_value * expected_third_value
                );
                break 'loops;
            }
        }
    }

    Ok(())
}

fn can_find_binary_search(values: &Vec<u32>, searched_value: u32) -> bool {
    let mut start_index = 0;
    let mut end_index = values.len() - 1;
    while start_index <= end_index {
        let current_index = (start_index + end_index) / 2;
        if let Some(current_value) = values.get(current_index) {
            if *current_value > searched_value {
                end_index = current_index - 1;
            } else if *current_value < searched_value {
                start_index = current_index + 1;
            } else {
                return true;
            }
        }
    }
    false
}
