use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Error> {
    let file = File::open("input").map_err(|e| Error::InputFileOpenError(e))?;
    let reader = BufReader::new(file);
    let tiles_to_flip: Vec<HexPosition> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(hex_position)
        .collect();

    let mut hex_grid = HexGrid::new();
    for tile_to_flip in tiles_to_flip {
        hex_grid.flip_tile(tile_to_flip);
    }

    println!("Black tiles: {}", hex_grid.black_tile_count());

    Ok(())
}

fn hex_position(mut line: String) -> HexPosition {
    let mut position = (0, 0, 0);

    while line.len() > 0 {
        let dir = if &line[0..1] == "n" || &line[0..1] == "s" {
            (&line[0..2]).to_owned()
        } else {
            (&line[0..1]).to_owned()
        };

        line = line[dir.len()..line.len()].to_owned();
        match dir.as_str() {
            "e" => {
                position.0 += 1;
                position.1 -= 1;
            }
            "w" => {
                position.0 -= 1;
                position.1 += 1;
            }
            "ne" => {
                position.0 += 1;
                position.2 -= 1;
            }
            "se" => {
                position.1 -= 1;
                position.2 += 1;
            }
            "nw" => {
                position.1 += 1;
                position.2 -= 1;
            }
            "sw" => {
                position.0 -= 1;
                position.2 += 1;
            }
            _ => panic!("unknown direction"),
        }
    }

    position
}

type HexPosition = (i32, i32, i32);

struct HexGrid {
    black_tiles: HashSet<HexPosition>,
}

impl HexGrid {
    pub fn new() -> Self {
        Self {
            black_tiles: HashSet::new(),
        }
    }

    pub fn flip_tile(&mut self, position: HexPosition) {
        if self.black_tiles.contains(&position) {
            self.black_tiles.remove(&position);
        } else {
            self.black_tiles.insert(position);
        }
    }

    pub fn black_tile_count(&self) -> usize {
        self.black_tiles.len()
    }
}

#[derive(Debug)]
enum Error {
    InputFileOpenError(std::io::Error),
}
