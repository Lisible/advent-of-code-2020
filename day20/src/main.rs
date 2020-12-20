use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

const PIECE_SIZE: i32 = 10;
const BORDER: u8 = b'#';

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|e| Error::InputFileNotFound)?;
    let buf_reader = BufReader::new(file);
    let pieces = parse_tiles(buf_reader)?;
    let corners = pieces.iter().fold(vec![], |mut acc, piece| {
        if is_corner(piece, &pieces) {
            acc.push(piece);
        }
        acc
    });

    println!(
        "{}",
        corners.iter().fold(1usize, |acc, c| c.id as usize * acc)
    );

    Ok(())
}

fn is_corner(piece: &Piece, pieces: &Vec<Piece>) -> bool {
    pieces
        .iter()
        .filter(|p| {
            let result = p.id != piece.id && match_piece(piece, p);
            result
        })
        .count()
        == 2
}

fn match_piece(piece: &Piece, other: &Piece) -> bool {
    let mut result = false;
    let mut other = other.clone();

    for _flip_y in 0..2 {
        other.flip_y();
        for _flip_x in 0..2 {
            other.flip_x();
            for _rotation in 0..4 {
                for _fx in 0..2 {
                    other.flip_x();
                    for _fy in 0..2 {
                        other.flip_y();
                        result |= piece.match_side(&other, Side::Right);
                        result |= piece.match_side(&other, Side::Left);
                        result |= piece.match_side(&other, Side::Top);
                        result |= piece.match_side(&other, Side::Bottom);
                    }
                }
                other.rotate_90();
            }
        }
    }
    result
}

fn parse_tiles<T: std::io::Read>(reader: BufReader<T>) -> Result<Vec<Piece>, Error> {
    let tiles_str: Vec<String> =
        reader
            .lines()
            .try_fold(vec![String::new()], |mut acc, line| {
                let line = line.map_err(|e| Error::InputReadError(e))?;
                if line.is_empty() {
                    acc.push("".into());
                } else {
                    let mut str = acc.last_mut().unwrap();
                    if !str.is_empty() {
                        str.push_str("\n");
                    }
                    str.push_str(&line);
                }

                Ok(acc)
            })?;
    tiles_str.iter().map(|str| Ok(str.parse()?)).collect()
}

#[derive(Debug, Clone)]
struct Piece {
    id: i32,
    data: HashSet<(i32, i32)>,
}

impl Piece {
    fn rotate_90(&mut self) {
        let cos = (PI / 2f32).cos();
        let sin = (PI / 2f32).sin();
        self.data = self
            .data
            .iter()
            .map(|coords| {
                (
                    (coords.0 as f32 * cos - coords.1 as f32 * sin).round() as i32,
                    (coords.1 as f32 * cos + coords.0 as f32 * sin).round() as i32,
                )
            })
            .collect();
    }

    fn flip_x(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|coords| (PIECE_SIZE - 1 - coords.0, coords.1))
            .collect();
    }

    fn flip_y(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|coords| (coords.0, PIECE_SIZE - 1 - coords.1))
            .collect();
    }

    fn match_side(&self, other: &Piece, side: Side) -> bool {
        let self_side: HashSet<(i32, i32)> =
            self.data.iter().filter(side.filter()).cloned().collect();
        let other_side: HashSet<(i32, i32)> = other
            .data
            .iter()
            .filter(side.opposite().filter())
            .cloned()
            .collect();

        self_side
            .iter()
            .map(|v| match side {
                Side::Right => ((v.0 + 1) % PIECE_SIZE, v.1),
                Side::Bottom => (v.0, (v.1 + 1) % PIECE_SIZE),
                Side::Top => (v.0, (PIECE_SIZE + (v.1 - 1) % PIECE_SIZE) % PIECE_SIZE),
                Side::Left => ((PIECE_SIZE + (v.0 - 1) % PIECE_SIZE) % PIECE_SIZE, v.1),
            })
            .collect::<HashSet<(i32, i32)>>()
            == other_side
    }
}

enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn filter(&self) -> impl FnMut(&&(i32, i32)) -> bool {
        match self {
            Side::Top => move |p: &&(i32, i32)| p.1 == 0,
            Side::Bottom => move |p: &&(i32, i32)| p.1 == PIECE_SIZE - 1,
            Side::Left => move |p: &&(i32, i32)| p.0 == 0,
            Side::Right => move |p: &&(i32, i32)| p.0 == PIECE_SIZE - 1,
            _ => panic!(),
        }
    }

    fn opposite(&self) -> Side {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl FromStr for Piece {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_s = s.trim().split("\n");

        let identifier_row = split_s.next().ok_or(Error::IdentifierRowNotFound)?;
        let mut id = identifier_row
            .split(" ")
            .nth(1)
            .ok_or(Error::IdentifierNotFound)?;
        let id = i32::from_str(&id[0..id.len() - 1]).map_err(|e| Error::IdentifierParseError(e))?;
        let data = split_s
            .enumerate()
            .fold(HashSet::new(), |mut acc, (row_index, row)| {
                for (col_index, byte) in row.bytes().enumerate() {
                    if byte == BORDER {
                        acc.insert((col_index as i32, row_index as i32));
                    }
                }
                acc
            });

        Ok(Piece { id, data })
    }
}

#[derive(Debug)]
enum Error {
    InputFileNotFound,
    InputReadError(std::io::Error),
    IdentifierRowNotFound,
    IdentifierNotFound,
    IdentifierParseError(std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_match_side() {
        let mut data = HashSet::new();
        data.insert((9, 2));
        data.insert((9, 5));

        let first_piece = Piece { id: 0, data };

        let mut second_piece = first_piece.clone();
        second_piece.flip_x();
        second_piece.rotate_90();
        second_piece.rotate_90();
        second_piece.rotate_90();
        second_piece.rotate_90();
        second_piece.id = 1;

        assert!(first_piece.match_side(&second_piece, Side::Right));
    }
}
