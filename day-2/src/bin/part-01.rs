use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut safe = 0;
    'outer: for line in reader.lines() {
        let line = line.unwrap();
        let elements = line
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut i = 1;

        while i < elements.len() - 1 {
            let d1 = elements[i] - elements[i - 1];
            let d2 = elements[i + 1] - elements[i];

            if d1 * d2 < 0 || !is_valid_number(d1) || !is_valid_number(d2) {
                continue 'outer;
            }

            if i == elements.len() - 2 {
                safe += 1;
            }

            i += 1;
        }
    }

    println!("{safe}")
}

fn is_valid_number(num: i64) -> bool {
    (1..=3).contains(&num) || (-3..=-1).contains(&num)
}
