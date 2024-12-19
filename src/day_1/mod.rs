use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::zip;

fn read_file() -> io::Result<BufReader<File>> {
    let path = "src/day_1/input/puzzle_input.txt";
    let file = File::open(path)?;

    Ok(io::BufReader::new(file))
}

pub fn part_1() -> Result<String, io::Error> {
    let reader = read_file()?;

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

// We need to find the similarity score of the left list with the right list
// this can be achieved by creating a hash map of the right list with the number and how many times
// it appears, and then iterate through the left list and multiply the value with how many
// occurrences from the hash map
pub fn part_2() -> Result<String, io::Error> {
    let reader = read_file()?;

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        column1.push(parts[0].parse::<i32>().unwrap());
        column2.push(parts[1].parse::<i32>().unwrap());
    }

    // Create a hash map of the right list
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for num in column2 {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    // For each value in the left column, we need to multiply the value with the number of
    // occurrences
    
    let mut sum = 0;
    for num in column1 {
        if let Some(value) = hash_map.get(&num) {
            sum += num * value;
        } 
    }
    Ok(sum.to_string())
}
