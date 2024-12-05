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


    let sum: u32 = left.iter().map(|&l| l * right.get_count(l) as u32).sum();

    println!("{sum}")
}

trait GetElementCount<T> {
    fn get_count(&self, n: T) -> usize;
}

impl<T: Sized + Ord> GetElementCount<T> for Vec<T> {
    fn get_count(&self, n: T) -> usize {
        self.iter().filter(|&x| x == &n).count()
    }
}
