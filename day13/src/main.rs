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

    let bus_ids = lines
        .next()
        .ok_or(Error::BusIdsNotFound)?
        .map_err(|e| Error::ReadError(e))?;
    let bus_ids: Vec<&str> = bus_ids.split(",").to_owned().collect();

    let bus_id: u32 = bus_ids
        .iter()
        .filter_map(|s| match s {
            &"x" => None,
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

    let eqs = bus_ids
        .iter()
        .enumerate()
        .filter(|(_, bus_id)| *bus_id != &"x")
        .map(|(i, bus_id)| {
            let bus_id = i64::from_str(bus_id).expect("Missing bus id");
            ((-(i as i64) % bus_id + bus_id) % bus_id, bus_id)
        })
        .collect();
    part_2(&eqs);

    Ok(())
}

fn part_2(eqs: &Vec<(i64, i64)>) {
    let t: i64 = eqs.iter().map(|e| e.1).product();

    println!(
        "{}",
        eqs.iter().fold(0i64, |acc, &(n, ni)| {
            let nci = t / ni;
            acc + n * nci * modular_inverse(ni, nci)
        }) % t
    )
}

fn modular_inverse(a: i64, b: i64) -> i64 {
    let (_, _, v) = extended_euclide(a, b);
    (v % a + a) % a
}

fn extended_euclide(a: i64, b: i64) -> (i64, i64, i64) {
    match b {
        0 => (a, 1, 0),
        _ => {
            let (dp, up, vp) = extended_euclide(b, a % b);
            (dp, vp, up - (a / b) * vp)
        }
    }
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
