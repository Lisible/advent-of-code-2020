fn main() -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<u32> = std::fs::read_to_string("../input")?
        .trim()
        .split("\n")
        .map(|v| v.parse::<u32>().unwrap())
        .collect();

    'loops: for i in 0..values.len() {
        for j in 0..values.len() {
            let first_value = values.get(i).unwrap();
            let second_value = values.get(j).unwrap();
            if i != j && first_value + second_value == 2020 {
                println!(
                    "{} * {} = {}",
                    first_value,
                    second_value,
                    first_value * second_value
                );
                break 'loops;
            }
        }
    }

    Ok(())
}
