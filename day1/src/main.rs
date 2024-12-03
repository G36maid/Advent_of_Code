use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = "input.txt";

    // Open the file
    let file = File::open(&path).expect("Unable to open input file");
    let reader = io::BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Read each line and parse the numbers
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut numbers = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number in file"));

        let left = numbers.next().expect("Missing left number");
        let right = numbers.next().expect("Missing right number");

        left_list.push(left);
        right_list.push(right);
    }

    // Create a frequency map for the right list
    let mut right_map: HashMap<i32, i32> = HashMap::new();
    for &num in &right_list {
        *right_map.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let mut similarity_score = 0;
    for &num in &left_list {
        if let Some(&count) = right_map.get(&num) {
            similarity_score += num * count;
        }
    }

    println!("The similarity score is: {}", similarity_score);
}
