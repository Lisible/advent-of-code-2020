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

    let bus_ids: Vec<u32> = lines
        .next()
        .ok_or(Error::BusIdsNotFound)?
        .map_err(|e| Error::ReadError(e))?
        .split(",")
        .filter_map(|s| match s {
            "x" => None,
            id => Some(u32::from_str(id).expect("Missing bus id")),
        })
        .collect();

    let bus_id: u32 = bus_ids
        .iter()
        .min_by(|&schedule_a, &schedule_b| {
            bus_from(timestamp, *schedule_a).cmp(&bus_from(timestamp, *schedule_b))
        })
        .cloned()
        .ok_or(Error::NoResultFound)?;

    println!(
        "result: {}",
        bus_id * (bus_from(timestamp, bus_id) - timestamp)
    );

    println!("{:?}", bus_ids);

    println!("{:?}", extended_euclide(120, 23));

    Ok(())
}

fn bus_from(timestamp: u32, bus_id: u32) -> u32 {
    let mut bus_timestamp = 0;
    while timestamp > bus_timestamp {
        bus_timestamp += bus_id;
    }

    bus_timestamp
}

fn extended_euclide_rec(r: i32, u: i32, v: i32, rp: i32, up: i32, vp: i32) -> (i32, i32, i32) {
    if rp == 0 {
        (r, u, v)
    } else {
        extended_euclide_rec(
            rp,
            up,
            vp,
            r - (r / rp) * rp,
            u - (r / rp) * up,
            v - (r / rp) * vp,
        )
    }
}

fn extended_euclide(a: i32, b: i32) -> (i32, i32, i32) {
    extended_euclide_rec(a, 1, 0, b, 0, 1)
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
