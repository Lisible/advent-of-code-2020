use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
}

fn main() -> Result<(), Error> {
    let input = std::fs::read_to_string("input").map_err(|e| Error::IoError(e))?;
    let count = input
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split(|c| c == ' ' || c == '\n')
                .fold(HashMap::new(), |mut acc, v| {
                    let mut split = v.split(":");
                    acc.insert(split.next().unwrap(), split.next().unwrap());
                    acc
                })
        })
        .filter(|passport| {
            vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|k| passport.contains_key(k))
        })
        .count();
    println!("{:?}", count);

    Ok(())
}
