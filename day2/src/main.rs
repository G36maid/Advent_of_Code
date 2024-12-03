use std::fs::File;
use std::io::{self, BufRead};
//use std::path::Path;

fn is_safe_report(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return false; 
    }

    let diffs: Vec<i32> = report.windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    if diffs.iter().any(|&diff| diff.abs() < 1 || diff.abs() > 3) {
        return false; 
    }

    let increasing = diffs.iter().all(|&diff| diff > 0);
    let decreasing = diffs.iter().all(|&diff| diff < 0);

    increasing || decreasing 
}
fn can_be_safe_by_removing_one(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe_report(&modified_report) {
            return true;
        }
    }
    false
}
fn main() {
    let input_path = "input.txt";

    let file = File::open(&input_path).expect("Failed to open input file");
    let reader = io::BufReader::new(file);

    let reports: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| line.ok()) 
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Invalid number in file"))
                .collect()
        })
        .collect();

    let safe_count = reports.iter().filter(|report| can_be_safe_by_removing_one(report)).count();
    println!("Number of safe reports: {}", safe_count);
}
