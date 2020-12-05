use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    NoInput,
}

fn main() -> Result<(), Error> {
    const ENTRY_SIZE: u32 = 10;
    let entries = std::fs::read_to_string("input").map_err(|_| Error::NoInput)?;
    let ids: HashSet<i32> = entries.split("\n").map(compute_id).collect();
    for i in 0..2i32.pow(ENTRY_SIZE) {
        if !ids.contains(&i) && ids.contains(&(i + 1)) && ids.contains(&(i - 1)) {
            println!("{}", i);
        }
    }
    Ok(())
}

fn compute_id(string: &str) -> i32 {
    string.chars().rev().enumerate().fold(0, |mut acc, (i, v)| {
        if v == 'B' || v == 'R' {
            acc += 2i32.pow(i as u32);
        }

        acc
    })
}
