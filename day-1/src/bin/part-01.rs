use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input-1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some((left_str, right_str)) = line.split_once("   ") {
            left.push(left_str.parse::<u32>().unwrap());
            right.push(right_str.parse::<u32>().unwrap());
        }
    }

    left.sort();
    right.sort();

    let sum: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("{sum}")
}
