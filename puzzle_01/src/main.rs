use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let nums: Vec<i64> = BufReader::new(file).lines().into_iter().map(|l| {
        l.ok().and_then(|s| s.parse().ok()).unwrap_or(0)
    }).collect();

    let sum: i64 = nums.iter().sum();

    println!("{}", sum);
}
