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

    for day in 0..100 {
        perform_step(&mut hex_grid);
        println!("Day {}: {}", day + 1, hex_grid.black_tile_count());
    }

    Ok(())
}

fn perform_step(hex_grid: &mut HexGrid) {
    let minimum_bounds = hex_grid.minimum_bounds().unwrap();
    let maximum_bounds = hex_grid.maximum_bounds().unwrap();

    let mut black_tiles = hex_grid.black_tiles.clone();
    for z in minimum_bounds.2 - 2..maximum_bounds.2 + 2 {
        for y in minimum_bounds.1 - 2..maximum_bounds.1 + 2 {
            for x in minimum_bounds.0 - 2..maximum_bounds.0 + 2 {
                let is_black = hex_grid.is_black((x, y, z));
                let black_neighbours_count = hex_grid.black_neighbours_count((x, y, z));
                if is_black && (black_neighbours_count == 0 || black_neighbours_count > 2) {
                    black_tiles.remove(&(x, y, z));
                } else if !is_black && black_neighbours_count == 2 {
                    black_tiles.insert((x, y, z));
                }
            }
        }
    }

    hex_grid.black_tiles = black_tiles;
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

    pub fn minimum_bounds(&self) -> Option<HexPosition> {
        self.black_tiles.iter().fold(None, |acc, v| {
            if acc == None {
                return Some((v.0, v.1, v.2));
            }

            let mut current_minimum = acc.unwrap();

            if current_minimum.0 > v.0 {
                current_minimum.0 = v.0;
            }

            if current_minimum.1 > v.1 {
                current_minimum.1 = v.1;
            }

            if current_minimum.2 > v.2 {
                current_minimum.2 = v.2;
            }

            Some(current_minimum)
        })
    }

    pub fn maximum_bounds(&self) -> Option<HexPosition> {
        self.black_tiles.iter().fold(None, |acc, v| {
            if acc == None {
                return Some((v.0, v.1, v.2));
            }

            let mut current_maximum = acc.unwrap();

            if current_maximum.0 < v.0 {
                current_maximum.0 = v.0;
            }

            if current_maximum.1 < v.1 {
                current_maximum.1 = v.1;
            }

            if current_maximum.2 < v.2 {
                current_maximum.2 = v.2;
            }

            Some(current_maximum)
        })
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

    pub fn is_black(&self, position: HexPosition) -> bool {
        self.black_tiles.contains(&position)
    }

    pub fn black_neighbours_count(&self, position: HexPosition) -> usize {
        [
            (-1, 1, 0),
            (0, 1, -1),
            (1, 0, -1),
            (1, -1, 0),
            (0, -1, 1),
            (-1, 0, 1),
        ]
        .iter()
        .filter(|&&p| self.is_black((position.0 + p.0, position.1 + p.1, position.2 + p.2)))
        .count()
    }
}

#[derive(Debug)]
enum Error {
    InputFileOpenError(std::io::Error),
}
