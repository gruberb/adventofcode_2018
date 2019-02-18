use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let vec = read(File::open("./src/input.txt")?)?;
    let first: HashMap<u32, u32> = HashMap::new();
    let more: HashMap<u32, u32> = HashMap::new();

    for entry in vec.iter() {
        let at_sign = entry.find('@');
        let colon = entry.find(':');

        let coord_start = at_sign.unwrap() + 2;
        let coord_end = colon.unwrap();

        let size_start = coord_end + 2;

        let coordinates: Vec<&str> = entry
            .get(coord_start..coord_end)
            .unwrap()
            .split(',')
            .collect();

        let fabric: Vec<&str> = entry
            .get(size_start..)
            .unwrap()
            .split('x')
            .collect();

        let left = coordinates[0].parse::<i32>().unwrap();
        let top = coordinates[1].parse::<i32>().unwrap();

        let fabric_wide = fabric[0].parse::<i32>().unwrap();
        let fabric_tall = fabric[1].parse::<i32>().unwrap();
        
        for wide in 0..fabric_wide {
            for tall in 0..fabric_tall {
                

            }
        }

        println!("{:?}, {:?}", left, top);
    }
    Ok(())
}

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }

    Ok(v)
}
