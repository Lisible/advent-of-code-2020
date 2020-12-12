use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::{Add, Sub};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|_| Error::FileNotFound)?;
    let buf_reader = BufReader::new(file);
    let actions: Vec<Action> = buf_reader
        .lines()
        .map(|s| s.expect("Read error").parse().expect("Parse error"))
        .collect();

    let mut position = Position(0i32, 0i32);
    let mut direction = Direction::East;

    for action in actions {
        let (pos, dir) = apply_action(position, direction, action)?;
        position = pos;
        direction = dir;
    }

    println!(
        "Position: {}, Result: {}",
        position,
        position.0.abs() + position.1.abs()
    );

    Ok(())
}

fn apply_action(
    position: Position,
    direction: Direction,
    action: Action,
) -> Result<(Position, Direction), Error> {
    let mut position = position;
    let mut direction = direction;
    match action {
        Action::Move(d, value) => {
            position = compute_new_position(position, d, value);
        }
        Action::MoveForward(value) => {
            position = compute_new_position(position, direction, value);
        }
        Action::TurnLeft(degrees) => {
            direction = direction - degrees;
        }
        Action::TurnRight(degrees) => {
            direction = direction + degrees;
        }
    }

    Ok((position, direction))
}

fn compute_new_position(position: Position, direction: Direction, value: i32) -> Position {
    let mut position = position;
    match direction {
        Direction::North => position.1 += value,
        Direction::East => position.0 += value,
        Direction::South => position.1 -= value,
        Direction::West => position.0 -= value,
    }

    position
}

#[derive(Debug)]
enum Action {
    Move(Direction, i32),
    TurnLeft(i32),
    TurnRight(i32),
    MoveForward(i32),
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = i32::from_str(&s[1..]).map_err(|e| Error::ParseValueError(e))?;
        Ok(match s.bytes().nth(0) {
            Some(b'L') => Action::TurnLeft(value),
            Some(b'R') => Action::TurnRight(value),
            Some(b'F') => Action::MoveForward(value),
            Some(direction) => Action::Move(direction.into(), value),
            c => return Err(Error::ParseActionError(c)),
        })
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'N' => Direction::North,
            b'S' => Direction::South,
            b'E' => Direction::East,
            b'W' => Direction::West,
            _ => panic!("Unknown direction"),
        }
    }
}

impl Add<i32> for Direction {
    type Output = Direction;

    fn add(self, angle_degrees: i32) -> Self::Output {
        const DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        let index = DIRECTIONS
            .iter()
            .position(|&v| v == self)
            .expect("Unknown direction");

        DIRECTIONS[(((index as i32 + angle_degrees / 90) % 4 + 4) % 4) as usize]
    }
}
impl Sub<i32> for Direction {
    type Output = Direction;

    fn sub(self, angle_degrees: i32) -> Self::Output {
        self + -angle_degrees
    }
}

#[derive(Debug, Copy, Clone)]
struct Position(i32, i32);
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.0, self.1)
    }
}

#[derive(Debug)]
enum Error {
    FileNotFound,
    ParseValueError(ParseIntError),
    ParseActionError(Option<u8>),
}
