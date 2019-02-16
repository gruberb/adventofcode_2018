use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let vec = read(File::open("./src/input.txt")?)?;
    let mut map: HashMap<char, u32> = HashMap::new();
    let mut two = 0;
    let mut three = 0;
    
    for word in vec.iter() {
        map.drain();
        for ch in word.chars() {
            map.entry(ch)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);
        }

        let mut ctwo = 0;
        let mut cthree = 0;
        for (_key, value) in map.iter() {
            if *value == 2 {
                ctwo += 1;
            }

            if *value == 3 {
                cthree += 1;
            }
        }

        if ctwo != 0 {
            two += 1;
        }

        if cthree != 0 {
            three += 1;
        }
    }

    let result = two * three;
    println!("{}", result);
    println!("{}", find_differ(vec));
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

fn find_differ(words: Vec<String>) -> String {
    let mut differ = Vec::new();
    let mut result = String::from("");

    'outer: for word in words.iter() {
        for w in words.iter() {
            let mut differ_amount = 0;
            differ.drain(..);
            for (c1, c2) in word.chars().zip(w.chars()) {
                if c1 != c2 {
                    differ_amount += 1;
                } else {
                    differ.push(c1);
                }
            }

            if differ_amount == 1 {
                for x in differ.iter() {
                    result.push(*x);
                }
                break 'outer;
            }
        }
    }

    return result
}
