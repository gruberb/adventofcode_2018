use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let file = File::open("input.txt").unwrap();

    let nums: Vec<i64> = BufReader::new(file).lines().into_iter().map(|l| {
        l.ok().and_then(|s| s.parse().ok()).unwrap_or(0)
    }).collect();

    let sum: i64 = nums.iter().sum();
    println!("{}", sum);
   
    part_two(&nums); 
}

fn part_two(nums: &Vec<i64>) {
    
    let mut seen = HashSet::new();
    seen.insert(0);

    let mut cur_sum = 0;
   
    for val in nums.into_iter().cycle() {
        cur_sum += val;
        if seen.contains(&cur_sum) {
            println!("FOUND: {}", &cur_sum);
            break 
        }
            
        seen.insert(cur_sum);
    }
}
