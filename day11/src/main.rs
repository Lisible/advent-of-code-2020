use std::cmp::{max, min};

const EMPTY_SEAT: u8 = b'L';
const OCCUPIED_SEAT: u8 = b'#';
const FLOOR: u8 = b'.';

fn main() -> Result<(), Error> {
    let input = std::fs::read_to_string("input").map_err(|_| Error::ReadFileError)?;
    let seat_map: Vec<Vec<u8>> = input
        .split("\n")
        .map(|s| s.replace("L", "#").bytes().collect())
        .collect();

    let mut previous_seat_map = vec![];
    let mut seat_map = compute_next_state(&seat_map);
    while previous_seat_map != seat_map {
        previous_seat_map = seat_map.clone();
        seat_map = compute_next_state(&seat_map);
    }

    let count = seat_map
        .iter()
        .flat_map(|v| v)
        .filter(|&&v| v == OCCUPIED_SEAT)
        .count();
    println!("count: {}", count);
    Ok(())
}

fn compute_next_state(seat_map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let width = seat_map[0].len();
    let height = seat_map.len();

    let mut next_state: Vec<Vec<u8>> = seat_map.clone();
    for y in 0..height {
        for x in 0..width {
            if &next_state[y][x] == &OCCUPIED_SEAT
                && count_adjacent(x as i32, y as i32, seat_map, OCCUPIED_SEAT) >= 4
            {
                next_state[y][x] = EMPTY_SEAT
            } else if &next_state[y][x] == &EMPTY_SEAT
                && count_adjacent(x as i32, y as i32, seat_map, OCCUPIED_SEAT) == 0
            {
                next_state[y][x] = OCCUPIED_SEAT
            }
        }
    }

    next_state
}

fn count_adjacent(x: i32, y: i32, seat_map: &Vec<Vec<u8>>, state: u8) -> usize {
    let mut number = 0;
    let min_x = max(x - 1, 0);
    let min_y = max(y - 1, 0);
    let max_x = min(x + 1, seat_map[0].len() as i32 - 1);
    let max_y = min(y + 1, seat_map.len() as i32 - 1);

    for j in min_y..=max_y {
        for i in min_x..=max_x {
            if j != y || i != x {
                number += (seat_map[j as usize][i as usize] == state) as usize;
            }
        }
    }

    number
}

#[derive(Debug)]
enum Error {
    ReadFileError,
}
