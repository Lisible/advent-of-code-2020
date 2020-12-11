#[derive(Debug)]
enum Error {
    FileNotFound,
    OutOfMap,
}

type Map<'a> = Vec<&'a str>;

fn main() -> Result<(), Error> {
    let file = std::fs::read_to_string("input").map_err(|_| Error::FileNotFound)?;
    let map: Map = file.lines().collect();
    let map_width = map[0].len();

    let result = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .try_fold(1u64, |acc, &val| {
            Ok(acc * count_trees_for_slope(val, &map, map_width)? as u64)
        })?;

    println!("{}", result);
    Ok(())
}

fn count_trees_for_slope(slope: (usize, usize), map: &Map, map_width: usize) -> Result<u32, Error> {
    let mut pos = (0usize, 0usize);
    let mut count = 0;
    while pos.1 < map.len() {
        if map[pos.1].bytes().nth(pos.0).ok_or(Error::OutOfMap)? == '#' as u8 {
            count += 1;
        }

        pos = ((pos.0 + slope.0) % map_width, pos.1 + slope.1);
    }

    Ok(count)
}
