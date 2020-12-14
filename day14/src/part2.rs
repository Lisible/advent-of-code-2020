use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|e| Error::FileOpenError(e))?;
    let buf_reader = BufReader::new(file);
    let (_, memory) = buf_reader.lines().try_fold(
        (String::new(), HashMap::<u64, u64>::new()),
        |(mask, mut memory), line| {
            let line = line.map_err(|e| Error::FileReadError(e))?;
            let mut split_line = line.split(" = ");
            let lhs = split_line.next().ok_or(Error::LhsNotFound)?;
            let rhs = split_line.next().ok_or(Error::RhsNotFound)?;
            if lhs == "mask" {
                return Ok((rhs.to_string(), memory));
            }

            let mask_1 = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
            let mask_x = u64::from_str_radix(&mask.replace("1", "0").replace("X", "1"), 2).unwrap();

            let mut dest_addr =
                u64::from_str(&lhs[4..lhs.len() - 1]).map_err(|e| Error::ParseError(e))?;
            dest_addr |= mask_1;
            let dest_addrs = generate_floating_addresses(dest_addr, mask_x, 0);
            let value = u64::from_str(rhs).unwrap();
            for dest_addr in dest_addrs {
                *memory.entry(dest_addr).or_insert(0) = value;
            }

            Ok((mask.into(), memory))
        },
    )?;

    println!("{}", memory.values().sum::<u64>());
    Ok(())
}

fn generate_floating_addresses(dest_addr: u64, mask_x: u64, pow: u64) -> HashSet<u64> {
    if pow == 64 {
        let mut set = HashSet::new();
        set.insert(dest_addr);
        return set;
    }

    if mask_x & (1 << pow) != 0 {
        generate_floating_addresses(dest_addr | (1 << pow), mask_x, pow + 1)
            .union(&mut generate_floating_addresses(
                dest_addr & !(1 << pow),
                mask_x,
                pow + 1,
            ))
            .cloned()
            .collect()
    } else {
        generate_floating_addresses(dest_addr, mask_x, pow + 1)
    }
}

#[derive(Debug)]
enum Error {
    FileOpenError(std::io::Error),
    FileReadError(std::io::Error),
    LhsNotFound,
    RhsNotFound,
    ParseError(std::num::ParseIntError),
}
