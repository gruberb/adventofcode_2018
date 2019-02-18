use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::{HashMap,HashSet};

fn main() -> Result<(), Error> {
    let vec = read(File::open("./src/input.txt")?)?;
    let mut grid: HashMap<String, String> = HashMap::new();
    let mut fabric: HashMap<String, String> = HashMap::new();
    let mut ids: HashSet<String> = HashSet::new();

    for entry in vec.iter() {
        let at_sign = entry.find('@');
        let colon = entry.find(':');
        let id = entry.get(1..(at_sign).unwrap()-1).unwrap();
        let coord_start = at_sign.unwrap() + 2;
        let coord_end = colon.unwrap();

        let size_start = coord_end + 2;

        let coordinates: Vec<&str> = entry
            .get(coord_start..coord_end)
            .unwrap()
            .split(',')
            .collect();

        let dimensions: Vec<&str> = entry
            .get(size_start..)
            .unwrap()
            .split('x')
            .collect();

        let left = coordinates[0].parse::<i32>().unwrap();
        let top = coordinates[1].parse::<i32>().unwrap();

        let fabric_wide = dimensions[0].parse::<i32>().unwrap();
        let fabric_tall = dimensions[1].parse::<i32>().unwrap();
        
        ids.insert(id.to_string());

        for wide in 0..fabric_wide {
            for tall in 0..fabric_tall {
                let x = left + wide;
                let y = top + tall;
                let coord = format!("{},{}", x, y);
                
                if grid.contains_key(&coord) {
                    ids.remove(grid.get(&coord).unwrap());
                    ids.remove(&id.to_string());
                    fabric.insert(format!("{},{}", x, y), String::from(id));
                } 
                
                grid.insert(format!("{},{}", x, y), String::from(id));
            }
        }
    }
    
    println!("{:?}", fabric.len());
    println!("{:?}", ids);
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
