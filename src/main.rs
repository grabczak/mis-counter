use std::fs::File;
use std::io::{self, BufRead};

fn read_file(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut data: Vec<Vec<i32>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let numbers: Vec<i32> = line
            .split(' ')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        data.push(numbers);
    }

    Ok(data)
}

fn print_tree(data: &[Vec<i32>]) {
    println!("Tree adjacency list:");
    for row in data {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}

fn main() {
    println!("Enter the CSV file name:");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read input");

    let filename = filename.trim();

    match read_file(filename) {
        Ok(data) => print_tree(&data),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
