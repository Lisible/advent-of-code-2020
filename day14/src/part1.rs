use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|e| Error::FileOpenError(e))?;
    let buf_reader = BufReader::new(file);
    let (_, memory) = buf_reader.lines().try_fold(
        ((0, 0), HashMap::<u64, u64>::new()),
        |((mask_0, mask_1), mut memory), line| {
            let line = line.map_err(|e| Error::FileReadError(e))?;
            let mut split_line = line.split(" = ");
            let lhs = split_line.next().ok_or(Error::LhsNotFound)?;
            let rhs = split_line.next().ok_or(Error::RhsNotFound)?;
            if lhs == "mask" {
                let mask_0 = u64::from_str_radix(&*rhs.replace("X", "1"), 2)
                    .map_err(|e| Error::ParseError(e))?;
                let mask_1 = u64::from_str_radix(&*rhs.replace("X", "0"), 2)
                    .map_err(|e| Error::ParseError(e))?;
                return Ok(((mask_0, mask_1), memory));
            }

            let addr = u64::from_str(&lhs[4..lhs.len() - 1]).map_err(|e| Error::ParseError(e))?;
            let value = u64::from_str(rhs).map_err(|e| Error::ParseError(e))?;
            let value = value | mask_1;
            let value = value & mask_0;
            *memory.entry(addr).or_insert(0) = value;
            Ok(((mask_0, mask_1), memory))
        },
    )?;

    println!("{}", memory.values().sum::<u64>());
    Ok(())
}

#[derive(Debug)]
enum Error {
    FileOpenError(std::io::Error),
    FileReadError(std::io::Error),
    LhsNotFound,
    RhsNotFound,
    ParseError(std::num::ParseIntError),
}
