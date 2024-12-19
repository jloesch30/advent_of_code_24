use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;

pub fn part_1() -> Result<String, io::Error> {
    let path = "src/day_1/input/puzzle_input.txt";
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        column1.push(parts[0].parse::<i32>().unwrap());
        column2.push(parts[1].parse::<i32>().unwrap());
    }

    // sort the columns respectively
    column1.sort();
    column2.sort();

    // measure the distance between each item in the arrays
    let zipped = zip(column1, column2);
    let mut sum = 0;
    for (a, b) in zipped {
        sum += (a - b).abs();
    }

    Ok(sum.to_string())
}
