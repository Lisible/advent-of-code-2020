fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut values: Vec<u32> = std::fs::read_to_string("../input")?
        .trim()
        .split("\n")
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    values.sort();

    let mut left = 0;
    let mut right = values.len() - 1;
    loop {
        let first_value = values[left];
        let second_value = values[right];

        let sum = first_value + second_value;
        if sum == 2020 {
            println!("{} * {} = {}", first_value, second_value, sum);
            break;
        } else if sum > 2020 {
            right -= 1;
        } else {
            left += 1
        }
    }

    Ok(())
}
