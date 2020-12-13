use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|_| Error::InputFileNotFound)?;
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    let timestamp = u32::from_str(
        &lines
            .next()
            .ok_or(Error::TimestampNotFound)?
            .map_err(|e| Error::ReadError(e))?,
    )
    .map_err(|e| Error::ParseIntError(e))?;

    let bus_id = lines
        .next()
        .ok_or(Error::BusIdsNotFound)?
        .map_err(|e| Error::ReadError(e))?
        .split(",")
        .filter_map(|s| match s {
            "x" => None,
            id => Some(u32::from_str(id).expect("Missing bus id")),
        })
        .min_by(|&schedule_a, &schedule_b| {
            bus_from(timestamp, schedule_a).cmp(&bus_from(timestamp, schedule_b))
        })
        .ok_or(Error::NoResultFound)?;

    println!(
        "result: {}",
        bus_id * (bus_from(timestamp, bus_id) - timestamp)
    );

    Ok(())
}

fn bus_from(timestamp: u32, bus_id: u32) -> u32 {
    let mut bus_timestamp = 0;
    while timestamp > bus_timestamp {
        bus_timestamp += bus_id;
    }

    bus_timestamp
}

#[derive(Debug)]
enum Error {
    InputFileNotFound,
    ReadError(std::io::Error),
    ParseIntError(ParseIntError),
    TimestampNotFound,
    BusIdsNotFound,
    NoResultFound,
}
