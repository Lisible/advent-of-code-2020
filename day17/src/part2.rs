use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let mut current_state: State = std::fs::read_to_string("input")
        .map_err(|_| Error::InputFileReadError)?
        .parse()?;

    println!("{}", current_state);
    for _ in 0..6 {
        let mut next_state = current_state.clone();
        let (min_bounds, max_bounds) = current_state.bounds().expect("Expected bounds");

        for w in min_bounds.w - 1..=max_bounds.w + 1 {
            for z in min_bounds.z - 1..=max_bounds.z + 1 {
                for y in min_bounds.y - 1..=max_bounds.y + 1 {
                    for x in min_bounds.x - 1..=max_bounds.x + 1 {
                        let position = Position { x, y, z, w };
                        let neighbours_count = current_state.count_activated_neighbours(&position);
                        if current_state.is_cell_activated(&position) {
                            if neighbours_count != 2 && neighbours_count != 3 {
                                next_state.deactivate_cell(&position);
                            }
                        } else if !current_state.is_cell_activated(&position)
                            && neighbours_count == 3
                        {
                            next_state.activate_cell(&position);
                        }
                    }
                }
            }
        }

        current_state = next_state.clone();
    }

    println!("{}", current_state.active_cells.len());

    Ok(())
}

#[derive(Clone, Debug)]
struct State {
    active_cells: HashSet<Cell>,
}

impl State {
    pub fn is_cell_activated(&self, p: &Position) -> bool {
        self.active_cells.contains(&Cell {
            position: p.clone(),
        })
    }

    pub fn activate_cell(&mut self, p: &Position) {
        self.active_cells.insert(Cell {
            position: p.clone(),
        });
    }

    pub fn deactivate_cell(&mut self, p: &Position) {
        self.active_cells.remove(&Cell {
            position: p.clone(),
        });
    }

    pub fn count_activated_neighbours(&self, p: &Position) -> i32 {
        let mut count = 0;
        for w in p.w - 1..=p.w + 1 {
            for z in p.z - 1..=p.z + 1 {
                for y in p.y - 1..=p.y + 1 {
                    for x in p.x - 1..=p.x + 1 {
                        if (x != p.x || y != p.y || z != p.z || w != p.w)
                            && self.active_cells.contains(&Cell {
                                position: Position { x, y, z, w },
                            })
                        {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    pub fn bounds(&self) -> Option<(Position, Position)> {
        let min_x = self
            .active_cells
            .iter()
            .min_by(|a, b| a.position.x.cmp(&b.position.x))?
            .position
            .x;
        let max_x = self
            .active_cells
            .iter()
            .max_by(|a, b| a.position.x.cmp(&b.position.x))?
            .position
            .x;
        let min_y = self
            .active_cells
            .iter()
            .min_by(|a, b| a.position.y.cmp(&b.position.y))?
            .position
            .y;
        let max_y = self
            .active_cells
            .iter()
            .max_by(|a, b| a.position.y.cmp(&b.position.y))?
            .position
            .y;
        let min_z = self
            .active_cells
            .iter()
            .min_by(|a, b| a.position.z.cmp(&b.position.z))?
            .position
            .z;
        let max_z = self
            .active_cells
            .iter()
            .max_by(|a, b| a.position.z.cmp(&b.position.z))?
            .position
            .z;
        let min_w = self
            .active_cells
            .iter()
            .min_by(|a, b| a.position.w.cmp(&b.position.w))?
            .position
            .w;
        let max_w = self
            .active_cells
            .iter()
            .max_by(|a, b| a.position.w.cmp(&b.position.w))?
            .position
            .w;

        Some((
            Position {
                x: min_x,
                y: min_y,
                z: min_z,
                w: min_w,
            },
            Position {
                x: max_x,
                y: max_y,
                z: max_z,
                w: max_w,
            },
        ))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min_bounds, max_bounds) = self.bounds().expect("Expected bounds");

        for w in min_bounds.w..=max_bounds.w {
            write!(f, "w={}\n", w)?;
            for z in min_bounds.z..=max_bounds.z {
                write!(f, "z={}\n", z)?;
                for y in min_bounds.y..=max_bounds.y {
                    for x in min_bounds.x..=max_bounds.x {
                        if self.is_cell_activated(&Position { x, y, z, w }) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    write!(f, "\n")?;
                }
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let active_cells =
            s.trim()
                .split("\n")
                .enumerate()
                .fold(HashSet::new(), |mut set, (row, line)| {
                    set = set
                        .union(&line.bytes().enumerate().fold(
                            HashSet::new(),
                            |mut set, (col, value)| {
                                if value == b'#' {
                                    set.insert(Cell {
                                        position: Position {
                                            x: col as i32,
                                            y: row as i32,
                                            z: 0,
                                            w: 0,
                                        },
                                    });
                                }
                                set
                            },
                        ))
                        .cloned()
                        .collect();
                    set
                });
        Ok(State { active_cells })
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Cell {
    position: Position,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

#[derive(Debug)]
enum Error {
    InputFileReadError,
}
