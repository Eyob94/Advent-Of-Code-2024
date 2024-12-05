use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("./input.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut safe = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };

        let elements = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect::<Vec<i64>>();

        if elements.len() < 3 {
            continue;
        }

        if is_safe(&elements) || can_be_made_safe(&elements) {
            safe += 1;
        }
    }

    println!("{safe}");
}

fn is_safe(elements: &[i64]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..elements.len() {
        let diff = elements[i] - elements[i - 1];
        if !is_valid_number(diff) {
            return false; 
        }
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn can_be_made_safe(elements: &[i64]) -> bool {
    for i in 0..elements.len() {
        let mut modified_elements = elements.to_vec();
        modified_elements.remove(i); 
        if is_safe(&modified_elements) {
            return true; 
        }
    }
    false
}

fn is_valid_number(num: i64) -> bool {
    (1..=3).contains(&num) || (-3..=-1).contains(&num)
}
