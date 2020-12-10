use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input").expect("Input file not found");
    let adapters: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    let diff1_times_diff3 = diff1_times_diff3(&adapters);

    let mut cache = HashMap::new();
    let adapters_set: HashSet<&i32> = adapters.iter().collect();
    let arrangements_count = compute_arrangements_count(0, &mut cache, &adapters_set);

    println!("Diff1 * Diff3 = {:?}", diff1_times_diff3);
    println!("Arrangements count = {:?}", arrangements_count);
}

fn compute_arrangements_count(
    value: i32,
    cache: &mut HashMap<i32, usize>,
    adapters: &HashSet<&i32>,
) -> usize {
    if value == 182 {
        return 1;
    }

    let mut sum = 0;
    for v in value + 1..=value + 3 {
        if adapters.contains(&v) {
            if cache.contains_key(&v) {
                sum += cache.get(&v).unwrap();
            } else {
                sum += compute_arrangements_count(v, cache, adapters);
            }
        }
    }

    cache.entry(value).or_insert(sum);
    sum
}

fn diff1_times_diff3(adapters: &Vec<i32>) -> i32 {
    let mut adapters = adapters.clone();
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
    difference_1 * difference_3
}
