use std::cmp::{max, min};

const EMPTY_SEAT: u8 = b'L';
const OCCUPIED_SEAT: u8 = b'#';

type SeatMap = Vec<Vec<u8>>;
fn main() -> Result<(), Error> {
    let input = std::fs::read_to_string("input").map_err(|_| Error::ReadFileError)?;
    let seat_map: SeatMap = input
        .split("\n")
        .map(|s| s.replace("L", "#").bytes().collect())
        .collect();

    println!("count: {}", part1(&seat_map));
    Ok(())
}

fn part1(seat_map: &SeatMap) -> usize {
    let mut previous_seat_map = vec![];
    let mut seat_map =
        compute_next_state(&seat_map, should_become_empty_p1, should_become_occupied_p1);
    while previous_seat_map != seat_map {
        previous_seat_map = seat_map.clone();
        seat_map = compute_next_state(&seat_map, should_become_empty_p1, should_become_occupied_p1);
    }

    seat_map
        .iter()
        .flat_map(|v| v)
        .filter(|&&v| v == OCCUPIED_SEAT)
        .count()
}

fn compute_next_state(
    seat_map: &SeatMap,
    empty_rule: fn(usize, usize, &SeatMap) -> bool,
    occupied_rule: fn(usize, usize, &SeatMap) -> bool,
) -> SeatMap {
    let width = seat_map[0].len();
    let height = seat_map.len();

    let mut next_state: SeatMap = seat_map.clone();
    for y in 0..height {
        for x in 0..width {
            if empty_rule(x, y, seat_map) {
                next_state[y][x] = EMPTY_SEAT
            } else if occupied_rule(x, y, seat_map) {
                next_state[y][x] = OCCUPIED_SEAT
            }
        }
    }

    next_state
}

fn should_become_empty_p1(x: usize, y: usize, seat_map: &SeatMap) -> bool {
    &seat_map[y][x] == &OCCUPIED_SEAT
        && count_adjacent(x as i32, y as i32, seat_map, OCCUPIED_SEAT) >= 4
}

fn should_become_occupied_p1(x: usize, y: usize, seat_map: &SeatMap) -> bool {
    &seat_map[y][x] == &EMPTY_SEAT
        && count_adjacent(x as i32, y as i32, seat_map, OCCUPIED_SEAT) == 0
}

fn count_adjacent(x: i32, y: i32, seat_map: &SeatMap, state: u8) -> usize {
    let mut number = 0;
    for j in max(y - 1, 0)..=min(y + 1, seat_map.len() as i32 - 1) {
        for i in max(x - 1, 0)..=min(x + 1, seat_map[0].len() as i32 - 1) {
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
