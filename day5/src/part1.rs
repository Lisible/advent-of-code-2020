#[derive(Debug)]
enum Error {
    NoInput,
}

fn main() -> Result<(), Error> {
    let entries = std::fs::read_to_string("input").map_err(|_| Error::NoInput)?;
    let max_seat_id = entries.split("\n").fold(0, |acc, value| {
        let id = compute_id(value);
        if acc > id {
            acc
        } else {
            id
        }
    });

    println!("max_seat_id: {}", max_seat_id);
    Ok(())
}

fn compute_id(string: &str) -> i32 {
    string.chars().rev().enumerate().fold(0, |mut acc, (i, v)| {
        if v == 'B' || v == 'R' {
            acc += 1 << i;
        }

        acc
    })
}
