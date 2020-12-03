#[derive(Debug)]
enum Error {
    FileNotFound,
    OutOfMap,
}

fn main() -> Result<(), Error> {
    let file = std::fs::read_to_string("input").map_err(|_| Error::FileNotFound)?;
    let map: Vec<&str> = file.lines().collect();
    let map_width = map[0].len();
    let mut pos = (0usize, 0usize);
    let mut count = 0;
    while pos.1 != map.len() {
        if map[pos.1].bytes().nth(pos.0).ok_or(Error::OutOfMap)? == '#' as u8 {
            count += 1;
        }

        pos = ((pos.0 + 3) % map_width, pos.1 + 1);
    }
    println!("{}", count);
    Ok(())
}
