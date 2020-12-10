use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").expect("Input file not found");
    let mut adapters: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    adapters.sort_by(|a, b| b.cmp(a));

    let mut differences: HashMap<i32, i32> = HashMap::new();
    let mut rating = 0;
    while !adapters.is_empty() {
        let adapter_rating = adapters.pop().unwrap();
        let diff = adapter_rating - rating;
        rating += diff;
        *differences.entry(diff).or_default() += 1;
    }
    *differences.entry(3).or_default() += 1;

    let difference_1 = *differences.get(&1).expect("No 1-jolt diff");
    let difference_3 = *differences.get(&3).expect("No 3-jolt diff");
    println!("{:?}", difference_1 * difference_3);
}
