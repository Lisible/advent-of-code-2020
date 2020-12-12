use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|_| Error::FileNotFound)?;
    let buf_reader = BufReader::new(file);
    let actions: Vec<Action> = buf_reader
        .lines()
        .map(|s| s.expect("Read error").parse().expect("Parse error"))
        .collect();

    let mut ship = Ship::new();
    for action in actions {
        ship.perform_action(action);
    }

    println!("Result: {}", ship.manhattan_distance_from_origin());

    Ok(())
}

#[derive(Debug)]
struct Ship {
    position: Position,
    waypoint_position: Position,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            position: Position(0, 0),
            waypoint_position: Position(10, 1),
        }
    }

    pub fn perform_action(&mut self, action: Action) {
        match action {
            Action::MoveForward(value) => {
                self.navigate(value);
            }
            Action::Move(d, value) => {
                self.move_waypoint(d, value);
            }
            Action::TurnLeft(degrees) => {
                self.rotate_waypoint(-degrees);
            }
            Action::TurnRight(degrees) => {
                self.rotate_waypoint(degrees);
            }
        }
    }

    fn move_waypoint(&mut self, d: Direction, value: i32) {
        match d {
            Direction::North => self.waypoint_position.1 += value,
            Direction::East => self.waypoint_position.0 += value,
            Direction::South => self.waypoint_position.1 -= value,
            Direction::West => self.waypoint_position.0 -= value,
        }
    }

    fn navigate(&mut self, steps: i32) {
        for _ in 0..steps {
            self.position.0 += self.waypoint_position.0;
            self.position.1 += self.waypoint_position.1;
        }
    }

    fn rotate_waypoint(&mut self, relative_angle_degrees: i32) {
        const SIN: [i32; 4] = [0, 1, 0, -1];
        const COS: [i32; 4] = [1, 0, -1, 0];

        let i = (4 + relative_angle_degrees as usize / 90 % 4) % 4;
        self.waypoint_position = Position(
            self.waypoint_position.0 * COS[i] + self.waypoint_position.1 * SIN[i],
            -self.waypoint_position.0 * SIN[i] + self.waypoint_position.1 * COS[i],
        );
    }

    fn manhattan_distance_from_origin(&mut self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }
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

#[derive(Debug, Copy, Clone)]
struct Position(i32, i32);

#[derive(Debug)]
enum Error {
    FileNotFound,
    ParseValueError(ParseIntError),
    ParseActionError(Option<u8>),
}
