use itertools::Itertools;

#[derive(Debug)]
enum Error {
    ReadFileError,
}

fn main() -> Result<(), Error> {
    let input = std::fs::read_to_string("input").map_err(|_| Error::ReadFileError)?;
    let count: usize = input.split("\n\n").fold(0, |acc, v| {
        acc + v
            .split(|c| c == '\0' || c == '\n')
            .flat_map(|s| s.bytes().collect::<Vec<u8>>())
            .unique()
            .count()
    });
    println!("{:?}", count);
    Ok(())
}
